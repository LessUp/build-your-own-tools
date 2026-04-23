# gzip Specification

## Purpose

File compression and decompression tool using the DEFLATE algorithm, compatible with the standard gzip format.

## Requirements

### Requirement: File Compression

The system SHALL compress files using the DEFLATE algorithm.

#### Scenario: Basic compression
- GIVEN a text file "document.txt"
- WHEN compression is invoked
- THEN a file "document.txt.gz" is created
- AND the compressed file is smaller than the original
- AND the original file is removed (unless --keep)

#### Scenario: Compression levels
- GIVEN a file with compression level 0
- AND the same file with compression level 9
- WHEN both are compressed
- THEN level 9 produces a smaller file than level 0
- AND both decompress to identical content

#### Scenario: Empty file
- GIVEN an empty file
- WHEN compression is invoked
- THEN a valid gzip file is created
- AND decompression restores an empty file

### Requirement: File Decompression

The system SHALL decompress .gz files to their original content.

#### Scenario: Basic decompression
- GIVEN a compressed file "document.txt.gz"
- WHEN decompression is invoked
- THEN the original file "document.txt" is restored
- AND the content matches the original

#### Scenario: Invalid gzip
- GIVEN a file "fake.gz" with non-gzip content
- WHEN decompression is invoked
- THEN an error is reported
- AND exit code is 1

### Requirement: Compression Levels

The system SHALL support compression levels 0-9.

| Level | Description |
|-------|-------------|
| 0 | No compression (store only) |
| 1-3 | Fast compression |
| 4-6 | Balanced (default: 6) |
| 7-9 | Best compression |

### Requirement: Pipeline Support

The system SHALL support stdin/stdout for pipeline usage.

#### Scenario: Pipeline compression
- GIVEN stdin contains data
- WHEN piped through gzip
- THEN stdout contains compressed gzip data

#### Scenario: Pipeline decompression
- GIVEN stdin contains valid gzip data
- WHEN piped through gzip -d
- THEN stdout contains decompressed data

### Requirement: Keep Source Files

The system SHALL support keeping source files with the --keep option.

#### Scenario: Keep on compress
- GIVEN a file "important.txt"
- WHEN compressed with --keep
- THEN both "important.txt" and "important.txt.gz" exist

## CLI Interface

```
gzip [OPTIONS] [FILE...]

Options:
  -h, --help              Print help information
  -V, --version           Print version information
  -d, --decompress        Decompress mode
  -k, --keep              Keep source files after processing
  -f, --force             Overwrite existing output files
  -r, --recursive         Process directories recursively [Go only]
  -l, --level <LEVEL>     Compression level (0-9, default: 6)
  -o, --output <PATH>     Output file path [Rust only]

Exit Codes:
  0  Success
  1  Error occurred
  2  Warning (file not found, etc.)
```

## Library API

The Rust implementation provides a library crate for embedding compression functionality.

### Compression Functions

```rust
/// Compress data from reader to writer
pub fn compress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    level: Compression,
) -> io::Result<u64>

/// Compress a file to gzip format
pub fn compress_file(
    input_path: &Path,
    output_path: &Path,
    level: Compression,
) -> io::Result<()>
```

### Decompression Functions

```rust
/// Decompress gzip data from reader to writer
pub fn decompress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<u64>

/// Decompress a gzip file
pub fn decompress_file(
    input_path: &Path,
    output_path: &Path,
) -> io::Result<()>
```

### Utility Functions

```rust
/// Generate default output path for compression/decompression
pub fn default_output_path(input_path: &Path, decompress: bool) -> PathBuf

/// Sanitize compression level to valid range (0-9)
pub fn sanitize_level(level: u32) -> Compression
```

## Performance Requirements

| Metric | Target |
|--------|--------|
| Compression speed | > 50 MB/s |
| Decompression speed | > 100 MB/s |

## Language Differences

| Feature | Rust | Go |
|---------|------|-----|
| Recursive (-r) | ❌ | ✅ |
| Parallel processing | ❌ | ✅ |
| Library API | ✅ | ❌ |
| Output path (-o) | ✅ | ❌ |

---

**Implementation Languages**: Rust + Go
**Complexity Level**: Intermediate (⭐⭐)
