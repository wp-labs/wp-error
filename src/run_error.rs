use crate::parse_error::OMLCodeReason;
use crate::{ConfReason, config_error::ConfCore};
use derive_more::From;

use orion_error::{OrionError, StructError, UnifiedReason};
use orion_sec::OrionSecReason;
use serde::Serialize;
use thiserror::Error;

pub type RunError = StructError<RunReason>;
pub type RunResult<T> = Result<T, RunError>;

#[derive(Debug, Error, PartialEq, Serialize, From)]
pub enum DistFocus {
    #[error("sink error : {0}")]
    SinkError(String),
    #[error("stg-ctrl")]
    StgCtrl,
}
#[derive(Debug, Error, PartialEq, Serialize)]
pub enum SourceFocus {
    #[error("no data")]
    NoData,
    #[error("eof")]
    Eof,
    #[error("supplier error : {0}")]
    SupplierError(String),
    #[error("other : {0}")]
    Other(String),
    #[from(skip)]
    #[error("disconnect : {0}")]
    Disconnect(String),
}

#[derive(Debug, PartialEq, Serialize, From, OrionError)]
pub enum RunReason {
    #[orion_error(identity = "biz.dist")]
    Dist(DistFocus),
    #[orion_error(identity = "biz.source")]
    Source(SourceFocus),
    #[orion_error(transparent)]
    Uvs(UnifiedReason),
}

impl From<ConfReason<ConfCore>> for RunReason {
    fn from(_: ConfReason<ConfCore>) -> Self {
        Self::core_conf()
    }
}
impl From<OMLCodeReason> for RunReason {
    fn from(_: OMLCodeReason) -> Self {
        Self::core_conf()
    }
}

impl From<OrionSecReason> for RunReason {
    fn from(value: OrionSecReason) -> Self {
        match value {
            OrionSecReason::Sec(_) => Self::core_conf(),
            OrionSecReason::General(uvs_reason) => Self::Uvs(uvs_reason),
        }
    }
}

pub trait RunErrorOwe<T> {
    fn owe_sink(self) -> RunResult<T>;
    fn owe_source(self) -> RunResult<T>;
}

impl<T, E> RunErrorOwe<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn owe_sink(self) -> RunResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(RunReason::Dist(DistFocus::SinkError("sink fail".into()))
                .to_err()
                .with_detail(e.to_string())),
        }
    }
    fn owe_source(self) -> RunResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(
                RunReason::Source(SourceFocus::SupplierError("source fail".into()))
                    .to_err()
                    .with_detail(e.to_string()),
            ),
        }
    }
}

/*

pub trait Option2RunResult<T> {
    fn owe_logic(self, msg: &str) -> RunResult<T>;
}

impl<T> Option2RunResult<T> for Option<T> {
    fn owe_logic(self, msg: &str) -> RunResult<T> {
        self.ok_or(RunError::from(UvsReason::from_logic(msg.to_string())))
    }
}

*/
impl From<SourceReason> for RunReason {
    fn from(e: SourceReason) -> Self {
        match e {
            SourceReason::NotData => Self::Source(SourceFocus::NoData),
            SourceReason::EOF => Self::Source(SourceFocus::Eof),
            SourceReason::SupplierError(info) => Self::Source(SourceFocus::SupplierError(info)),
            SourceReason::Disconnect(info) => Self::Source(SourceFocus::Disconnect(info)),
            SourceReason::Other(info) => Self::Source(SourceFocus::Other(info)),
            SourceReason::Uvs(uvs) => Self::Uvs(uvs),
        }
        //Self::Domain(RunReason::Source(e))
    }
}

impl From<SinkReason> for RunReason {
    fn from(e: SinkReason) -> Self {
        match e {
            SinkReason::Sink(info) => Self::Dist(DistFocus::SinkError(info)),
            // Map mock to stage control path for now (no panic in production paths)
            SinkReason::Mock => Self::Dist(DistFocus::StgCtrl),
            SinkReason::StgCtrl => Self::Dist(DistFocus::StgCtrl),
            SinkReason::Uvs(uvs) => Self::Uvs(uvs),
        }
    }
}
use orion_conf::ToStructError as _;
use orion_conf::error::ConfIOReason;
use wp_connector_api::{SinkReason, SourceReason};
impl From<ConfIOReason> for RunReason {
    fn from(value: ConfIOReason) -> Self {
        match value {
            ConfIOReason::Other(_) => RunReason::core_conf(),
            ConfIOReason::General(uvs) => RunReason::Uvs(uvs),
            ConfIOReason::NoFormatEnabled => RunReason::core_conf(),
        }
    }
}
