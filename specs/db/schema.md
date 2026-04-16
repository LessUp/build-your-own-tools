# Database Schema Specifications

> Database model definitions and schema conventions

## Overview

This project does not use traditional databases. It is a CLI tool project that operates on files and system processes directly.

## Data Models

### Process Information (htop)

```rust
pub struct ProcRow {
    /// Process ID
    pub pid: Pid,
    /// Process name
    pub name: String,
    /// CPU usage percentage (0.0 - 100.0)
    pub cpu: f32,
    /// Memory usage in MiB
    pub mem_mb: u64,
}
```

### Configuration Files

The project uses configuration files instead of databases:

| File | Purpose | Format |
|------|---------|--------|
| Cargo.toml | Rust workspace & dependencies | TOML |
| go.mod / go.work | Go modules & workspaces | Go Module |
| .github/workflows/*.yml | CI/CD pipelines | YAML |
| rustfmt.toml | Rust formatting rules | TOML |
| .golangci.yml | Go lint configuration | YAML |

## Future Considerations

If a future feature requires data persistence (e.g., process history tracking), consider:

1. **SQLite** for local storage
2. **JSON files** for simple configuration
3. **SQLite migration scripts** in this directory
