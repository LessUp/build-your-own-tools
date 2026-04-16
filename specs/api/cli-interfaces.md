# API/CLI Interface Specifications

> API interface definitions for machine and human readability

| Metadata | Value |
|----------|-------|
| **Version** | 1.0 |
| **Last Updated** | 2026-04-17 |

---

## Overview

This document defines the CLI interface specifications for all tools in the Build-Your-Own-Tools project. All implementations must conform to these specifications.

---

## CLI Interface Definitions

### dos2unix

**Purpose**: Convert CRLF line endings to LF

```
dos2unix [OPTIONS] [FILE]

Arguments:
  [FILE]  Input file path (use stdin if not specified)

Options:
  -h, --help       Print help information
  -V, --version    Print version information
  -c, --check      Check mode (detect CRLF, no modification)
  -i, --inplace    Convert file in-place
  -q, --quiet      Suppress output messages

Exit Codes:
  0  Success (no CRLF found or conversion complete)
  1  Error occurred
  2  CRLF detected in check mode

Examples:
  # Convert file in-place
  dos2unix -i file.txt
  
  # Check for CRLF without modifying
  dos2unix -c file.txt
  
  # Pipeline usage
  cat file.txt | dos2unix > output.txt
```

---

### gzip

**Purpose**: Compress or decompress files using DEFLATE algorithm

```
gzip [OPTIONS] [FILE...]

Arguments:
  [FILE...]  Input file(s) to process

Options:
  -h, --help              Print help information
  -V, --version           Print version information
  -d, --decompress        Decompress mode
  -k, --keep              Keep source files after processing
  -f, --force             Overwrite existing output files
  -r, --recursive         Process directories recursively [Go only]
  -l, --level <LEVEL>     Compression level (0-9, default: 6)
  -o, --output <PATH>     Output file path [Rust only]

Exit Codes:
  0  Success
  1  Error occurred
  2  Warning (file not found, etc.)

Examples:
  # Compress a file
  gzip file.txt
  
  # Decompress a file
  gzip -d file.txt.gz
  
  # High compression
  gzip -l 9 file.txt
  
  # Recursive compression [Go only]
  gzip -r ./directory
  
  # Keep original file
  gzip -k file.txt
```

---

### htop

**Purpose**: Interactive system monitoring TUI

```
htop [OPTIONS]

Options:
  -h, --help              Print help information
  -V, --version           Print version information
  -s, --sort <COLUMN>     Sort column (cpu, mem, pid, name)
  -i, --interval <MS>     Refresh interval in milliseconds (default: 1000)
  -p, --pid <PID>         Show only the given PID(s)

Keyboard Shortcuts:
  ↑/↓     Navigate process list
  /       Search/filter processes
  k       Kill selected process
  F5      Refresh
  F6      Sort by column
  q       Quit

Exit Codes:
  0  Normal exit
  1  Error occurred

Examples:
  # Start with default settings
  htop
  
  # Sort by memory usage
  htop -s mem
  
  # Fast refresh rate
  htop -i 500
  
  # Monitor specific PID
  htop -p 1234,5678
```

---

## Library API Specifications

### rgzip Library (Rust)

The Rust gzip implementation provides a library crate for embedding compression functionality.

#### Compression Functions

```rust
/// Compress data from reader to writer
///
/// # Arguments
/// * `reader` - Source of uncompressed data
/// * `writer` - Destination for compressed data
/// * `level` - Compression level (0-9)
///
/// # Returns
/// Number of bytes written to output
pub fn compress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    level: Compression,
) -> io::Result<u64>

/// Compress a file to gzip format
///
/// # Arguments
/// * `input_path` - Path to input file
/// * `output_path` - Path to output .gz file
/// * `level` - Compression level (0-9)
pub fn compress_file(
    input_path: &Path,
    output_path: &Path,
    level: Compression,
) -> io::Result<()>
```

#### Decompression Functions

```rust
/// Decompress gzip data from reader to writer
///
/// # Arguments
/// * `reader` - Source of compressed data (must be gzip format)
/// * `writer` - Destination for decompressed data
///
/// # Returns
/// Number of bytes written to output
pub fn decompress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<u64>

/// Decompress a gzip file
///
/// # Arguments
/// * `input_path` - Path to input .gz file
/// * `output_path` - Path to output file
pub fn decompress_file(
    input_path: &Path,
    output_path: &Path,
) -> io::Result<()>
```

#### Utility Functions

```rust
/// Generate default output path for compression/decompression
///
/// # Behavior
/// - Compression: adds .gz suffix
/// - Decompression: removes .gz suffix, or adds .out if no suffix
pub fn default_output_path(input_path: &Path, decompress: bool) -> PathBuf

/// Sanitize compression level to valid range (0-9)
pub fn sanitize_level(level: u32) -> Compression
```

---

### htop_shared Library (Rust)

The shared library for htop implementations.

#### Data Structures

```rust
/// Process information for display
pub struct ProcRow {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_mib: u64,
    pub status: String,
    pub command: String,
    pub exe_path: String,
}

/// Sort column keys
pub enum SortKey {
    PID,
    Name,
    CPU,
    Memory,
}
```

#### Functions

```rust
/// Compare two process rows for sorting
pub fn compare_proc_rows(
    a: &ProcRow,
    b: &ProcRow,
    sort_key: SortKey,
    ascending: bool,
) -> Ordering

/// Filter processes by search query (case-insensitive)
pub fn filter_processes(processes: &[ProcRow], query: &str) -> Vec<ProcRow>

/// Get color for usage ratio (0.0 - 1.0)
///
/// # Thresholds
/// - 0.0-0.6: Green
/// - 0.6-0.85: Yellow
/// - 0.85-1.0: Red
pub fn color_for_ratio(ratio: f32) -> Color

/// Get highlight style for selected rows
pub fn highlight_style() -> Style

/// Resolve selected index to valid range
pub fn resolve_selected_index(current: usize, filtered_count: usize) -> usize

/// Get PID at selected index
pub fn selected_pid(processes: &[ProcRow], selected_index: usize) -> Option<u32>
```

---

## Error Handling Specifications

### Rust Error Types

All functions return `std::io::Error` with appropriate error kinds:

| Error Kind | Description |
|------------|-------------|
| `NotFound` | Input file not found |
| `PermissionDenied` | Insufficient permissions |
| `AlreadyExists` | Output file exists (without --force) |
| `InvalidData` | Invalid gzip data |
| `UnexpectedEof` | Truncated gzip data |

### Go Error Handling

Functions return `error` with descriptive messages:

```go
// Standard error messages
var (
    ErrFileNotFound      = errors.New("file not found")
    ErrPermissionDenied  = errors.New("permission denied")
    ErrInvalidGzip       = errors.New("invalid gzip format")
    ErrTruncatedData     = errors.New("truncated gzip data")
)
```

---

## Cross-Platform Considerations

### Path Handling

- Use forward slashes in documentation
- Implementations must handle platform-specific separators
- `--output` paths should be validated

### Line Endings

- Internal processing uses LF only
- dos2unix converts CRLF to LF
- Output should be LF-terminated

### Signal Handling

- Unix: Handle SIGTERM, SIGINT gracefully
- Windows: Handle Ctrl+C, Ctrl+Break

---

## Backward Compatibility

### Version Compatibility Rules

1. New options must not change default behavior
2. Deprecated options must work for at least one major version
3. Exit codes must remain stable
4. Output format changes require major version bump

### Deprecation Process

1. Mark option as deprecated in documentation
2. Add runtime warning message
3. Remove in next major version

---

**Last Updated**: 2026-04-17
