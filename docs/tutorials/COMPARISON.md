# Rust vs Go Comparison

> A comprehensive comparison of Rust and Go implementations across all projects

**English** | [简体中文](COMPARISON.zh-CN.md)

---

## Table of Contents

- [Overview](#overview)
- [Language Features](#language-features)
- [Code Statistics](#code-statistics)
- [Implementation Comparison](#implementation-comparison)
- [Performance Benchmarks](#performance-benchmarks)
- [Development Experience](#development-experience)
- [Recommendations](#recommendations)
- [Summary](#summary)

---

## Overview

This document provides a side-by-side comparison of the Rust and Go implementations in the build-your-own-tools project. Both languages are excellent for systems programming, but have different strengths and trade-offs.

### Quick Comparison

| Feature | Rust | Go |
|------|------|-----|
| **Memory Management** | Ownership system, zero-cost abstractions | Garbage collection |
| **Concurrency** | Threads/channels with ownership constraints | Goroutines, channels |
| **Error Handling** | `Result<T, E>` and `?` operator | `error` interface, `if err != nil` |
| **Compilation** | Slower, more optimizations | Fast |
| **Runtime** | Minimal | GC runtime |
| **Learning Curve** | Steeper | Gentler |
| **Binary Size** | Smaller | Larger (includes runtime) |
| **Build Time** | Slower | Faster |

---

## Language Features

### Memory Safety

**Rust Memory Model**:
- Ownership system
- Borrow checker
- No null pointers
- Zero-cost abstractions

**Go Memory Model**:
- Garbage collector
- Null pointers allowed
- Runtime safety
- Simple semantics

### Concurrency Model

| Feature | Rust | Go |
|---------|------|-----|
| Primitives | Threads, async/await | Goroutines |
| Communication | Channels (bounded/unbounded) | Channels (buffered/unbuffered) |
| Shared Memory | Compile-time data race freedom | "Share by communicating" |
| Overhead | OS threads | Lightweight (2KB stack) |
| Scheduling | OS scheduler | Go runtime scheduler |

---

## Code Statistics

### Lines of Code Comparison

| Project | Rust (LOC) | Go (LOC) | Ratio |
|---------|-----------|----------|-------|
| dos2unix | ~300 | N/A | - |
| gzip | ~400 | ~450 | 1:1.1 |
| htop | ~800 | ~600 | 1:0.75 |
| **Total** | **~1500** | **~1050** | **1:0.7** |

### Dependency Comparison

| Dependency Type | Rust | Go |
|----------------|------|-----|
| **gzip deps** | 2 (flate2, clap) | 0 (stdlib only) |
| **htop deps** | 3 (ratatui, crossterm, sysinfo) | 2 (tview, gopsutil) |
| **Compile time** | ~30s (cold) | ~5s |
| **Binary size** | ~2MB | ~4MB |

---

## Implementation Comparison

### gzip: Compression Logic

**Rust (`rgzip`)**:

```rust
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, copy};

/// Compress a file with specified compression level
/// 
/// # Arguments
/// 
/// * `input` - Input file path
/// * `output` - Output file path
/// * `level` - Compression level (0-9)
/// 
/// # Returns
/// 
/// `Ok(())` on success, I/O error on failure
fn compress(input: &Path, output: &Path, level: u32) -> io::Result<()> {
    let input_file = File::open(input)?;
    let output_file = File::create(output)?;

    let mut reader = BufReader::new(input_file);
    let writer = BufWriter::new(output_file);
    let mut encoder = GzEncoder::new(writer, Compression::new(level));

    copy(&mut reader, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}
```

**Go (`gzip-go`)**:

```go
import (
    "compress/gzip"
    "io"
    "os"
)

// compress compresses a file with specified compression level
func compress(input, output string, level int) error {
    in, err := os.Open(input)
    if err != nil {
        return err
    }
    defer in.Close()

    out, err := os.Create(output)
    if err != nil {
        return err
    }
    defer out.Close()

    gz, err := gzip.NewWriterLevel(out, level)
    if err != nil {
        return err
    }
    defer gz.Close()

    _, err = io.Copy(gz, in)
    return err
}
```

#### Analysis

| Feature | Rust | Go |
|--------|------|-----|
| **Error handling** | `?` operator propagation | Explicit `if err != nil` |
| **Resource cleanup** | RAII (Drop trait) | `defer` statements |
| **Type safety** | Compile-time path validation | Runtime validation |
| **Dependencies** | External (`flate2`, `clap`) | Standard library |

---

### htop: TUI Framework

**Rust (ratatui)**:

```rust
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Table, Row},
    style::{Color, Style},
    Frame,
};

fn render(frame: &mut Frame, app: &App) {
    // Layout definition
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    // Type-safe table widget
    let table = Table::new(
        app.rows.iter().map(|r| {
            Row::new(vec![
                r.pid.to_string(),
                r.name.clone(),
                format!("{:.1}", r.cpu),
                format!("{}", r.memory),
            ])
        })
    )
    .header(Row::new(vec!["PID", "NAME", "CPU%", "MEM"]))
    .block(Block::default().borders(Borders::ALL))
    .row_highlight_style(Style::default().bg(Color::Blue));

    frame.render_widget(table, chunks[1]);
}
```

**Go (tview)**:

```go
import "github.com/rivo/tview"

func createUI(app *App) *tview.Application {
    table := tview.NewTable().
        SetBorders(true).
        SetSelectable(true, false)

    // Header row
    headers := []string{"PID", "NAME", "CPU%", "MEM"}
    for i, h := range headers {
        table.SetCell(0, i, 
            tview.NewTableCell(h).
                SetSelectable(false).
                SetAttributes(tview.AttrBold))
    }

    // Data rows
    for i, row := range app.rows {
        table.SetCell(i+1, 0, tview.NewTableCell(fmt.Sprintf("%d", row.PID)))
        table.SetCell(i+1, 1, tview.NewTableCell(row.Name))
        table.SetCell(i+1, 2, tview.NewTableCell(fmt.Sprintf("%.1f", row.CPU)))
        table.SetCell(i+1, 3, tview.NewTableCell(fmt.Sprintf("%d", row.Memory)))
    }

    return tview.NewApplication().SetRoot(table, true)
}
```

#### Analysis

| Feature | Rust (ratatui) | Go (tview) |
|--------|---------------|------------|
| **Widget composition** | Type-safe builder | Fluent interface |
| **Layout system** | Constraint-based | Manual positioning |
| **Event handling** | Message passing | Callback-based |
| **Styling** | Type-safe styles | Method chaining |

---

### htop: System Information

**Rust (sysinfo)**:

```rust
use sysinfo::{System, SystemExt, ProcessExt, CpuExt};

fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let mem_total = sys.total_memory();
    let mem_used = sys.used_memory();

    let processes: Vec<_> = sys.processes()
        .iter()
        .map(|(pid, proc)| ProcessInfo {
            pid: pid.as_u32(),
            name: proc.name().to_string(),
            cpu: proc.cpu_usage(),
            memory: proc.memory(),
        })
        .collect();

    SystemInfo { 
        cpu_usage, 
        mem_total, 
        mem_used, 
        processes 
    }
}
```

**Go (gopsutil)**:

```go
import (
    "github.com/shirou/gopsutil/v3/cpu"
    "github.com/shirou/gopsutil/v3/mem"
    "github.com/shirou/gopsutil/v3/process"
)

func getSystemInfo() (*SystemInfo, error) {
    cpuPercent, _ := cpu.Percent(0, false)
    memInfo, _ := mem.VirtualMemory()

    procs, _ := process.Processes()
    var processes []ProcessInfo
    for _, p := range procs {
        name, _ := p.Name()
        cpuPct, _ := p.CPUPercent()
        memInfo, _ := p.MemoryInfo()

        processes = append(processes, ProcessInfo{
            PID:    p.Pid,
            Name:   name,
            CPU:    cpuPct,
            Memory: memInfo.RSS,
        })
    }

    return &SystemInfo{
        CPUUsage:   cpuPercent[0],
        MemTotal:   memInfo.Total,
        MemUsed:    memInfo.Used,
        Processes:  processes,
    }, nil
}
```

#### Analysis

| Feature | Rust (sysinfo) | Go (gopsutil) |
|--------|---------------|---------------|
| **API style** | Builder pattern | Functional |
| **Error handling** | Implicit refresh | Explicit error returns |
| **PID handling** | Strongly typed (Pid) | Native int32 |
| **Memory safety** | Compile-time checks | Runtime checks |

---

## Performance Benchmarks

### gzip Performance

| Metric | Rust (flate2) | Go (compress/gzip) |
|--------|---------------|-------------------|
| **Compression speed** | ~50 MB/s | ~60 MB/s |
| **Decompression speed** | ~150 MB/s | ~120 MB/s |
| **Memory usage** | Low (streaming) | Low (streaming) |
| **CPU usage** | Single-threaded | Parallelizable |

### htop Performance

| Metric | Rust (ratatui) | Go (tview) |
|--------|---------------|------------|
| **Startup time** | ~50ms | ~30ms |
| **Memory usage** | ~10MB | ~15MB |
| **Refresh rate** | Stable 60 FPS | Stable 60 FPS |
| **CPU overhead** | <1% | <1% |

> Note: Benchmarks run on Linux x86_64, 16GB RAM, SSD

---

## Development Experience

### Rust Strengths

1. **Compile-time Safety**
   - Catches null pointer errors before runtime
   - Ownership prevents data races
   - Exhaustive pattern matching

2. **Zero-cost Abstractions**
   - High-level code compiles to efficient machine code
   - No runtime overhead for safety features
   - Predictable performance

3. **Excellent Tooling**
   - `cargo` - Dependency management, building, testing
   - `clippy` - Advanced linting
   - `rustfmt` - Consistent formatting
   - `rust-analyzer` - IDE support

4. **Expressive Type System**
   - Generics with trait bounds
   - Algebraic data types (enums)
   - Pattern matching

### Rust Challenges

1. **Steep Learning Curve**
   - Ownership and borrowing concepts
   - Lifetime annotations
   - Complex error types

2. **Slower Compilation**
   - Deep analysis for safety guarantees
   - Generic monomorphization

3. **Complex Ecosystem**
   - Async runtime fragmentation
   - Many competing libraries

### Go Strengths

1. **Fast Iteration**
   - Quick compilation
   - Simple syntax
   - Built-in testing

2. **Built-in Concurrency**
   - Lightweight goroutines
   - Channel-based communication
   - Race detector

3. **Rich Standard Library**
   - HTTP server/client
   - JSON encoding
   - Compression (gzip, zlib)

4. **Simple Deployment**
   - Static binaries by default
   - Built-in cross-compilation
   - Single executable

### Go Challenges

1. **Verbose Error Handling**
   - Repetitive `if err != nil` checks
   - No `?` operator like Rust

2. **Limited Type System**
   - No generics before 1.18
   - No sum types
   - No immutability

3. **GC Pauses**
   - Unpredictable latency
   - Memory overhead

---

## Recommendations

### When to Choose Rust:

- ✅ Performance is critical
- ✅ Memory safety is paramount (systems programming)
- ✅ Building low-level tools or libraries
- ✅ Long-term maintenance matters
- ✅ No GC pauses needed
- ✅ Embedding in other languages

**Best for**: System tools, libraries, game engines, web servers

### When to Choose Go:

- ✅ Fast development is priority
- ✅ Building network services (APIs, microservices)
- ✅ Team has Go experience
- ✅ Simple concurrency needed
- ✅ Quick compilation matters
- ✅ Simple deployment needed

**Best for**: Web services, CLI tools, DevOps tools, cloud infrastructure

---

## Summary

### Per-Project Recommendations

| Use Case | Recommendation |
|----------|-------------|
| CLI tools | Either works |
| Systems programming | **Rust** |
| Network services | **Go** |
| Performance-critical | **Rust** |
| Rapid prototyping | **Go** |
| Cross-platform | Either works |
| TUI applications | Rust (ratatui) |
| Web backends | Go |

### Code Comparison Summary

```rust
// Rust: Explicit, type-safe, compile-time guarantees
fn process(data: Vec<Item>) -> Result<Output, Error> {
    let result = data.iter()
        .filter(|i| i.is_valid())
        .map(|i| i.transform())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Output::new(result))
}
```

```go
// Go: Simple, explicit, straightforward
func process(data []Item) (Output, error) {
    var result []TransformedItem
    for _, i := range data {
        if !i.IsValid() {
            continue
        }
        t, err := i.Transform()
        if err != nil {
            return Output{}, err
        }
        result = append(result, t)
    }
    return NewOutput(result), nil
}
```

### Recommended Learning Path

```
Start
  │
  ├── Beginner ──▶ Start with Go
  │                │
  │                ├── Build gzip-go
  │                ├── Build htop-win-go
  │                └── Then learn Rust
  │
  ├── Intermediate ──▶ Choose based on goals
  │            ├── Systems programming → Rust
  │            └── Web services → Go
  │
  └── Advanced ──▶ Start with Rust
               │
               ├── Build dos2unix
               ├── Build rgzip
               └── Build htop-unix-rust

               Compare implementations → Understand trade-offs
```

---

**Last updated**: 2026-04-16  
**Version**: 2.0
