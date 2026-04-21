# Changelog

> All notable changes to htop will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added 🎉

#### htop/shared

New shared library crate for common htop functionality:

- **`ProcRow`** - Process data structure
  - PID, name, CPU%, memory, status, command, executable path
  - `as_row()` method for table rendering
  
- **`SortKey`** - Sorting options enum
  - `PID`, `Name`, `CPU`, `Memory` variants
  - Direction support (ascending/descending)
  
- **Core Functions**
  - `compare_proc_rows()` - Flexible process comparison
  - `filter_processes()` - Case-insensitive search/filter
  - `resolve_selected_index()` - Bounds-safe selection handling
  - `selected_pid()` - Safe PID retrieval
  - `color_for_ratio()` - Usage-based color coding
  - `highlight_style()` - Consistent row highlighting

### Changed 🔧

- **Standardization**: Memory units standardized to MiB across all implementations
  - Previous: Mixed MB/MiB/KB displays
  - Now: Consistent MiB (mebibytes) for accuracy
  - Affects all htop variants (Unix/Rust, Win/Rust, Win/Go)

### Migration Notes

For downstream users of process data:

```rust
// Before
let mem_mb = process.memory_kb / 1024;  // Approximate

// After  
let mem_mib = process.mem_mib;  // Precise
```

---

## [0.1.0] - 2025-10-20

### Added 🎉

#### Repository Standardization

- Root-level `.gitignore`, `.gitattributes`, `.editorconfig`
  - Consistent line endings (LF)
  - Editor configuration
  - Ignore patterns for build artifacts

- Root-level `README.md`
  - Project overview
  - Quick start guide
  - Build instructions

#### CI/CD Infrastructure

- **GitHub Actions** multi-language CI
  - Rust: `unix/rust`, `win/rust` builds
  - Go: `win/go` builds
  - Matrix builds across platforms (Linux, macOS, Windows)
  - Automated testing and linting

### Technical Details

| Implementation | Platform | Language | Status |
|----------------|----------|----------|--------|
| htop-unix-rust | Linux/macOS | Rust | ✅ Stable |
| htop-win-rust | Windows | Rust | ✅ Stable |
| htop-win-go | Windows | Go | ✅ Stable |

---

## Sub-project Changelogs

For detailed changes in each implementation, see:

| Implementation | Changelog | Latest Version |
|----------------|-----------|----------------|
| Unix/Rust | [unix/rust/changelog/CHANGELOG.md](unix/rust/changelog/CHANGELOG.md) | v0.1.5 |
| Windows/Rust | [win/rust/changelog/CHANGELOG.md](win/rust/changelog/CHANGELOG.md) | v0.1.0 |
| Windows/Go | [win/go/changelog/CHANGELOG.md](win/go/changelog/CHANGELOG.md) | v0.1.3 |

---

## Feature Matrix

Cross-implementation feature comparison:

| Feature | Unix/Rust | Win/Rust | Win/Go | htop/shared |
|---------|-----------|----------|--------|-------------|
| Process List | ✅ | ✅ | ✅ | ✅ |
| CPU Monitoring | ✅ | ✅ | ✅ | ✅ |
| Memory Monitoring | ✅ | ✅ | ✅ | ✅ |
| Process Kill | ✅ | ✅ | ✅ | N/A |
| Search/Filter | ✅ | ✅ | ✅ | ✅ |
| Column Sorting | ✅ | ✅ | ✅ | ✅ |
| Sparkline History | ❌ | ✅ | ❌ | N/A |
| Refresh Interval | ✅ | ✅ | ✅ | N/A |
| Shared Library | ❌ | ✅ | ❌ | ✅ |

---

## Dependencies

### Rust Implementations

| Crate | Version | Purpose |
|-------|---------|---------|
| ratatui | 0.29 | TUI framework |
| crossterm | 0.29 | Terminal control |
| sysinfo | 0.29 | System information |
| htop_shared | local | Shared utilities |

### Go Implementation

| Package | Version | Purpose |
|---------|---------|---------|
| tview | latest | TUI framework |
| gopsutil | v3 | System information |

---

## Compatibility

| Version | Rust | Go | Platforms |
|---------|------|------|-----------|
| v0.1.0+ | 1.70+ | 1.21+ | Linux, macOS, Windows |

---

## Performance Notes

### Memory Usage

| Implementation | Idle | 100 processes | 500 processes |
|----------------|------|---------------|---------------|
| Unix/Rust | ~8MB | ~12MB | ~18MB |
| Win/Rust | ~10MB | ~15MB | ~22MB |
| Win/Go | ~12MB | ~18MB | ~28MB |

### Refresh Rate

All implementations target 60 FPS with adaptive refresh:
- Default interval: 1 second
- Minimum interval: 100ms
- Maximum interval: 5 seconds

---

## Roadmap

### Planned for v0.2.0

- [ ] Linux/Go implementation
- [ ] macOS/Go implementation
- [ ] Network monitoring (all platforms)
- [ ] Disk I/O monitoring
- [ ] Export to CSV/JSON

### Under Consideration

- [ ] GPU monitoring
- [ ] Container-aware process grouping
- [ ] Remote system monitoring
- [ ] Plugin system

---

## Related

- [Main Changelog](../../CHANGELOG.md) - Project-wide changes
- [API Documentation](../../docs/architecture/API.md) - htop_shared API reference
- [GitHub Releases](https://github.com/LessUp/build-your-own-tools/releases)

---

**Last Updated**: 2026-04-16
