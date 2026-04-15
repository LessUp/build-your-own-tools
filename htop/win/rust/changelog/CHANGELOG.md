# Changelog

All notable changes to htop-win-rust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Removed duplicate `color_for_ratio()` function
  - Now imports from `htop_shared`
- Standardized memory units to MiB

## [0.1.0] - 2026-02-13

### Changed

- Upgraded dependencies for ratatui 0.29 compatibility
  - `crossterm` 0.27 → 0.29
  - `ratatui` 0.26 → 0.29
- API adaptations:
  - `frame.size()` → `frame.area()`
  - Removed unnecessary `EnableMouseCapture`/`DisableMouseCapture`

### Added

- Initial Windows Rust implementation
- Real-time CPU and memory monitoring
- Process list with sorting and filtering
- Interactive TUI with ratatui
- Process details view
- Keyboard navigation and shortcuts
- Sparkline history for CPU/memory
