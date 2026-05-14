# API Reference

> API overview of core library modules

**English** | [简体中文](API.zh-CN.md)

---

## Overview

This document provides an API overview of the library crates in the build-your-own-tools project.

### Module Availability

| Module | Language | Type | Public API |
|--------|----------|------|------------|
| dos2unix | Rust | Binary only | ❌ |
| gzip/rust | Rust | Library + Binary | ✅ |
| htop/shared | Rust | Library | ✅ |
| gzip/go | Go | Binary only | ❌ |
| htop/win/go | Go | Binary only | ❌ |

---

## gzip (rgzip)

The Rust implementation of gzip provides a library crate (`rgzip`) for embedding compression functionality.

### Core Functions

```rust
// Compression
pub fn compress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    level: Compression,
) -> io::Result<u64>

pub fn compress_file(input: &Path, output: &Path, level: Compression) -> io::Result<u64>

// Decompression
pub fn decompress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<u64>

pub fn decompress_file(input: &Path, output: &Path) -> io::Result<u64>
```

### Compression Enum

```rust
pub enum Compression {
    Fast,    // Fast compression, lower ratio
    Default, // Default balanced
    Best,    // Best compression, slower
}
```

### Usage Example

```rust
use rgzip::{compress_file, decompress_file, Compression};

// Compress file
compress_file(Path::new("input.txt"), Path::new("output.gz"), Compression::Default)?;

// Decompress file
decompress_file(Path::new("output.gz"), Path::new("output.txt"))?;
```

---

## htop (htop_shared)

The shared library for htop provides process table and sorting functionality.

### Core Types

```rust
/// Sort key
pub enum SortKey {
    Cpu,  // Sort by CPU usage
    Mem,  // Sort by memory usage
    Pid,  // Sort by PID
    Name, // Sort by process name
}

/// Process table row
pub struct ProcRow {
    pub pid: Pid,
    pub name: String,
    pub cpu: f32,
    pub mem_mb: u64,
}
```

### Core Functions

```rust
// Compare process rows
pub fn compare_proc_rows(a: &ProcRow, b: &ProcRow, sort: SortKey) -> Ordering

// Filter processes
pub fn filter_processes(processes: Vec<ProcRow>, filter: &str) -> Vec<ProcRow>

// Get selected PID
pub fn selected_pid(processes: &[ProcRow], selected: usize) -> Option<Pid>

// Resolve selected index
pub fn resolve_selected_index(
    processes: &[ProcRow],
    preferred_pid: Option<Pid>,
    fallback_index: usize,
) -> usize

// Color calculation
pub fn color_for_ratio(ratio: f32) -> Color

// Highlight style
pub fn highlight_style() -> Style
```

### Usage Example

```rust
use htop_shared::{ProcRow, SortKey, compare_proc_rows, filter_processes};

// Sort processes
processes.sort_by(|a, b| compare_proc_rows(a, b, SortKey::Cpu));

// Filter processes
let filtered = filter_processes(processes, "python");
```

---

## dos2unix

> **Note**: dos2unix is currently a binary-only crate; core conversion functions are internal.

### Internal Functions

```rust
// CRLF to LF
fn convert_crlf_to_lf(input: &str) -> String

// LF to CRLF
fn convert_lf_to_crlf(input: &str) -> String
```

---

## Error Handling

All library functions use `std::io::Result` or `anyhow::Result` for error handling:

```rust
// Common error types
- io::ErrorKind::NotFound      // File not found
- io::ErrorKind::InvalidData   // Invalid compressed data
- io::ErrorKind::PermissionDenied // Permission denied
```

---

## More Information

For detailed function documentation, refer to each module's rustdoc:

```bash
cargo doc --open
```
