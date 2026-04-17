# Changelog

> All notable changes to rgzip will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

_No changes yet._

---

## [0.3.0] - 2026-04-16

### Changed 🔧

- **Version**: Bumped to 0.3.0 for consistency with project-wide version tracking

---

## [0.1.1] - 2026-02-13

### Changed 🔧

- **Core**: Improved `same_path()` function
  - Now uses `fs::canonicalize` for path normalization
  - Fixes `./foo` vs `foo` comparison inconsistencies
  - Handles symbolic links correctly
  - More robust path comparison for overwrite detection

### Tests 🧪

- **Comprehensive Test Suite**: Added 7 unit tests

| Test Name | Description | Coverage |
|-----------|-------------|----------|
| `test_sanitize_level_clamp` | Compression level boundary validation | Input validation |
| `test_default_output_for_compress` | Default `.gz` suffix generation | Path handling |
| `test_default_output_for_decompress_gz` | Strip `.gz` suffix logic | Path handling |
| `test_default_output_for_decompress_no_gz` | Append `.out` for non-.gz | Path handling |
| `test_compress_decompress_roundtrip` | Full cycle validation | End-to-end |
| `test_ensure_writable_no_force` | Overwrite protection | Safety |
| `test_compress_reader_to_writer` | Streaming API validation | Library API |

---

## [0.1.0] - 2025-09-25

### Added 🎉

- **Initial Release**: rgzip CLI tool and library crate
  - Dual-purpose crate: library (`lib.rs`) + binary (`main.rs`)
  - Full gzip compression and decompression support

### Library Features (lib.rs)

- **`compress_reader_to_writer<R, W>`** - Streaming compression
  - Generic over `Read` and `Write` traits
  - Configurable compression level
  - Returns bytes written

- **`decompress_reader_to_writer<R, W>`** - Streaming decompression
  - Generic streaming decompression
  - Automatic format detection
  - Error handling for corrupt data

- **`compress_file`** - High-level file compression
  - Convenience wrapper with path handling
  - Overwrite protection

- **`decompress_file`** - High-level file decompression
  - Automatic output path generation
  - Permission preservation

- **Utility Functions**
  - `default_output_path` - Generate default output paths
  - `sanitize_level` - Clamp compression levels to 0-9

### CLI Features (main.rs)

- **Core Options**
  - `-d`, `--decompress` - Decompression mode
  - `-o`, `--output` - Custom output path
  - `-k`, `--keep` - Keep source file
  - `-f`, `--force` - Force overwrite
  - `-l`, `--level` - Compression level (0-9)

- **Additional Options**
  - `-h`, `--help` - Display help with examples
  - `-V`, `--version` - Display version

### Technical Details

| Aspect | Implementation |
|--------|---------------|
| Compression | `flate2` crate (DEFLATE) |
| CLI Parsing | `clap` with derive macros |
| Error Handling | `anyhow` for ergonomics |
| Library API | `std::io::Result` for stdlib compatibility |

### Library Design

```rust
// Streaming API
use rgzip::{compress_reader_to_writer, decompress_reader_to_writer};
use flate2::Compression;

let compressed = compress_reader_to_writer(
    &mut input,
    &mut output,
    Compression::default()
)?;

// File API
use rgzip::{compress_file, decompress_file};

compress_file(input_path, output_path, Compression::best())?;
decompress_file(input_path, output_path)?;
```

---

## Performance Benchmarks

### Compression Speed

| File Size | Level | Time | Throughput | Ratio |
|-----------|-------|------|------------|-------|
| 10MB | 6 (default) | 180ms | ~55 MB/s | 65% |
| 10MB | 1 (fast) | 80ms | ~125 MB/s | 70% |
| 10MB | 9 (best) | 350ms | ~28 MB/s | 60% |
| 100MB | 6 | 1.8s | ~55 MB/s | 65% |

> Benchmarks on AMD Ryzen 5 5600X, NVMe SSD, Rust 1.80

### Memory Usage

| Operation | Peak Memory |
|-----------|-------------|
| Streaming | ~16KB (buffer only) |
| File API | ~16KB + file handles |

---

## Compression Levels

| Level | Speed | Size | Use Case |
|-------|-------|------|----------|
| 0 | Fastest | Largest | Store only |
| 1 | Fast | Large | Log files |
| 3 | Quick | Medium | Build artifacts |
| 6 | Balanced | Balanced | **Default** |
| 9 | Slowest | Smallest | Distribution |

---

## Compatibility

| Version | Rust | Platforms | Breaking Changes |
|---------|------|-----------|------------------|
| v0.1.1 | 1.70+ | Linux, macOS, Windows | None |
| v0.1.0 | 1.70+ | Linux, macOS, Windows | Initial release |

---

## Comparison with gzip-go

| Feature | rgzip | gzip-go |
|---------|-------|---------|
| Library crate | ✅ `lib.rs` | ❌ |
| Parallel processing | ❌ (use external) | ✅ Built-in |
| Recursive directory | ❌ | ✅ `-r` flag |
| Keep source | ✅ `-k` flag | ✅ Default |
| Compression levels | ✅ `-l` | ✅ `-l` |
| stdin/stdout | ✅ | ✅ |
| Binary size | ~2MB | ~4MB |

---

## Roadmap

### Planned for v0.2.0

- [ ] Parallel compression for multiple files
- [ ] Recursive directory support (`-r` flag)
- [ ] Progress bar for large files
- [ ] Async/await API variant

### Under Consideration

- [ ] zstd compression format support
- [ ] Dictionary-based compression
- [ ] WASM target support

---

## Related

- [Go Implementation](../../go/changelog/CHANGELOG.md) - gzip-go changelog
- [Main Changelog](../../../CHANGELOG.md) - Project-wide changes
- [API Documentation](../../../docs/en/API.md) - Library API reference
- [GitHub Releases](https://github.com/LessUp/build-your-own-tools/releases)

---

**Last Updated**: 2026-04-16
