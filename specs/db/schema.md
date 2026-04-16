# Data Model Specifications

> Database model definitions and schema conventions

| Metadata | Value |
|----------|-------|
| **Version** | 1.0 |
| **Last Updated** | 2026-04-17 |

---

## Overview

This project does not use traditional databases. It is a CLI tool project that operates on files and system processes directly. This document defines the data models used in memory and for inter-process communication.

---

## Core Data Models

### Process Information (htop)

```rust
/// Process information for display and manipulation
/// 
/// Used across all htop implementations (Unix/Rust, Win/Rust, Win/Go)
pub struct ProcRow {
    /// Process ID (unique identifier)
    pub pid: u32,
    
    /// Process name (executable name)
    pub name: String,
    
    /// CPU usage percentage (0.0 - 100.0)
    /// 
    /// # Calculation
    /// Calculated as (process_cpu_time / total_cpu_time) * 100
    pub cpu_percent: f32,
    
    /// Memory usage in MiB
    /// 
    /// # Platform Notes
    /// - Unix: RSS (Resident Set Size)
    /// - Windows: Working Set
    pub mem_mib: u64,
    
    /// Process status string
    /// 
    /// # Possible Values
    /// - Unix: "Running", "Sleeping", "Stopped", "Zombie", "Unknown"
    /// - Windows: "Running", "Not Responding", "Unknown"
    pub status: String,
    
    /// Full command line with arguments
    pub command: String,
    
    /// Path to executable file
    pub exe_path: String,
}
```

### Sort Configuration

```rust
/// Column keys for process sorting
pub enum SortKey {
    /// Sort by Process ID (ascending by default)
    PID,
    
    /// Sort by process name (alphabetical)
    Name,
    
    /// Sort by CPU usage (descending by default)
    CPU,
    
    /// Sort by memory usage (descending by default)
    Memory,
}
```

### Compression Configuration

```rust
/// Compression level configuration
/// 
/// Maps to DEFLATE algorithm compression levels
pub enum CompressionLevel {
    /// No compression (0)
    None = 0,
    
    /// Fast compression (1-3)
    Fast = 3,
    
    /// Default compression (6)
    Default = 6,
    
    /// Best compression (9)
    Best = 9,
}
```

---

## File Format Specifications

### Gzip File Structure

```
+---+---+---+---+---+---+---+---+---+---+
|ID1|ID2|CM |FLG|     MTIME     |XFL|OS |
+---+---+---+---+---+---+---+---+---+---+
|      ...compressed blocks...          |
+---+---+---+---+---+---+---+---+
|       CRC32       |     ISIZE     |
+---+---+---+---+---+---+---+---+

Fields:
- ID1, ID2: Magic number (0x1f, 0x8b)
- CM: Compression method (8 = DEFLATE)
- FLG: Flags
- MTIME: Modification time (Unix timestamp)
- XFL: Extra flags
- OS: Operating system
- CRC32: CRC-32 of original data
- ISIZE: Original size modulo 2^32
```

### Line Ending Detection

```rust
/// Line ending type detection result
pub enum LineEnding {
    /// Unix line ending (LF, \n)
    LF,
    
    /// Windows line ending (CRLF, \r\n)
    CRLF,
    
    /// Mixed line endings (both found)
    Mixed,
    
    /// No line endings (single line or empty)
    None,
}

/// Detection algorithm
pub fn detect_line_endings(content: &[u8]) -> LineEnding {
    let has_lf = content.contains(&b'\n');
    let has_crlf = content.windows(2).any(|w| w == b"\r\n");
    
    match (has_lf, has_crlf) {
        (true, true) => LineEnding::Mixed,
        (true, false) => LineEnding::LF,
        (false, true) => LineEnding::CRLF,
        (false, false) => LineEnding::None,
    }
}
```

---

## Configuration Files

### Project Configuration Files

| File | Purpose | Format |
|------|---------|--------|
| Cargo.toml | Rust workspace & dependencies | TOML |
| go.mod / go.work | Go modules & workspaces | Go Module |
| .github/workflows/*.yml | CI/CD pipelines | YAML |
| rustfmt.toml | Rust formatting rules | TOML |
| .golangci.yml | Go lint configuration | YAML |
| .editorconfig | Editor unified configuration | INI |
| AGENTS.md | AI Agent workflow | Markdown |

### EditorConfig Schema

```ini
# .editorconfig
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true
indent_style = space
indent_size = 4

[*.rs]
indent_size = 4

[*.go]
indent_style = tab

[Makefile]
indent_style = tab

[*.yml]
indent_size = 2

[*.toml]
indent_size = 4
```

---

## Memory Layout Specifications

### Buffer Sizes

| Use Case | Size | Rationale |
|----------|------|-----------|
| Line conversion | 8 KB | Balance between memory and I/O efficiency |
| Compression streaming | 32 KB | Optimal for DEFLATE algorithm |
| Process list cache | Dynamic | Scales with number of processes |

### Alignment Requirements

```rust
/// Align buffer to optimize SIMD operations where available
#[cfg(target_arch = "x86_64")]
const BUFFER_ALIGNMENT: usize = 32; // AVX2 alignment

#[cfg(target_arch = "aarch64")]
const BUFFER_ALIGNMENT: usize = 16; // NEON alignment
```

---

## Serialization Formats

### JSON Output (Future Feature)

Process information can be serialized to JSON for API consumers:

```json
{
  "pid": 1234,
  "name": "firefox",
  "cpu_percent": 15.5,
  "mem_mib": 512,
  "status": "Running",
  "command": "/usr/bin/firefox",
  "exe_path": "/usr/lib/firefox/firefox"
}
```

---

## Inter-Process Communication

### Signal Definitions (Unix)

| Signal | Value | Action | Description |
|--------|-------|--------|-------------|
| SIGTERM | 15 | Terminate | Graceful termination request |
| SIGKILL | 9 | Terminate | Force termination |
| SIGINT | 2 | Interrupt | Ctrl+C interrupt |

### Process Termination Protocol

```
1. User requests termination (k key)
2. Send SIGTERM (Unix) or WM_CLOSE (Windows)
3. Wait 5 seconds for graceful shutdown
4. If process still running, send SIGKILL (Unix) or TerminateProcess (Windows)
```

---

## Future Considerations

### SQLite Storage (Planned)

If a future feature requires data persistence (e.g., process history tracking):

```sql
-- Schema version tracking
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);

-- Process history (example)
CREATE TABLE process_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    pid INTEGER NOT NULL,
    name TEXT NOT NULL,
    cpu_percent REAL,
    mem_mib INTEGER
);

CREATE INDEX idx_process_history_timestamp ON process_history(timestamp);
CREATE INDEX idx_process_history_pid ON process_history(pid);
```

### JSON Configuration Files

For complex tool configurations:

```json
{
  "$schema": "https://example.com/config.schema.json",
  "version": 1,
  "defaults": {
    "compression_level": 6,
    "refresh_interval_ms": 1000
  }
}
```

---

**Last Updated**: 2026-04-17
