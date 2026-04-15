# Changelog

All notable changes to htop will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **htop/shared**: New shared library crate for common code
  - `ProcRow` struct with `as_row()` method
  - `SortKey` enum for process sorting
  - `compare_proc_rows()`, `filter_processes()`, `resolve_selected_index()`, `selected_pid()` functions
  - `color_for_ratio()` for usage-based coloring
  - `highlight_style()` for table row styling

### Changed

- Standardized memory units to MiB across all implementations

## [0.1.0] - 2025-10-20

### Added

- Repository structure standardization
  - Root-level `.gitignore`, `.gitattributes`, `.editorconfig`
  - Root-level `README.md`
  - GitHub Actions multi-language CI (Rust: `unix/rust`, `win/rust`; Go: `win/go`)

---

## Sub-project Changelogs

| Project | Changelog |
|---------|-----------|
| htop/unix/rust | [htop/unix/rust/changelog/CHANGELOG.md](./unix/rust/changelog/CHANGELOG.md) |
| htop/win/rust | [htop/win/rust/changelog/CHANGELOG.md](./win/rust/changelog/CHANGELOG.md) |
| htop/win/go | [htop/win/go/changelog/](./win/go/changelog/) |
