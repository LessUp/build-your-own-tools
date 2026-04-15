# Build-Your-Own-Tools

[![CI](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml)
[![Docs](https://github.com/LessUp/build-your-own-tools/actions/workflows/pages.yml/badge.svg)](https://lessup.github.io/build-your-own-tools/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/Go-1.21+-00ADD8.svg)](https://golang.org/)

**English** | [简体中文](README.zh-CN.md)

A learning-focused repository for re-implementing common CLI tools from scratch in **Rust** and **Go**. Perfect for understanding low-level system programming, CLI design patterns, and cross-language implementation comparisons.

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/LessUp/build-your-own-tools.git
cd build-your-own-tools

# Build all Rust projects
make build-rust

# Build all Go projects
make build-go

# Run all tests
make test-all
```

## 📦 Projects

| Project | Languages | Description | Status |
|---------|-----------|-------------|--------|
| [dos2unix](./dos2unix/) | Rust | CRLF → LF line ending converter | ✅ Stable |
| [gzip](./gzip/) | Rust, Go | Gzip compression/decompression CLI | ✅ Stable |
| [htop](./htop/) | Rust, Go | Cross-platform TUI system monitor | ✅ Stable |

## 🎯 Features

- **Multi-Language Implementation** — Same tool in Rust and Go for side-by-side comparison
- **Cross-Platform** — Linux, macOS, Windows support
- **Production-Ready** — Unit tests, CI/CD, automated releases
- **Well-Documented** — Architecture docs, API references, comparison guides

## 🏗️ Project Structure

```
build-your-own-tools/
├── dos2unix/                 # CRLF to LF converter
│   └── src/main.rs          # Rust implementation
├── gzip/
│   ├── rust/                 # Rust gzip implementation
│   │   ├── src/lib.rs       # Core library
│   │   └── src/main.rs      # CLI entry point
│   └── go/                   # Go gzip implementation
│       └── cmd/gzip-go/
├── htop/
│   ├── shared/               # Shared Rust library
│   ├── unix/rust/            # Unix htop (Rust)
│   ├── win/rust/             # Windows htop (Rust)
│   └── win/go/               # Windows htop (Go)
├── docs/                     # Documentation
│   ├── ARCHITECTURE.md       # System architecture
│   ├── COMPARISON.md         # Rust vs Go comparison
│   └── API.md               # API documentation
└── Makefile                  # Unified build commands
```

## 🛠️ Development

### Prerequisites

- **Rust** 1.70+ ([install](https://rustup.rs/))
- **Go** 1.21+ ([install](https://golang.org/dl/))
- **make** (optional, for convenience)

### Build Commands

```bash
# Build everything
make build-all

# Build specific projects
make build-dos2unix
make build-gzip-rust
make build-gzip-go
make build-htop-unix-rust
make build-htop-win-rust

# Run tests
make test-all
make test-rust
make test-go

# Lint code
make lint-all
make lint-rust
make lint-go

# Format code
make fmt-all
```

### Development Workflow

```bash
# 1. Format code
make fmt-all

# 2. Run linter
make lint-all

# 3. Run tests
make test-all

# 4. Build release
make build-all
```

## 📖 Documentation

- [Architecture Guide](docs/ARCHITECTURE.md) — System design and patterns
- [Rust vs Go Comparison](docs/COMPARISON.md) — Language trade-offs
- [API Reference](docs/API.md) — Function documentation
- [Contributing Guide](CONTRIBUTING.md) — How to contribute

## 🧪 Testing

```bash
# Run all tests with verbose output
cargo test --all -- --nocapture
go test -v ./...

# Run specific test
cargo test -p dos2unix-rust test_stream_large_data
go test -C gzip/go -run TestGzipStream
```

## 📋 Learning Goals

Each sub-project teaches:

| Topic | dos2unix | gzip | htop |
|-------|----------|------|------|
| File I/O | ✅ Streaming | ✅ Streaming | - |
| CLI Design | ✅ Manual args | ✅ clap/pflag | ✅ Interactive |
| Error Handling | ✅ anyhow | ✅ anyhow | ✅ anyhow |
| Compression | - | ✅ DEFLATE | - |
| System APIs | - | - | ✅ Process info |
| TUI | - | - | ✅ ratatui/tview |
| Concurrency | - | ✅ goroutines | ✅ Async refresh |

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](http://opensource.org/licenses/MIT))

at your option.

## 🙏 Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) — Rust TUI framework
- [tview](https://github.com/rivo/tview) — Go TUI framework
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) — Rust system info
- [gopsutil](https://github.com/shirou/gopsutil) — Go system info
- [flate2](https://github.com/rust-lang/flate2-rs) — Rust DEFLATE compression
