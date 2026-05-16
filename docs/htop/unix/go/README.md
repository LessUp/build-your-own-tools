# htop-go (Unix)

A minimal htop clone in Go for Unix systems (Linux, macOS).

## Features

- Real-time CPU and memory usage display
- Process list with sorting (CPU%, Mem%, PID, Name)
- TUI built with [tview](https://github.com/rivo/tview)
- System metrics via [gopsutil](https://github.com/shirou/gopsutil)

## Build

```bash
make build
```

## Run

```bash
./bin/htop-go
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `c` | Sort by CPU% |
| `m` | Sort by Mem% |
| `p` | Sort by PID |
| `n` | Sort by Name |

## Architecture

```
htop/unix/go/
├── cmd/htop-go/main.go       # Entry point
├── internal/
│   ├── metrics/               # System & process metrics
│   │   ├── system.go
│   │   └── processes.go
│   └── ui/                    # TUI components
│       └── ui.go
├── go.mod
└── Makefile
```

## Comparison with Rust Version

| Feature | Go (this) | Rust (htop/unix/rust) |
|---------|-----------|----------------------|
| TUI Framework | tview | ratatui |
| Sparkline History | ❌ | ✅ |
| Process Kill | ❌ | ✅ |
| Search Filter | ❌ | ✅ |
| Adjustable Refresh | ❌ | ✅ |

This Go implementation focuses on simplicity and demonstrates idiomatic Go patterns for TUI applications.

## License

MIT OR Apache-2.0
