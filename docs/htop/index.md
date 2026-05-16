---
title: htop - System Monitor
---

# htop

Cross-platform TUI (Terminal User Interface) system monitor with multiple implementations.

## Overview

This project implements an interactive system monitor similar to the popular `htop` utility, with versions for:
- **Unix/Linux** (Rust + ratatui)
- **Windows** (Rust + ratatui)
- **Windows** (Go + tview)

Perfect for learning:
- TUI development
- System programming
- Process management
- Cross-platform development

## Features

### Core Features

- ✅ **Real-time CPU/Memory monitoring**
- ✅ **Process list with sorting** (CPU, Memory, PID, Name)
- ✅ **Process search/filter**
- ✅ **Process kill functionality**
- ✅ **Adjustable refresh interval**
- ✅ **Color-coded usage indicators**

### Platform-Specific

| Feature | Unix Rust | Win Rust | Win Go |
|---------|:---------:|:--------:|:------:|
| Process List | ✅ | ✅ | ✅ |
| CPU/Memory | ✅ | ✅ | ✅ |
| Process Kill | ✅ | ✅ | ✅ |
| Sparkline History | - | ✅ | - |
| Network Stats | Planned | Planned | Planned |

## Quick Start

```bash
# Build Unix version (Linux/macOS)
cargo build --release -p htop-rust

# Run
./target/release/htop-unix-rust

# Build Windows Rust version
cargo build --release -p htop-win-rust

# Build Windows Go version
cd htop/win/go && go build -o bin/htop-win-go ./cmd/htop-win-go
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` | Quit |
| `k` | Kill selected process |
| `/` | Search/filter processes |
| `s` | Change sort column |
| `+`/`-` | Adjust refresh interval |
| `↑`/`↓` | Navigate process list |

## Learning Topics

| Topic | Description |
|-------|-------------|
| TUI Development | ratatui (Rust) / tview (Go) |
| System APIs | Process info, CPU, memory stats |
| Event Handling | Keyboard input, async refresh |
| Cross-platform | Unix vs Windows differences |
| Concurrency | Async refresh loops |

## Architecture

```
htop/
├── shared/          # Shared Rust library
├── unix/rust/       # Unix implementation
└── win/
    ├── rust/        # Windows Rust implementation
    └── go/          # Windows Go implementation
```

## Source Code

- [Shared Library](https://github.com/LessUp/build-your-own-tools/tree/master/htop/shared)
- [Unix Rust](https://github.com/LessUp/build-your-own-tools/tree/master/htop/unix/rust)
- [Windows Rust](https://github.com/LessUp/build-your-own-tools/tree/master/htop/win/rust)
- [Windows Go](https://github.com/LessUp/build-your-own-tools/tree/master/htop/win/go)
- [Changelog](/htop/changelog/CHANGELOG.md)

## Related

- [dos2unix](/dos2unix/) - Line ending converter
- [gzip](/gzip/) - Compression tool
