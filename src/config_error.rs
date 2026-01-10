use std::marker::PhantomData;

use derive_more::From;
use orion_error::{ConfErrReason, ErrorCode, StructError, UvsConfFrom, UvsReason};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfCore {}
#[derive(Debug, Clone, PartialEq)]
pub struct ConfFeature {}
#[derive(Debug, Clone, PartialEq)]
pub struct ConfDynamic {}

#[derive(Debug, Clone)]
pub enum ConfType {
    Core,
    Feature,
    Dynamic,
}

#[derive(Error, Debug, Clone, PartialEq, Serialize, From)]
pub enum ConfReason<T>
where
    T: Clone + PartialEq,
{
    #[error("syntax err:{0}")]
    Syntax(String),
    #[from(skip)]
    #[error("not found: {0}")]
    NotFound(String),
    #[error("{0}")]
    Uvs(UvsReason),
    #[error("_")]
    _Take(PhantomData<T>),
}
impl ErrorCode for ConfReason<ConfCore> {
    fn error_code(&self) -> i32 {
        crate::codes::SysErrorCode::sys_code(self) as i32
    }
}

impl ErrorCode for ConfReason<ConfFeature> {
    fn error_code(&self) -> i32 {
        crate::codes::SysErrorCode::sys_code(self) as i32
    }
}

impl ErrorCode for ConfReason<ConfDynamic> {
    fn error_code(&self) -> i32 {
        crate::codes::SysErrorCode::sys_code(self) as i32
    }
}

pub type ConfError = StructError<ConfReason<ConfCore>>;

pub type FeatureConfError = StructError<ConfReason<ConfFeature>>;

pub type DynamicConfError = StructError<ConfReason<ConfDynamic>>;
pub type DynamicConfReason = ConfReason<ConfDynamic>;
pub type CoreConfReason = ConfReason<ConfCore>;

pub type ConfResult<T> = Result<T, ConfError>;
pub type DynConfResult<T> = Result<T, DynamicConfError>;
pub type FeatureConfResult<T> = Result<T, FeatureConfError>;

impl From<ConfReason<ConfCore>> for UvsReason {
    fn from(e: ConfReason<ConfCore>) -> Self {
        let error = format!("{}", e);
        UvsReason::from_conf(ConfErrReason::Core(error))
    }
}
impl From<ConfReason<ConfFeature>> for UvsReason {
    fn from(e: ConfReason<ConfFeature>) -> Self {
        let error = format!("{}", e);
        UvsReason::feature_conf(error)
    }
}
impl From<ConfReason<ConfDynamic>> for UvsReason {
    fn from(e: ConfReason<ConfDynamic>) -> Self {
        let error = format!("{}", e);
        UvsReason::dynamic_conf(error)
    }
}

impl From<ConfReason<ConfCore>> for ConfReason<ConfFeature> {
    fn from(value: ConfReason<ConfCore>) -> Self {
        match value {
            ConfReason::Syntax(v) => ConfReason::<ConfFeature>::Syntax(v),
            ConfReason::NotFound(v) => ConfReason::<ConfFeature>::NotFound(v),
            ConfReason::Uvs(uvs) => ConfReason::Uvs(uvs),
            // propagate phantom to keep type-level marker consistent
            ConfReason::_Take(_) => ConfReason::<ConfFeature>::_Take(PhantomData),
        }
    }
}

impl From<ConfReason<ConfCore>> for ConfReason<ConfDynamic> {
    fn from(value: ConfReason<ConfCore>) -> Self {
        match value {
            ConfReason::Syntax(v) => ConfReason::<ConfDynamic>::Syntax(v),
            ConfReason::NotFound(v) => ConfReason::<ConfDynamic>::NotFound(v),
            ConfReason::_Take(_) => ConfReason::<ConfDynamic>::_Take(PhantomData),
            ConfReason::Uvs(uvs) => ConfReason::Uvs(uvs),
        }
    }
}
