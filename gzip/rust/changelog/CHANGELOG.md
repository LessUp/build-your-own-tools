# Changelog

All notable changes to rgzip will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_No changes yet._

## [0.1.1] - 2026-02-13

### Changed

- `same_path()` now uses `fs::canonicalize` for path normalization
  - Fixes `./foo` vs `foo` comparison issues

### Tests

- Added 6 unit tests:
  - `test_sanitize_level_clamp` - compression level clamping
  - `test_default_output_for_compress` - default compression output path
  - `test_default_output_for_decompress_gz` - strip `.gz` suffix
  - `test_default_output_for_decompress_no_gz` - append `.out`
  - `test_compress_decompress_roundtrip` - file-level roundtrip
  - `test_ensure_writable_no_force` - overwrite detection
  - `test_compress_reader_to_writer` - streaming roundtrip

## [0.1.0] - 2025-09-25

### Added

- Initial project structure (`Cargo.toml`, `.gitignore`, `src/main.rs`, `README.md`)
- Core gzip compression and decompression (based on `flate2`)
- CLI argument parsing (based on `clap`)
- Options: `-d` (decompress), `-o` (output), `-k` (keep), `-f` (force), `-l` (level)
- Separate library crate in `src/lib.rs` for embedding
