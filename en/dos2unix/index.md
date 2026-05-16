---
title: dos2unix - CRLF to LF Converter
---

# dos2unix

A Rust implementation of CRLF (Windows) to LF (Unix) line ending converter.

## Overview

This tool converts Windows-style line endings (CRLF, `\r\n`) to Unix-style line endings (LF, `\n`). It's perfect for:
- Normalizing files across different operating systems
- Preparing files for Unix/Linux environments
- Learning about file I/O and streaming processing in Rust

## Features

- ✅ **Streaming processing** - 8KB buffer for memory-efficient large file handling
- ✅ **In-place conversion** - Modify files directly
- ✅ **stdin/stdout support** - Pipeline-friendly
- ✅ **Check mode** - Detect CRLF without modification
- ✅ **Quiet mode** - Minimal output for scripts

## Quick Start

```bash
# Build
cargo build --release -p dos2unix-rust

# Convert a file
./target/release/dos2unix-rust file.txt

# Check for CRLF (exit code 2 if found)
./target/release/dos2unix-rust --check file.txt

# Pipeline usage
echo "Hello\r\nWorld" | ./target/release/dos2unix-rust
```

## Learning Topics

| Topic | Description |
|-------|-------------|
| File I/O | Streaming read/write with buffered I/O |
| Error Handling | Using `anyhow` for ergonomic error propagation |
| CLI Design | Manual argument parsing |
| Cross-platform | Handling different line endings |

## Technical Details

- **Buffer Size**: 8KB for optimal streaming performance
- **Memory Usage**: O(1) - constant memory regardless of file size
- **Performance**: ~120 MB/s on large files

## Source Code

- [Main Implementation](https://github.com/LessUp/build-your-own-tools/tree/master/dos2unix/src)
- [Changelog](/dos2unix/changelog/CHANGELOG.md)

## Related

- [gzip](/gzip/) - Compression tool
- [htop](/htop/) - System monitor
