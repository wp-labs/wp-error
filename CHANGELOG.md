# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/wp-labs/wp-error/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/wp-labs/wp-error/releases/tag/v0.9.0
