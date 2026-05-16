# htop-rust (Unix)

[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../../../LICENSE)

A minimal htop clone for Unix systems built with Rust and ratatui.

## Features

- **Real-time Monitoring** — CPU usage, memory consumption
- **Process Management** — View, search, sort, and kill processes
- **Interactive TUI** — Built with ratatui for responsive terminal UI
- **Process Details** — View detailed process information
- **Flexible Refresh** — Adjustable refresh interval

## Installation

```bash
cd htop/unix/rust
cargo build --release

# Binary location
./target/release/htop-unix-rust
```

## Usage

Run directly:
```bash
cargo run --release
```

## Keyboard Shortcuts

### Navigation
| Key | Action |
|-----|--------|
| `↑`/`↓` | Select previous/next process |
| `PgUp`/`PgDn` | Page up/down |
| `Home`/`End` | Jump to first/last |

### Sorting
| Key | Action |
|-----|--------|
| `s` | Cycle sort column (CPU → MEM → PID → Name) |
| `r` | Reverse sort order |

### Search/Filter
| Key | Action |
|-----|--------|
| `/` | Enter search mode |
| `Enter` | Apply filter |
| `Esc` | Clear filter |
| `Backspace` | Delete character |

### Refresh Control
| Key | Action |
|-----|--------|
| `p` | Pause/resume refresh |
| `F5` | Force refresh |
| `-` | Slow down (max 5000ms) |
| `+`/`=` | Speed up (min 100ms) |

### Process Actions
| Key | Action |
|-----|--------|
| `k` | Kill selected process (SIGTERM → SIGKILL) |
| `d`/`Enter` | Show/hide process details |

### General
| Key | Action |
|-----|--------|
| `q` | Quit |
| `Esc` | Close popup/cancel input |

## Process Kill Behavior

On Unix systems:
1. First sends `SIGTERM` for graceful termination
2. If process still alive after 2 seconds, sends `SIGKILL`

**Note:** Some system processes require root privileges to kill.

## Display

### Header
- CPU: Average usage across all cores
- Memory: Used/Total (MiB) with percentage

### Process Table
| Column | Description |
|--------|-------------|
| PID | Process ID |
| NAME | Process name |
| CPU% | CPU usage percentage |
| MEM(MiB) | Memory usage in MiB |

### Process Details
- Name and status
- CPU% and memory
- Executable path
- Command line arguments

## Dependencies

- **ratatui** — Terminal UI framework
- **crossterm** — Cross-platform terminal manipulation
- **sysinfo** — System information

## Testing

```bash
cargo test
```

## Comparison with Windows Implementation

| Feature | Unix | Windows |
|---------|------|---------|
| Process kill | SIGTERM → SIGKILL | taskkill |
| Sparkline history | ❌ | ✅ |
| Shared library | ✅ htop_shared | ✅ htop_shared |

## License

MIT OR Apache-2.0
