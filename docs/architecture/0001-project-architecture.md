# RFC 0001: Project Architecture

| Metadata | Value |
|----------|-------|
| **Number** | 0001 |
| **Title** | Project Architecture & Standardization |
| **Status** | Accepted |
| **Created** | 2026-02-13 |
| **Updated** | 2026-04-17 |
| **Authors** | Project Maintainers |
| **Category** | Architecture |

---

## Summary

This RFC describes the architecture design and standardization approach for the Build-Your-Own-Tools project, establishing the foundational patterns for all sub-projects.

---

## Motivation

The project needs a consistent architecture to:

1. Enable learning by providing clear, comparable implementations
2. Support multiple languages (Rust, Go) with idiomatic patterns
3. Maintain production-level quality standards
4. Facilitate cross-platform development

---

## Detailed Design

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
├── .opencode/                # OpenCode slash commands and skills
│   ├── commands/             # /opsx:* commands
│   └── skills/               # OpenSpec skills
├── docs/                     # Project documentation
│   ├── architecture/         # Architecture docs (this file)
│   ├── setup/                # Setup guides
│   ├── tutorials/            # Learning tutorials
│   └── changelogs/           # Changelog index
├── openspec/                 # OpenSpec specifications
│   ├── config.yaml           # Project configuration
│   ├── schemas/              # Custom workflow schemas
│   ├── specs/                # Source of truth specs
│   └── changes/              # Proposed modifications
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
├── AGENTS.md                 # AI Agent workflow config
├── CHANGELOG.md              # Changelog
├── CODE_OF_CONDUCT.md        # Code of conduct
├── CONTRIBUTING.md           # Contribution guidelines
├── Cargo.toml                # Rust workspace
├── LICENSE                   # License
├── Makefile                  # Build scripts
├── README.md                 # Project documentation (English)
├── README.zh-CN.md           # Project documentation (Chinese)
├── SECURITY.md               # Security policy
└── go.work                   # Go workspace
```

### Components

#### 1. Documentation System

| File | Purpose |
|------|---------|
| README.md | Project overview, quick start, feature list (English default) |
| README.zh-CN.md | Chinese version of README |
| CHANGELOG.md | Version change history (Keep a Changelog format) |
| CONTRIBUTING.md | Contribution guidelines, code standards |
| CODE_OF_CONDUCT.md | Code of conduct (Contributor Covenant) |
| SECURITY.md | Security policy |
| AGENTS.md | AI Agent workflow configuration for SDD |

#### 2. CI/CD Pipelines

| Workflow | Trigger | Function |
|----------|---------|----------|
| ci.yml | Push/PR | Format checks, linting, tests, cross-platform builds |
| release.yml | Tag push | Build release binaries, create GitHub releases |
| pages.yml | main/master push | Deploy documentation to GitHub Pages |

#### 3. Code Quality Tools

| Tool | Configuration File | Purpose |
|------|-------------------|---------|
| rustfmt | rustfmt.toml | Rust code formatting |
| clippy | - | Rust linting |
| gofmt | - | Go code formatting |
| go vet | - | Go static analysis |
| EditorConfig | .editorconfig | Editor unified configuration |

#### 4. Sub-Project Structure

Each sub-project includes:
- `README.md` - Usage instructions and examples
- `changelog/CHANGELOG.md` - Version history
- Language-specific configuration (Cargo.toml / go.mod)
- Test files

---

## Design Decisions

### Decision 1: Dual-Language Implementation

**Rationale**: Implementing the same tool in both Rust and Go enables:
- Direct comparison of language idioms
- Understanding of trade-offs between languages
- Learning different approaches to system programming

**Implementation**:
- gzip: Rust (library + CLI) + Go (CLI only)
- htop: Rust (Unix, Windows) + Go (Windows)
- dos2unix: Rust only (simplicity demonstration)

### Decision 2: Workspace-Based Organization

**Rationale**: Using Cargo workspaces and Go workspaces provides:
- Shared dependency management
- Unified build commands
- Cross-crate optimization

**Implementation**:
- Root `Cargo.toml` defines Rust workspace
- Root `go.work` defines Go workspace

### Decision 3: Makefile as Build Orchestrator

**Rationale**: A single Makefile provides:
- Unified build interface across languages
- Simple commands for common operations
- CI/CD integration point

**Implementation**:
```bash
make build-all     # Build all projects
make test-all      # Run all tests
make lint-all      # Run all linters
make fmt-all       # Format all code
```

### Decision 4: VitePress for Documentation

**Rationale**: VitePress offers:
- Markdown-based documentation
- Vue.js component support
- Fast static site generation
- GitHub Pages integration

---

## Versioning Strategy

Follows [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking API changes
- **MINOR**: Backward-compatible feature additions
- **PATCH**: Backward-compatible bug fixes

---

## Cross-Platform Support

| Platform | Rust | Go |
|----------|------|-----|
| Linux (x86_64) | ✅ | ✅ |
| macOS (x86_64, ARM64) | ✅ | ✅ |
| Windows (x86_64) | ✅ | ✅ |

---

## Alternatives Considered

### Alternative 1: Single Language

**Rejected**: Would not provide the educational value of cross-language comparison.

### Alternative 2: Microservices Architecture

**Rejected**: CLI tools don't benefit from microservices; adds unnecessary complexity.

### Alternative 3: Git Submodules for Sub-projects

**Rejected**: Increases complexity for contributors; monorepo approach is simpler.

---

## Consequences

### Positive

- Clear structure for contributors
- Consistent quality across implementations
- Easy to add new tools or languages
- Comprehensive documentation

### Negative

- Larger repository size
- More CI/CD time for cross-platform builds
- Need to maintain parallel implementations

---

## References

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Go Workspaces](https://go.dev/doc/tutorial/workspaces)

---

**Last Updated**: 2026-04-17
