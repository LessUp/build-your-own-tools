# Changelog

All notable changes to htop-win-go will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_No changes yet._

## [0.1.3] - 2025-10-20

### Added

- Header title `htop-win-go`
- CPU and memory text progress bars
  - Color thresholds: green <60%, yellow 60-85%, red ≥85%
  - Shows percentage and memory usage
- Process count display on the right side

### Changed

- Refactored header from single `TextView` to `Flex(Row)`
  - Contains title row + progress bar row
- Uses `TextView` for ASCII progress bars instead of tview Gauge

## [0.1.2] - 2025-10-12

### Added

- Sort shortcuts: `C`/`M`/`P`/`N` for CPU%/Memory/PID/Name
- Optimized bottom hints display

## [0.1.1] - 2025-09-25

### Fixed

- Fixed `NumCPU` usage for accurate CPU count

## [0.1.0] - 2025-09-25

### Added

- Initial Windows Go implementation
- Real-time CPU and memory monitoring
- Process list with sorting and filtering
- Interactive TUI with tview
- Keyboard navigation and shortcuts
