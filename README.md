# Build Your Own Tools

[![CI](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml)
[![Docs](https://github.com/LessUp/build-your-own-tools/actions/workflows/pages.yml/badge.svg)](https://lessup.github.io/build-your-own-tools/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/Go-1.21+-00ADD8.svg)](https://golang.org/)

**English** | [简体中文](README.zh-CN.md)

Rebuild real CLI tools in **Rust** and **Go** to learn file I/O, compression, TUI design, cross-platform behavior, and the trade-offs between two systems languages.

[Live Site](https://lessup.github.io/build-your-own-tools/) · [Getting Started](docs/setup/GETTING-STARTED.md) · [Architecture](docs/architecture/ARCHITECTURE.md) · [Rust vs Go Comparison](docs/tutorials/COMPARISON.md)

## Included projects

| Tool | Languages | What it teaches | Status |
| --- | --- | --- | --- |
| [`dos2unix`](./dos2unix/) | Rust | streaming file I/O, buffer boundaries, newline handling | Stable |
| [`gzip`](./gzip/) | Rust + Go | compression pipelines, CLI design, idiomatic error handling | Stable |
| [`htop`](./htop/) | Rust + Go | TUI architecture, process metrics, cross-platform system APIs | Stable |

## Why this repo is useful

- **One idea, two implementations**: compare Rust and Go on the same problem space.
- **Progressive complexity**: start with a small streaming tool and end with a cross-platform terminal UI.
- **Spec-driven development**: repository-wide changes are tracked with OpenSpec instead of ad hoc drift.
- **Practical engineering**: shared build commands, CI, release automation, and a documentation site are part of the learning surface.

## Quick start

```bash
git clone https://github.com/LessUp/build-your-own-tools.git
cd build-your-own-tools

make build-all
make test-all

# Example: convert CRLF input from stdin
printf 'hello\r\nworld\r\n' | ./target/release/dos2unix-rust
```

## Development workflow

Repository-wide work follows OpenSpec:

```bash
openspec list
/opsx:propose "describe the change"
/opsx:apply
/opsx:archive
```

For implementation and review:

```bash
make lint-all
make test-all
npm run docs:check
npm run docs:build
```

See [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md), and [.github/copilot-instructions.md](.github/copilot-instructions.md) for the shared AI-assisted workflow rules used in this repository.

## Documentation map

| Document | Purpose |
| --- | --- |
| [Getting Started](docs/setup/GETTING-STARTED.md) | set up the toolchains and run the project |
| [Architecture](docs/architecture/ARCHITECTURE.md) | understand repo structure and system design |
| [Comparison](docs/tutorials/COMPARISON.md) | compare Rust and Go design choices |
| [Project spec](openspec/specs/project/spec.md) | project-wide standards and workflow requirements |
| [CHANGELOG.md](CHANGELOG.md) | project release history |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)
