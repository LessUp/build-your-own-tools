# Tasks: Add cat Tool

## 1. Core Implementation

- [ ] 1.1 Create `cat/` directory structure (Rust and Go)
- [ ] 1.2 Define CatOptions struct/interface
- [ ] 1.3 Implement streaming file reading function
- [ ] 1.4 Implement line numbering logic for -n option

## 2. Rust Implementation

- [ ] 2.1 Create `cat/Cargo.toml` with dependencies
- [ ] 2.2 Implement `src/lib.rs` with public API
- [ ] 2.3 Implement `src/cat.rs` with core logic
  - [ ] 2.3.1 `cat_file()` - basic streaming
  - [ ] 2.3.2 `cat_files()` - multiple files
  - [ ] 2.3.3 `cat_stdin()` - stdin handling
  - [ ] 2.3.4 `cat_with_line_numbers()` - -n option
- [ ] 2.4 Implement `src/main.rs` with CLI using clap
  - [ ] 2.4.1 Parse --help, --version
  - [ ] 2.4.2 Parse -n, --number
  - [ ] 2.4.3 Handle file arguments
- [ ] 2.5 Add to root `Cargo.toml` workspace members

## 3. Go Implementation

- [ ] 3.1 Create `cat/go.mod` module
- [ ] 3.2 Implement `cat.go` with core logic
  - [ ] 3.2.1 `CatFile()` - basic streaming
  - [ ] 3.2.2 `CatFiles()` - multiple files
  - [ ] 3.2.3 `CatStdin()` - stdin handling
  - [ ] 3.2.4 `CatWithLineNumbers()` - -n option
- [ ] 3.3 Implement `main.go` with CLI using flag
  - [ ] 3.3.1 Parse -h, -V
  - [ ] 3.3.2 Parse -n, --number
  - [ ] 3.3.3 Handle file arguments
- [ ] 3.4 Add to root `go.work` use directives

## 4. Testing

### Rust Unit Tests

- [ ] 4.1 Create `tests/fixtures/` with test files
  - [ ] 4.1.1 empty.txt
  - [ ] 4.1.2 small.txt (single line)
  - [ ] 4.1.3 multiline.txt (multiple lines)
  - [ ] 4.1.4 large.txt (1MB+ for streaming test)
- [ ] 4.2 Test empty file handling
- [ ] 4.3 Test single file output
- [ ] 4.4 Test multiple file concatenation
- [ ] 4.5 Test stdin passthrough
- [ ] 4.6 Test line numbering (-n)
- [ ] 4.7 Test file not found error
- [ ] 4.8 Test permission denied error

### Go Unit Tests

- [ ] 4.9 Create `testdata/` with test files
- [ ] 4.10 Test empty file handling
- [ ] 4.11 Test single file output
- [ ] 4.12 Test multiple file concatenation
- [ ] 4.13 Test stdin passthrough
- [ ] 4.14 Test line numbering (-n)
- [ ] 4.15 Test file not found error

### Integration Tests

- [ ] 4.16 Create `tests/integration/test_cat.sh`
- [ ] 4.17 Test CLI basic invocation
- [ ] 4.18 Test pipeline usage
- [ ] 4.19 Test exit codes

## 5. Documentation

- [ ] 5.1 Create `cat/README.md`
  - [ ] 5.1.1 Usage examples
  - [ ] 5.1.2 Options reference
  - [ ] 5.1.3 Performance notes
- [ ] 5.2 Create `cat/changelog/CHANGELOG.md`
- [ ] 5.3 Add doc comments to public APIs
  - [ ] 5.3.1 Rust: `lib.rs` exports
  - [ ] 5.3.2 Go: exported functions
- [ ] 5.4 Update root `README.md`
  - [ ] 5.4.1 Add cat to projects table
  - [ ] 5.4.2 Update project structure diagram
- [ ] 5.5 Update `docs/tutorials/COMPARISON.md`
  - [ ] 5.5.1 Add Rust vs Go comparison for cat
  - [ ] 5.5.2 Note simplicity (beginner level)

## 6. Quality Assurance

### Rust

- [ ] 6.1 Run `cargo fmt`
- [ ] 6.2 Run `cargo clippy -- -D warnings`
- [ ] 6.3 Run `cargo test`
- [ ] 6.4 Verify build: `cargo build --release`

### Go

- [ ] 6.5 Run `gofmt -w .`
- [ ] 6.6 Run `go vet ./...`
- [ ] 6.7 Run `go test -v ./...`
- [ ] 6.8 Verify build: `go build`

### Integration

- [ ] 6.9 Update `Makefile` with cat targets
  - [ ] 6.9.1 `build-cat-rust`
  - [ ] 6.9.2 `build-cat-go`
  - [ ] 6.9.3 `test-cat-rust`
  - [ ] 6.9.4 `test-cat-go`
- [ ] 6.10 Run `make test-all`
- [ ] 6.11 Verify cross-platform builds (Linux, macOS, Windows)

## 7. CI/CD

- [ ] 7.1 Update `.github/workflows/ci.yml`
  - [ ] 7.1.1 Add cat to test matrix
- [ ] 7.2 Update `.github/workflows/release.yml`
  - [ ] 7.2.1 Add cat binaries to release artifacts

## 8. Final Verification

- [ ] 8.1 Test all CLI options manually
- [ ] 8.2 Test with large file (1GB+)
- [ ] 8.3 Test memory usage stays under 10MB
- [ ] 8.4 Test pipeline: `echo "test" | cat`
- [ ] 8.5 Test multiple files: `cat a.txt b.txt c.txt`
- [ ] 8.6 Review all code comments and documentation
