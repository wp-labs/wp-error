use derive_more::From;
use orion_error::{OrionError, StructError, UnifiedReason};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, From, OrionError)]
pub enum OMLCodeReason {
    #[orion_error(identity = "biz.syntax")]
    Syntax(String),
    #[orion_error(identity = "biz.not_found")]
    #[from(skip)]
    NotFound(String),
    #[orion_error(transparent)]
    Uvs(UnifiedReason),
}

pub type OMLCodeError = StructError<OMLCodeReason>;

pub type OMLCodeResult<T> = Result<T, OMLCodeError>;

#[derive(Error, Debug, PartialEq)]
pub enum DataErrKind {
    #[error("format error : {0}\n{1:?} ")]
    FormatError(String, Option<String>),
    #[error("not complete")]
    NotComplete,
    #[error("no parse data: {0}")]
    UnParse(String),

    #[error("less data")]
    LessData,
    #[error("empty data")]
    EmptyData,
    #[error("struct less : {0}")]
    LessStc(String),
    #[error("define less : {0}")]
    LessDef(String),
}
impl From<DataErrKind> for OMLCodeReason {
    fn from(_: DataErrKind) -> Self {
        OMLCodeReason::data_error()
    }
}
pub type OmlCodeResult<T> = Result<T, OMLCodeError>;

impl From<OMLCodeReason> for UnifiedReason {
    fn from(_: OMLCodeReason) -> Self {
        UnifiedReason::resource_error()
    }
}
