# Testing Specifications

> BDD test case specifications and testing conventions

## Overview

This directory defines testing standards and conventions for the Build-Your-Own-Tools project.

## Testing Strategy

### Unit Tests

Each sub-project must include unit tests for core functionality.

### Integration Tests

End-to-end tests verify CLI tools work correctly with real files.

## Test Conventions

### Rust Testing

```bash
# Run all tests
cargo test --all

# Run with verbose output
cargo test --all -- --nocapture

# Run specific test
cargo test -p dos2unix-rust test_stream_large_data

# Run with coverage
cargo tarpaulin --all
```

### Go Testing

```bash
# Run all tests
go test -v ./...

# Run specific test
go test -C gzip/go -run TestGzipStream

# Run with coverage
go test -cover ./...
```

## BDD Feature Files

### dos2unix: Line Ending Conversion

```gherkin
Feature: CRLF to LF conversion
  As a developer
  I want to convert Windows line endings to Unix line endings
  So that my files are compatible across platforms

  Scenario: Convert a file with CRLF to LF
    Given a file with CRLF line endings
    When I run dos2unix on the file
    Then all CRLF sequences should be converted to LF

  Scenario: Check mode without modification
    Given a file with CRLF line endings
    When I run dos2unix in check mode
    Then the file should not be modified
    And the exit code should indicate CRLF was detected

  Scenario: Handle Unix line endings
    Given a file with LF line endings
    When I run dos2unix on the file
    Then the file should remain unchanged
```

### gzip: Compression/Decompression

```gherkin
Feature: File compression and decompression
  As a user
  I want to compress and decompress files
  So that I can save disk space and transfer files efficiently

  Scenario: Compress a file
    Given a text file
    When I run gzip in compress mode
    Then a .gz file should be created
    And the compressed file should be smaller than the original

  Scenario: Decompress a file
    Given a .gz compressed file
    When I run gzip in decompress mode
    Then the original file should be restored
    And the restored file should match the original content

  Scenario: Compression with different levels
    Given a text file
    When I compress it with level 0 (no compression)
    And I compress it with level 9 (maximum compression)
    Then level 9 should produce a smaller file than level 0

  Scenario: Keep source file option
    Given a text file
    When I compress it with --keep flag
    Then both the original and compressed files should exist
```

### htop: System Monitoring

```gherkin
Feature: System process monitoring
  As a system administrator
  I want to monitor system processes and resource usage
  So that I can identify performance issues and manage resources

  Scenario: Display process list
    When I start htop
    Then I should see a list of running processes
    And each process should show PID, name, CPU%, and memory

  Scenario: Sort processes by CPU usage
    When I sort processes by CPU column
    Then processes should be ordered by CPU usage (highest first)

  Scenario: Search and filter processes
    Given a process list
    When I enter a search query
    Then only matching processes should be displayed

  Scenario: Kill a process
    Given a selected process
    When I send SIGTERM
    Then the process should be terminated
    And the process list should update
```

## Acceptance Criteria

| ID | Criterion | Verification Method |
|----|-----------|---------------------|
| AC-1 | All unit tests pass | `cargo test` / `go test` |
| AC-2 | Code formatting is correct | `cargo fmt --check` / `gofmt -l` |
| AC-3 | No lint warnings | `cargo clippy -- -D warnings` / `go vet` |
| AC-4 | Cross-platform builds succeed | GitHub Actions CI matrix |
| AC-5 | CLI tools work as documented | Manual testing + integration tests |
