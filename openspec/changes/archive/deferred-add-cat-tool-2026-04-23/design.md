# Design: Add cat Tool

## Technical Approach

Implement `cat` as a streaming file reader that writes directly to stdout. Use a fixed-size buffer (8KB) for efficient I/O. The -n option requires line-by-line processing, which changes the streaming strategy.

## Architecture Decisions

### Decision: Streaming vs Buffered

**Context**: Need to handle large files efficiently without loading entire file into memory.

**Decision**: Use streaming I/O with a fixed buffer for basic concatenation. For -n option, use line-buffered reading.

**Rationale**: 
- Streaming provides constant memory usage
- Line buffering for -n is necessary to number lines
- 8KB buffer is optimal for most filesystems

**Alternatives Considered**:
1. Memory-mapped files - Rejected: complexity not justified for simple concatenation
2. Async I/O - Rejected: overhead not beneficial for sequential reading

### Decision: Buffer Size

**Context**: Need to choose optimal buffer size for I/O efficiency.

**Decision**: Use 8KB (8192 bytes) buffer.

**Rationale**:
- Matches typical filesystem block size
- Good balance between memory usage and I/O calls
- Works well across platforms

## Data Structures

```rust
// Rust
use std::io::{self, BufReader, BufWriter, Read, Write};

const BUFFER_SIZE: usize = 8 * 1024; // 8KB

pub struct CatOptions {
    pub number_lines: bool,
}

pub fn cat_file<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    options: &CatOptions,
) -> io::Result<()>
```

```go
// Go
package cat

const BufferSize = 8 * 1024 // 8KB

type Options struct {
    NumberLines bool
}

func CatFile(r io.Reader, w io.Writer, opts *Options) error
```

## Algorithms

### Basic Concatenation (without -n)

```
1. Open file or use stdin
2. Read into buffer
3. Write buffer to stdout
4. Repeat until EOF
5. Close file
```

Time Complexity: O(n) where n is total bytes
Space Complexity: O(1) - fixed buffer

### Line Numbering (with -n)

```
1. Open file or use stdin
2. Create line reader
3. For each line:
   a. Format line number (right-aligned, width 6)
   b. Write "     N\t"
   c. Write line content
   d. Increment counter
4. Close file
```

Time Complexity: O(n)
Space Complexity: O(max_line_length)

## File Organization

### Rust

```
cat/
├── Cargo.toml
├── src/
│   ├── main.rs       # CLI entry point, argument parsing
│   ├── lib.rs        # Library exports
│   └── cat.rs        # Core cat implementation
└── tests/
    ├── integration_tests.rs
    └── fixtures/
        ├── empty.txt
        ├── small.txt
        └── multiline.txt
```

### Go

```
cat/
├── go.mod
├── main.go           # CLI entry point
├── cat.go            # Core implementation
├── cat_test.go       # Unit tests
└── testdata/
    ├── empty.txt
    ├── small.txt
    └── multiline.txt
```

## Error Handling

| Error Type | Rust | Go |
|------------|------|-----|
| FileNotFound | `io::ErrorKind::NotFound` | `os.ErrNotExist` |
| PermissionDenied | `io::ErrorKind::PermissionDenied` | `os.ErrPermission` |
| StdinRead | Propagate `io::Error` | Return `error` |

### Error Messages

```
cat: <filename>: No such file or directory
cat: <filename>: Permission denied
cat: error reading stdin: <details>
```

## Testing Strategy

### Unit Tests

| Test | Description |
|------|-------------|
| `test_cat_empty_file` | Empty file produces no output |
| `test_cat_single_file` | Single file content output |
| `test_cat_multiple_files` | Multiple files concatenated |
| `test_cat_stdin` | Stdin passthrough |
| `test_cat_line_numbers` | -n option works correctly |
| `test_cat_file_not_found` | Error handling for missing file |
| `test_cat_large_file` | Streaming with 1GB+ file |

### Integration Tests

| Test | Description |
|------|-------------|
| `test_cli_basic` | CLI invocation works |
| `test_cli_multiple_args` | Multiple file arguments |
| `test_cli_stdin_pipe` | Pipeline: `echo test \| cat` |
| `test_cli_exit_codes` | Exit codes are correct |

### Performance Tests

```bash
# Throughput test
dd if=/dev/zero bs=1M count=1000 | ./cat > /dev/null

# Memory test
/usr/bin/time -v ./cat largefile.txt > /dev/null
```

## Dependencies

| Dependency | Rust Crate | Go Package | Purpose |
|------------|------------|------------|---------|
| CLI parsing | clap | flag | Argument parsing |
| Testing | built-in | built-in | Unit tests |
| I/O | std::io | io, os | File operations |

Both implementations use only standard library for core functionality.

## Cross-Platform Considerations

### Path Handling
- Rust: Use `std::path::Path` for portable paths
- Go: Use `filepath` package

### Line Endings
- Output preserves original line endings
- No automatic conversion (unlike dos2unix)

### Binary Mode
- Open files in binary mode to preserve exact content
- No text mode translation

## Implementation Order

1. Rust core implementation (cat.rs)
2. Rust CLI (main.rs)
3. Rust tests
4. Go core implementation (cat.go)
5. Go CLI (main.go)
6. Go tests
7. Documentation
8. CI integration
