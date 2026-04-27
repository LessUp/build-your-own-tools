# API 参考

> 核心库模块的 API 概览

[English](API.md) | **简体中文**

---

## 概述

本文档提供 build-your-own-tools 项目中库 crate 的 API 概览。

### 模块可用性

| 模块 | 语言 | 类型 | 公开 API |
|--------|----------|------|------------|
| dos2unix | Rust | 仅二进制 | ❌ |
| gzip/rust | Rust | 库 + 二进制 | ✅ |
| htop/shared | Rust | 库 | ✅ |
| gzip/go | Go | 仅二进制 | ❌ |
| htop/win/go | Go | 仅二进制 | ❌ |

---

## gzip (rgzip)

gzip 的 Rust 实现提供库 crate (`rgzip`) 用于嵌入压缩功能。

### 核心函数

```rust
// 压缩
pub fn compress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    level: Compression,
) -> io::Result<u64>

pub fn compress_file(input: &Path, output: &Path, level: Compression) -> io::Result<u64>

// 解压
pub fn decompress_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<u64>

pub fn decompress_file(input: &Path, output: &Path) -> io::Result<u64>
```

### Compression 枚举

```rust
pub enum Compression {
    Fast,    // 快速压缩，较低压缩率
    Default, // 默认平衡
    Best,    // 最佳压缩，较慢
}
```

### 使用示例

```rust
use rgzip::{compress_file, decompress_file, Compression};

// 压缩文件
compress_file(Path::new("input.txt"), Path::new("output.gz"), Compression::Default)?;

// 解压文件
decompress_file(Path::new("output.gz"), Path::new("output.txt"))?;
```

---

## htop (htop_shared)

htop 的共享库提供进程表格和排序功能。

### 核心类型

```rust
/// 排序键
pub enum SortKey {
    Cpu,  // 按 CPU 使用率
    Mem,  // 按内存使用
    Pid,  // 按 PID
    Name, // 按进程名
}

/// 进程表格行
pub struct ProcRow {
    pub pid: Pid,
    pub name: String,
    pub cpu: f32,
    pub mem_mb: u64,
}
```

### 核心函数

```rust
// 比较进程行
pub fn compare_proc_rows(a: &ProcRow, b: &ProcRow, sort: SortKey) -> Ordering

// 过滤进程
pub fn filter_processes(processes: Vec<ProcRow>, filter: &str) -> Vec<ProcRow>

// 获取选中 PID
pub fn selected_pid(processes: &[ProcRow], selected: usize) -> Option<Pid>

// 解析选中索引
pub fn resolve_selected_index(
    processes: &[ProcRow],
    preferred_pid: Option<Pid>,
    fallback_index: usize,
) -> usize

// 颜色计算
pub fn color_for_ratio(ratio: f32) -> Color

// 高亮样式
pub fn highlight_style() -> Style
```

### 使用示例

```rust
use htop_shared::{ProcRow, SortKey, compare_proc_rows, filter_processes};

// 排序进程
processes.sort_by(|a, b| compare_proc_rows(a, b, SortKey::Cpu));

// 过滤进程
let filtered = filter_processes(processes, "python");
```

---

## dos2unix

> **注意**: dos2unix 目前仅为二进制 crate，核心转换函数是内部的。

### 内部函数

```rust
// CRLF 转 LF
fn convert_crlf_to_lf(input: &str) -> String

// LF 转 CRLF
fn convert_lf_to_crlf(input: &str) -> String
```

---

## 错误处理

所有库函数使用 `std::io::Result` 或 `anyhow::Result` 进行错误处理：

```rust
// 常见错误类型
- io::ErrorKind::NotFound      // 文件不存在
- io::ErrorKind::InvalidData   // 无效的压缩数据
- io::ErrorKind::PermissionDenied // 权限不足
```

---

## 更多信息

详细的函数文档请参阅各模块的 rustdoc：

```bash
cargo doc --open
```
