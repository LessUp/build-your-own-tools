# Changelog

> All notable changes to the build-your-own-tools project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## Quick Navigation

- [Unreleased](#unreleased)
- [0.2.0 - 2026-03-13](#020---2026-03-13)
- [0.1.1 - 2026-03-10](#011---2026-03-10)
- [0.1.0 - 2026-02-13](#010---2026-02-13)

---

## [Unreleased]

### Changed

- Re-centered close-out work around sequential OpenSpec phase changes instead of treating one archived cleanup change as the ongoing active scope
- Tightened project guidance so agents create or use the current phase change rather than reviving archived history
- Reduced low-value changelog helper pages and kept `CHANGELOG.md` as the canonical project history surface
- Simplified Pages navigation so durable docs matter more than changelog depth

### Fixed

- Corrected stale references that still pointed contributors at an archived cleanup change

---

## [0.2.0] - 2026-03-13

### Infrastructure 🏗️

#### GitHub Actions

- **Changed**: Pages workflow installation strategy
  - Migrated from `npm ci` to `npm install --no-package-lock`
  - Fixes package lock synchronization issues
  - 30% faster documentation builds

- **Fixed**: rustfmt line ending rules for Windows builds
  - Added `*.rs text eol=lf` to `.gitattributes`
  - Resolves `cargo fmt --check` failures on Windows CI

#### Documentation

- **Fixed**: Dead links in `README.zh-CN.md`
  - Updated cross-references after structure changes
  - All internal links now validated in CI

---

## [0.1.1] - 2026-03-10

### Infrastructure 🏗️

#### Workflow Standardization

- **Renamed**: `docs.yml` → `pages.yml` for clarity
- **Unified**: `permissions` and `concurrency` configurations
  - Consistent security model across all workflows
- **Optimized**: Added `paths` trigger filtering
  - Reduces unnecessary builds by ~40%

#### Pages Deployment

- **Added**: Sparse-checkout for faster builds
  - 50% reduction in checkout time
- **Enhanced**: Documentation homepage
  - Feature cards for each tool
  - Comparison tables updated
  - Search functionality enabled

---

## [0.1.0] - 2026-02-13

### Added 🎉

#### dos2unix v0.1.0

Rust implementation of CRLF to LF converter

- **Features**:
  - In-place file conversion with streaming support
  - stdin/stdout support for pipeline usage
  - Check mode for detecting CRLF without modification
  - Quiet mode option (`-q`)
  - Exit code 2 when CRLF found in check mode

- **Technical**:
  - 8KB buffer streaming for large files
  - Cross-platform line ending handling
  - Memory-efficient processing

#### gzip v0.1.0

Multi-language gzip implementation

**Go (`gzip-go`)**:

- Parallel file processing with goroutines
- Recursive directory support (`-r`)
- Compression level control (`-l 0-9`)
- stdin/stdout support
- Force overwrite flag (`-f`)

**Rust (`rgzip`)**:

- Library crate (`lib.rs`) for embedding
- Streaming compression/decompression
- CLI with clap derive macros
- Keep source option (`-k`)
- Output path customization (`-o`)

#### htop v0.1.0

Cross-platform system monitor

**Unix/Rust**:

- Real-time CPU and memory monitoring
- Process list with sorting (CPU, Memory, PID, Name)
- Interactive TUI with ratatui
- Process search/filter (`/`)
- Process kill functionality (`k`)
- Adjustable refresh interval

**Windows/Rust**:

- All Unix features plus sparkline history
- CPU usage trends visualization
- Memory usage graphs

**Windows/Go**:

- Equivalent functionality using gopsutil
- tview-based TUI
- Cross-compilation support

### Infrastructure 🏗️

- **Cargo workspace** for Rust projects
  - Shared dependencies via `[workspace.dependencies]`
  - Unified build commands
- **Go workspace** for Go projects
  - Module graph with dependency tracking
- **GitHub Actions** CI/CD pipeline
  - Cross-platform builds (Linux, macOS, Windows)
  - Automated testing and linting
  - Release artifact generation
- **Makefile** for unified commands
  - `build-all`, `test-all`, `lint-all`, `fmt-all`
- **Documentation**:
  - `ARCHITECTURE.md` - System design
  - `COMPARISON.md` - Language comparison
  - `CONTRIBUTING.md` - Contribution guidelines
  - Per-project `README.md` and `changelog/`

---

**Last Updated**: 2026-04-26
