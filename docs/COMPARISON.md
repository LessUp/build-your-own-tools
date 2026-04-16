# Rust vs Go Comparison

This document compares the Rust and Go implementations in the build-your-own-tools project, highlighting language differences and trade-offs.

## Overview

| Aspect | Rust | Go |
|--------|------|-----|
| Memory Management | Ownership system, zero-cost abstraction | Garbage collection |
| Concurrency | Threads/channels with ownership constraints | Goroutines, channels |
| Error Handling | `Result<T, E>` with `?` operator | `error` interface, `if err != nil` |
| Compilation | Slower, more optimizations | Fast |
| Runtime | Minimal | GC runtime |
| Learning Curve | Steeper | Gentler |

## gzip Implementation

### Code Comparison

**Rust (`rgzip`):**

```rust
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, copy};

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

**Go (`gzip-go`):**

```go
import (
    "compress/gzip"
    "io"
    "os"
)

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

### Feature Comparison

| Feature | Rust (`rgzip`) | Go (`gzip-go`) |
|---------|----------------|----------------|
| Parallel processing | ❌ (use rayon) | ✅ goroutines |
| Recursive directory | ❌ | ✅ `-r` flag |
| Library crate | ✅ `lib.rs` | ❌ |
| Keep source | ✅ `-k` flag | ✅ (default) |
| Compression level | ✅ `-l 0-9` | ✅ `-l 0-9` |
| stdin/stdout | ✅ | ✅ |

### Analysis

| Aspect | Rust | Go |
|--------|------|-----|
| Error handling | `?` propagates cleanly | Explicit `if err != nil` |
| Resource cleanup | RAII (Drop trait) | `defer` statements |
| Type safety | Compile-time guarantees | Runtime checks |
| Dependencies | `flate2`, `clap` | Standard library |

## htop Implementation

### TUI Framework Comparison

**Rust (ratatui):**

```rust
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Table, Row},
    Frame,
};

fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let table = Table::new(app.rows.iter().map(|r| {
        Row::new(vec![r.pid.to_string(), r.name.clone()])
    }))
    .header(Row::new(vec!["PID", "NAME"]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(table, chunks[1]);
}
```

**Go (tview):**

```go
import "github.com/rivo/tview"

func createUI(app *App) *tview.Application {
    table := tview.NewTable().
        SetBorders(true).
        SetSelectable(true, false)

    // Header
    table.SetCell(0, 0, tview.NewTableCell("PID").SetSelectable(false))
    table.SetCell(0, 1, tview.NewTableCell("NAME").SetSelectable(false))

    // Data rows
    for i, row := range app.rows {
        table.SetCell(i+1, 0, tview.NewTableCell(fmt.Sprintf("%d", row.PID)))
        table.SetCell(i+1, 1, tview.NewTableCell(row.Name))
    }

    return tview.NewApplication().SetRoot(table, true)
}
```

### System Information APIs

**Rust (sysinfo):**

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

    SystemInfo { cpu_usage, mem_total, mem_used, processes }
}
```

**Go (gopsutil):**

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

### Feature Comparison

| Feature | Rust (Unix/Win) | Go (Win) |
|---------|-----------------|----------|
| Process list | ✅ | ✅ |
| CPU monitoring | ✅ | ✅ |
| Memory monitoring | ✅ | ✅ |
| Process kill | ✅ | ✅ |
| Search/filter | ✅ | ✅ |
| Sort by column | ✅ | ✅ |
| Sparkline history | ✅ (Win) | ❌ |
| Refresh interval | ✅ | ✅ |

## Development Experience

### Rust Advantages

1. **Compile-time safety** - Catches errors before runtime
2. **Zero-cost abstractions** - Predictable performance
3. **Pattern matching** - Expressive control flow
4. **Cargo** - Excellent package manager
5. **No GC pauses** - Consistent latency

### Rust Challenges

1. **Steep learning curve** - Ownership, lifetimes
2. **Slower compilation** - More analysis time
3. **Smaller ecosystem** - Fewer mature libraries

### Go Advantages

1. **Fast compilation** - Quick iteration
2. **Simple syntax** - Easy to learn
3. **Built-in concurrency** - Goroutines/channels
4. **Rich standard library** - Many batteries included
5. **Tooling** - gofmt, go vet, go test

### Go Challenges

1. **Verbose error handling** - `if err != nil` everywhere
2. **No generics** (pre-1.18) - Limited code reuse
3. **GC pauses** - Unpredictable latency
4. **No enums** - Use constants instead

## When to Choose

### Choose Rust When:

- Performance is critical
- Memory safety is paramount
- Building system-level tools
- Long-term maintenance matters
- No GC pauses required

### Choose Go When:

- Rapid development is priority
- Building network services
- Team has Go experience
- Simple concurrency needed
- Fast compilation matters

## Summary

Both languages excel in different scenarios:

| Use Case | Recommended |
|----------|-------------|
| CLI tools | Both work well |
| System programming | Rust |
| Network services | Go |
| Performance-critical | Rust |
| Quick prototyping | Go |
| Cross-platform | Both |

This project demonstrates both approaches, allowing learners to compare and understand the trade-offs firsthand.
