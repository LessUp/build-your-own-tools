# htop-win-rust

[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../../../LICENSE)

A minimal htop clone for Windows built with Rust and ratatui, featuring sparkline history charts.

## Features

- **Real-time Monitoring** — CPU and memory usage with visual gauges
- **Sparkline History** — Visual history charts for CPU/memory trends
- **Process Management** — View, search, sort, and kill processes
- **Interactive TUI** — Built with ratatui for responsive terminal UI
- **Process Details** — View detailed process information

## Installation

```powershell
cd htop\win\rust
cargo build --release

# Binary location
.\target\release\htop-win-rust.exe
```

### Prerequisites

- Rust (MSVC toolchain): https://rustup.rs/
- Windows Terminal or PowerShell (recommended)

## Usage

```powershell
# Run directly
cargo run --release

# Or run the built binary
.\target\release\htop-win-rust.exe
```

**Note:** If rebuilding, quit the running TUI first with `q` to avoid file lock errors.

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
| `+`/`=` | Speed up (min 200ms) |

### Process Actions
| Key | Action |
|-----|--------|
| `k` | Kill selected process |
| `i` | Show/hide process details |

### General
| Key | Action |
|-----|--------|
| `q` | Quit |
| `Esc` | Close popup/cancel input |

## Display

### Header Panel
- **CPU Gauge** — Visual percentage bar with usage
- **Memory Gauge** — Visual percentage bar with usage
- **Sparkline Charts** — CPU and memory history

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

## Process Kill on Windows

Uses Windows `taskkill` command:
1. First attempts graceful termination
2. Falls back to `/F` (force) if needed

**Note:** Administrator privileges may be required for some processes.

## Dependencies

- **ratatui** — Terminal UI framework
- **crossterm** — Cross-platform terminal manipulation
- **sysinfo** — System information
- **htop_shared** — Shared library for common functions

## Known Limitations

- Some system processes cannot be terminated (permission denied)
- Certain processes may not report full information
- CPU% stabilizes after 1-2 refresh cycles

## Testing

```powershell
cargo test
```

## Comparison with Go Implementation

| Feature | Rust | Go |
|---------|------|-----|
| TUI Framework | ratatui | tview |
| Sparkline history | ✅ | ❌ |
| Shared library | ✅ | ❌ |
| System info | sysinfo | gopsutil |

## License

MIT OR Apache-2.0
