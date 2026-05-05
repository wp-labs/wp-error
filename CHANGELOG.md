# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.1] - 2026-05-05

### Changed
- Bump orion-error from 0.7 to 0.8; rename `UvsReason` → `UnifiedReason`
- Bump `wp-connector-api` from `0.9` to `0.10`
- Replace `ErrStrategy` with `ErrorHandlingStrategy` (type removed in 0.8)
- Use delegate constructors (`core_conf()`, `data_error()`, `resource_error()`, `logic_error()`) instead of legacy `UvsFrom` methods (`from_conf_reason`, `from_data`, `from_res`, `from_logic`)
- Replace `StructError::from(reason)` with `reason.to_err()`
- Update `DiagnosticReport` field access to public getters (fields are private in 0.8)
- Update `SourceFrame` import path: `runtime` → `runtime::source`
- Update `ErrorMetadata` import path: `types` → `runtime`
- Match renamed variants: `ConfIOReason::Uvs` → `General`, `OrionSecReason::Uvs` → `General`
- Adapt `SourceReason` and `SinkReason` system-code mappings to unit variants with runtime details carried by `StructError`

### Removed
- Remove `UvsFrom` and `ConfErrReason` imports (removed in 0.8)
- Remove `From<ErrStrategy> for ErrorHandlingStrategy` impl (type removed in 0.8)

## [0.9.0] - 2026-04-30

### Changed
- Migrate `ConfReason<T>`, `RunReason`, `OMLCodeReason`, `KnowledgeReason` to
  `derive(OrionError)`, replacing manual `ErrorCode`/`DomainReason` impls
- Update import paths for `ConfErrReason`, `ErrStrategy`, `SourceFrame`,
  `ErrorReport` → `DiagnosticReport` to match orion-error 0.7.2 API layout
- Replace deprecated `OperationContext::want()` with `doing()`

### Removed
- `ErrorCode` trait no longer supported; numeric code assertions removed from
  integration tests

[Unreleased]: https://github.com/wp-labs/wp-error/compare/v0.10.1...HEAD
[0.10.1]: https://github.com/wp-labs/wp-error/compare/v0.9.0...v0.10.1
[0.9.0]: https://github.com/wp-labs/wp-error/releases/tag/v0.9.0
