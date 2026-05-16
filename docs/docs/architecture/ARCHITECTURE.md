# Architecture Guide

> A practical map of the repository, its tool families, and the engineering surfaces around them.

**English** | [简体中文](ARCHITECTURE.zh-CN.md)

## What this repository is

Build Your Own Tools is a monorepo for learning systems programming by re-implementing a small set of CLI tools in Rust and Go. The repository is intentionally organized so you can study:

- the tool implementations themselves
- the Rust vs Go trade-offs
- shared engineering surfaces such as specs, CI, releases, and docs

## Core design principles

1. **Teach by implementation** - each tool is real enough to show meaningful trade-offs.
2. **Compare languages on the same problem** - gzip and htop expose idiomatic differences clearly.
3. **Keep engineering visible** - build scripts, OpenSpec, CI, and the docs site are part of the project surface.
4. **Prefer a small, coherent tool set** - depth is more valuable than a large but inconsistent catalog.

## Repository map

```text
build-your-own-tools/
├── openspec/                  repository-wide specs and active changes
├── docs/                      supporting documentation
├── .vitepress/                docs site configuration
├── .github/workflows/         CI, Pages, release, and maintenance workflows
├── .githooks/                 tracked git hooks
├── dos2unix/                  Rust newline conversion tool
├── gzip/go/                   Go gzip implementation
├── gzip/rust/                 Rust gzip implementation
├── htop/shared/               shared Rust logic for htop
├── htop/unix/rust/            Unix Rust htop
├── htop/win/rust/             Windows Rust htop
├── htop/win/go/               Windows Go htop
├── README.md                  repository entry point
└── index.md                   Pages landing page
```

## Tool map

| Tool | Languages | Main concepts |
| --- | --- | --- |
| `dos2unix` | Rust | streaming I/O, newline conversion, buffer edges |
| `gzip` | Rust + Go | compression pipelines, CLI behavior, error handling |
| `htop` | Rust + Go | terminal UI, process metrics, cross-platform system APIs |

## Build and verification surfaces

The repository keeps a small shared command surface:

```bash
make build-all
make lint-all
make test-all
npm run docs:check
npm run docs:build
```

- `make` covers Rust and Go implementations.
- npm scripts cover the docs site and typed VitePress checks.

## OpenSpec and workflow model

Repository-wide work is tracked through OpenSpec:

- `openspec/specs/` holds stable project requirements
- `openspec/changes/` holds active or historical changes
- `.opencode/commands/` exposes `/opsx:*` commands for the same workflow

For a non-trivial change, the expected sequence is:

1. propose the change
2. write/update specs and design
3. implement against tasks
4. verify with the existing commands
5. archive when complete

## Public surfaces

The project deliberately separates its public entry points:

| Surface | Role |
| --- | --- |
| `README.md` | repository overview and quick orientation |
| `index.md` | landing page for new visitors |
| `docs/` | supporting explanations and deep dives |
| `CHANGELOG.md` | project release history |

That separation keeps the site from becoming a plain README mirror.
