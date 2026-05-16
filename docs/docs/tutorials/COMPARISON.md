# Rust vs Go 对比

> 所有项目中 Rust 和 Go 实现的全面比较

[English](COMPARISON.md) | **简体中文**

---

## 目录

- [概述](#概述)
- [语言特性](#语言特性)
- [代码统计](#代码统计)
- [实现对比](#实现对比)
- [性能基准测试](#性能基准测试)
- [开发体验](#开发体验)
- [选择建议](#选择建议)
- [总结](#总结)

---

## 概述

本文档提供了 build-your-own-tools 项目中 Rust 和 Go 实现的并排比较。两种语言都非常适合系统编程，但有不同的优势和权衡。

### 快速对比

| 特性 | Rust | Go |
|------|------|-----|
| **内存管理** | 所有权系统，零成本抽象 | 垃圾回收 |
| **并发** | 线程/通道，带所有权约束 | Goroutines，通道 |
| **错误处理** | `Result<T, E>` 和 `?` 操作符 | `error` 接口，`if err != nil` |
| **编译** | 较慢，更多优化 | 快速 |
| **运行时** | 最小化 | GC 运行时 |
| **学习曲线** | 较陡峭 | 较平缓 |
| **二进制大小** | 较小 | 较大（包含运行时） |
| **构建时间** | 较慢 | 较快 |

---

## 语言特性

### 内存安全

**Rust 内存模型**:
- 所有权系统
- 借用检查器
- 无空指针
- 零成本抽象

**Go 内存模型**:
- 垃圾回收器
- 允许空指针
- 运行时安全
- 简单语义

### 并发模型

| 特性 | Rust | Go |
|---------|------|-----|
| 原语 | 线程、async/await | Goroutines |
| 通信 | 通道（有界/无界） | 通道（缓冲/非缓冲） |
| 内存共享 | 编译时无数据竞争 | "通过通信共享" |
| 开销 | 操作系统线程 | 轻量级（2KB 栈） |
| 调度 | 操作系统调度器 | Go 运行时调度器 |

---

## 代码统计

### 行数对比

| 项目 | Rust (LOC) | Go (LOC) | 比例 |
|---------|-----------|----------|-------|
| dos2unix | ~300 | N/A | - |
| gzip | ~400 | ~450 | 1:1.1 |
| htop | ~800 | ~600 | 1:0.75 |
| **总计** | **~1500** | **~1050** | **1:0.7** |

### 依赖对比

| 依赖类型 | Rust | Go |
|----------------|------|-----|
| **gzip 依赖** | 2 (flate2, clap) | 0 (仅标准库) |
| **htop 依赖** | 3 (ratatui, crossterm, sysinfo) | 2 (tview, gopsutil) |
| **编译时间** | ~30s (冷启动) | ~5s |
| **二进制大小** | ~2MB | ~4MB |

---

## 实现对比

### gzip: 压缩逻辑

**Rust (`rgzip`)**:

```rust
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, copy};

/// 使用指定的压缩级别压缩文件
/// 
/// # 参数
/// 
/// * `input` - 输入文件路径
/// * `output` - 输出文件路径
/// * `level` - 压缩级别 (0-9)
/// 
/// # 返回
/// 
/// 成功返回 `Ok(())`，失败返回 I/O 错误
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

// compress 使用指定的压缩级别压缩文件
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

#### 分析

| 特性 | Rust | Go |
|--------|------|-----|
| **错误处理** | `?` 操作符传播错误 | 显式的 `if err != nil` |
| **资源清理** | RAII (Drop trait) | `defer` 语句 |
| **类型安全** | 编译时路径验证 | 运行时验证 |
| **依赖** | 外部依赖 (`flate2`, `clap`) | 标准库 |

---

### htop: TUI 框架

**Rust (ratatui)**:

```rust
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Table, Row},
    style::{Color, Style},
    Frame,
};

fn render(frame: &mut Frame, app: &App) {
    // 布局定义
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    // 类型安全的表格小部件
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

    // 表头行
    headers := []string{"PID", "NAME", "CPU%", "MEM"}
    for i, h := range headers {
        table.SetCell(0, i, 
            tview.NewTableCell(h).
                SetSelectable(false).
                SetAttributes(tview.AttrBold))
    }

    // 数据行
    for i, row := range app.rows {
        table.SetCell(i+1, 0, tview.NewTableCell(fmt.Sprintf("%d", row.PID)))
        table.SetCell(i+1, 1, tview.NewTableCell(row.Name))
        table.SetCell(i+1, 2, tview.NewTableCell(fmt.Sprintf("%.1f", row.CPU)))
        table.SetCell(i+1, 3, tview.NewTableCell(fmt.Sprintf("%d", row.Memory)))
    }

    return tview.NewApplication().SetRoot(table, true)
}
```

#### 分析

| 特性 | Rust (ratatui) | Go (tview) |
|--------|---------------|------------|
| **小部件组合** | 类型安全构建器 | 流式接口 |
| **布局系统** | 基于约束 | 手动定位 |
| **事件处理** | 消息传递 | 基于回调 |
| **样式设置** | 类型安全样式 | 方法链式调用 |

---

### htop: 系统信息

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

#### 分析

| 特性 | Rust (sysinfo) | Go (gopsutil) |
|--------|---------------|---------------|
| **API 风格** | 构建器模式 | 函数式 |
| **错误处理** | 隐式刷新 | 显式错误返回 |
| **PID 处理** | 强类型 (Pid) | 原生 int32 |
| **内存安全** | 编译时检查 | 运行时检查 |

---

## 性能基准测试

### gzip 性能

| 指标 | Rust (flate2) | Go (compress/gzip) |
|--------|---------------|-------------------|
| **压缩速度** | ~50 MB/s | ~60 MB/s |
| **解压速度** | ~150 MB/s | ~120 MB/s |
| **内存使用** | 低（流式） | 低（流式） |
| **CPU 使用** | 单线程 | 可并行 |

### htop 性能

| 指标 | Rust (ratatui) | Go (tview) |
|--------|---------------|------------|
| **启动时间** | ~50ms | ~30ms |
| **内存使用** | ~10MB | ~15MB |
| **刷新率** | 稳定 60 FPS | 稳定 60 FPS |
| **CPU 开销** | <1% | <1% |

> 注意：基准测试在 Linux x86_64, 16GB RAM, SSD 上运行

---

## 开发体验

### Rust 优势

1. **编译时安全**
   - 在运行前捕获空指针错误
   - 所有权防止数据竞争
   - 穷尽模式匹配

2. **零成本抽象**
   - 高级代码编译为高效机器码
   - 安全特性无运行时开销
   - 可预测的性能

3. **优秀的工具链**
   - `cargo` - 依赖管理、构建、测试
   - `clippy` - 高级 linting
   - `rustfmt` - 一致格式化
   - `rust-analyzer` - IDE 支持

4. **富有表现力的类型系统**
   - 带 trait 约束的泛型
   - 代数数据类型（枚举）
   - 模式匹配

### Rust 挑战

1. **陡峭的学习曲线**
   - 所有权和借用概念
   - 生命周期注解
   - 复杂错误类型

2. **较慢的编译**
   - 深度分析安全保证
   - 泛型单态化

3. **复杂的生态系统**
   - 异步运行时碎片化
   - 许多竞争库

### Go 优势

1. **快速迭代**
   - 快速编译
   - 简单语法
   - 内置测试

2. **内置并发**
   - 轻量级 goroutines
   - 基于通道的通信
   - 竞争检测器

3. **丰富的标准库**
   - HTTP 服务器/客户端
   - JSON 编码
   - 压缩 (gzip, zlib)

4. **简单部署**
   - 默认静态二进制
   - 内置交叉编译
   - 单一可执行文件

### Go 挑战

1. **冗长的错误处理**
   - 重复的 `if err != nil` 检查
   - 没有像 Rust 的 `?` 操作符

2. **有限的类型系统**
   - 1.18 之前无泛型
   - 无求和类型
   - 无不变性

3. **GC 停顿**
   - 不可预测的延迟
   - 内存开销

---

## 选择建议

### 选择 Rust 的场景：

- ✅ 性能至关重要
- ✅ 内存安全是首要任务（系统编程）
- ✅ 构建底层工具或库
- ✅ 长期维护很重要
- ✅ 不需要 GC 停顿
- ✅ 嵌入其他语言

**最适合**: 系统工具、库、游戏引擎、Web 服务器

### 选择 Go 的场景：

- ✅ 快速开发是优先事项
- ✅ 构建网络服务（API、微服务）
- ✅ 团队有 Go 经验
- ✅ 需要简单并发
- ✅ 快速编译很重要
- ✅ 需要简单部署

**最适合**: Web 服务、CLI、DevOps 工具、云基础设施

---

## 总结

### 按项目推荐

| 使用场景 | 推荐 |
|----------|-------------|
| CLI 工具 | 两者都可以 |
| 系统编程 | **Rust** |
| 网络服务 | **Go** |
| 性能关键 | **Rust** |
| 快速原型 | **Go** |
| 跨平台 | 两者都可以 |
| TUI 应用 | Rust (ratatui) |
| Web 后端 | Go |

### 代码对比总结

```rust
// Rust: 显式、类型安全、编译时保证
fn process(data: Vec<Item>) -> Result<Output, Error> {
    let result = data.iter()
        .filter(|i| i.is_valid())
        .map(|i| i.transform())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Output::new(result))
}
```

```go
// Go: 简单、显式、直接
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

### 学习路径推荐

```
开始
  │
  ├── 初学者 ──▶ 从 Go 开始
  │                │
  │                ├── 构建 gzip-go
  │                ├── 构建 htop-win-go
  │                └── 接下来学习 Rust
  │
  ├── 中级 ──▶ 根据目标选择
  │            ├── 系统编程 → Rust
  │            └── Web 服务 → Go
  │
  └── 高级 ──▶ 从 Rust 开始
               │
               ├── 构建 dos2unix
               ├── 构建 rgzip
               └── 构建 htop-unix-rust

               比较实现 → 理解权衡
```

---

**术语中英文对照表**

| 英文 | 中文 |
|------|------|
| Borrow Checker | 借用检查器 |
| Concurrency | 并发 |
| Error Handling | 错误处理 |
| Garbage Collection | 垃圾回收 |
| Generic | 泛型 |
| Goroutine | Go 协程 |
| Memory Safety | 内存安全 |
| Ownership | 所有权 |
| Pattern Matching | 模式匹配 |
| Trait | 特性/特征 |
| Zero-cost Abstraction | 零成本抽象 |

---

**最后更新**: 2026-04-16  
**版本**: 2.0
