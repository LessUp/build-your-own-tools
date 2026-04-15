# dos2unix-rust

[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../../LICENSE)

A lightweight, streaming CRLF to LF converter written in Rust.

## Features

- **Streaming Processing** — Handles files larger than memory
- **In-place Conversion** — Modifies files directly
- **Stdin/Stdout Support** — Works in pipelines
- **Check Mode** — Detect CRLF without modification
- **Cross-buffer CRLF Handling** — Correctly handles CRLF sequences that span buffer boundaries

## Installation

```bash
# Build from source
cd dos2unix
cargo build --release

# Binary location
./target/release/dos2unix-rust  # Unix
.\target\release\dos2unix-rust.exe  # Windows
```

## Usage

```
dos2unix-rust [OPTIONS] [FILES...]

OPTIONS:
  -h, --help       Show help message
  -v, --version    Show version
  -c, --check      Check mode (report CRLF, don't modify)
  -q, --quiet      Quiet mode (less output)

ARGS:
  FILES...         Files to convert (use '-' for stdin)
```

## Examples

### Convert files in-place

```bash
dos2unix-rust file1.txt file2.txt
```

### Pipeline usage

```bash
# Unix
cat file.txt | dos2unix-rust > output.txt

# Windows PowerShell
type file.txt | dos2unix-rust > output.txt
```

### Check for CRLF

```bash
# Exit code 2 if CRLF found
dos2unix-rust --check file.txt
if [ $? -eq 2 ]; then
    echo "File contains CRLF"
fi
```

### Mixed stdin and files

```bash
dos2unix-rust - file1.txt file2.txt
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (no CRLF found in check mode) |
| 1 | Error occurred |
| 2 | CRLF found (check mode only) |

## Implementation Details

### Streaming Algorithm

The tool uses an 8KB buffer for streaming conversion. It tracks the previous byte to handle CRLF sequences that span buffer boundaries:

```
Buffer 1: [..., '\r']  ← CR at end
Buffer 2: ['\n', ...]   ← LF at start → CRLF detected!
```

### Quick Path Optimization

For files without CRLF:
1. Read first 4KB
2. If no CRLF detected, read remaining file
3. Only convert if CRLF found

This avoids unnecessary memory allocation for clean files.

## Testing

```bash
cargo test

# Run specific test
cargo test test_stream_large_data
```

## License

MIT OR Apache-2.0
