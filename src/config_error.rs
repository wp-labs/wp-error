use std::fmt::Debug;
use std::marker::PhantomData;

use derive_more::From;

use orion_error::{OrionError, StructError, UnifiedReason};
use serde::Serialize;

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

#[derive(Debug, Clone, PartialEq, Serialize, From, OrionError)]
pub enum ConfReason<T: Clone + PartialEq + Debug + Send + Sync + 'static> {
    #[orion_error(identity = "conf.syntax")]
    Syntax(String),
    #[orion_error(identity = "conf.not_found")]
    #[from(skip)]
    NotFound(String),
    #[orion_error(transparent)]
    Uvs(UnifiedReason),
    #[orion_error(identity = "conf.take")]
    _Take(PhantomData<T>),
}

pub type ConfError = StructError<ConfReason<ConfCore>>;

pub type FeatureConfError = StructError<ConfReason<ConfFeature>>;

pub type DynamicConfError = StructError<ConfReason<ConfDynamic>>;
pub type DynamicConfReason = ConfReason<ConfDynamic>;
pub type CoreConfReason = ConfReason<ConfCore>;

pub type ConfResult<T> = Result<T, ConfError>;
pub type DynConfResult<T> = Result<T, DynamicConfError>;
pub type FeatureConfResult<T> = Result<T, FeatureConfError>;

impl From<ConfReason<ConfCore>> for UnifiedReason {
    fn from(_: ConfReason<ConfCore>) -> Self {
        UnifiedReason::core_conf()
    }
}
impl From<ConfReason<ConfFeature>> for UnifiedReason {
    fn from(_: ConfReason<ConfFeature>) -> Self {
        UnifiedReason::feature_conf()
    }
}
impl From<ConfReason<ConfDynamic>> for UnifiedReason {
    fn from(_: ConfReason<ConfDynamic>) -> Self {
        UnifiedReason::dynamic_conf()
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
