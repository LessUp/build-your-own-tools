# dos2unix Specification

## Purpose

Line ending conversion tool that transforms CRLF (Windows) line endings to LF (Unix) line endings, designed for cross-platform file compatibility.

## Requirements

### Requirement: CRLF to LF Conversion

The system SHALL convert CRLF (`\r\n`) sequences to LF (`\n`) sequences.

#### Scenario: Basic conversion
- GIVEN a file containing `line1\r\nline2\r\n`
- WHEN the file is processed
- THEN the output contains `line1\nline2\n`

#### Scenario: Preserve standalone CR
- GIVEN a file containing `line1\r\nline2\r`
- WHEN the file is processed
- THEN the output contains `line1\nline2\r`
- AND standalone CR characters are preserved

#### Scenario: Handle Unix line endings
- GIVEN a file containing only LF line endings
- WHEN the file is processed
- THEN the file remains unchanged

### Requirement: Streaming Processing

The system SHALL support streaming processing for arbitrarily large files.

#### Scenario: Buffer boundary handling
- GIVEN an 8193-byte file with CRLF at position 8192
- WHEN the file is processed
- THEN the CRLF is correctly converted to LF
- AND no data is lost or corrupted

#### Scenario: Empty file handling
- GIVEN an empty file
- WHEN the file is processed
- THEN an empty output is produced
- AND no error occurs

### Requirement: Check Mode

The system SHALL provide a check mode that detects CRLF without modifying files.

#### Scenario: CRLF detection
- GIVEN a file containing CRLF sequences
- WHEN check mode is invoked
- THEN the file is not modified
- AND exit code is 2
- AND CRLF presence is reported

#### Scenario: No CRLF found
- GIVEN a file containing only LF line endings
- WHEN check mode is invoked
- THEN exit code is 0

### Requirement: Pipeline Support

The system SHALL support stdin/stdout for pipeline usage.

#### Scenario: Stdin processing
- GIVEN stdin contains `input\r\ndata`
- WHEN processed via pipeline
- THEN stdout contains `input\ndata`

## CLI Interface

```
dos2unix [OPTIONS] [FILE]

Options:
  -h, --help       Print help information
  -V, --version    Print version information
  -c, --check      Check mode (detect CRLF, no modification)
  -i, --inplace    Convert file in-place
  -q, --quiet      Suppress output messages

Exit Codes:
  0  Success (no CRLF found or conversion complete)
  1  Error occurred
  2  CRLF detected in check mode
```

## Library API

The dos2unix implementation provides a library interface for programmatic use.

### Core Functions

```rust
/// Convert CRLF to LF from reader to writer
///
/// # Arguments
/// * `reader` - Source of input data
/// * `writer` - Destination for converted output
///
/// # Returns
/// Number of CRLF sequences converted
pub fn convert_crlf_to_lf<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<u64>

/// Check if content contains CRLF sequences
///
/// # Returns
/// (has_crlf, crlf_count)
pub fn check_crlf<R: Read>(reader: &mut R) -> io::Result<(bool, u64)>

/// Convert file in-place
///
/// # Arguments
/// * `path` - Path to file to convert
///
/// # Returns
/// Number of CRLF sequences converted
pub fn convert_file_inplace(path: &Path) -> io::Result<u64>
```

## Performance Requirements

| Metric | Target |
|--------|--------|
| Throughput | > 100 MB/s on large files |
| Memory | Constant (streaming) |

---

**Implementation Language**: Rust only
**Complexity Level**: Beginner (⭐)
