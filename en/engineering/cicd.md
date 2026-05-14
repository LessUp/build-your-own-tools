# CI/CD Design

This document describes the continuous integration and deployment workflow for the Build Your Own Tools project.

## Workflow Overview

```mermaid
graph TB
    A[Push/PR] --> B[CI Workflow]
    
    B --> C[Lint]
    B --> D[Test]
    B --> E[Build]
    
    C --> F{Passed?}
    D --> F
    E --> F
    
    F -->|Yes| G[Merge]
    F -->|No| H[Block]
    
    G --> I{main branch?}
    I -->|Yes| J[Release Workflow]
    I -->|No| K[End]
    
    J --> L[Build Release]
    L --> M[Create GitHub Release]
    L --> N[Deploy Documentation]
    
    style A fill:#f59e0b,color:#fff
    style F fill:#3b82f6,color:#fff
    style J fill:#10b981,color:#fff
```

## Workflow Files

### Main Workflows

| File | Trigger | Function |
|------|---------|----------|
| `ci.yml` | Push/PR | Code quality checks |
| `release.yml` | Tag push | Release publishing |
| `pages.yml` | main push | Documentation site deployment |

### CI Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      
      - name: Setup Go
        uses: actions/setup-go@v5
        with:
          go-version: '1.22'
      
      - name: Run Rust lint
        run: cargo clippy -- -D warnings
      
      - name: Run Rust fmt check
        run: cargo fmt -- --check
      
      - name: Run Go lint
        uses: golangci/golangci-lint-action@v4

  test:
    needs: lint
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Setup Go
        uses: actions/setup-go@v5
        with:
          go-version: '1.22'
      
      - name: Run Rust tests
        run: cargo test --all
      
      - name: Run Go tests
        run: go test ./...

  build:
    needs: test
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Build Release
        run: cargo build --release
```

### Release Workflow

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  build-release:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            archive: tar.gz
          - target: x86_64-apple-darwin
            os: macos-latest
            archive: tar.gz
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            archive: zip
    
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Build
        run: cargo build --release
      
      - name: Package
        run: |
          # Create archive
          if [ "${{ matrix.archive }}" = "zip" ]; then
            7z a release.zip target/release/*.exe
          else
            tar -czvf release.tar.gz -C target/release dos2unix gzip htop
          fi
      
      - name: Upload Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            release.${{ matrix.archive }}

  deploy-docs:
    needs: build-release
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Build docs
        run: npm run docs:build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./.vitepress/dist
```

### Pages Workflow

```yaml
# .github/workflows/pages.yml
name: GitHub Pages

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Build
        run: npm run docs:build
      
      - name: Add .nojekyll
        run: touch .vitepress/dist/.nojekyll
      
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: .vitepress/dist

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

## Quality Gates

```mermaid
graph LR
    A[Code Commit] --> B{Lint}
    B -->|Pass| C{Test}
    B -->|Fail| D[Block Merge]
    C -->|Pass| E{Build}
    C -->|Fail| D
    E -->|Pass| F[Allow Merge]
    E -->|Fail| D
    
    style D fill:#ef4444,color:#fff
    style F fill:#10b981,color:#fff
```

### Gate Requirements

| Check | Requirement | Failure Consequence |
|-------|-------------|---------------------|
| Clippy | No warnings | Block merge |
| rustfmt | Consistent formatting | Block merge |
| golangci-lint | No errors | Block merge |
| Tests | All pass | Block merge |
| Build | Compilation succeeds | Block merge |

## Makefile Integration

```makefile
# Makefile

.PHONY: lint-all test-all build-all

lint-all: lint-rust lint-go
	@echo "All lints passed"

lint-rust:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt -- --check

lint-go:
	golangci-lint run ./...

test-all: test-rust test-go
	@echo "All tests passed"

test-rust:
	cargo test --all

test-go:
	go test ./...

build-all: build-rust build-go
	@echo "All builds passed"

build-rust:
	cargo build --release

build-go:
	go build ./...
```

## Caching Strategy

```yaml
- name: Cache Rust
  uses: Swatinem/rust-cache@v2
  with:
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

- name: Cache Go
  uses: actions/cache@v4
  with:
    path: |
      ~/go/pkg/mod
      ~/.cache/go-build
    key: ${{ runner.os }}-go-${{ hashFiles('**/go.sum') }}

- name: Cache npm
  uses: actions/cache@v4
  with:
    path: ~/.npm
    key: ${{ runner.os }}-npm-${{ hashFiles('package-lock.json') }}
```

## Related Documents

- [Engineering Practices Overview](/engineering/) — Engineering Overview
- [OpenSpec Workflow](/specs/openspec-workflow) — Change Management
- [Documentation Strategy](/engineering/documentation) — Documentation Deployment
