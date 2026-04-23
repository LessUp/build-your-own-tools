# Proposal: Add cat Tool

## Intent

Implement a `cat` command-line tool that concatenates and displays file contents, as a learning exercise for basic file I/O and streaming in both Rust and Go.

## Scope

### In Scope

- Basic file concatenation to stdout
- Line numbering option (-n)
- Support for multiple files
- stdin support (pipeline usage)
- Both Rust and Go implementations

### Out of Scope

- Syntax highlighting
- Non-printable character display (-v)
- Squeeze blank lines (-s)
- Binary file handling

## Approach

Use streaming I/O to handle large files efficiently. Start with Rust implementation (beginner-friendly), then implement Go version with goroutines for parallel file reading.

## Languages Affected

- [x] Rust
- [x] Go
- [ ] Both (initial implementation)
- [ ] N/A

## Impact

**Affected Tools**: New tool `cat`

**Complexity Level**: Beginner (⭐)

**Learning Goals**:
- File I/O streaming
- Buffer management
- CLI argument parsing
- Cross-platform path handling
