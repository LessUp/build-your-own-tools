# Architecture

This document describes the overall architecture and design philosophy of the build-your-own-tools project.

## Overview

**build-your-own-tools** is a learning-focused repository that re-implements common CLI tools from scratch in Rust and Go. Perfect for understanding low-level system programming, CLI design patterns, and cross-language implementation comparisons.

## Design Philosophy

### 1. Learning First

- Clean, readable code optimized for understanding
- Detailed comments and documentation for each tool
- Prioritize readability over micro-optimizations

### 2. Multi-Language Implementation

- Same tool implemented in both Rust and Go
- Side-by-side comparison of language features
- Demonstrates strengths of each language in system programming

### 3. Cross-Platform Support

- Support for Linux, macOS, and Windows
- Handle platform differences gracefully
- Use conditional compilation for platform-specific code

## Project Structure

```
build-your-own-tools/
├── dos2unix/                 # CRLF → LF converter (Rust)
│   ├── src/main.rs
│   └── changelog/CHANGELOG.md
├── gzip/
│   ├── go/                   # Go implementation
│   │   ├── cmd/gzip-go/
│   │   └── changelog/CHANGELOG.md
│   └── rust/                 # Rust implementation
│       ├── src/{lib.rs, main.rs}
│       └── changelog/CHANGELOG.md
├── htop/
│   ├── shared/               # Shared Rust library
│   │   └── src/lib.rs
│   ├── unix/rust/            # Unix Rust implementation
│   │   └── src/main.rs
│   ├── win/
│   │   ├── go/               # Windows Go implementation
│   │   │   └── cmd/htop-win-go/
│   │   └── rust/             # Windows Rust implementation
│   │       └── src/main.rs
│   └── changelog/CHANGELOG.md
├── docs/
│   ├── ARCHITECTURE.md       # This file
│   └── COMPARISON.md         # Rust vs Go comparison
├── .github/
│   ├── workflows/
│   │   ├── ci.yml            # CI pipeline
│   │   ├── release.yml       # Release automation
│   │   └── pages.yml         # Docs deployment
│   └── ISSUE_TEMPLATE/
└── .kiro/specs/              # Project specification docs
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                   build-your-own-tools                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │  dos2unix   │  │      gzip       │  │      htop       │  │
│  │   (Rust)    │  │                 │  │                 │  │
│  │             │  │  ┌─────┐ ┌────┐ │  │  ┌─────┐ ┌────┐ │  │
│  │ • File I/O  │  │  │ Go  │ │Rust│ │  │  │Unix │ │Win │ │  │
│  │ • Streaming │  │  └─────┘ └────┘ │  │  │Rust │ │Rust│ │  │
│  │ • CRLF→LF   │  │                 │  │  └─────┘ └────┘ │  │
│  └─────────────┘  │  • DEFLATE      │  │                 │  │
│                   │  • Streaming    │  │  ┌─────┐ ┌────┐ │  │
│                   │  • Parallel     │  │  │ Go  │ │shared│ │  │
│                   └─────────────────┘  │  │(Win)│ │lib │ │  │
│                                        │  └─────┘ └────┘ │  │
│                                        │                 │  │
│                                        │  • TUI          │  │
│                                        │  • Process info │  │
│                                        │  • Real-time    │  │
│                                        └─────────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Sub-Project Details

### dos2unix

A simple text processing tool demonstrating basic file I/O and streaming.

**Key Features:**
- Streaming processing for large files
- Cross-buffer CRLF detection
- Check mode (detect without modify)
- stdin/stdout support

**Dependencies:** `anyhow`

### gzip

File compression tool demonstrating DEFLATE algorithm and stream processing.

**Rust (`rgzip`):**
- Library crate (`lib.rs`) for embedding
- Streaming compression/decompression
- CLI with clap

**Go (`gzip-go`):**
- Parallel file processing
- Recursive directory support
- Built-in goroutines for concurrency

**Dependencies:** `flate2` (Rust), `compress/gzip` (Go)

### htop

System monitor demonstrating TUI development and system information APIs.

**Shared Library (`htop_shared`):**
- `ProcRow` struct for process data
- `SortKey` enum for sorting
- `color_for_ratio()` for usage coloring

**Unix Rust:**
- Uses `sysinfo` for process info
- `ratatui` for TUI
- SIGTERM → SIGKILL for process kill

**Windows Rust:**
- Same stack as Unix
- Sparkline history for CPU/memory

**Windows Go:**
- Uses `gopsutil` for process info
- `tview` for TUI

**Dependencies:** `sysinfo`, `ratatui`, `crossterm` (Rust), `gopsutil`, `tview` (Go)

## Build System

### Cargo Workspace (Rust)

```toml
[workspace]
members = [
    "dos2unix",
    "gzip/rust",
    "htop/shared",
    "htop/unix/rust",
    "htop/win/rust",
]
```

### Go Workspace

```go
go 1.23

use (
    ./gzip/go
    ./htop/win/go
)
```

### Makefile Targets

```bash
make build-all      # Build all projects
make test-all       # Run all tests
make lint-all       # Lint all code
make fmt-all        # Format all code
```

## CI/CD Pipeline

```
Push/PR ──► CI Pipeline
              │
              ├── Rust (Linux, macOS, Windows)
              │   ├── cargo fmt --check
              │   ├── cargo clippy
              │   ├── cargo test
              │   └── cargo build --release
              │
              ├── Go gzip (Linux, macOS)
              │   ├── gofmt
              │   ├── go vet
              │   ├── go test
              │   └── go build
              │
              └── Go htop (Windows)
                  └── (same as above)

Tag Push ──► Release Pipeline
              │
              ├── Build binaries (all platforms)
              ├── Package artifacts
              └── Create GitHub Release
```

## Extension Guide

### Adding a New Tool

1. Create subdirectory at project root
2. Add to Cargo workspace or Go workspace
3. Create `README.md` and `changelog/CHANGELOG.md`
4. Update root `README.md` projects table
5. Add to CI matrix if needed

### Adding a New Language Implementation

1. Create language subdirectory in existing tool
2. Implement same functionality
3. Add build configuration
4. Update documentation

## References

- [Rust Book](https://doc.rust-lang.org/book/)
- [Go Documentation](https://golang.org/doc/)
- [ratatui](https://github.com/ratatui-org/ratatui)
- [tview](https://github.com/rivo/tview)
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
- [gopsutil](https://github.com/shirou/gopsutil)
