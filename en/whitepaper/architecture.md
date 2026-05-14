# System Architecture

This document describes the overall architecture design of the Build Your Own Tools project.

## Architecture Overview

```mermaid
graph TB
    subgraph "User Layer"
        U[User] --> CLI[CLI Tools]
    end
    
    subgraph "Application Layer"
        CLI --> D1[dos2unix]
        CLI --> D2[gzip]
        CLI --> D3[htop]
    end
    
    subgraph "Library Layer"
        D1 --> L1[Stream Processing Library]
        D2 --> L2[Compression Library]
        D3 --> L3[System Monitoring Library]
    end
    
    subgraph "Platform Layer"
        L3 --> P1[Unix API]
        L3 --> P2[Windows API]
    end
    
    subgraph "Engineering"
        E1[OpenSpec] --> E2[Requirements]
        E1 --> E3[Changes]
        E4[CI/CD] --> E5[Build]
        E4 --> E6[Test]
        E4 --> E7[Release]
    end
    
    style U fill:#f59e0b,color:#fff
    style CLI fill:#3b82f6,color:#fff
    style E1 fill:#10b981,color:#fff
    style E4 fill:#10b981,color:#fff
```

## Repository Structure Design

### Monorepo Architecture

The project uses a **Monorepo** architecture, placing all tools in a single repository:

```mermaid
graph LR
    A[build-your-own-tools] --> B[dos2unix/]
    A --> C[gzip/]
    A --> D[htop/]
    A --> E[openspec/]
    A --> F[docs/]
    A --> G[.github/]
    
    B --> B1[Cargo.toml]
    B --> B2[src/]
    
    C --> C1[go/]
    C --> C2[rust/]
    
    D --> D1[unix/]
    D --> D2[win/]
    
    style A fill:#f59e0b,color:#fff
```

**Advantages**:
- Unified version management
- Shared CI/CD
- Atomic commits
- Simplified dependencies

**Disadvantages**:
- Large repository size
- Long build times
- Coarse permission granularity

### Rust Workspace

`gzip/rust/` and `htop/unix/rust/` use Rust workspace:

```toml
# gzip/rust/Cargo.toml
[workspace]
members = ["."]

[package]
name = "gzip"
version = "0.1.0"
```

### Go Workspace

`gzip/go/` and `htop/` use Go workspace:

```text
# go.work
go 1.22

use ./gzip/go
use ./htop/unix/go
use ./htop/win/go
```

## Module Design

### Library + Binary Pattern

Using `gzip/rust/` as an example:

```mermaid
graph LR
    A[gzip] --> B[src/lib.rs]
    A --> C[src/bin/gzip.rs]
    
    B --> D[Compression API]
    B --> E[Decompression API]
    
    C --> F[CLI Parsing]
    C --> G[Call lib]
    
    style A fill:#f59e0b,color:#fff
    style B fill:#3b82f6,color:#fff
    style C fill:#06b6d4,color:#fff
```

**Benefits**:
- The library can be referenced by other projects
- The binary focuses on CLI
- Easy to test

### Source Code Structure

```
gzip/rust/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Library entry point
│   ├── compress.rs     # Compression logic
│   ├── decompress.rs   # Decompression logic
│   └── bin/
│       └── gzip.rs     # CLI entry point
└── tests/
    └── integration.rs
```

## Cross-Platform Strategy

### htop Architecture

htop needs to support both Unix and Windows platforms, using a layered design:

```mermaid
graph TB
    subgraph "Platform-Independent Layer"
        A[CLI] --> B[UI Components]
        B --> C[Platform Abstraction]
    end
    
    subgraph "Platform Adaptation Layer"
        C --> D[Unix Adapter]
        C --> E[Windows Adapter]
    end
    
    subgraph "System Calls"
        D --> F[libc/syscall]
        E --> G[Win32 API]
    end
    
    style A fill:#f59e0b,color:#fff
    style C fill:#10b981,color:#fff
```

### Conditional Compilation

Rust uses `cfg` attributes:

```rust
#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;
```

Go uses build tags:

```go
//go:build unix
package main

//go:build windows
package main
```

### Difference Handling

| Feature | Unix | Windows |
|---------|------|---------|
| Process List | `/proc` filesystem | CreateToolhelp32Snapshot |
| CPU Usage | `/proc/stat` | GetSystemTimes |
| Memory Info | `/proc/meminfo` | GlobalMemoryStatusEx |
| Terminal Size | ioctl TIOCGWINSZ | GetConsoleScreenBufferInfo |

## Data Flow Design

### dos2unix Stream Processing

```mermaid
sequenceDiagram
    participant F as File
    participant R as Reader
    participant P as Processor
    participant W as Writer
    participant O as Output
    
    F->>R: Open file
    R->>P: Read block (8KB)
    P->>P: Convert line endings
    P->>W: Write block
    W->>O: Write to file
    loop Until EOF
        R->>P: Read block
        P->>P: Convert
        P->>W: Write
    end
```

### gzip Compression Pipeline

```mermaid
sequenceDiagram
    participant CLI as CLI
    participant R as Reader
    participant C as Compressor
    participant W as Writer
    
    CLI->>R: Open source file
    CLI->>W: Create output file
    loop
        R->>C: Read data block
        C->>C: Compression processing
        C->>W: Write compressed data
    end
    C->>W: Write GZIP footer
```

## Engineering Architecture

### CI/CD Pipeline

```mermaid
graph LR
    A[Push/PR] --> B[lint]
    A --> C[test]
    A --> D[build]
    
    B --> E{All passed?}
    C --> E
    D --> E
    
    E -->|Yes| F[Merge]
    E -->|No| G[Block merge]
    
    F --> H[Release]
    H --> I[Build artifacts]
    I --> J[GitHub Release]
    
    style A fill:#f59e0b,color:#fff
    style E fill:#3b82f6,color:#fff
    style H fill:#10b981,color:#fff
```

### OpenSpec Workflow

```mermaid
stateDiagram-v2
    [*] --> Proposal: Submit proposal
    Proposal --> Design: Design document
    Design --> Tasks: Task breakdown
    Tasks --> Implementation: Start implementation
    Implementation --> Review: Code review
    Review --> Archived: Merge and archive
    Archived --> [*]
    
    note right of Proposal: openspec/changes/
    note right of Design: design.md
    note right of Tasks: tasks.md
    note right of Archived: archive/
```

## Technical Debt

### Known Issues

| Issue | Impact | Priority |
|-------|--------|----------|
| htop Windows version has incomplete features | Missing functionality | High |
| Lack of benchmark coverage | Performance not measurable | Medium |
| Incomplete documentation internationalization | User experience | Low |

### Improvement Directions

1. **Performance Benchmarks** — Add criterion integration
2. **Fuzz Testing** — Add cargo-fuzz testing
3. **API Documentation** — Generate rustdoc and godoc

## Related Documents

- [Design Decisions](/whitepaper/decisions) — ADR-style decision records
- [Performance Analysis](/whitepaper/performance) — Benchmarks and optimization
- [CI/CD Design](/engineering/cicd) — Workflow details
