# Changelog

All notable changes to gzip-go will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `-k` flag to keep source files after compression

## [0.3.0] - 2026-02-13

### Changed

- Upgraded Go version from 1.21 to 1.23 (consistent with `go.work`)

### Tests

- Added unit tests in `main_test.go`:
  - `TestGzipStream` - streaming compression/decompression roundtrip
  - `TestGzipFile` - file-level compression
  - `TestGunzipFile` - file-level decompression roundtrip
  - `TestCollectInputs_SkipDir` - skip directories in input collection
  - `TestCollectInputs_SkipGz` - skip .gz files when compressing
  - `TestGzipToWriter` - compression to `io.Writer`

## [0.2.0] - 2025-09-25

### Added

- Makefile with `BIN_DIR` and auto-creation of `bin/` directory
- Cross-compilation target `build-linux` (`GOOS=linux GOARCH=amd64`)

### Changed

- `install` target now depends on `build`

### Documentation

- Updated `README.md` with cross-compilation instructions

## [0.1.1] - 2025-09-25

### Fixed

- Removed unused `errors` import in `cmd/gzip-go/main.go`

## [0.1.0] - 2025-09-25

### Added

- Initial project structure (`go.mod`, `cmd/gzip-go/main.go`, `Makefile`, `README.md`)
- Core features:
  - Compression (default) and decompression (`-d`)
  - Recursive directory processing (`-r`)
  - Parallel file processing (`-p`, defaults to CPU cores)
  - stdin/stdout support (no args or `-` reads stdin; `-stdout` outputs to stdout)
  - Force overwrite (`-f`)
  - Compression level (`-l 0..9`)
- KISS principle: does not delete source files by default
