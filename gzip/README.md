# gzip

A minimalist gzip CLI tool collection implemented in multiple languages (Go and Rust), following the KISS principle: simple, maintainable, and easy to distribute, with a default focus on Linux usage scenarios.

## Subprojects
- **Go version**: `go/`, executable name `gzip-go`, entry point at `go/cmd/gzip-go/main.go`.
  - See `go/README.md` for details
- **Rust version**: `rust/`, executable name `rgzip`, core logic in `rust/src/lib.rs`.
  - See `rust/README.md` for details

## Directory Structure
```text
./
├── go/
│   ├── cmd/gzip-go/main.go
│   ├── go.mod
│   ├── Makefile
│   ├── README.md
│   └── changelog/
├── rust/
│   ├── Cargo.toml
│   ├── README.md
│   ├── src/{main.rs, lib.rs}
│   └── changelog/
├── .gitignore
├── .gitattributes
├── .editorconfig
└── changelog/
```

## Quick Start
- **Build Go version**
  ```bash
  cd go
  make build      # Generates go/bin/gzip-go
  # or
  go build -o bin/gzip-go ./cmd/gzip-go
  ```
- **Build Rust version**
  ```bash
  cd rust
  cargo build --release   # Generates target/release/rgzip
  ```

## Usage
- See subproject documentation: `go/README.md` and `rust/README.md`.

## License
- This project follows the `LICENSE` file in the repository root, using `MIT OR Apache-2.0` dual license.
- Both Rust and Go implementations can be used and distributed under the above licenses.

## Contributing
- PRs welcome. Please follow:
  - Maintain the KISS principle; minimize dependencies and complexity.
  - Add a record in the corresponding subproject's `changelog/` or the repository root `changelog/` for each change.
  - Unified code style: see `.editorconfig`; line endings LF, text auto-normalization see `.gitattributes`.
