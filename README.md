# Build-Your-Own-Tools

[![CI](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml)
[![Docs](https://github.com/LessUp/build-your-own-tools/actions/workflows/pages.yml/badge.svg)](https://lessup.github.io/build-your-own-tools/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

English | [简体中文](README.zh-CN.md)

A learning repository for re-implementing common CLI tools from scratch in **Rust / Go**, focusing on low-level implementation, CLI design, and cross-language comparison.

| Sub-project | Language | Description |
|-------------|----------|-------------|
| `dos2unix/` | Rust | Lightweight dos2unix: CRLF → LF conversion |
| `gzip/` | Go, Rust | Minimal gzip compress/decompress CLI |
| `htop/` | Rust, Go | Cross-platform htop TUI task manager (Unix + Windows) |

## Features

- **Multi-Language Implementation** — Same tool in Rust and Go for side-by-side comparison
- **Learning-Focused CLI** — clap / standard-library flag parsing, terminal output, and cross-language implementation comparison
- **Cross-Platform** — Linux, macOS, Windows support
- **Automated Verification** — Unit tests, build checks, CI validation
- **CI/CD** — GitHub Actions with build matrix and automated releases

## Quick Start

### Rust Projects

```bash
cd dos2unix
cargo build --release
cargo test
```

### Go Projects

```bash
cd gzip/go
go build -o gzip-go ./cmd/gzip-go
go test ./...
```

## Project Structure

```
build-your-own-tools/
├── dos2unix/          # Rust dos2unix implementation
├── gzip/
│   ├── go/            # Go gzip implementation
│   └── rust/          # Rust gzip implementation
├── htop/
│   ├── go/            # Go htop implementation
│   └── rust/          # Rust htop implementation
├── docs/              # Documentation
└── .github/workflows/ # CI configuration
```

## Learning Goals

Each sub-project is designed to teach:

1. **File I/O** — Efficient streaming read/write, memory-mapped files
2. **Compression Algorithms** — DEFLATE, LZ77, Huffman coding
3. **System APIs** — Process info, CPU/memory stats, terminal control
4. **CLI Best Practices** — Argument parsing, error handling, exit codes

## License

MIT OR Apache-2.0
