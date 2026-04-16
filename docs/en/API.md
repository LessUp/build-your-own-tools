# API Reference

> Complete API documentation for all libraries and modules

**English** | [简体中文](../zh-CN/API.md)

---

## Table of Contents

- [Overview](#overview)
- [dos2unix](#dos2unix)
- [gzip (rgzip)](#gzip-rgzip)
- [htop (htop_shared)](#htop-htop_shared)
- [Common Types](#common-types)
- [Error Handling](#error-handling)
- [Examples](#examples)

---

## Overview

This document provides detailed API documentation for the library crates in the build-your-own-tools project. Each module is documented with:

- Function signatures
- Parameter descriptions
- Return values
- Error conditions
- Usage examples

### Module Availability

| Module | Language | Type | Public API |
|--------|----------|------|------------|
| dos2unix | Rust | Binary only | ❌ |
| gzip/rust | Rust | Library + Binary | ✅ |
| htop/shared | Rust | Library | ✅ |
| gzip/go | Go | Binary only | ❌ |
| htop/win/go | Go | Binary only | ❌ |

---

## dos2unix

> **Note**: dos2unix is currently a binary-only crate. The core conversion functions are internal and may be extracted to a library in the future.

### Internal Functions

These functions are documented for reference but are not part of the public API.

#### `convert_crlf_to_lf`

```rust
fn convert_crlf_to_lf(input: &str) -> String
```

Converts all CRLF (`\r\n`) sequences to LF (`\n`).

**Parameters**:
- `input` - Input string containing potential CRLF sequences

**Returns**:
- String with all CRLF replaced by LF

**Example**:
```rust
let input = "line1\r\nline2\r\n";
let output = convert_crlf_to_lf(input);
assert_eq!(output, "line1\nline2\n");
```

---

## gzip (rgzip)

The Rust implementation of gzip provides a library crate (`rgzip`) for embedding compression functionality.

### Module Structure

```
rgzip/
├── lib.rs          # Public API
├── compress.rs     # Compression functions
├── decompress.rs   # Decompression functions
├── utils.rs        # Utility functions
└── main.rs         # CLI entry point
```

### Compression Functions

#### `compress_reader_to_writer`

```rust
pub fn compress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    level: Compression,
) -> io::Result<u64>
```

Compresses data from a reader to a writer using DEFLATE compression.

**Parameters**:
- `reader` - Source of uncompressed data
- `writer` - Destination for compressed data
- `level` - Compression level (`Compression::default()`, `Compression::fast()`, `Compression::best()`, or `Compression::new(0-9)`)

**Returns**:
- `Ok(bytes_written)` - Number of bytes written to output
- `Err(io::Error)` - I/O error during compression

**Example**:
```rust
use flate2::Compression;
use rgzip::compress_reader_to_writer;
use std::fs::File;
use std::io::{BufReader, BufWriter};

let input = File::open("input.txt")?;
let output = File::create("output.txt.gz")?;

let mut reader = BufReader::new(input);
let mut writer = BufWriter::new(output);

let bytes_written = compress_reader_to_writer(
    &mut reader,
    &mut writer,
    Compression::default()
)?;

println!("Compressed {} bytes", bytes_written);
```

#### `compress_file`

```rust
pub fn compress_file(
    input_path: &Path,
    output_path: &Path,
    level: Compression,
) -> io::Result<()>
```

Compresses a file to a gzip archive.

**Parameters**:
- `input_path` - Path to input file
- `output_path` - Path to output `.gz` file
- `level` - Compression level

**Returns**:
- `Ok(())` on success
- `Err(io::Error)` on failure (file not found, permission denied, etc.)

**Errors**:
- `io::ErrorKind::NotFound` - Input file doesn't exist
- `io::ErrorKind::PermissionDenied` - Insufficient permissions
- `io::ErrorKind::AlreadyExists` - Output file exists (without force flag)

**Example**:
```rust
use flate2::Compression;
use rgzip::compress_file;
use std::path::Path;

compress_file(
    Path::new("document.txt"),
    Path::new("document.txt.gz"),
    Compression::best()
)?;
```

### Decompression Functions

#### `decompress_reader_to_writer`

```rust
pub fn decompress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<u64>
```

Decompresses gzip data from a reader to a writer.

**Parameters**:
- `reader` - Source of compressed data (must be gzip format)
- `writer` - Destination for decompressed data

**Returns**:
- `Ok(bytes_written)` - Number of bytes written to output
- `Err(io::Error)` - I/O error or invalid gzip data

**Errors**:
- `io::ErrorKind::InvalidData` - Input is not valid gzip data
- `io::ErrorKind::UnexpectedEof` - Truncated gzip data

**Example**:
```rust
use rgzip::decompress_reader_to_writer;
use std::fs::File;
use std::io::{BufReader, BufWriter};

let input = File::open("archive.txt.gz")?;
let output = File::create("archive.txt")?;

let mut reader = BufReader::new(input);
let mut writer = BufWriter::new(output);

let bytes_written = decompress_reader_to_writer(&mut reader, &mut writer)?;
println!("Decompressed {} bytes", bytes_written);
```

#### `decompress_file`

```rust
pub fn decompress_file(
    input_path: &Path,
    output_path: &Path,
) -> io::Result<()>
```

Decompresses a gzip archive to a file.

**Parameters**:
- `input_path` - Path to input `.gz` file
- `output_path` - Path to output file

**Returns**:
- `Ok(())` on success
- `Err(io::Error)` on failure

**Example**:
```rust
use rgzip::decompress_file;
use std::path::Path;

decompress_file(
    Path::new("backup.txt.gz"),
    Path::new("backup.txt")
)?;
```

### Utility Functions

#### `default_output_path`

```rust
pub fn default_output_path(
    input_path: &Path,
    decompress: bool,
) -> PathBuf
```

Generates the default output path for compression or decompression.

**Behavior**:
- Compression: Adds `.gz` suffix
- Decompression: Removes `.gz` suffix, or adds `.out` if no suffix

**Parameters**:
- `input_path` - Input file path
- `decompress` - Whether this is for decompression

**Returns**:
- Generated output path

**Example**:
```rust
use rgzip::default_output_path;
use std::path::Path;

// Compression
let out = default_output_path(Path::new("file.txt"), false);
assert_eq!(out, PathBuf::from("file.txt.gz"));

// Decompression
let out = default_output_path(Path::new("file.txt.gz"), true);
assert_eq!(out, PathBuf::from("file.txt"));
```

#### `sanitize_level`

```rust
pub fn sanitize_level(level: u32) -> Compression
```

Converts a raw compression level to a valid `Compression` value.

**Parameters**:
- `level` - Raw level (0-9, but accepts any u32)

**Returns**:
- `Compression` clamped to valid range (0-9)

**Example**:
```rust
use flate2::Compression;
use rgzip::sanitize_level;

assert_eq!(sanitize_level(5), Compression::new(5));
assert_eq!(sanitize_level(15), Compression::best()); // clamped to 9
assert_eq!(sanitize_level(0), Compression::none());
```

### Complete Example

```rust
use rgzip::{compress_file, decompress_file, sanitize_level};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compress
    compress_file(
        Path::new("data.txt"),
        Path::new("data.txt.gz"),
        sanitize_level(6)
    )?;
    
    println!("Compression complete!");
    
    // Decompress
    decompress_file(
        Path::new("data.txt.gz"),
        Path::new("data_restored.txt")
    )?;
    
    println!("Decompression complete!");
    
    Ok(())
}
```

---

## htop (htop_shared)

Shared library for htop implementations providing common data structures and utilities.

### Module Structure

```
htop_shared/
├── lib.rs              # Public API
├── models.rs           # Data structures
├── sorting.rs          # Sorting logic
├── filtering.rs        # Filtering logic
└── styling.rs          # Terminal styling
```

### Data Structures

#### `ProcRow`

Process information row for display.

```rust
pub struct ProcRow {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_mib: u64,
    pub status: String,
    pub command: String,
    pub exe_path: String,
}
```

**Fields**:
| Field | Type | Description |
|-------|------|-------------|
| `pid` | `u32` | Process ID |
| `name` | `String` | Process name |
| `cpu_percent` | `f32` | CPU usage percentage |
| `mem_mib` | `u64` | Memory usage in MiB |
| `status` | `String` | Process status (Running, Sleeping, etc.) |
| `command` | `String` | Full command line |
| `exe_path` | `String` | Executable path |

**Example**:
```rust
use htop_shared::ProcRow;

let row = ProcRow {
    pid: 1234,
    name: "firefox".to_string(),
    cpu_percent: 15.5,
    mem_mib: 512,
    status: "Running".to_string(),
    command: "/usr/bin/firefox".to_string(),
    exe_path: "/usr/lib/firefox/firefox".to_string(),
};
```

#### `SortKey`

Column key for process sorting.

```rust
pub enum SortKey {
    PID,
    Name,
    CPU,
    Memory,
}
```

**Variants**:
- `PID` - Sort by process ID
- `Name` - Sort by process name (alphabetical)
- `CPU` - Sort by CPU usage (descending)
- `Memory` - Sort by memory usage (descending)

**Example**:
```rust
use htop_shared::SortKey;

let sort = SortKey::CPU; // Sort by CPU usage
```

### Functions

#### `compare_proc_rows`

```rust
pub fn compare_proc_rows(
    a: &ProcRow,
    b: &ProcRow,
    sort_key: SortKey,
    ascending: bool,
) -> Ordering
```

Compares two process rows for sorting.

**Parameters**:
- `a` - First process row
- `b` - Second process row
- `sort_key` - Column to sort by
- `ascending` - Sort direction (true = ascending, false = descending)

**Returns**:
- `Ordering` - Comparison result

**Example**:
```rust
use htop_shared::{compare_proc_rows, ProcRow, SortKey};

let rows: Vec<ProcRow> = /* fetch processes */;
let mut sorted = rows.clone();
sorted.sort_by(|a, b| compare_proc_rows(a, b, SortKey::CPU, false));
```

#### `filter_processes`

```rust
pub fn filter_processes(
    processes: &[ProcRow],
    query: &str,
) -> Vec<ProcRow>
```

Filters processes by a search query.

**Parameters**:
- `processes` - Source process list
- `query` - Search string (case-insensitive, matches name or command)

**Returns**:
- Filtered vector of matching processes

**Example**:
```rust
use htop_shared::{filter_processes, ProcRow};

let all: Vec<ProcRow> = /* fetch all processes */;
let filtered = filter_processes(&all, "firefox");
// Returns processes with "firefox" in name or command
```

#### `color_for_ratio`

```rust
pub fn color_for_ratio(ratio: f32) -> Color
```

Returns an appropriate color for a usage ratio (0.0 - 1.0).

**Color Thresholds**:
| Ratio Range | Color |
|-------------|-------|
| 0.0 - 0.6 | Green |
| 0.6 - 0.85 | Yellow |
| 0.85 - 1.0 | Red |

**Parameters**:
- `ratio` - Usage ratio (0.0 to 1.0)

**Returns**:
- `Color` - ratatui color for display

**Example**:
```rust
use htop_shared::color_for_ratio;
use ratatui::style::Color;

let cpu_color = color_for_ratio(0.75); // Returns Color::Yellow
let mem_color = color_for_ratio(0.95); // Returns Color::Red
```

#### `highlight_style`

```rust
pub fn highlight_style() -> Style
```

Returns the default style for highlighted/selected table rows.

**Returns**:
- `Style` - ratatui style with blue background

**Example**:
```rust
use htop_shared::highlight_style;
use ratatui::widgets::Table;

let table = Table::new(/* rows */)
    .row_highlight_style(highlight_style());
```

#### `resolve_selected_index`

```rust
pub fn resolve_selected_index(
    current: usize,
    filtered_count: usize,
) -> usize
```

Resolves the selected index after filtering, ensuring it stays within bounds.

**Parameters**:
- `current` - Currently selected index
- `filtered_count` - Number of items after filtering

**Returns**:
- `usize` - Validated index

**Example**:
```rust
use htop_shared::resolve_selected_index;

let selected = 10;
let filtered = 5;
let new_selected = resolve_selected_index(selected, filtered);
// Returns 4 (last valid index)
```

#### `selected_pid`

```rust
pub fn selected_pid(
    processes: &[ProcRow],
    selected_index: usize,
) -> Option<u32>
```

Gets the PID of the process at the selected index.

**Parameters**:
- `processes` - Process list
- `selected_index` - Selected index in the list

**Returns**:
- `Some(pid)` - PID if index is valid
- `None` - If index is out of bounds

**Example**:
```rust
use htop_shared::{selected_pid, ProcRow};

let processes: Vec<ProcRow> = /* fetch */;
if let Some(pid) = selected_pid(&processes, 0) {
    println!("Selected PID: {}", pid);
}
```

### Complete Example

```rust
use htop_shared::{
    compare_proc_rows, filter_processes, color_for_ratio,
    highlight_style, selected_pid, ProcRow, SortKey
};
use ratatui::widgets::Table;

fn main() {
    // Fetch processes (platform-specific)
    let processes: Vec<ProcRow> = fetch_processes();
    
    // Filter
    let filtered = filter_processes(&processes, "firefox");
    
    // Sort by CPU usage
    let mut sorted = filtered.clone();
    sorted.sort_by(|a, b| compare_proc_rows(a, b, SortKey::CPU, false));
    
    // Create table with highlighting
    let table = Table::new(sorted.iter().map(|p| /* row conversion */))
        .row_highlight_style(highlight_style());
    
    // Get selected PID
    if let Some(pid) = selected_pid(&sorted, 0) {
        println!("Kill PID: {}", pid);
    }
}
```

---

## Common Types

### Error Types

All functions return `std::io::Error` for consistency:

```rust
use std::io::{Error, ErrorKind};

// Creating errors
let not_found = Error::new(ErrorKind::NotFound, "file not found");
let invalid = Error::new(ErrorKind::InvalidData, "corrupt gzip data");
```

### Path Handling

```rust
use std::path::{Path, PathBuf};

// From string
let path = Path::new("/home/user/file.txt");

// From PathBuf
let pathbuf = PathBuf::from("/home/user/file.txt");

// Extension manipulation
assert_eq!(path.extension(), Some("txt".as_ref()));
```

---

## Error Handling

### Rust Patterns

**Using `?` operator**:
```rust
fn process_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;  // Returns early on error
    let mut reader = BufReader::new(file);
    // ...
    Ok(())
}
```

**Using `match`**:
```rust
match compress_file(input, output, level) {
    Ok(()) => println!("Success!"),
    Err(e) => eprintln!("Error: {}", e),
}
```

**Using `if let`**:
```rust
if let Err(e) = decompress_file(input, output) {
    eprintln!("Decompression failed: {}", e);
}
```

---

## Examples

### Example 1: Batch Compression

```rust
use rgzip::{compress_file, sanitize_level};
use std::path::Path;

fn compress_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().is_none() {
            let output = path.with_extension("gz");
            compress_file(&path, &output, sanitize_level(6))?;
            println!("Compressed: {} -> {}", path.display(), output.display());
        }
    }
    Ok(())
}
```

### Example 2: Process Monitor

```rust
use htop_shared::{filter_processes, compare_proc_rows, ProcRow, SortKey};

fn find_top_cpu_processes(processes: &[ProcRow], n: usize) -> Vec<ProcRow> {
    let mut sorted = processes.to_vec();
    sorted.sort_by(|a, b| compare_proc_rows(a, b, SortKey::CPU, false));
    sorted.into_iter().take(n).collect()
}

fn monitor_processes(name: &str) {
    // Fetch processes (platform-specific implementation)
    let all = fetch_all_processes();
    let matching = filter_processes(&all, name);
    let top = find_top_cpu_processes(&matching, 5);
    
    for p in top {
        println!("{}: CPU {:.1}%, MEM {} MiB", 
            p.name, p.cpu_percent, p.mem_mib);
    }
}
```

### Example 3: Streaming Pipeline

```rust
use rgzip::compress_reader_to_writer;
use flate2::Compression;
use std::io::{self, Read, Write};

fn pipe_compress() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();
    
    let bytes = compress_reader_to_writer(
        &mut reader,
        &mut writer,
        Compression::fast()
    )?;
    
    eprintln!("Compressed {} bytes", bytes);
    Ok(())
}
```

---

**Last Updated**: 2026-04-16  
**Version**: 1.0
