use crate::config_error::{ConfCore, ConfError, ConfReason};
use crate::parse_error::{OMLCodeError, OMLCodeReason};
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

/// 将 OML 错误转换为 `RunError`，并保留解析详情。
///
/// `conv_err()` 仅按 reason 转换（`OMLCodeReason → RunReason`），
/// 而 OML 的富错误消息（`[path]/[where]/[error]`）存放在
/// `Syntax/NotFound` 的 inner String 中、不在 StructError 的 detail 字段里，
/// 因此转换后会被压缩为无 detail 的配置错误。
/// 本 trait 把 inner 消息提取为 detail 并保留 source，供加载层调用。
pub trait IntoRunError {
    fn into_run_err(self) -> RunError;
}

impl IntoRunError for OMLCodeError {
    fn into_run_err(self) -> RunError {
        let detail = match self.reason() {
            OMLCodeReason::Syntax(s) => s.clone(),
            OMLCodeReason::NotFound(s) => s.clone(),
            OMLCodeReason::Uvs(_) => self.to_string(),
        };
        RunReason::core_conf()
            .to_err()
            .with_detail(detail)
            .with_source(self)
    }
}

impl IntoRunError for ConfError {
    fn into_run_err(self) -> RunError {
        // 与 OML 同构：Syntax/NotFound 的内层消息才是真正的解析详情
        let detail = match self.reason() {
            ConfReason::Syntax(s) => s.clone(),
            ConfReason::NotFound(s) => s.clone(),
            ConfReason::Uvs(_) | ConfReason::_Take(_) => self.to_string(),
        };
        RunReason::core_conf()
            .to_err()
            .with_detail(detail)
            .with_source(self)
    }
}

impl IntoRunError for orion_conf::OrionConfError {
    fn into_run_err(self) -> RunError {
        // ConfIOReason::Other 携带底层解析错误（如 toml 语法错误），
        // 转换时若丢弃则错误提示只剩泛化的 "配置错误"
        let detail = match self.reason() {
            orion_conf::ConfIOReason::Other(s) => s.clone(),
            _ => self.to_string(),
        };
        RunReason::core_conf()
            .to_err()
            .with_detail(detail)
            .with_source(self)
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
        self.ok_or(RunError::from(UnifiedReason::from_logic(msg.to_string())))
    }
}

*/
impl From<SourceReason> for RunReason {
    fn from(e: SourceReason) -> Self {
        match e {
            SourceReason::NotData => Self::Source(SourceFocus::NoData),
            SourceReason::EOF => Self::Source(SourceFocus::Eof),
            SourceReason::SupplierError => Self::Source(SourceFocus::SupplierError(String::new())),
            SourceReason::Disconnect => Self::Source(SourceFocus::Disconnect(String::new())),
            SourceReason::Other => Self::Source(SourceFocus::Other(String::new())),
            SourceReason::Uvs(uvs) => Self::Uvs(uvs),
        }
        //Self::Domain(RunReason::Source(e))
    }
}

impl From<SinkReason> for RunReason {
    fn from(e: SinkReason) -> Self {
        match e {
            SinkReason::Sink => Self::Dist(DistFocus::SinkError(String::new())),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_error::OMLCodeReason;

    #[test]
    fn into_run_err_preserves_syntax_detail() {
        let err = OMLCodeReason::Syntax(":oml code parse fail!\n[where]: line 5".into()).to_err();
        let run = err.into_run_err();
        assert!(
            run.detail()
                .as_deref()
                .is_some_and(|d| d.contains("line 5")),
            "detail should carry the OML inner message, got {:?}",
            run.detail()
        );
    }

    #[test]
    fn into_run_err_keeps_config_reason() {
        let err = OMLCodeReason::NotFound("missing file".into()).to_err();
        let run = err.into_run_err();
        assert!(
            format!("{}", run.reason()).contains("config"),
            "reason should stay a config error, got {}",
            run.reason()
        );
    }

    #[test]
    fn conf_into_run_err_preserves_syntax_detail() {
        let err = ConfReason::<ConfCore>::Syntax("invalid [sources] block".into()).to_err();
        let run = err.into_run_err();
        assert!(
            run.detail()
                .as_deref()
                .is_some_and(|d| d.contains("invalid [sources] block")),
            "ConfError detail should carry the inner message, got {:?}",
            run.detail()
        );
    }

    #[test]
    fn conf_io_into_run_err_preserves_other_detail() {
        use orion_conf::ConfIOReason;
        let err = ConfIOReason::Other("expected `]` at line 2".into()).to_err();
        let run = err.into_run_err();
        assert!(
            run.detail()
                .as_deref()
                .is_some_and(|d| d.contains("expected `]`")),
            "ConfIOError detail should carry the toml error, got {:?}",
            run.detail()
        );
    }
}
