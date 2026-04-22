# AGENTS.md

## Spec-Driven Development

This project follows SDD. All implementations must be based on `/specs`:

| Need | Location |
|------|----------|
| Product requirements | `specs/product/project-standardization.md` |
| Architecture | `specs/rfc/0001-project-architecture.md` |
| CLI interfaces | `specs/api/cli-interfaces.md` |
| Test specs | `specs/testing/test-specifications.md` |

**Workflow**: Read specs first → propose spec changes for new features/interfaces → implement per spec → test against acceptance criteria.

## Build Commands

```bash
make fmt-all && make lint-all && make test-all && make build-all
```

Specific targets: `make build-dos2unix`, `make build-gzip-rust`, `make build-gzip-go`, `make build-htop-unix-rust`, `make test-rust`, `make test-go`.

## Project Structure

- **Rust workspace**: Root `Cargo.toml` with members: `dos2unix`, `gzip/rust`, `htop/shared`, `htop/unix/rust`, `htop/win/rust`
- **Go modules**: `gzip/go`, `htop/win/go` (each has own `go.mod`)
- **Shared library**: `htop/shared` is a Rust crate used by both Unix and Windows htop

## Dual-Language Implementations

When modifying one language implementation, consider if the other needs equivalent changes:
- `gzip`: Rust + Go
- `htop`: Rust (Unix, Windows) + Go (Windows)
- `dos2unix`: Rust only

## Code Standards

- Rust: `cargo fmt`, `cargo clippy -- -D warnings`
- Go: `gofmt`, `go vet`
- All public APIs must have doc comments
- Use `anyhow` for Rust error handling

## Commit Convention

[Conventional Commits](https://www.conventionalcommits.org/) with scopes: `dos2unix`, `gzip`, `htop`, `ci`, `docs`, `specs`.

Examples: `feat(gzip): add parallel processing`, `fix(htop): resolve memory leak`.
