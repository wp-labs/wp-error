use derive_more::From;
use orion_error::{OrionError, StructError, UnifiedReason};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, From, OrionError)]
pub enum KnowledgeReason {
    #[orion_error(identity = "biz.not_data", message = "not data")]
    NotData,
    #[orion_error(transparent)]
    Uvs(UnifiedReason),
}

pub type KnowledgeError = StructError<KnowledgeReason>;
pub type KnowledgeResult<T> = Result<T, StructError<KnowledgeReason>>;
