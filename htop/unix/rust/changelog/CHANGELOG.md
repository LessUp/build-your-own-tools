# Changelog

All notable changes to htop-unix-rust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Removed shell-based kill implementation
  - Now uses `sysinfo::Process::kill()` directly
- Standardized memory units to MiB

## [0.1.5] - 2026-02-13

### Fixed

- Implemented `kill_process()` for Unix platforms
  - SIGTERM → SIGKILL escalation for graceful termination

### Documentation

- Updated README to reflect cross-platform kill functionality

## [0.1.4] - 2025-10-20

### Fixed

- Suppressed unused variable warning in non-Windows branch

## [0.1.3] - 2025-09-25

### Added

- Process details panel (Enter or d to toggle)
  - Shows name, status, CPU%, memory, executable path, command line
- Kill selected process (k)
- Table header shows current sort column with direction arrow (↑/↓)
- Home/End keys to jump to first/last row
- Updated summary key hints

## [0.1.2] - 2025-09-25

### Added

- Process search/filter
  - `/` to enter search mode
  - Backspace to delete, Enter to confirm, Esc to clear and exit
  - Case-insensitive substring match
- Pause and manual refresh (p to toggle, F5 to refresh)
- Adjustable refresh interval
  - `-` to decrease, `+` or `=` to increase
  - Range: 100ms - 5000ms
- Summary bar shows Filter/Mode/Paused status and key hints

### Changed

- Extracted `do_refresh()` for unified refresh/filter/sort logic

## [0.1.1] - 2025-09-25

### Fixed

- Updated `Table` API for ratatui 0.29
  - `highlight_style` → `row_highlight_style`
  - `highlight_symbol` → `row_highlight_symbol`

## [0.1.0] - 2025-09-25

### Added

- Initial TUI implementation
  - Top summary: average CPU, memory usage/total with percentage
  - Bottom table: PID, NAME, CPU%, MEM(MB)
  - Scroll support (↑/↓, PgUp/PgDn)
  - Sort key cycling (s), ascending/descending toggle (r)
- Dependencies: `ratatui 0.29`, `crossterm 0.29`, `sysinfo 0.29`

### Fixed

- Compatibility fixes for ratatui 0.29
  - `PidExt` trait for `pid.as_u32()`
  - Process name via `p.name().to_string()`
  - `Table` widths with `.widths(&[...])` syntax
  - `Frame` explicit lifetime `Frame<'_>`
