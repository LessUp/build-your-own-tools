# rgzip

[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/LessUp/build-your-own-tools/blob/master/LICENSE)

A minimal, streaming gzip CLI tool in Rust with a reusable library crate.

## Features

- **Minimal Dependencies** — Only `flate2` and `clap`
- **Streaming I/O** — Handle files of any size
- **Reusable Library** — Core logic in `lib.rs` for embedding
- **Flexible Output** — File, stdout, or custom paths

## Installation

```bash
cd gzip/rust
cargo build --release

# Binary location
./target/release/rgzip
```

## Usage

```
rgzip [OPTIONS] [INPUT]

ARGS:
  INPUT         Input file (default: stdin)

OPTIONS:
  -d, --decompress    Decompress mode
  -o, --output <PATH> Output file path
  -f, --force         Overwrite existing files
  -k, --keep          Keep source file after processing
  -l, --level <0-9>   Compression level (default: 6)
  -h, --help          Show help
  -V, --version       Show version
```

## Examples

### Compression

```bash
# Compress file (creates file.txt.gz, deletes original)
rgzip file.txt

# Compress and keep original
rgzip -k file.txt

# Compress to specific output
rgzip -o archive.gz file.txt

# Compress from stdin
echo "hello world" | rgzip > hello.gz

# Specify compression level
rgzip -l 9 large_file.bin
```

### Decompression

```bash
# Decompress file (creates file.txt, deletes .gz)
rgzip -d file.txt.gz

# Decompress to specific output
rgzip -d -o output.txt file.txt.gz

# Decompress from stdin
rgzip -d < file.txt.gz > file.txt
```

## Library API

```rust
use rgzip::{compress_path, decompress_path, compress_reader_to_writer};
use std::path::Path;

// File to file
compress_path(Path::new("input.txt"), Path::new("output.gz"), 6)?;
decompress_path(Path::new("input.gz"), Path::new("output.txt"))?;

// Stream to stream
let stdin = io::stdin();
let stdout = io::stdout();
compress_reader_to_writer(stdin, stdout, 6)?;
```

### Available Functions

| Function | Description |
|----------|-------------|
| `compress_path` | Compress file to file |
| `decompress_path` | Decompress file to file |
| `compress_reader_to_path` | Compress stream to file |
| `decompress_reader_to_path` | Decompress stream to file |
| `compress_reader_to_writer` | Compress stream to stream |
| `decompress_reader_to_writer` | Decompress stream to stream |
| `default_output_for_compress` | Get default output path (`file.txt` → `file.txt.gz`) |
| `default_output_for_decompress` | Get default output path (`file.txt.gz` → `file.txt`) |
| `ensure_writable` | Check output path is writable |
| `same_path` | Check if two paths are identical |
| `sanitize_level` | Clamp compression level to 0-9 |

## Testing

```bash
cargo test
```

## Comparison with Go Implementation

See [gzip/go/](../go/) for the Go implementation.

| Aspect | Rust | Go |
|--------|------|-----|
| Error handling | `Result<T, E>` with `?` | `if err != nil` |
| Parallelism | Single-threaded (use rayon for parallel) | Built-in goroutines |
| Library | Separate `lib.rs` crate | Single package |

## License

MIT OR Apache-2.0
