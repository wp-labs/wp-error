use crate::parse_error::OMLCodeReason;
use crate::{ConfReason, config_error::ConfCore};
use derive_more::From;
use orion_error::{ConfErrReason, ErrorCode, StructError, UvsConfFrom, UvsReason};
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

#[derive(Debug, Error, PartialEq, Serialize, From)]
pub enum RunReason {
    #[error("sink error {0}")]
    Dist(DistFocus),
    #[error("source error {0}")]
    Source(SourceFocus),
    #[error("{0}")]
    Uvs(UvsReason),
}

impl ErrorCode for RunReason {
    fn error_code(&self) -> i32 {
        crate::codes::SysErrorCode::sys_code(self) as i32
    }
}

impl From<ConfReason<ConfCore>> for RunReason {
    fn from(value: ConfReason<ConfCore>) -> Self {
        Self::Uvs(UvsReason::from_conf(ConfErrReason::Core(value.to_string())))
    }
}
impl From<OMLCodeReason> for RunReason {
    fn from(value: OMLCodeReason) -> Self {
        Self::Uvs(UvsReason::from_conf(value.to_string()))
    }
}

impl From<OrionSecReason> for RunReason {
    fn from(value: OrionSecReason) -> Self {
        match value {
            OrionSecReason::Sec(sec_reason) => {
                Self::Uvs(UvsReason::from_conf(sec_reason.to_string()))
            }
            OrionSecReason::Uvs(uvs_reason) => Self::Uvs(uvs_reason),
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
            Err(e) => Err(StructError::from(RunReason::Dist(DistFocus::SinkError(
                "sink fail".into(),
            )))
            .with_detail(e.to_string())),
        }
    }
    fn owe_source(self) -> RunResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(
                StructError::from(RunReason::Source(SourceFocus::SupplierError(
                    "source fail".into(),
                )))
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
use orion_conf::error::ConfIOReason;
use wp_connector_api::{SinkReason, SourceReason};
impl From<ConfIOReason> for RunReason {
    fn from(value: ConfIOReason) -> Self {
        match value {
            ConfIOReason::Other(msg) => RunReason::from_conf(msg),
            ConfIOReason::Uvs(uvs) => RunReason::Uvs(uvs),
            ConfIOReason::NoFormatEnabled => RunReason::from_conf("fmt unsupport"),
        }
    }
}
