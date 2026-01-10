use derive_more::From;
use orion_error::{ErrorCode, StructError, UvsReason};
use serde::Serialize;
use thiserror::Error;
#[derive(Error, Debug, Clone, PartialEq, Serialize, From)]
pub enum KnowledgeReason {
    #[error("not data")]
    NotData,
    #[error("{0}")]
    Uvs(UvsReason),
}

impl ErrorCode for KnowledgeReason {
    fn error_code(&self) -> i32 {
        crate::codes::SysErrorCode::sys_code(self) as i32
    }
}

pub type KnowledgeError = StructError<KnowledgeReason>;
pub type KnowledgeResult<T> = Result<T, StructError<KnowledgeReason>>;
