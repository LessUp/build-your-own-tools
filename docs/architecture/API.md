# API 参考

> 所有库和模块的完整 API 文档

[English](API.md) | **简体中文**

---

## 目录

- [概述](#概述)
- [dos2unix](#dos2unix)
- [gzip (rgzip)](#gzip-rgzip)
- [htop (htop_shared)](#htop-htop_shared)
- [通用类型](#通用类型)
- [错误处理](#错误处理)
- [示例](#示例)

---

## 概述

本文档提供 build-your-own-tools 项目中库 crate 的详细 API 文档。每个模块都包含：

- 函数签名
- 参数说明
- 返回值
- 错误条件
- 使用示例

### 模块可用性

| 模块 | 语言 | 类型 | 公开 API |
|--------|----------|------|------------|
| dos2unix | Rust | 仅二进制 | ❌ |
| gzip/rust | Rust | 库 + 二进制 | ✅ |
| htop/shared | Rust | 库 | ✅ |
| gzip/go | Go | 仅二进制 | ❌ |
| htop/win/go | Go | 仅二进制 | ❌ |

---

## dos2unix

> **注意**: dos2unix 目前仅为二进制 crate。核心转换函数是内部的，未来可能提取到库中。

### 内部函数

这些函数仅供参考，但不是公共 API 的一部分。

#### `convert_crlf_to_lf`

```rust
fn convert_crlf_to_lf(input: &str) -> String
```

将所有 CRLF (`\r\n`) 序列转换为 LF (`\n`)。

**参数**:
- `input` - 包含潜在 CRLF 序列的输入字符串

**返回**:
- 所有 CRLF 被替换为 LF 的字符串

**示例**:
```rust
let input = "line1\r\nline2\r\n";
let output = convert_crlf_to_lf(input);
assert_eq!(output, "line1\nline2\n");
```

---

## gzip (rgzip)

gzip 的 Rust 实现提供了一个库 crate (`rgzip`) 用于嵌入压缩功能。

### 模块结构

```
rgzip/
├── lib.rs          # 公共 API
├── compress.rs     # 压缩函数
├── decompress.rs   # 解压函数
├── utils.rs        # 工具函数
└── main.rs         # CLI 入口
```

### 压缩函数

#### `compress_reader_to_writer`

```rust
pub fn compress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    level: Compression,
) -> io::Result<u64>
```

使用 DEFLATE 压缩将数据从读取器压缩到写入器。

**参数**:
- `reader` - 未压缩数据的源
- `writer` - 压缩数据的目标
- `level` - 压缩级别 (`Compression::default()`, `Compression::fast()`, `Compression::best()`, 或 `Compression::new(0-9)`)

**返回**:
- `Ok(写入字节数)` - 写入输出的字节数
- `Err(io::Error)` - 压缩过程中的 I/O 错误

**示例**:
```rust
use flate2::Compression;
use rgzip::compress_reader_to_writer;
use std::fs::File;
use std::io::{BufReader, BufWriter};

let input = File::open("input.txt")?;
let output = File::create("output.txt.gz")?;

let mut reader = BufReader::new(input);
let mut writer = BufWriter::new(output);

let bytes_written = compress_reader_to_writer(
    &mut reader,
    &mut writer,
    Compression::default()
)?;

println!("压缩了 {} 字节", bytes_written);
```

#### `compress_file`

```rust
pub fn compress_file(
    input_path: &Path,
    output_path: &Path,
    level: Compression,
) -> io::Result<()>
```

将文件压缩为 gzip 归档。

**参数**:
- `input_path` - 输入文件路径
- `output_path` - 输出 `.gz` 文件路径
- `level` - 压缩级别

**返回**:
- `Ok(())` - 成功
- `Err(io::Error)` - 失败（文件未找到、权限被拒绝等）

**错误**:
- `io::ErrorKind::NotFound` - 输入文件不存在
- `io::ErrorKind::PermissionDenied` - 权限不足
- `io::ErrorKind::AlreadyExists` - 输出文件已存在（无强制标志）

**示例**:
```rust
use flate2::Compression;
use rgzip::compress_file;
use std::path::Path;

compress_file(
    Path::new("document.txt"),
    Path::new("document.txt.gz"),
    Compression::best()
)?;
```

### 解压函数

#### `decompress_reader_to_writer`

```rust
pub fn decompress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<u64>
```

将 gzip 数据从读取器解压到写入器。

**参数**:
- `reader` - 压缩数据的源（必须是 gzip 格式）
- `writer` - 解压数据的目标

**返回**:
- `Ok(写入字节数)` - 写入输出的字节数
- `Err(io::Error)` - I/O 错误或无效的 gzip 数据

**错误**:
- `io::ErrorKind::InvalidData` - 输入不是有效的 gzip 数据
- `io::ErrorKind::UnexpectedEof` - 被截断的 gzip 数据

**示例**:
```rust
use rgzip::decompress_reader_to_writer;
use std::fs::File;
use std::io::{BufReader, BufWriter};

let input = File::open("archive.txt.gz")?;
let output = File::create("archive.txt")?;

let mut reader = BufReader::new(input);
let mut writer = BufWriter::new(output);

let bytes_written = decompress_reader_to_writer(&mut reader, &mut writer)?;
println!("解压了 {} 字节", bytes_written);
```

#### `decompress_file`

```rust
pub fn decompress_file(
    input_path: &Path,
    output_path: &Path,
) -> io::Result<()>
```

将 gzip 归档解压为文件。

**参数**:
- `input_path` - 输入 `.gz` 文件路径
- `output_path` - 输出文件路径

**返回**:
- `Ok(())` - 成功
- `Err(io::Error)` - 失败

**示例**:
```rust
use rgzip::decompress_file;
use std::path::Path;

decompress_file(
    Path::new("backup.txt.gz"),
    Path::new("backup.txt")
)?;
```

### 工具函数

#### `default_output_path`

```rust
pub fn default_output_path(
    input_path: &Path,
    decompress: bool,
) -> PathBuf
```

生成压缩或解压的默认输出路径。

**行为**:
- 压缩: 添加 `.gz` 后缀
- 解压: 移除 `.gz` 后缀，或无后缀时添加 `.out`

**参数**:
- `input_path` - 输入文件路径
- `decompress` - 是否用于解压

**返回**:
- 生成的输出路径

**示例**:
```rust
use rgzip::default_output_path;
use std::path::Path;

// 压缩
let out = default_output_path(Path::new("file.txt"), false);
assert_eq!(out, PathBuf::from("file.txt.gz"));

// 解压
let out = default_output_path(Path::new("file.txt.gz"), true);
assert_eq!(out, PathBuf::from("file.txt"));
```

#### `sanitize_level`

```rust
pub fn sanitize_level(level: u32) -> Compression
```

将原始压缩级别转换为有效的 `Compression` 值。

**参数**:
- `level` - 原始级别 (0-9，但接受任何 u32)

**返回**:
- 限制在有效范围 (0-9) 内的 `Compression`

**示例**:
```rust
use flate2::Compression;
use rgzip::sanitize_level;

assert_eq!(sanitize_level(5), Compression::new(5));
assert_eq!(sanitize_level(15), Compression::best()); // 限制为 9
assert_eq!(sanitize_level(0), Compression::none());
```

### 完整示例

```rust
use rgzip::{compress_file, decompress_file, sanitize_level};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 压缩
    compress_file(
        Path::new("data.txt"),
        Path::new("data.txt.gz"),
        sanitize_level(6)
    )?;
    
    println!("压缩完成!");
    
    // 解压
    decompress_file(
        Path::new("data.txt.gz"),
        Path::new("data_restored.txt")
    )?;
    
    println!("解压完成!");
    
    Ok(())
}
```

---

## htop (htop_shared)

htop 实现的共享库，提供通用的数据结构和工具。

### 模块结构

```
htop_shared/
├── lib.rs              # 公共 API
├── models.rs           # 数据结构
├── sorting.rs          # 排序逻辑
├── filtering.rs        # 过滤逻辑
└── styling.rs          # 终端样式
```

### 数据结构

#### `ProcRow`

用于显示的进程信息行。

```rust
pub struct ProcRow {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_mib: u64,
    pub status: String,
    pub command: String,
    pub exe_path: String,
}
```

**字段**:
| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `pid` | `u32` | 进程 ID |
| `name` | `String` | 进程名称 |
| `cpu_percent` | `f32` | CPU 使用百分比 |
| `mem_mib` | `u64` | 内存使用 (MiB) |
| `status` | `String` | 进程状态 (Running, Sleeping 等) |
| `command` | `String` | 完整命令行 |
| `exe_path` | `String` | 可执行文件路径 |

**示例**:
```rust
use htop_shared::ProcRow;

let row = ProcRow {
    pid: 1234,
    name: "firefox".to_string(),
    cpu_percent: 15.5,
    mem_mib: 512,
    status: "Running".to_string(),
    command: "/usr/bin/firefox".to_string(),
    exe_path: "/usr/lib/firefox/firefox".to_string(),
};
```

#### `SortKey`

进程排序的列键。

```rust
pub enum SortKey {
    PID,
    Name,
    CPU,
    Memory,
}
```

**变体**:
- `PID` - 按进程 ID 排序
- `Name` - 按进程名称排序（字母顺序）
- `CPU` - 按 CPU 使用率排序（降序）
- `Memory` - 按内存使用率排序（降序）

**示例**:
```rust
use htop_shared::SortKey;

let sort = SortKey::CPU; // 按 CPU 使用率排序
```

### 函数

#### `compare_proc_rows`

```rust
pub fn compare_proc_rows(
    a: &ProcRow,
    b: &ProcRow,
    sort_key: SortKey,
    ascending: bool,
) -> Ordering
```

比较两个进程行用于排序。

**参数**:
- `a` - 第一个进程行
- `b` - 第二个进程行
- `sort_key` - 排序列
- `ascending` - 排序方向 (true = 升序, false = 降序)

**返回**:
- `Ordering` - 比较结果

**示例**:
```rust
use htop_shared::{compare_proc_rows, ProcRow, SortKey};

let rows: Vec<ProcRow> = /* 获取进程 */;
let mut sorted = rows.clone();
sorted.sort_by(|a, b| compare_proc_rows(a, b, SortKey::CPU, false));
```

#### `filter_processes`

```rust
pub fn filter_processes(
    processes: &[ProcRow],
    query: &str,
) -> Vec<ProcRow>
```

通过搜索查询过滤进程。

**参数**:
- `processes` - 源进程列表
- `query` - 搜索字符串（不区分大小写，匹配名称或命令）

**返回**:
- 匹配进程的过滤向量

**示例**:
```rust
use htop_shared::{filter_processes, ProcRow};

let all: Vec<ProcRow> = /* 获取所有进程 */;
let filtered = filter_processes(&all, "firefox");
// 返回名称或命令中包含 "firefox" 的进程
```

#### `color_for_ratio`

```rust
pub fn color_for_ratio(ratio: f32) -> Color
```

返回使用率比例 (0.0 - 1.0) 的适当颜色。

**颜色阈值**:
| 比例范围 | 颜色 |
|-------------|-------|
| 0.0 - 0.6 | 绿色 |
| 0.6 - 0.85 | 黄色 |
| 0.85 - 1.0 | 红色 |

**参数**:
- `ratio` - 使用率比例 (0.0 到 1.0)

**返回**:
- `Color` - ratatui 显示颜色

**示例**:
```rust
use htop_shared::color_for_ratio;
use ratatui::style::Color;

let cpu_color = color_for_ratio(0.75); // 返回 Color::Yellow
let mem_color = color_for_ratio(0.95); // 返回 Color::Red
```

#### `highlight_style`

```rust
pub fn highlight_style() -> Style
```

返回高亮/选中表格行的默认样式。

**返回**:
- `Style` - 蓝色背景的 ratatui 样式

**示例**:
```rust
use htop_shared::highlight_style;
use ratatui::widgets::Table;

let table = Table::new(/* rows */)
    .row_highlight_style(highlight_style());
```

#### `resolve_selected_index`

```rust
pub fn resolve_selected_index(
    current: usize,
    filtered_count: usize,
) -> usize
```

解析过滤后的选中索引，确保其在范围内。

**参数**:
- `current` - 当前选中的索引
- `filtered_count` - 过滤后的项目数量

**返回**:
- `usize` - 验证后的索引

**示例**:
```rust
use htop_shared::resolve_selected_index;

let selected = 10;
let filtered = 5;
let new_selected = resolve_selected_index(selected, filtered);
// 返回 4 (最后一个有效索引)
```

#### `selected_pid`

```rust
pub fn selected_pid(
    processes: &[ProcRow],
    selected_index: usize,
) -> Option<u32>
```

获取列表中选中索引处进程的 PID。

**参数**:
- `processes` - 进程列表
- `selected_index` - 列表中的选中索引

**返回**:
- `Some(pid)` - 索引有效时的 PID
- `None` - 索引越界时

**示例**:
```rust
use htop_shared::{selected_pid, ProcRow};

let processes: Vec<ProcRow> = /* 获取 */;
if let Some(pid) = selected_pid(&processes, 0) {
    println!("选中的 PID: {}", pid);
}
```

### 完整示例

```rust
use htop_shared::{
    compare_proc_rows, filter_processes, color_for_ratio,
    highlight_style, selected_pid, ProcRow, SortKey
};
use ratatui::widgets::Table;

fn main() {
    // 获取进程（平台特定）
    let processes: Vec<ProcRow> = fetch_processes();
    
    // 过滤
    let filtered = filter_processes(&processes, "firefox");
    
    // 按 CPU 使用率排序
    let mut sorted = filtered.clone();
    sorted.sort_by(|a, b| compare_proc_rows(a, b, SortKey::CPU, false));
    
    // 创建带高亮的表格
    let table = Table::new(sorted.iter().map(|p| /* 行转换 */))
        .row_highlight_style(highlight_style());
    
    // 获取选中的 PID
    if let Some(pid) = selected_pid(&sorted, 0) {
        println!("终止 PID: {}", pid);
    }
}
```

---

## 通用类型

### 错误类型

所有函数都返回 `std::io::Error` 以保持一致性：

```rust
use std::io::{Error, ErrorKind};

// 创建错误
let not_found = Error::new(ErrorKind::NotFound, "文件未找到");
let invalid = Error::new(ErrorKind::InvalidData, "损坏的 gzip 数据");
```

### 路径处理

```rust
use std::path::{Path, PathBuf};

// 从字符串
let path = Path::new("/home/user/file.txt");

// 从 PathBuf
let pathbuf = PathBuf::from("/home/user/file.txt");

// 扩展名操作
assert_eq!(path.extension(), Some("txt".as_ref()));
```

---

## 错误处理

### Rust 模式

**使用 `?` 操作符**:
```rust
fn process_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;  // 错误时提前返回
    let mut reader = BufReader::new(file);
    // ...
    Ok(())
}
```

**使用 `match`**:
```rust
match compress_file(input, output, level) {
    Ok(()) => println!("成功!"),
    Err(e) => eprintln!("错误: {}", e),
}
```

**使用 `if let`**:
```rust
if let Err(e) = decompress_file(input, output) {
    eprintln!("解压失败: {}", e);
}
```

---

## 示例

### 示例 1: 批量压缩

```rust
use rgzip::{compress_file, sanitize_level};
use std::path::Path;

fn compress_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().is_none() {
            let output = path.with_extension("gz");
            compress_file(&path, &output, sanitize_level(6))?;
            println!("压缩: {} -> {}", path.display(), output.display());
        }
    }
    Ok(())
}
```

### 示例 2: 进程监控

```rust
use htop_shared::{filter_processes, compare_proc_rows, ProcRow, SortKey};

fn find_top_cpu_processes(processes: &[ProcRow], n: usize) -> Vec<ProcRow> {
    let mut sorted = processes.to_vec();
    sorted.sort_by(|a, b| compare_proc_rows(a, b, SortKey::CPU, false));
    sorted.into_iter().take(n).collect()
}

fn monitor_processes(name: &str) {
    // 获取进程（平台特定实现）
    let all = fetch_all_processes();
    let matching = filter_processes(&all, name);
    let top = find_top_cpu_processes(&matching, 5);
    
    for p in top {
        println!("{}: CPU {:.1}%, MEM {} MiB", 
            p.name, p.cpu_percent, p.mem_mib);
    }
}
```

### 示例 3: 流式管道

```rust
use rgzip::compress_reader_to_writer;
use flate2::Compression;
use std::io::{self, Read, Write};

fn pipe_compress() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();
    
    let bytes = compress_reader_to_writer(
        &mut reader,
        &mut writer,
        Compression::fast()
    )?;
    
    eprintln!("压缩了 {} 字节", bytes);
    Ok(())
}
```

---

**术语中英文对照表**

| 英文 | 中文 |
|------|------|
| Compression | 压缩 |
| Decompression | 解压 |
| Error Handling | 错误处理 |
| Filter | 过滤 |
| Module | 模块 |
| Process | 进程 |
| Sort | 排序 |
| Stream | 流 |
| Utility | 工具 |

---

**最后更新**: 2026-04-16  
**版本**: 1.0
