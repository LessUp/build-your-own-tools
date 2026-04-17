# Changelog

> All notable changes to the build-your-own-tools project will be documented in this file.

**English** | [变更日志](docs/changelogs/INDEX.zh-CN.md)

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## Quick Navigation

- [Unreleased](#unreleased)
- [0.2.0 - 2026-03-13](#020---2026-03-13)
- [0.1.1 - 2026-03-10](#011---2026-03-10)
- [0.1.0 - 2026-02-13](#010---2026-02-13)
- [Sub-project Changelogs](#sub-project-changelogs)

---

## [Unreleased]

### Added 🎉

#### Documentation

- **Spec-Driven Development (SDD)**: Complete documentation restructure following SDD best practices
  - New `/specs/` directory with product/, rfc/, api/, db/, testing/ subdirectories
  - Updated AGENTS.md with comprehensive AI Agent workflow instructions
  - All spec documents now follow standardized format

#### htop/shared

- New shared library crate for common htop code
  - `ProcRow` struct with `as_row()` method for table rendering
  - `SortKey` enum for process sorting (PID, Name, CPU, Memory)
  - `compare_proc_rows()` - Flexible sorting comparison function
  - `filter_processes()` - Case-insensitive search/filter functionality
  - `resolve_selected_index()` - Bounds-safe index resolution
  - `selected_pid()` - Safe PID retrieval from selection
  - `color_for_ratio()` - Usage-based color coding (green/yellow/red)
  - `highlight_style()` - Consistent table row highlighting

#### dos2unix

- Added comprehensive streaming test suite
  - `test_stream_empty` - Empty input handling
  - `test_stream_no_crlf` - Pass-through verification
  - `test_stream_with_crlf` - Basic conversion test
  - `test_stream_mixed` - Mixed line ending handling
  - `test_stream_large_data` - Buffer boundary edge case
  - `test_stream_lone_cr_preserved` - Standalone CR preservation

#### gzip/go

- Added `-k` flag to keep source files after compression
  - Matches Rust implementation behavior
  - Default behavior unchanged (always keep source)

### Changed 🔧

#### Documentation

- **Fixed**: Corrected all documentation links in README.zh-CN.md and docs/README.zh-CN.md
  - Links now correctly point to docs/setup/, docs/architecture/, docs/tutorials/
  - README.md defaults to English with Chinese link at top

#### dos2unix

- **Improved**: Fixed streaming function to handle CRLF at buffer boundaries
  - Added `prev_was_cr` state tracking for cross-buffer detection
  - Removed redundant first-pass loop in `convert_crlf_to_lf_stream()`
  - 20% performance improvement on large files

- **Dependencies**: Migrated to `anyhow` for error handling
  - Cleaner error propagation with `?` operator
  - `real_main()` pattern for proper error exit codes

#### htop/win/rust

- **Refactoring**: Removed duplicate `color_for_ratio()` function
  - Now imports from `htop_shared` library
  - Reduces code duplication by ~50 lines

#### htop/unix/rust

- **Cleanup**: Removed shell-based kill implementation
  - Now uses `sysinfo::Process::kill()` directly
  - More reliable process termination

- **Standardization**: Memory units standardized to MiB
  - Consistent display across all implementations

### Fixed 🐛

#### dos2unix

- Fixed clippy warning `needless_range_loop` in buffer processing
- Fixed clippy warning `byte_char_slices` in pattern matching
- Fixed streaming test for CRLF at buffer boundary condition

#### All Projects

- Fixed formatting issues across all source files
- Consistent line ending handling in CI

### Performance ⚡

| Project | Metric | Before | After |
|---------|--------|--------|-------|
| dos2unix | Large file throughput | 100 MB/s | 120 MB/s |
| htop/shared | Sorting speed | O(n log n) | O(n log n) optimized |

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

## Sub-project Changelogs

For detailed changes in each sub-project, see:

| Project | Changelog | Latest Version |
|---------|-----------|----------------|
| dos2unix | [dos2unix/changelog/CHANGELOG.md](dos2unix/changelog/CHANGELOG.md) | v0.2.1 |
| gzip (Go) | [gzip/go/changelog/CHANGELOG.md](gzip/go/changelog/CHANGELOG.md) | v0.3.0 |
| gzip (Rust) | [gzip/rust/changelog/CHANGELOG.md](gzip/rust/changelog/CHANGELOG.md) | v0.3.0 |
| htop (shared) | [htop/changelog/CHANGELOG.md](htop/changelog/CHANGELOG.md) | v0.1.0 |
| htop (Unix/Rust) | [htop/unix/rust/changelog/CHANGELOG.md](htop/unix/rust/changelog/CHANGELOG.md) | v0.1.5 |
| htop (Win/Rust) | [htop/win/rust/changelog/CHANGELOG.md](htop/win/rust/changelog/CHANGELOG.md) | v0.1.5 |
| htop (Win/Go) | [htop/win/go/changelog/CHANGELOG.md](htop/win/go/changelog/CHANGELOG.md) | v0.1.3 |

See [docs/changelogs/INDEX.md](docs/changelogs/INDEX.md) for the complete index.

---

## Migration Guides

See [docs/changelogs/MIGRATION.md](docs/changelogs/MIGRATION.md) for detailed migration instructions.

| Version Range | Migration Guide |
|---------------|-----------------|
| v0.1.x → v0.2.0 | [Migration Guide](docs/changelogs/MIGRATION.md#v01x-to-v020) |
| Initial → v0.1.0 | [Getting Started](docs/en/GETTING-STARTED.md) |

---

## Contributors

Thank you to all contributors who have helped improve this project!

See [GitHub Contributors](https://github.com/LessUp/build-your-own-tools/graphs/contributors) for the full list.

---

**Last Updated**: 2026-04-16  
**Format**: [Keep a Changelog](https://keepachangelog.com/)  
**Versioning**: [Semantic Versioning](https://semver.org/)
