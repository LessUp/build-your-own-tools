# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **htop/shared**: New shared library crate for common htop code
  - `ProcRow` struct with `as_row()` method
  - `SortKey` enum for process sorting
  - `compare_proc_rows()`, `filter_processes()`, `resolve_selected_index()`, `selected_pid()` functions
  - `color_for_ratio()` for usage-based coloring
  - `highlight_style()` for table row styling

### Changed

- **dos2unix**: Fixed streaming function to handle CRLF at buffer boundaries
  - Added `prev_was_cr` state tracking for cross-buffer CRLF detection
  - Removed redundant first-pass loop in `convert_crlf_to_lf_stream()`
- **dos2unix**: Migrated to `anyhow` for error handling
  - Cleaner error propagation with `?` operator
  - `real_main()` pattern for proper error handling
- **htop/win/rust**: Removed duplicate `color_for_ratio()` function
  - Now imports from `htop_shared`
- **htop/unix/rust**: Removed shell-based kill implementation
  - Now uses `sysinfo::Process::kill()` directly
- **htop**: Standardized memory units to MiB across implementations
- **gzip/go**: Added `-k` flag to keep source files after compression

### Fixed

- **dos2unix**: Fixed clippy warning `needless_range_loop`
- **dos2unix**: Fixed clippy warning `byte_char_slices`
- **dos2unix**: Fixed streaming test for CRLF at buffer boundary
- All projects: Fixed formatting issues

### Tests

- **dos2unix**: Added 6 new streaming tests
  - `test_stream_empty`
  - `test_stream_no_crlf`
  - `test_stream_with_crlf`
  - `test_stream_mixed`
  - `test_stream_large_data` (buffer boundary edge case)
  - `test_stream_lone_cr_preserved`

## [0.2.0] - 2026-03-13

### Infrastructure

- **Workflows**: Fixed Pages workflow installation strategy
  - Changed from `npm ci` to `npm install --no-package-lock`
  - Fixed dead links in `README.zh-CN.md`
- **Workflows**: Fixed rustfmt line ending rules for `htop/win/rust`
  - Added `*.rs text eol=lf` to `.gitattributes`
  - Fixed `cargo fmt --check` failures on Windows CI

## [0.1.1] - 2026-03-10

### Infrastructure

- **Workflows**: Deep standardization across CI/CD
  - Renamed `docs.yml` → `pages.yml`
  - Unified `permissions` and `concurrency` configurations
  - Added `paths` trigger filtering to reduce unnecessary builds
- **Pages**: Optimized GitHub Pages deployment
  - Added sparse-checkout for faster builds
  - Expanded documentation homepage with feature cards and comparison tables

## [0.1.0] - 2026-02-13

### Added

- **dos2unix**: Rust implementation of CRLF to LF converter
  - In-place file conversion with streaming support for large files
  - stdin/stdout support
  - Check mode for detecting CRLF without modification
  - Quiet mode option
- **gzip**: Multi-language gzip implementation
  - Go implementation (gzip-go) with parallel processing
  - Rust implementation (rgzip) with library crate
  - Streaming compression/decompression
  - Keep source file option (`-k`)
- **htop**: Cross-platform system monitor
  - Unix/Rust implementation with ratatui TUI
  - Windows/Rust implementation with sparkline history
  - Windows/Go implementation with tview TUI
  - Real-time CPU/memory monitoring
  - Process sorting (CPU, Memory, PID, Name)
  - Process filtering and search
  - Process kill functionality
  - Adjustable refresh interval

### Infrastructure

- **Cargo workspace** for Rust projects with shared dependencies
- **Go workspace** configuration
- **GitHub Actions** CI pipeline with cross-platform builds
- **GitHub Actions** release workflow with binary artifacts
- **Makefile** for unified build commands
- **Root-level** `.editorconfig`, `.gitattributes`, `.gitignore`
- **Documentation**: `ARCHITECTURE.md`, `COMPARISON.md`, `CONTRIBUTING.md`

---

## Sub-project Changelogs

For detailed changes in each sub-project, see:

| Project | Changelog |
|---------|-----------|
| dos2unix | [dos2unix/changelog/CHANGELOG.md](./dos2unix/changelog/CHANGELOG.md) |
| gzip/go | [gzip/go/changelog/CHANGELOG.md](./gzip/go/changelog/CHANGELOG.md) |
| gzip/rust | [gzip/rust/changelog/CHANGELOG.md](./gzip/rust/changelog/CHANGELOG.md) |
| htop | [htop/changelog/CHANGELOG.md](./htop/changelog/CHANGELOG.md) |
| htop/unix/rust | [htop/unix/rust/changelog/CHANGELOG.md](./htop/unix/rust/changelog/CHANGELOG.md) |
| htop/win/rust | [htop/win/rust/changelog/CHANGELOG.md](./htop/win/rust/changelog/CHANGELOG.md) |
| htop/win/go | [htop/win/go/changelog/CHANGELOG.md](./htop/win/go/changelog/CHANGELOG.md) |
