# API Specifications

> API interface definitions for machine and human readability

## Overview

This directory contains API specifications and interface definitions for all tools in the project.

## Current API Documents

The detailed API documentation for library modules can be found in:
- [API Reference](../../docs/en/API.md) - Complete API documentation for all library modules
- [API Reference (中文)](../../docs/zh-CN/API.md) - API参考文档（中文）

## REST API / GraphQL

This project does not expose REST or GraphQL APIs as it consists of CLI tools.

## CLI Interface Specifications

### dos2unix

```
dos2unix [OPTIONS] [FILE]

Options:
  -h, --help     Print help information
  -V, --version  Print version information
  -c, --check    Check mode (detect CRLF, no modification)
  -i, --inplace  Convert file in-place
```

### gzip

```
gzip [OPTIONS] [FILE...]

Options:
  -h, --help             Print help information
  -V, --version          Print version information
  -d, --decompress       Decompress mode
  -k, --keep             Keep source files
  -r, --recursive        Process directories recursively
  -l, --level <LEVEL>    Compression level (0-9, default: 6)
```

### htop

```
htop [OPTIONS]

Options:
  -h, --help             Print help information
  -V, --version          Print version information
  -s, --sort <COLUMN>    Sort column (cpu, mem, pid, name)
  -i, --interval <MS>    Refresh interval in milliseconds
```

## Library API (Rust)

### rgzip Library

```rust
pub fn compress_reader_to_writer<R: Read, W: Write>(
    reader: R,
    writer: W,
    level: u32,
) -> io::Result<W>

pub fn decompress_reader_to_writer<R: Read, W: Write>(
    reader: R,
    writer: W,
) -> io::Result<()>

pub fn compress_path(input_path: &Path, output_path: &Path, level: u32) -> io::Result<()>
pub fn decompress_path(input_path: &Path, output_path: &Path) -> io::Result<()>
```

### htop_shared Library

```rust
pub struct ProcRow {
    pub pid: Pid,
    pub name: String,
    pub cpu: f32,
    pub mem_mb: u64,
}

pub enum SortKey {
    Cpu,
    Mem,
    Pid,
}

pub fn compare_proc_rows(a: &ProcRow, b: &ProcRow, sort_key: SortKey) -> Ordering
pub fn filter_processes(processes: Vec<ProcRow>, query: &str) -> Vec<ProcRow>
pub fn color_for_ratio(ratio: f32) -> Color
```
