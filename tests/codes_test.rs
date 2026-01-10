use orion_error::ErrorCode;
use std::marker::PhantomData;
use wp_connector_api::{SinkReason, SourceReason};
use wp_err::{
    codes::SysErrorCode,
    config_error::{ConfCore, ConfReason},
    parse_error::OMLCodeReason,
    run_error::{DistFocus, RunReason, SourceFocus},
};

#[test]
fn test_sys_codes_conf_and_parse() {
    let c = ConfReason::<ConfCore>::Syntax("bad".into());
    assert_eq!(c.sys_code(), 42201);
    assert_eq!(c.error_code(), 42201);
    let c = ConfReason::<ConfCore>::NotFound("x".into());
    assert_eq!(c.sys_code(), 40401);
    assert_eq!(c.error_code(), 40401);

    let o = OMLCodeReason::Syntax("e".into());
    assert_eq!(o.sys_code(), 42211);
}

#[test]
fn test_sys_codes_source_and_run() {
    let s = SourceReason::NotData;
    assert_eq!(s.sys_code(), 20401);
    let s = SourceReason::Disconnect("net".into());
    assert_eq!(s.sys_code(), 49901);
    let s = SourceReason::Other("weird".into());
    assert_eq!(s.sys_code(), 50209);

    let r = RunReason::Dist(DistFocus::StgCtrl);
    assert_eq!(r.sys_code(), 50311);
    let r = RunReason::Source(SourceFocus::NoData);
    assert_eq!(r.sys_code(), 20401);
    let r = RunReason::Source(SourceFocus::Other("x".into()));
    assert_eq!(r.sys_code(), 50209);
}

#[test]
fn test_conf_take_propagation() {
    // Ensure _Take can be converted without panic
    let take = ConfReason::<ConfCore>::_Take(PhantomData);
    let _f: ConfReason<wp_err::config_error::ConfFeature> = take.clone().into();
    let _d: ConfReason<wp_err::config_error::ConfDynamic> = take.into();
}

#[test]
fn test_dist_mock_to_run() {
    // SinkReason::Mock should map to RunReason::Dist(StgCtrl)
    let rr: RunReason = SinkReason::Mock.into();
    assert_eq!(rr.sys_code(), 50311);
}

#[test]
fn test_disconnect_message_keeps_detail() {
    let rr = RunReason::Source(SourceFocus::Disconnect("net down".into()));
    assert!(rr.to_string().contains("net down"));
}

#[test]
fn test_source_other_preserves_code() {
    let rr: RunReason = SourceReason::Other("strange".into()).into();
    assert_eq!(rr.sys_code(), 50209);
}

#[test]
fn test_robust_mode_from_str_fallback() {
    use wp_err::error_handling::RobustnessMode;
    // unknown should fallback to Debug instead of panic
    let m: RobustnessMode = "unknown-mode".into();
    assert_eq!(format!("{}", m), "debug");
}
