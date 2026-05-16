---
title: gzip - Compression Tool
---

# gzip

Multi-language implementation (Go + Rust) of the gzip compression/decompression tool.

## Overview

This project provides both Go and Rust implementations of the standard gzip utility, allowing you to compare:
- Different approaches to streaming compression
- Error handling patterns in both languages
- Concurrency models (goroutines vs async)
- CLI library design (pflag vs clap)

## Features

### Both Implementations

- ✅ **Streaming compression/decompression**
- ✅ **Multiple compression levels** (0-9)
- ✅ **Keep source files** (`-k` flag)
- ✅ **Force overwrite** (`-f` flag)
- ✅ **Recursive directory support** (`-r` flag)
- ✅ **stdin/stdout support**

### Go Implementation

- Parallel file processing with goroutines
- Standard library `compress/gzip`
- Built-in concurrency support

### Rust Implementation

- Library crate for embedding
- `clap` derive macros for CLI
- `flate2` for DEFLATE compression

## Quick Start

```bash
# Build Go version
cd gzip/go && go build -o bin/gzip-go ./cmd/gzip-go

# Build Rust version
cargo build --release -p rgzip

# Compress a file
./gzip/go/bin/gzip-go file.txt
./target/release/rgzip file.txt

# Decompress
./gzip/go/bin/gzip-go -d file.txt.gz
./target/release/rgzip -d file.txt.gz

# With options
./target/release/rgzip -k -l 9 file.txt  # Keep source, max compression
```

## Learning Topics

| Topic | Go | Rust |
|-------|:--:|:----:|
| Streaming | ✅ | ✅ |
| Concurrency | ✅ goroutines | ✅ async |
| Error Handling | ✅ error type | ✅ anyhow/Result |
| CLI Framework | ✅ pflag | ✅ clap |
| Library Design | ✅ | ✅ |

## Comparison Highlights

| Aspect | Go | Rust |
|--------|:--:|:----:|
| Binary Size | ~4MB | ~2MB |
| Build Time | Fast | Moderate |
| Memory | GC managed | Ownership system |
| Safety | Runtime checks | Compile-time checks |

## Source Code

- [Go Implementation](https://github.com/LessUp/build-your-own-tools/tree/master/gzip/go)
- [Rust Implementation](https://github.com/LessUp/build-your-own-tools/tree/master/gzip/rust)
- [Go Changelog](/gzip/go/changelog/CHANGELOG.md)
- [Rust Changelog](/gzip/rust/changelog/CHANGELOG.md)

## Related

- [dos2unix](/dos2unix/) - Line ending converter
- [htop](/htop/) - System monitor
