//! System-wide error code contract for wp-error.
//! Provide a single place to convert domain reasons into stable numeric codes.

/// Centralized plan for the 5-digit system error codes.
///
/// The first digit hints the HTTP mapping (2=NoContent, 4=Client, 5=Server), while the
/// middle two digits reserve blocks per domain (20=conf, 21=parse, 22=source, 23=dist,
/// 24=run, 25=knowledge, 26=security, etc.). The last two digits are scoped reasons.
/// Keeping these constants together makes future assignments deliberate and visible.
pub mod plan {
    pub const DEFAULT_TAG: &str = "wp.err";

    pub mod conf {
        pub mod shared {
            pub const TAKE: u16 = 50009;
        }

        pub mod core {
            pub const TAG: &str = "conf.core";
            pub const SYNTAX: u16 = 42201;
            pub const NOT_FOUND: u16 = 40401;
            pub const UVS: u16 = 50001;
        }

        pub mod feature {
            pub const TAG: &str = "conf.feature";
            pub const SYNTAX: u16 = 42202;
            pub const NOT_FOUND: u16 = 40402;
            pub const UVS: u16 = 50002;
        }

        pub mod dynamic {
            pub const TAG: &str = "conf.dynamic";
            pub const SYNTAX: u16 = 42203;
            pub const NOT_FOUND: u16 = 40403;
            pub const UVS: u16 = 50003;
        }
    }

    pub mod security {
        pub const SEC: u16 = 62001;
        pub const UVS: u16 = 50003;
    }

    pub mod parse {
        pub mod oml {
            pub const TAG: &str = "parse.oml";
            pub const SYNTAX: u16 = 42211;
            pub const NOT_FOUND: u16 = 40411;
            pub const UVS: u16 = 50011;
        }

        pub mod data {
            pub const TAG: &str = "parse.data";
            pub const FORMAT_ERROR: u16 = 42212;
            pub const NOT_COMPLETE: u16 = 42213;
            pub const UNPARSE: u16 = 40412;
            pub const LESS_DATA: u16 = 42214;
            pub const EMPTY_DATA: u16 = 42215;
            pub const LESS_STC: u16 = 42216;
            pub const LESS_DEF: u16 = 42217;
        }
    }

    pub mod source {
        pub const TAG: &str = "source";
        pub const NOT_DATA: u16 = 20401;
        pub const EOF: u16 = 20402;
        pub const SUPPLIER_ERROR: u16 = 50201;
        pub const DISCONNECT: u16 = 49901;
        pub const OTHER: u16 = 50209;
        pub const UVS: u16 = 50021;
    }

    pub mod dist {
        pub const TAG: &str = "dist";
        pub const SINK_ERROR: u16 = 50211;
        pub const STG_CTRL: u16 = 50311;
        pub const MOCK: u16 = 50312;
        pub const UVS: u16 = 50031;
    }

    pub mod knowledge {
        pub const TAG: &str = "knowledge";
        pub const UVS: u16 = 50041;
        pub const NOT_DATA: u16 = 50042;
    }

    pub mod run {
        pub const TAG: &str = "run";

        pub mod dist {
            pub use super::super::dist::{SINK_ERROR, STG_CTRL};
        }

        pub mod source {
            pub use super::super::source::{DISCONNECT, EOF, NOT_DATA, OTHER, SUPPLIER_ERROR};
        }

        pub const UVS: u16 = 50041;
    }
}

pub trait SysErrorCode {
    fn sys_code(&self) -> u16;
    fn sys_tag(&self) -> &'static str {
        plan::DEFAULT_TAG
    }
}

use orion_sec::OrionSecReason;
use wp_connector_api::{SinkReason, SourceReason};

// ----------------- Config -----------------
use crate::KnowledgeReason;
use crate::config_error::{ConfCore, ConfDynamic, ConfFeature, ConfReason};
use crate::parse_error::{DataErrKind, OMLCodeReason};

impl SysErrorCode for ConfReason<ConfCore> {
    fn sys_code(&self) -> u16 {
        match self {
            ConfReason::Syntax(_) => plan::conf::core::SYNTAX,
            ConfReason::NotFound(_) => plan::conf::core::NOT_FOUND,
            ConfReason::Uvs(_) => plan::conf::core::UVS,
            ConfReason::_Take(_) => plan::conf::shared::TAKE,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::conf::core::TAG
    }
}

impl SysErrorCode for ConfReason<ConfFeature> {
    fn sys_code(&self) -> u16 {
        match self {
            ConfReason::Syntax(_) => plan::conf::feature::SYNTAX,
            ConfReason::NotFound(_) => plan::conf::feature::NOT_FOUND,
            ConfReason::Uvs(_) => plan::conf::feature::UVS,
            ConfReason::_Take(_) => plan::conf::shared::TAKE,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::conf::feature::TAG
    }
}

impl SysErrorCode for ConfReason<ConfDynamic> {
    fn sys_code(&self) -> u16 {
        match self {
            ConfReason::Syntax(_) => plan::conf::dynamic::SYNTAX,
            ConfReason::NotFound(_) => plan::conf::dynamic::NOT_FOUND,
            ConfReason::Uvs(_) => plan::conf::dynamic::UVS,
            ConfReason::_Take(_) => plan::conf::shared::TAKE,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::conf::dynamic::TAG
    }
}

impl SysErrorCode for OrionSecReason {
    fn sys_code(&self) -> u16 {
        match self {
            OrionSecReason::Sec(_) => plan::security::SEC,
            OrionSecReason::Uvs(_) => plan::security::UVS,
        }
    }
}
// ----------------- Parse / OML -----------------
impl SysErrorCode for OMLCodeReason {
    fn sys_code(&self) -> u16 {
        match self {
            OMLCodeReason::Syntax(_) => plan::parse::oml::SYNTAX,
            OMLCodeReason::NotFound(_) => plan::parse::oml::NOT_FOUND,
            OMLCodeReason::Uvs(_) => plan::parse::oml::UVS,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::parse::oml::TAG
    }
}

impl SysErrorCode for DataErrKind {
    fn sys_code(&self) -> u16 {
        match self {
            DataErrKind::FormatError(_, _) => plan::parse::data::FORMAT_ERROR,
            DataErrKind::NotComplete => plan::parse::data::NOT_COMPLETE,
            DataErrKind::UnParse(_) => plan::parse::data::UNPARSE,
            DataErrKind::LessData => plan::parse::data::LESS_DATA,
            DataErrKind::EmptyData => plan::parse::data::EMPTY_DATA,
            DataErrKind::LessStc(_) => plan::parse::data::LESS_STC,
            DataErrKind::LessDef(_) => plan::parse::data::LESS_DEF,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::parse::data::TAG
    }
}

// ----------------- Source -----------------
impl SysErrorCode for SourceReason {
    fn sys_code(&self) -> u16 {
        match self {
            SourceReason::NotData => plan::source::NOT_DATA,
            SourceReason::EOF => plan::source::EOF,
            SourceReason::SupplierError(_) => plan::source::SUPPLIER_ERROR,
            SourceReason::Disconnect(_) => plan::source::DISCONNECT,
            SourceReason::Other(_) => plan::source::OTHER,
            SourceReason::Uvs(_) => plan::source::UVS,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::source::TAG
    }
}

// ----------------- Dist -----------------
impl SysErrorCode for SinkReason {
    fn sys_code(&self) -> u16 {
        match self {
            SinkReason::Sink(_) => plan::dist::SINK_ERROR,
            SinkReason::Mock => plan::dist::MOCK,
            SinkReason::StgCtrl => plan::dist::STG_CTRL,
            SinkReason::Uvs(_) => plan::dist::UVS,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::dist::TAG
    }
}

// ----------------- Dist -----------------
impl SysErrorCode for KnowledgeReason {
    fn sys_code(&self) -> u16 {
        match self {
            KnowledgeReason::Uvs(_) => plan::knowledge::UVS,
            KnowledgeReason::NotData => plan::knowledge::NOT_DATA,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::knowledge::TAG
    }
}

// ----------------- Run (aggregate) -----------------
use crate::run_error::{DistFocus, RunReason as RR, SourceFocus};
impl SysErrorCode for RR {
    fn sys_code(&self) -> u16 {
        match self {
            RR::Dist(DistFocus::SinkError(_)) => plan::run::dist::SINK_ERROR,
            RR::Dist(DistFocus::StgCtrl) => plan::run::dist::STG_CTRL,
            RR::Source(SourceFocus::NoData) => plan::run::source::NOT_DATA,
            RR::Source(SourceFocus::Eof) => plan::run::source::EOF,
            RR::Source(SourceFocus::SupplierError(_)) => plan::run::source::SUPPLIER_ERROR,
            RR::Source(SourceFocus::Other(_)) => plan::run::source::OTHER,
            RR::Source(SourceFocus::Disconnect(_)) => plan::run::source::DISCONNECT,
            RR::Uvs(_) => plan::run::UVS,
        }
    }
    fn sys_tag(&self) -> &'static str {
        plan::run::TAG
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orion_error::{UvsLogicFrom, UvsReason};
    use std::marker::PhantomData;

    #[test]
    fn test_sys_error_code_default_tag() {
        struct DummyReason;
        impl SysErrorCode for DummyReason {
            fn sys_code(&self) -> u16 {
                12345
            }
        }
        let dummy = DummyReason;
        assert_eq!(dummy.sys_tag(), "wp.err");
        assert_eq!(dummy.sys_code(), 12345);
    }

    // ConfReason<ConfCore> tests
    #[test]
    fn test_conf_core_syntax_code() {
        let reason: ConfReason<ConfCore> = ConfReason::Syntax("test".into());
        assert_eq!(reason.sys_code(), 42201);
        assert_eq!(reason.sys_tag(), "conf.core");
    }

    #[test]
    fn test_conf_core_not_found_code() {
        let reason: ConfReason<ConfCore> = ConfReason::NotFound("missing".into());
        assert_eq!(reason.sys_code(), 40401);
    }

    #[test]
    fn test_conf_core_uvs_code() {
        let reason: ConfReason<ConfCore> =
            ConfReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50001);
    }

    #[test]
    fn test_conf_core_take_code() {
        let reason: ConfReason<ConfCore> = ConfReason::_Take(PhantomData);
        assert_eq!(reason.sys_code(), 50009);
    }

    // ConfReason<ConfFeature> tests
    #[test]
    fn test_conf_feature_syntax_code() {
        let reason: ConfReason<ConfFeature> = ConfReason::Syntax("test".into());
        assert_eq!(reason.sys_code(), 42202);
        assert_eq!(reason.sys_tag(), "conf.feature");
    }

    #[test]
    fn test_conf_feature_not_found_code() {
        let reason: ConfReason<ConfFeature> = ConfReason::NotFound("missing".into());
        assert_eq!(reason.sys_code(), 40402);
    }

    #[test]
    fn test_conf_feature_uvs_code() {
        let reason: ConfReason<ConfFeature> =
            ConfReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50002);
    }

    // ConfReason<ConfDynamic> tests
    #[test]
    fn test_conf_dynamic_syntax_code() {
        let reason: ConfReason<ConfDynamic> = ConfReason::Syntax("test".into());
        assert_eq!(reason.sys_code(), 42203);
        assert_eq!(reason.sys_tag(), "conf.dynamic");
    }

    #[test]
    fn test_conf_dynamic_not_found_code() {
        let reason: ConfReason<ConfDynamic> = ConfReason::NotFound("missing".into());
        assert_eq!(reason.sys_code(), 40403);
    }

    #[test]
    fn test_conf_dynamic_uvs_code() {
        let reason: ConfReason<ConfDynamic> =
            ConfReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50003);
    }

    // OMLCodeReason tests
    #[test]
    fn test_oml_syntax_code() {
        let reason = OMLCodeReason::Syntax("parse error".into());
        assert_eq!(reason.sys_code(), 42211);
        assert_eq!(reason.sys_tag(), "parse.oml");
    }

    #[test]
    fn test_oml_not_found_code() {
        let reason = OMLCodeReason::NotFound("file.oml".into());
        assert_eq!(reason.sys_code(), 40411);
    }

    #[test]
    fn test_oml_uvs_code() {
        let reason = OMLCodeReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50011);
    }

    // DataErrKind tests
    #[test]
    fn test_data_err_format_error_code() {
        let reason = DataErrKind::FormatError("bad format".into(), None);
        assert_eq!(reason.sys_code(), 42212);
        assert_eq!(reason.sys_tag(), "parse.data");
    }

    #[test]
    fn test_data_err_not_complete_code() {
        let reason = DataErrKind::NotComplete;
        assert_eq!(reason.sys_code(), 42213);
    }

    #[test]
    fn test_data_err_unparse_code() {
        let reason = DataErrKind::UnParse("unparseable".into());
        assert_eq!(reason.sys_code(), 40412);
    }

    #[test]
    fn test_data_err_less_data_code() {
        let reason = DataErrKind::LessData;
        assert_eq!(reason.sys_code(), 42214);
    }

    #[test]
    fn test_data_err_empty_data_code() {
        let reason = DataErrKind::EmptyData;
        assert_eq!(reason.sys_code(), 42215);
    }

    #[test]
    fn test_data_err_less_stc_code() {
        let reason = DataErrKind::LessStc("struct".into());
        assert_eq!(reason.sys_code(), 42216);
    }

    #[test]
    fn test_data_err_less_def_code() {
        let reason = DataErrKind::LessDef("define".into());
        assert_eq!(reason.sys_code(), 42217);
    }

    // SourceReason tests
    #[test]
    fn test_source_not_data_code() {
        let reason = SourceReason::NotData;
        assert_eq!(reason.sys_code(), 20401);
        assert_eq!(reason.sys_tag(), "source");
    }

    #[test]
    fn test_source_eof_code() {
        let reason = SourceReason::EOF;
        assert_eq!(reason.sys_code(), 20402);
    }

    #[test]
    fn test_source_supplier_error_code() {
        let reason = SourceReason::SupplierError("supplier failed".into());
        assert_eq!(reason.sys_code(), 50201);
    }

    #[test]
    fn test_source_disconnect_code() {
        let reason = SourceReason::Disconnect("connection lost".into());
        assert_eq!(reason.sys_code(), 49901);
    }

    #[test]
    fn test_source_other_code() {
        let reason = SourceReason::Other("unknown".into());
        assert_eq!(reason.sys_code(), 50209);
    }

    #[test]
    fn test_source_uvs_code() {
        let reason = SourceReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50021);
    }

    // SinkReason tests
    #[test]
    fn test_sink_sink_code() {
        let reason = SinkReason::Sink("sink failed".into());
        assert_eq!(reason.sys_code(), 50211);
        assert_eq!(reason.sys_tag(), "dist");
    }

    #[test]
    fn test_sink_mock_code() {
        let reason = SinkReason::Mock;
        assert_eq!(reason.sys_code(), 50312);
    }

    #[test]
    fn test_sink_stg_ctrl_code() {
        let reason = SinkReason::StgCtrl;
        assert_eq!(reason.sys_code(), 50311);
    }

    #[test]
    fn test_sink_uvs_code() {
        let reason = SinkReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50031);
    }

    // KnowledgeReason tests
    #[test]
    fn test_knowledge_not_data_code() {
        let reason = KnowledgeReason::NotData;
        assert_eq!(reason.sys_code(), 50042);
        assert_eq!(reason.sys_tag(), "knowledge");
    }

    #[test]
    fn test_knowledge_uvs_code() {
        let reason = KnowledgeReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50041);
    }

    // RunReason tests
    #[test]
    fn test_run_dist_sink_error_code() {
        let reason = RR::Dist(DistFocus::SinkError("error".into()));
        assert_eq!(reason.sys_code(), 50211);
        assert_eq!(reason.sys_tag(), "run");
    }

    #[test]
    fn test_run_dist_stg_ctrl_code() {
        let reason = RR::Dist(DistFocus::StgCtrl);
        assert_eq!(reason.sys_code(), 50311);
    }

    #[test]
    fn test_run_source_no_data_code() {
        let reason = RR::Source(SourceFocus::NoData);
        assert_eq!(reason.sys_code(), 20401);
    }

    #[test]
    fn test_run_source_eof_code() {
        let reason = RR::Source(SourceFocus::Eof);
        assert_eq!(reason.sys_code(), 20402);
    }

    #[test]
    fn test_run_source_supplier_error_code() {
        let reason = RR::Source(SourceFocus::SupplierError("error".into()));
        assert_eq!(reason.sys_code(), 50201);
    }

    #[test]
    fn test_run_source_other_code() {
        let reason = RR::Source(SourceFocus::Other("other".into()));
        assert_eq!(reason.sys_code(), 50209);
    }

    #[test]
    fn test_run_source_disconnect_code() {
        let reason = RR::Source(SourceFocus::Disconnect("disconnect".into()));
        assert_eq!(reason.sys_code(), 49901);
    }

    #[test]
    fn test_run_uvs_code() {
        let reason = RR::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50041);
    }
}
