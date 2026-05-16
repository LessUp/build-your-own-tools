# htop-win-go

[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../../../LICENSE)

A minimal htop clone for Windows built with Go and tview.

## Features

- **Real-time Monitoring** — CPU and memory usage with visual gauges
- **Process Management** — View and sort processes
- **Interactive TUI** — Built with tview for responsive terminal UI
- **Sort Controls** — Quick keyboard shortcuts for column sorting

## Installation

```powershell
cd htop\win\go
go build -o bin\htop-win-go.exe ./cmd/htop-win-go
```

### Prerequisites

- Go 1.21+ (1.22+ recommended)
- Windows Terminal or PowerShell

## Usage

```powershell
# Run directly
go run ./cmd/htop-win-go

# Or run the built binary
.\bin\htop-win-go.exe
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `C` | Sort by CPU% |
| `M` | Sort by Memory% |
| `P` | Sort by PID |
| `N` | Sort by Name |

## Display

### Header
- **Title** — `htop-win-go`
- **CPU Gauge** — Visual bar with percentage
- **Memory Gauge** — Visual bar with usage
- **Process Count** — Number of processes

### Process Table
| Column | Description |
|--------|-------------|
| PID | Process ID |
| NAME | Process name |
| CPU% | CPU usage percentage |
| RSS | Resident Set Size |
| MEM% | Memory percentage |
| START | Process start time |

## CPU% Calculation

CPU percentage is calculated by:
1. Getting process CPU time (user + system)
2. Measuring wall clock time difference
3. Dividing by `runtime.NumCPU()` for total percentage

## Dependencies

- **gopsutil** — System and process information
- **tview** — Terminal UI framework
- **tcell** — Terminal handling (tview dependency)

## Project Structure

```
htop-win-go/
├── cmd/
│   └── htop-win-go/
│       └── main.go
├── internal/
│   ├── metrics/
│   │   ├── system.go
│   │   └── processes.go
│   └── ui/
│       └── ui.go
├── changelog/
│   └── CHANGELOG.md
├── go.mod
└── Makefile
```

## Known Limitations

- Some system processes may fail to report information (permission denied)
- First refresh may show 0% CPU, subsequent refreshes show accurate values
- No process kill functionality yet

## Testing

```powershell
go test ./...
```

## Comparison with Rust Implementation

| Feature | Go | Rust |
|---------|-----|------|
| TUI Framework | tview | ratatui |
| Sparkline history | ❌ | ✅ |
| Process kill | ❌ | ✅ |
| Search/filter | ❌ | ✅ |
| System info | gopsutil | sysinfo |

## Roadmap

- [ ] Search/filter processes
- [ ] Process termination
- [ ] Adjustable refresh interval
- [ ] Per-core CPU display
- [ ] Disk/network metrics

## License

MIT OR Apache-2.0
