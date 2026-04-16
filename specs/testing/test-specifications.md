# Testing Specifications

> BDD test case specifications and testing conventions

| Metadata | Value |
|----------|-------|
| **Version** | 1.0 |
| **Last Updated** | 2026-04-17 |

---

## Overview

This document defines testing standards, conventions, and acceptance criteria for the Build-Your-Own-Tools project. All implementations must have tests that verify compliance with these specifications.

---

## Testing Strategy

### Test Pyramid

```
                    ┌─────────┐
                    │   E2E   │  ← Integration Tests (Few)
                    │  Tests  │
                    ├─────────┤
                    │  API    │  ← API/CLI Tests (Some)
                    │  Tests  │
                    ├─────────┤
                    │  Unit   │  ← Unit Tests (Many)
                    │  Tests  │
                    └─────────┘
```

### Test Categories

| Category | Coverage Target | Description |
|----------|-----------------|-------------|
| Unit Tests | > 80% | Test individual functions and methods |
| Integration Tests | Critical paths | Test component interactions |
| E2E Tests | Happy paths | Test complete user scenarios |

---

## Test Conventions

### Rust Testing

```bash
# Run all tests
cargo test --all

# Run with verbose output
cargo test --all -- --nocapture

# Run specific test
cargo test -p dos2unix-rust test_stream_large_data

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --all --out Html
```

### Go Testing

```bash
# Run all tests
go test -v ./...

# Run specific test
go test -C gzip/go -run TestGzipStream

# Run with coverage
go test -cover ./...
go test -coverprofile=coverage.out ./...
go tool cover -html=coverage.out
```

### Test File Naming

| Language | Convention | Example |
|----------|------------|---------|
| Rust | `#[test]` attribute in source files | `src/lib.rs` |
| Go | `*_test.go` files | `gzip_test.go` |

---

## BDD Feature Files

### dos2unix: Line Ending Conversion

```gherkin
Feature: CRLF to LF conversion
  As a developer
  I want to convert Windows line endings to Unix line endings
  So that my files are compatible across platforms

  Background:
    Given a test file with known content

  Scenario: Convert a file with CRLF to LF
    Given a file "test.txt" with content "line1\r\nline2\r\n"
    When I run dos2unix on "test.txt"
    Then the output should be "line1\nline2\n"
    And all CRLF sequences should be converted to LF

  Scenario: Check mode without modification
    Given a file "test.txt" with content "line1\r\nline2\r\n"
    When I run dos2unix --check on "test.txt"
    Then the file should not be modified
    And the exit code should be 2

  Scenario: Handle Unix line endings
    Given a file "test.txt" with content "line1\nline2\n"
    When I run dos2unix on "test.txt"
    Then the file should remain unchanged
    And the exit code should be 0

  Scenario: Handle empty file
    Given an empty file "empty.txt"
    When I run dos2unix on "empty.txt"
    Then the output should be empty
    And the exit code should be 0

  Scenario: Handle CRLF at buffer boundary
    Given a file "large.txt" with 8193 bytes and CRLF at position 8192
    When I run dos2unix on "large.txt"
    Then the CRLF should be correctly converted to LF
    And no data should be lost

  Scenario: Preserve standalone CR
    Given a file "test.txt" with content "line1\r\nline2\r"
    When I run dos2unix on "test.txt"
    Then the output should be "line1\nline2\r"
    And standalone CR should be preserved
```

---

### gzip: Compression/Decompression

```gherkin
Feature: File compression and decompression
  As a user
  I want to compress and decompress files
  So that I can save disk space and transfer files efficiently

  Background:
    Given a test directory with sample files

  Scenario: Compress a file
    Given a text file "document.txt" with content "Hello, World!"
    When I run gzip on "document.txt"
    Then a file "document.txt.gz" should be created
    And the compressed file should be smaller than the original
    And the original file should be removed

  Scenario: Decompress a file
    Given a compressed file "document.txt.gz"
    When I run gzip --decompress on "document.txt.gz"
    Then the original file "document.txt" should be restored
    And the restored content should match the original

  Scenario: Compression with different levels
    Given a text file "data.txt" with 1MB of content
    When I compress it with level 0 (no compression)
    And I compress it with level 9 (maximum compression)
    Then level 9 should produce a smaller file than level 0
    And both should decompress to the same content

  Scenario: Keep source file option
    Given a text file "important.txt"
    When I compress it with --keep flag
    Then both "important.txt" and "important.txt.gz" should exist

  Scenario: Handle empty file
    Given an empty file "empty.txt"
    When I run gzip on "empty.txt"
    Then a valid gzip file "empty.txt.gz" should be created
    And decompressing it should produce an empty file

  Scenario: Invalid gzip file handling
    Given a file "fake.gz" with content "not a gzip file"
    When I run gzip --decompress on "fake.gz"
    Then an error should be reported
    And the exit code should be 1

  Scenario: stdin/stdout support
    Given stdin contains "Hello, World!"
    When I pipe through gzip --decompress
    Then stdout should contain the decompressed content

  Scenario Outline: Compression levels
    Given a text file "test.txt"
    When I compress with level <level>
    Then compression should succeed
    And decompression should restore original content

    Examples:
      | level |
      | 0     |
      | 1     |
      | 6     |
      | 9     |
```

---

### htop: System Monitoring

```gherkin
Feature: System process monitoring
  As a system administrator
  I want to monitor system processes and resource usage
  So that I can identify performance issues and manage resources

  Background:
    Given htop is running
    And the terminal supports TUI

  Scenario: Display process list
    When htop starts
    Then I should see a list of running processes
    And each process should show PID, name, CPU%, and memory
    And the CPU and memory bars should be visible

  Scenario: Sort processes by CPU usage
    Given a process list is displayed
    When I sort by CPU column
    Then processes should be ordered by CPU usage (highest first)

  Scenario: Search and filter processes
    Given a process list is displayed
    When I enter search mode with "/"
    And I type "firefox"
    Then only processes matching "firefox" should be displayed
    And the filter should be case-insensitive

  Scenario: Kill a process
    Given a process with PID 1234 is selected
    When I press "k" to kill
    And I confirm the action
    Then SIGTERM should be sent to PID 1234
    And the process list should update

  Scenario: Adjust refresh interval
    Given htop is running with default 1000ms interval
    When I start with "--interval 500"
    Then the display should refresh every 500ms

  Scenario: Handle terminal resize
    Given htop is running in a 80x24 terminal
    When the terminal is resized to 120x40
    Then the layout should adapt to new dimensions
    And no content should be lost

  Scenario: Exit cleanly
    Given htop is running
    When I press "q"
    Then htop should exit with code 0
    And the terminal should be restored to normal state
```

---

## Unit Test Specifications

### dos2unix Unit Tests

| Test Name | Description | Input | Expected Output |
|-----------|-------------|-------|-----------------|
| `test_stream_empty` | Handle empty input | `""` | `""` |
| `test_stream_no_crlf` | Pass-through verification | `"hello\nworld\n"` | `"hello\nworld\n"` |
| `test_stream_with_crlf` | Basic conversion | `"a\r\nb\r\n"` | `"a\nb\n"` |
| `test_stream_mixed` | Mixed line endings | `"a\r\nb\nc\r\n"` | `"a\nb\nc\n"` |
| `test_stream_large_data` | Buffer boundary test | 8193 bytes with CRLF at 8192 | Correct conversion |
| `test_stream_lone_cr_preserved` | Standalone CR | `"a\rb\n"` | `"a\rb\n"` |

### gzip Unit Tests

| Test Name | Description | Input | Expected Output |
|-----------|-------------|-------|-----------------|
| `test_compress_empty` | Compress empty file | `""` | Valid gzip file |
| `test_decompress_invalid` | Handle invalid gzip | Random bytes | Error |
| `test_roundtrip` | Compress then decompress | Any content | Original content |
| `test_level_0` | No compression | "test" | Valid gzip |
| `test_level_9` | Best compression | Large text | Smaller output |

### htop Unit Tests

| Test Name | Description | Input | Expected Output |
|-----------|-------------|-------|-----------------|
| `test_sort_by_cpu` | CPU sorting | Unsorted list | Sorted by CPU desc |
| `test_filter_processes` | Filter by name | List + "fire" | Filtered list |
| `test_color_for_ratio` | Color thresholds | Various ratios | Correct colors |
| `test_resolve_selected_index` | Bounds checking | 10, 5 | 4 |

---

## Integration Test Specifications

### CI Test Matrix

| OS | Rust Version | Go Version | Test Scope |
|----|--------------|------------|------------|
| Ubuntu 22.04 | stable, beta | 1.21, 1.22 | Full |
| macOS 14 | stable | 1.21 | Full |
| Windows Server 2022 | stable | 1.21 | Full |

### Test Execution Order

1. **Format Check**: `cargo fmt --check`, `gofmt -l`
2. **Lint Check**: `cargo clippy`, `go vet`
3. **Unit Tests**: `cargo test`, `go test`
4. **Integration Tests**: Build and run tools
5. **Cross-Platform**: Run on all OS matrices

---

## Acceptance Criteria

| ID | Criterion | Verification Method | Priority |
|----|-----------|---------------------|----------|
| AC-1 | All unit tests pass | `cargo test` / `go test` | High |
| AC-2 | Code formatting is correct | `cargo fmt --check` / `gofmt -l` | High |
| AC-3 | No lint warnings | `cargo clippy -- -D warnings` / `go vet` | High |
| AC-4 | Cross-platform builds succeed | GitHub Actions CI matrix | High |
| AC-5 | CLI tools work as documented | Manual testing + integration tests | High |
| AC-6 | Test coverage > 80% | `cargo tarpaulin` / `go test -cover` | Medium |

---

## Test Data Management

### Test Fixtures Location

```
tests/
├── fixtures/
│   ├── dos2unix/
│   │   ├── empty.txt
│   │   ├── crlf.txt
│   │   ├── lf.txt
│   │   └── mixed.txt
│   ├── gzip/
│   │   ├── small.txt
│   │   ├── large.txt
│   │   └── invalid.gz
│   └── htop/
│       └── mock_processes.json
└── integration/
    ├── test_dos2unix.sh
    ├── test_gzip.sh
    └── test_htop.sh
```

### Test Data Generation

```bash
# Generate test files
dd if=/dev/urandom of=tests/fixtures/gzip/random.bin bs=1M count=10
yes "Hello World" | head -n 1000 > tests/fixtures/gzip/repetitive.txt
```

---

## Performance Testing

### Benchmarks

| Tool | Metric | Target | Measurement |
|------|--------|--------|-------------|
| dos2unix | Throughput | > 100 MB/s | `hyperfine` |
| gzip | Compression speed | > 50 MB/s | `hyperfine` |
| gzip | Decompression speed | > 100 MB/s | `hyperfine` |
| htop | Startup time | < 100ms | Manual timing |
| htop | CPU overhead | < 1% | System monitor |

### Memory Profiling

```bash
# Rust memory profiling
cargo build --release
valgrind --tool=massif ./target/release/dos2unix large.txt

# Go memory profiling
go test -memprofile=mem.prof ./...
go tool pprof mem.prof
```

---

## Continuous Integration

### CI Test Workflow

```yaml
# .github/workflows/ci.yml (excerpt)
test:
  strategy:
    matrix:
      os: [ubuntu-latest, macos-latest, windows-latest]
  steps:
    - name: Run Rust tests
      run: cargo test --all --verbose
    
    - name: Run Go tests  
      run: go test -v ./...
    
    - name: Generate coverage
      run: cargo tarpaulin --out Xml
```

---

**Last Updated**: 2026-04-17
