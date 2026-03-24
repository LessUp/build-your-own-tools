# Rust vs Go 实现对比

本文档对比 build-your-own-tools 项目中 Rust 和 Go 两种语言实现的差异和特点。

## 概述

| 特性 | Rust | Go |
|------|------|-----|
| 内存管理 | 所有权系统，零成本抽象 | 垃圾回收 |
| 并发模型 | 线程 / channel / 所有权约束下的并发 | goroutine, channel |
| 错误处理 | Result<T, E> | error 接口 |
| 编译速度 | 较慢 | 快 |
| 运行性能 | 极高 | 高 |
| 学习曲线 | 陡峭 | 平缓 |

## gzip 实现对比

### 代码结构

**Rust 实现**:
```rust
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufReader, BufWriter, copy};

fn compress(input: &Path, output: &Path) -> io::Result<()> {
    let input_file = File::open(input)?;
    let output_file = File::create(output)?;
    
    let mut reader = BufReader::new(input_file);
    let writer = BufWriter::new(output_file);
    let mut encoder = GzEncoder::new(writer, Compression::default());
    
    copy(&mut reader, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}
```

**Go 实现**:
```go
import (
    "compress/gzip"
    "io"
    "os"
)

func compress(input, output string) error {
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
    
    gz := gzip.NewWriter(out)
    defer gz.Close()
    
    _, err = io.Copy(gz, in)
    return err
}
```

### 对比分析

| 方面 | Rust | Go |
|------|------|-----|
| 错误处理 | `?` 操作符简洁传播 | 显式 `if err != nil` |
| 资源管理 | RAII 自动释放 | `defer` 延迟调用 |
| 类型推断 | 强大的类型推断 | 有限的类型推断 |
| 代码行数 | 略多 | 略少 |

## htop 实现对比

### TUI 框架

**Rust (ratatui)**:
```rust
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Table, Row},
    Frame,
};

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.size());
    
    let table = Table::new(app.rows.iter().map(|r| {
        Row::new(vec![r.pid.to_string(), r.name.clone()])
    }))
    .header(Row::new(vec!["PID", "NAME"]))
    .block(Block::default().borders(Borders::ALL));
    
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

### 系统信息获取

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
    
    SystemInfo { cpu_usage, mem_total, mem_used, processes }
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
        CPUUsage: cpuPercent[0],
        MemTotal: memInfo.Total,
        MemUsed:  memInfo.Used,
        Processes: processes,
    }, nil
}
```

## 性能对比

### 编译时间 / 体积 / 运行开销

这些指标会随平台、工具链版本、依赖和构建参数变化很大，仓库当前未维护稳定基准数据。若需要可靠对比，建议在同一机器上按统一脚本重新测量后再记录。

## 开发体验对比

### 优势

**Rust**:
- 编译时捕获更多错误
- 零成本抽象，性能可预测
- 强大的模式匹配
- 优秀的包管理 (Cargo)

**Go**:
- 编译速度快，迭代效率高
- 语法简单，上手快
- 内置并发原语
- 标准库丰富

### 挑战

**Rust**:
- 所有权系统学习曲线陡峭
- 编译时间长
- 生态系统相对年轻

**Go**:
- 泛型支持有限（Go 1.18+改善）
- 错误处理冗长
- 缺少枚举类型

## 选择建议

### 选择 Rust 当：

- 需要极致性能
- 内存安全是关键需求
- 系统级编程
- 长期维护的项目

### 选择 Go 当：

- 需要快速开发迭代
- 网络服务和微服务
- 团队 Go 经验丰富
- 需要简单的并发模型

## 总结

两种语言各有优势，选择取决于具体需求：

- **性能关键型应用**: Rust
- **快速开发**: Go
- **学习系统编程**: 两者都值得学习

本项目同时使用两种语言，正是为了展示它们各自的特点和最佳实践。
