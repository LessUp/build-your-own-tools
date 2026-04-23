# Delta for cat

## ADDED Requirements

### Requirement: File Concatenation

The system SHALL concatenate and output the contents of one or more files to stdout.

#### Scenario: Single file output
- GIVEN a file "hello.txt" containing "Hello, World!"
- WHEN `cat hello.txt` is executed
- THEN stdout contains "Hello, World!"
- AND exit code is 0

#### Scenario: Multiple file concatenation
- GIVEN files "a.txt" containing "line1" and "b.txt" containing "line2"
- WHEN `cat a.txt b.txt` is executed
- THEN stdout contains "line1line2"
- AND exit code is 0

#### Scenario: stdin passthrough
- GIVEN stdin contains "input data"
- WHEN `cat` is executed (no file arguments)
- THEN stdout contains "input data"
- AND exit code is 0

### Requirement: Line Numbering

The system SHALL support line numbering with the -n option.

#### Scenario: Number all lines
- GIVEN a file "test.txt" containing:
  ```
  line1
  line2
  line3
  ```
- WHEN `cat -n test.txt` is executed
- THEN stdout contains:
  ```
       1	line1
       2	line2
       3	line3
  ```
- AND numbers are right-aligned with 6 characters width

#### Scenario: Number with stdin
- GIVEN stdin contains "line1\nline2"
- WHEN `cat -n` is executed
- THEN stdout contains numbered lines

### Requirement: Error Handling

The system SHALL handle errors gracefully.

#### Scenario: File not found
- GIVEN a non-existent file "missing.txt"
- WHEN `cat missing.txt` is executed
- THEN stderr contains error message
- AND exit code is 1

#### Scenario: Permission denied
- GIVEN a file without read permission
- WHEN `cat` is executed on that file
- THEN stderr contains permission error
- AND exit code is 1

### Requirement: Performance

The system SHALL handle large files efficiently.

#### Scenario: Large file streaming
- GIVEN a 1GB file
- WHEN `cat` is executed
- THEN memory usage remains under 10MB
- AND output begins immediately (streaming)

## CLI Interface

```
cat [OPTIONS] [FILE...]

Arguments:
  [FILE...]  Files to concatenate (use stdin if not specified)

Options:
  -h, --help       Print help information
  -n, --number     Number all output lines
  -V, --version    Print version information

Exit Codes:
  0  Success
  1  Error (file not found, permission denied, etc.)
```

## Performance Requirements

| Metric | Target |
|--------|--------|
| Throughput | > 500 MB/s |
| Memory | < 10 MB (streaming) |
| Startup | < 50ms |

## Language Differences

| Feature | Rust | Go |
|---------|------|-----|
| Buffer size | 8KB | 8KB |
| Parallel files | No | Optional (goroutines) |
| Dependencies | std only | std only |
