mod strategy;
pub mod target;

pub use strategy::{
    ErrorHandlingStrategy, RobustnessMode, switch_sys_robust_mode, sys_robust_mode,
};
