# RFC 0001: Project Architecture & Standardization

| Metadata | Value |
|----------|-------|
| **Status** | Accepted |
| **Created** | 2026-04-16 |
| **Authors** | Project Maintainers |
| **Category** | Architecture |

## Overview

This RFC describes the architecture design and standardization approach for the Build-Your-Own-Tools project.

## Architecture

### Project Structure

```
Build-Your-Own-Tools/
├── .github/
│   ├── ISSUE_TEMPLATE/       # Issue templates
│   ├── workflows/            # CI/CD workflows
│   │   ├── ci.yml            # Continuous integration
│   │   ├── release.yml       # Automated releases
│   │   └── pages.yml         # Documentation deployment
│   └── pull_request_template.md
├── docs/                     # Project documentation
│   ├── ARCHITECTURE.md       # Architecture guide
│   └── COMPARISON.md         # Rust vs Go comparison
├── dos2unix/                 # CRLF → LF conversion tool
├── gzip/                     # Compression/decompression tool
│   ├── go/                   # Go implementation
│   └── rust/                 # Rust implementation
├── htop/                     # System monitoring tool
│   ├── shared/               # Shared library
│   ├── unix/rust/            # Unix Rust implementation
│   └── win/
│       ├── go/               # Windows Go implementation
│       └── rust/             # Windows Rust implementation
├── .editorconfig             # Editor configuration
├── .gitattributes            # Git attributes
├── .gitignore                # Git ignore rules
├── CHANGELOG.md              # Changelog
├── CODE_OF_CONDUCT.md        # Code of conduct
├── CONTRIBUTING.md           # Contribution guidelines
├── Cargo.toml                # Rust workspace
├── LICENSE                   # License
├── Makefile                  # Build scripts
├── README.md                 # Main project documentation
├── SECURITY.md               # Security policy
└── go.work                   # Go workspace
```

## Components

### 1. Documentation System

| File | Purpose |
|------|---------|
| README.md | Project overview, quick start, feature list |
| CHANGELOG.md | Version change history (Keep a Changelog format) |
| CONTRIBUTING.md | Contribution guidelines, code standards |
| CODE_OF_CONDUCT.md | Code of conduct (Contributor Covenant) |
| SECURITY.md | Security policy |

### 2. CI/CD Pipelines

| Workflow | Trigger | Function |
|----------|---------|----------|
| ci.yml | Push/PR | Format checks, linting, tests, cross-platform builds |
| release.yml | Tag push | Build release binaries, create GitHub releases |
| pages.yml | main/master push | Deploy documentation to GitHub Pages |

### 3. Code Quality Tools

| Tool | Configuration File | Purpose |
|------|-------------------|---------|
| rustfmt | rustfmt.toml | Rust code formatting |
| clippy | - | Rust linting |
| gofmt | - | Go code formatting |
| go vet | - | Go static analysis |
| EditorConfig | .editorconfig | Editor统一 configuration |

### 4. Sub-Project Structure

Each sub-project includes:
- `README.md` - Usage instructions and examples
- `changelog/CHANGELOG.md` - Version history
- Language-specific configuration (Cargo.toml / go.mod)
- Test files

## Versioning

Follows [Semantic Versioning](https://semver.org/):
- MAJOR: Breaking API changes
- MINOR: Backward-compatible feature additions
- PATCH: Backward-compatible bug fixes

## Changelog Format

Follows [Keep a Changelog](https://keepachangelog.com/):

```markdown
## [Unreleased]
### Added / Changed / Fixed / Security

## [0.1.0] - YYYY-MM-DD
### Added
- Initial release
```

## Cross-Platform Support

| Platform | Rust | Go |
|----------|------|-----|
| Linux (x86_64) | ✅ | ✅ |
| macOS (x86_64, ARM64) | ✅ | ✅ |
| Windows (x86_64) | ✅ | ✅ |
