use std::path::Path;

use orion_error::{ErrorReport, OperationContext, SourceFrame};

pub mod key {
    pub const CONFIG_KIND: &str = "config.kind";
    pub const CONFIG_GROUP: &str = "config.group";
    pub const CONFIG_SECTION: &str = "config.section";
    pub const CONFIG_TYPE_NAME: &str = "config.type_name";
    pub const CONFIG_LOADER: &str = "config.loader";

    pub const FILE_PATH: &str = "file.path";
    pub const DIR_PATH: &str = "dir.path";
    pub const RESOURCE_PATH: &str = "resource.path";
    pub const NETWORK_URL: &str = "network.url";

    pub const RUNTIME_STAGE: &str = "runtime.stage";
    pub const OPERATION_KIND: &str = "operation.kind";
    pub const COMPONENT_KIND: &str = "component.kind";
    pub const COMPONENT_NAME: &str = "component.name";

    pub const HINT_CODE: &str = "hint.code";
}

pub trait MetaValue: Copy + Sized {
    const KEY: &'static str;

    fn as_str(self) -> &'static str;

    fn parse(raw: &str) -> Option<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigKind {
    Wpsrc,
    SinkRoute,
    SinkDefaults,
    ConnectorDef,
    Engine,
    Wpgen,
    Knowdb,
    SourceRuntime,
    SinkRuntime,
}

impl MetaValue for ConfigKind {
    const KEY: &'static str = key::CONFIG_KIND;

    fn as_str(self) -> &'static str {
        match self {
            Self::Wpsrc => "wpsrc",
            Self::SinkRoute => "sink_route",
            Self::SinkDefaults => "sink_defaults",
            Self::ConnectorDef => "connector_def",
            Self::Engine => "engine",
            Self::Wpgen => "wpgen",
            Self::Knowdb => "knowdb",
            Self::SourceRuntime => "source_runtime",
            Self::SinkRuntime => "sink_runtime",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "wpsrc" => Self::Wpsrc,
            "sink_route" => Self::SinkRoute,
            "sink_defaults" => Self::SinkDefaults,
            "connector_def" => Self::ConnectorDef,
            "engine" => Self::Engine,
            "wpgen" => Self::Wpgen,
            "knowdb" => Self::Knowdb,
            "source_runtime" => Self::SourceRuntime,
            "sink_runtime" => Self::SinkRuntime,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigGroup {
    Infra,
    Business,
}

impl MetaValue for ConfigGroup {
    const KEY: &'static str = key::CONFIG_GROUP;

    fn as_str(self) -> &'static str {
        match self {
            Self::Infra => "infra",
            Self::Business => "business",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "infra" => Self::Infra,
            "business" => Self::Business,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeStage {
    GeneratorGenerate,
    SupervisorMonitor,
    CollectorRecovery,
    OrchestratorConfigLoad,
    SystemOperations,
}

impl MetaValue for RuntimeStage {
    const KEY: &'static str = key::RUNTIME_STAGE;

    fn as_str(self) -> &'static str {
        match self {
            Self::GeneratorGenerate => "generator.generate",
            Self::SupervisorMonitor => "supervisor.monitor",
            Self::CollectorRecovery => "collector.recovery",
            Self::OrchestratorConfigLoad => "orchestrator.config.load",
            Self::SystemOperations => "system_operations",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "generator.generate" => Self::GeneratorGenerate,
            "supervisor.monitor" => Self::SupervisorMonitor,
            "collector.recovery" => Self::CollectorRecovery,
            "orchestrator.config.load" => Self::OrchestratorConfigLoad,
            "system_operations" => Self::SystemOperations,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationKind {
    LoadConfigFile,
    ParseConfig,
    ValidateConfig,
    ReadDir,
    ReadDirEntry,
    BuildSourceInstance,
    BuildSinkInstance,
    PluginValidate,
    ReplayRescueFile,
}

impl MetaValue for OperationKind {
    const KEY: &'static str = key::OPERATION_KIND;

    fn as_str(self) -> &'static str {
        match self {
            Self::LoadConfigFile => "load_config_file",
            Self::ParseConfig => "parse_config",
            Self::ValidateConfig => "validate_config",
            Self::ReadDir => "read_dir",
            Self::ReadDirEntry => "read_dir_entry",
            Self::BuildSourceInstance => "build_source_instance",
            Self::BuildSinkInstance => "build_sink_instance",
            Self::PluginValidate => "plugin_validate",
            Self::ReplayRescueFile => "replay_rescue_file",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "load_config_file" => Self::LoadConfigFile,
            "parse_config" => Self::ParseConfig,
            "validate_config" => Self::ValidateConfig,
            "read_dir" => Self::ReadDir,
            "read_dir_entry" => Self::ReadDirEntry,
            "build_source_instance" => Self::BuildSourceInstance,
            "build_sink_instance" => Self::BuildSinkInstance,
            "plugin_validate" => Self::PluginValidate,
            "replay_rescue_file" => Self::ReplayRescueFile,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    Source,
    Sink,
    Connector,
    Generator,
    Monitor,
    Rescue,
    Checkpoint,
}

impl MetaValue for ComponentKind {
    const KEY: &'static str = key::COMPONENT_KIND;

    fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Sink => "sink",
            Self::Connector => "connector",
            Self::Generator => "generator",
            Self::Monitor => "monitor",
            Self::Rescue => "rescue",
            Self::Checkpoint => "checkpoint",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "source" => Self::Source,
            "sink" => Self::Sink,
            "connector" => Self::Connector,
            "generator" => Self::Generator,
            "monitor" => Self::Monitor,
            "rescue" => Self::Rescue,
            "checkpoint" => Self::Checkpoint,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintCode {
    WpsrcTomlSchema,
    SinkRouteTomlSchema,
    SinkDefaultsTomlSchema,
    KafkaFeatureRequired,
    DuplicateSourceKey,
    InvalidSyslogProtocol,
}

impl MetaValue for HintCode {
    const KEY: &'static str = key::HINT_CODE;

    fn as_str(self) -> &'static str {
        match self {
            Self::WpsrcTomlSchema => "wpsrc_toml_schema",
            Self::SinkRouteTomlSchema => "sink_route_toml_schema",
            Self::SinkDefaultsTomlSchema => "sink_defaults_toml_schema",
            Self::KafkaFeatureRequired => "kafka_feature_required",
            Self::DuplicateSourceKey => "duplicate_source_key",
            Self::InvalidSyslogProtocol => "invalid_syslog_protocol",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "wpsrc_toml_schema" => Self::WpsrcTomlSchema,
            "sink_route_toml_schema" => Self::SinkRouteTomlSchema,
            "sink_defaults_toml_schema" => Self::SinkDefaultsTomlSchema,
            "kafka_feature_required" => Self::KafkaFeatureRequired,
            "duplicate_source_key" => Self::DuplicateSourceKey,
            "invalid_syslog_protocol" => Self::InvalidSyslogProtocol,
            _ => return None,
        })
    }
}

pub trait OperationContextMetaExt: Sized {
    fn with_meta_value<M: MetaValue>(self, value: M) -> Self;

    fn with_file_path(self, path: &Path) -> Self;

    fn with_dir_path(self, path: &Path) -> Self;

    fn with_resource_path(self, path: &Path) -> Self;

    fn with_network_url(self, url: impl Into<String>) -> Self;

    fn with_component_name(self, name: impl Into<String>) -> Self;

    fn with_config_type_name(self, name: impl Into<String>) -> Self;

    fn with_config_section(self, section: impl Into<String>) -> Self;
}

impl OperationContextMetaExt for OperationContext {
    fn with_meta_value<M: MetaValue>(mut self, value: M) -> Self {
        self.record_meta(M::KEY, value.as_str());
        self
    }

    fn with_file_path(mut self, path: &Path) -> Self {
        self.record_meta(key::FILE_PATH, path.display().to_string());
        self
    }

    fn with_dir_path(mut self, path: &Path) -> Self {
        self.record_meta(key::DIR_PATH, path.display().to_string());
        self
    }

    fn with_resource_path(mut self, path: &Path) -> Self {
        self.record_meta(key::RESOURCE_PATH, path.display().to_string());
        self
    }

    fn with_network_url(mut self, url: impl Into<String>) -> Self {
        self.record_meta(key::NETWORK_URL, url.into());
        self
    }

    fn with_component_name(mut self, name: impl Into<String>) -> Self {
        self.record_meta(key::COMPONENT_NAME, name.into());
        self
    }

    fn with_config_type_name(mut self, name: impl Into<String>) -> Self {
        self.record_meta(key::CONFIG_TYPE_NAME, name.into());
        self
    }

    fn with_config_section(mut self, section: impl Into<String>) -> Self {
        self.record_meta(key::CONFIG_SECTION, section.into());
        self
    }
}

pub fn first_meta_str<'a>(report: &'a ErrorReport, key: &str) -> Option<&'a str> {
    report.root_metadata.get_str(key).or_else(|| {
        report
            .source_frames
            .iter()
            .find_map(|frame| frame.metadata.get_str(key))
    })
}

pub fn first_meta_enum<M: MetaValue>(report: &ErrorReport) -> Option<M> {
    first_meta_str(report, M::KEY).and_then(M::parse)
}

pub fn frame_meta_enum<M: MetaValue>(frame: &SourceFrame) -> Option<M> {
    frame.metadata.get_str(M::KEY).and_then(M::parse)
}

#[cfg(test)]
mod tests {
    use super::*;
    use orion_error::ErrorMetadata;

    #[test]
    fn config_kind_round_trip() {
        assert_eq!(
            ConfigKind::parse(ConfigKind::Wpsrc.as_str()),
            Some(ConfigKind::Wpsrc)
        );
        assert_eq!(
            ConfigKind::parse(ConfigKind::SinkDefaults.as_str()),
            Some(ConfigKind::SinkDefaults)
        );
    }

    #[test]
    fn operation_context_meta_ext_records_typed_and_path_metadata() {
        let ctx = OperationContext::want("load")
            .with_meta_value(ConfigKind::Wpsrc)
            .with_meta_value(OperationKind::LoadConfigFile)
            .with_file_path(Path::new("/tmp/wpsrc.toml"))
            .with_component_name("file_1");

        assert_eq!(ctx.metadata().get_str(key::CONFIG_KIND), Some("wpsrc"));
        assert_eq!(
            ctx.metadata().get_str(key::OPERATION_KIND),
            Some("load_config_file")
        );
        assert_eq!(
            ctx.metadata().get_str(key::FILE_PATH),
            Some("/tmp/wpsrc.toml")
        );
        assert_eq!(ctx.metadata().get_str(key::COMPONENT_NAME), Some("file_1"));
    }

    #[test]
    fn first_meta_enum_prefers_root_metadata() {
        let mut root = ErrorMetadata::new();
        root.insert(key::CONFIG_KIND, ConfigKind::SinkRoute.as_str());

        let report = ErrorReport {
            reason: "configuration error".to_string(),
            detail: None,
            position: None,
            want: None,
            path: None,
            context: Vec::new(),
            root_metadata: root,
            source_frames: Vec::new(),
        };

        assert_eq!(
            first_meta_enum::<ConfigKind>(&report),
            Some(ConfigKind::SinkRoute)
        );
    }

    #[test]
    fn frame_meta_enum_reads_typed_metadata() {
        let mut metadata = ErrorMetadata::new();
        metadata.insert(key::CONFIG_GROUP, ConfigGroup::Infra.as_str());

        let frame = SourceFrame {
            index: 0,
            message: "configuration error".to_string(),
            display: None,
            debug: String::new(),
            type_name: None,
            error_code: None,
            reason: None,
            want: None,
            path: None,
            detail: None,
            metadata,
            is_root_cause: true,
        };

        assert_eq!(
            frame_meta_enum::<ConfigGroup>(&frame),
            Some(ConfigGroup::Infra)
        );
    }
}
