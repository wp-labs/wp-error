use orion_error::ErrStrategy;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Formatter,
    sync::atomic::{AtomicU8, Ordering},
};
use wp_log::{error_ctrl, warn_ctrl};

pub enum ErrorHandlingStrategy {
    FixRetry,
    Tolerant,
    Throw,
    Ignore,
    Terminate,
}

impl From<ErrStrategy> for ErrorHandlingStrategy {
    fn from(value: ErrStrategy) -> Self {
        match value {
            ErrStrategy::Retry => ErrorHandlingStrategy::FixRetry,
            ErrStrategy::Ignore => ErrorHandlingStrategy::Ignore,
            ErrStrategy::Throw => ErrorHandlingStrategy::Throw,
        }
    }
}

static ROBUST_MODE: AtomicU8 = AtomicU8::new(0);
//static mut ROBUST_MODE: RobustnessMode = RobustnessMode::Debug;

pub fn sys_robust_mode() -> RobustnessMode {
    match ROBUST_MODE.load(Ordering::SeqCst) {
        0 => RobustnessMode::Debug,
        1 => RobustnessMode::Normal,
        2 => RobustnessMode::Strict,
        _ => unreachable!("Invalid robustness mode"),
    }
}

pub fn switch_sys_robust_mode(mode: RobustnessMode) -> RobustnessMode {
    let old = sys_robust_mode();
    let new_mode = match mode {
        RobustnessMode::Debug => 0,
        RobustnessMode::Normal => 1,
        RobustnessMode::Strict => 2,
    };
    // runtime-level change is part of engine control plane
    warn_ctrl!("switch robust mode from {} to {}", old, mode);
    ROBUST_MODE.store(new_mode, Ordering::SeqCst);
    old
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize, Clone)]
pub enum RobustnessMode {
    #[default]
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "strict")]
    Strict,
}

impl From<&str> for RobustnessMode {
    fn from(s: &str) -> Self {
        match s {
            "debug" => RobustnessMode::Debug,
            "normal" => RobustnessMode::Normal,
            "strict" => RobustnessMode::Strict,
            other => {
                // Fallback to debug to avoid panic in production; log a warning.
                error_ctrl!("unknown robust mode '{}', fallback to 'debug'", other);
                RobustnessMode::Debug
            }
        }
    }
}

impl std::fmt::Display for RobustnessMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RobustnessMode::Debug => write!(f, "debug"),
            RobustnessMode::Normal => write!(f, "normal"),
            RobustnessMode::Strict => write!(f, "strict"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robustness_mode_default() {
        let mode = RobustnessMode::default();
        assert_eq!(mode, RobustnessMode::Debug);
    }

    #[test]
    fn test_robustness_mode_from_str_debug() {
        let mode = RobustnessMode::from("debug");
        assert_eq!(mode, RobustnessMode::Debug);
    }

    #[test]
    fn test_robustness_mode_from_str_normal() {
        let mode = RobustnessMode::from("normal");
        assert_eq!(mode, RobustnessMode::Normal);
    }

    #[test]
    fn test_robustness_mode_from_str_strict() {
        let mode = RobustnessMode::from("strict");
        assert_eq!(mode, RobustnessMode::Strict);
    }

    #[test]
    fn test_robustness_mode_from_str_unknown() {
        let mode = RobustnessMode::from("unknown");
        assert_eq!(mode, RobustnessMode::Debug); // fallback
    }

    #[test]
    fn test_robustness_mode_display() {
        assert_eq!(RobustnessMode::Debug.to_string(), "debug");
        assert_eq!(RobustnessMode::Normal.to_string(), "normal");
        assert_eq!(RobustnessMode::Strict.to_string(), "strict");
    }

    #[test]
    fn test_robustness_mode_serde() {
        let json = "\"debug\"";
        let mode: RobustnessMode = serde_json::from_str(json).unwrap();
        assert_eq!(mode, RobustnessMode::Debug);

        let json = "\"normal\"";
        let mode: RobustnessMode = serde_json::from_str(json).unwrap();
        assert_eq!(mode, RobustnessMode::Normal);

        let json = "\"strict\"";
        let mode: RobustnessMode = serde_json::from_str(json).unwrap();
        assert_eq!(mode, RobustnessMode::Strict);
    }

    #[test]
    fn test_robustness_mode_serialize() {
        assert_eq!(
            serde_json::to_string(&RobustnessMode::Debug).unwrap(),
            "\"debug\""
        );
        assert_eq!(
            serde_json::to_string(&RobustnessMode::Normal).unwrap(),
            "\"normal\""
        );
        assert_eq!(
            serde_json::to_string(&RobustnessMode::Strict).unwrap(),
            "\"strict\""
        );
    }

    #[test]
    fn test_robustness_mode_clone() {
        let mode = RobustnessMode::Normal;
        let cloned = mode.clone();
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_error_handling_strategy_from_err_strategy() {
        let retry: ErrorHandlingStrategy = ErrStrategy::Retry.into();
        matches!(retry, ErrorHandlingStrategy::FixRetry);

        let ignore: ErrorHandlingStrategy = ErrStrategy::Ignore.into();
        matches!(ignore, ErrorHandlingStrategy::Ignore);

        let throw: ErrorHandlingStrategy = ErrStrategy::Throw.into();
        matches!(throw, ErrorHandlingStrategy::Throw);
    }
}
