# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `open_decoder` / `open_encoder` (+ `_with` taking `CodecPreferences`
  and an `ExecutionContext`): build the preferred implementation from a
  `RuntimeContext` with a thread budget already announced — default
  `ExecutionContext::auto()`. A codec built straight from the registry
  runs single-threaded until `set_execution_context` is called; the
  crate docs and README now show that one line.
- `ExecutionContext` re-exported at the crate root.

### Changed

- edition 2024, MSRV 1.95.

## [0.0.4](https://github.com/OxideAV/oxideav/compare/v0.0.3...v0.0.4) - 2026-05-06

### Other

- reframe FFI claim — HW-engine crates use OS FFI by necessity
- drop with_all_features* (use oxideav_meta::register_all instead)
