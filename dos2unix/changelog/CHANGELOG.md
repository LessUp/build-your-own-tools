# Changelog

All notable changes to dos2unix-rust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Migrated to `anyhow` for error handling
  - Cleaner error propagation with `?` operator
  - `real_main()` pattern for proper error handling
- Fixed streaming function to handle CRLF at buffer boundaries
  - Added `prev_was_cr` state tracking for cross-buffer CRLF detection
  - Removed redundant first-pass loop in `convert_crlf_to_lf_stream()`

### Fixed

- Fixed clippy warning `needless_range_loop`
- Fixed clippy warning `byte_char_slices`

### Tests

- Added 6 new streaming tests for `convert_crlf_to_lf_stream()`:
  - `test_stream_empty`
  - `test_stream_no_crlf`
  - `test_stream_with_crlf`
  - `test_stream_mixed`
  - `test_stream_large_data` (buffer boundary edge case)
  - `test_stream_lone_cr_preserved`

## [0.2.1] - 2026-02-13

### Added

- Added `.editorconfig` and `.gitattributes`

### Tests

- Added 4 new unit tests: `lone_cr_not_converted`, `trailing_cr_not_converted`, `consecutive_crlf`, `no_newlines`

## [0.2.0] - 2025-10-12

### Added

- New `--check/-c` mode for detecting CRLF without modification
- Exit code 2 when CRLF found in check mode

### Changed

- Updated `README.md` with check mode documentation

### Tests

- Added unit tests for `convert_crlf_to_lf()` function

## [0.1.0] - 2025-09-25

### Added

- Initial project structure (`Cargo.toml`, `src/`, `changelog/`)
- Basic CRLF → LF conversion with stdin/stdout support
- CLI options: `-h`, `-v`, `-q`
