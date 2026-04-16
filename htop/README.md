# htop

[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)

A cross-platform htop clone for learning system programming and TUI development.

## Features

- **Cross-Platform** — Unix and Windows support
- **Multi-Language** — Rust and Go implementations
- **Real-time Monitoring** — CPU and memory usage
- **Process Management** — View, sort, search, and kill processes
- **Interactive TUI** — Keyboard-driven interface

## Implementations

| Platform | Language | Directory | Features |
|----------|----------|-----------|----------|
| Unix | Rust | `unix/rust/` | Full features, SIGTERM→SIGKILL |
| Windows | Rust | `win/rust/` | Full features, sparkline history |
| Windows | Go | `win/go/` | Basic features, sort controls |

## Project Structure

```
htop/
├── shared/           # Shared Rust library (htop_shared)
│   └── src/lib.rs
├── unix/rust/        # Unix Rust implementation
│   └── src/main.rs
├── win/
│   ├── rust/         # Windows Rust implementation
│   │   └── src/main.rs
│   └── go/           # Windows Go implementation
│       └── cmd/htop-win-go/
└── changelog/
    └── CHANGELOG.md
```

## Quick Start

### Unix (Rust)

```bash
cd htop/unix/rust
cargo run --release
```

### Windows (Rust)

```powershell
cd htop\win\rust
cargo run --release
```

### Windows (Go)

```powershell
cd htop\win\go
go run ./cmd/htop-win-go
```

## Shared Library

The `htop_shared` crate provides common functionality:

| Function | Description |
|----------|-------------|
| `ProcRow` | Process data struct |
| `SortKey` | Sort column enum |
| `compare_proc_rows()` | Compare processes for sorting |
| `filter_processes()` | Filter by search term |
| `color_for_ratio()` | Color based on usage percentage |
| `highlight_style()` | Table row highlight style |

## Common Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` | Quit |
| `↑`/`↓` | Navigate |
| `s` | Cycle sort column |
| `r` | Reverse sort |
| `/` | Search |
| `p` | Pause refresh |
| `k` | Kill process |

## Comparison

| Feature | Unix Rust | Win Rust | Win Go |
|---------|-----------|----------|--------|
| Process list | ✅ | ✅ | ✅ |
| CPU/memory | ✅ | ✅ | ✅ |
| Process kill | ✅ | ✅ | ❌ |
| Search/filter | ✅ | ✅ | ❌ |
| Sparkline | ❌ | ✅ | ❌ |
| Shared lib | ✅ | ✅ | ❌ |

## Testing

```bash
# Rust
cargo test --all

# Go
cd win/go && go test ./...
```

## License

MIT OR Apache-2.0
