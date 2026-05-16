# gzip-go

[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../../LICENSE)

A minimal Go gzip CLI tool with parallel processing and recursive directory support.

## Features

- **Parallel Processing** — Compress multiple files concurrently using goroutines
- **Recursive Mode** — Process entire directories with `-r` flag
- **Streaming I/O** — Full stdin/stdout support for pipelines
- **Compression Levels** — Support for levels 0-9
- **KISS Principle** — Does not delete source files by default

## Installation

```bash
cd gzip/go
mkdir -p bin
go build -o bin/gzip-go ./cmd/gzip-go
```

### Cross-Compilation

```bash
# Linux amd64
GOOS=linux GOARCH=amd64 go build -o bin/gzip-go ./cmd/gzip-go

# macOS arm64
GOOS=darwin GOARCH=arm64 go build -o bin/gzip-go ./cmd/gzip-go
```

### Using Makefile

```bash
make build        # Build for current platform
make build-linux  # Cross-compile for Linux amd64
sudo make install # Install to /usr/local/bin/gzip-go
```

## Usage

```
gzip-go [flags] [files or dirs]

flags:
  -d           Decompress mode (gunzip)
  -r           Recursive directory processing
  -stdout      Output to stdout (single input only)
  -f           Overwrite existing files
  -l int       Compression level 0-9 (default -1)
  -p int       Parallel workers (default: CPU cores)
  -k           Keep source files (default behavior)
```

## Examples

### Compression

```bash
# Compress files (keeps originals)
gzip-go file1.txt file2.txt

# Compress from stdin
echo "hello world" | gzip-go > hello.gz

# Recursive directory compression
gzip-go -r /var/log

# Specify compression level
gzip-go -l 9 large_file.bin
```

### Decompression

```bash
# Decompress files
gzip-go -d file.txt.gz

# Decompress to stdout
gzip-go -d -stdout file.txt.gz > file.txt

# Recursive decompression
gzip-go -d -r /data/backups
```

## Behavior

| Operation | Input | Output | Source File |
|-----------|-------|--------|-------------|
| Compress | `file.txt` | `file.txt.gz` | Kept |
| Decompress | `file.txt.gz` | `file.txt` | Kept |
| Stdin | stdin | stdout | N/A |

- Source files are **not deleted** (unlike GNU gzip)
- Existing files are skipped unless `-f` is specified
- Symbolic links are dereferenced

## Comparison with Rust Implementation

| Feature | Go (`gzip-go`) | Rust (`rgzip`) |
|---------|----------------|----------------|
| Parallel processing | ✅ goroutines | ❌ (use rayon) |
| Recursive mode | ✅ `-r` | ❌ |
| Library crate | ❌ | ✅ `lib.rs` |
| Keep source | ✅ (default) | ✅ `-k` flag |
| Compression level | ✅ | ✅ |

## Testing

```bash
go test -v ./...
```

## License

MIT OR Apache-2.0
