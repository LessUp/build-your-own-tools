---
title: 模块一 - dos2unix
---

# 模块一：dos2unix — 流处理入门

> ⭐ 复杂度：入门级
>
> 从最简单的真实工具开始，理解流式 I/O 和文本处理的基础模式。

## 为什么从 dos2unix 开始？

dos2unix 是一个看似简单但蕴含重要设计考量的工具：

1. **概念简单**：只做一件事——转换换行符
2. **工程完整**：包含 CLI 解析、文件 I/O、错误处理
3. **流式处理**：天然适合理解 I/O 管道的概念
4. **边界丰富**：看似简单的问题，实际有大量的边界条件需要处理

## 核心知识点

### 1. 流式 I/O

dos2unix 的核心是**流式处理**——逐字节读取、转换、输出，而非将整个文件加载到内存。

```rust
// Rust: 使用 BufReader 实现流式读取
use std::io::{BufReader, BufWriter, Read, Write};

fn process_stream<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), io::Error> {
    let mut buf_reader = BufReader::new(reader);
    let mut buf_writer = BufWriter::new(writer);
    let mut prev = None;
    
    let mut byte = [0u8; 1];
    while buf_reader.read_exact(&mut byte).is_ok() {
        match (prev, byte[0]) {
            (Some(b'\r'), b'\n') => {
                // CRLF -> LF: 跳过 \r，输出 \n
                buf_writer.write_all(b"\n")?;
            }
            (Some(b'\r'), other) => {
                // 孤立的 \r: 输出 \r 和当前字节
                buf_writer.write_all(&[b'\r', other])?;
            }
            (None | Some(_), b'\r') => {
                // 可能是 CRLF 的开始，暂存
            }
            (None | Some(_), other) => {
                buf_writer.write_all(&[other])?;
            }
        }
        prev = if byte[0] == b'\r' { Some(b'\r') } else { None };
    }
    
    // 处理末尾孤立的 \r
    if prev == Some(b'\r') {
        buf_writer.write_all(b"\r")?;
    }
    
    buf_writer.flush()?;
    Ok(())
}
```

**关键设计考量**：
- 为什么选择 BufReader/BufWriter？—— 减少 syscall 开销
- 缓冲区大小如何选择？—— 通常 8KB 是一个合理的默认值
- 何时 flush？—— 确保所有数据写入磁盘

### 2. 换行符的历史与差异

| 格式 | 字节序列 | 操作系统 | 起源 |
|------|----------|----------|------|
| LF | `0x0A` | Unix/Linux/macOS | Multics 时代 |
| CRLF | `0x0D 0x0A` | Windows | 电传打字机时代 |
| CR | `0x0D` | 经典 Mac OS (≤9) | Apple II 时代

> **💡 历史注记**：CRLF 的起源可以追溯到电传打字机（Teletype）。CR（Carriage Return）让打印头回到行首，LF（Line Feed）让纸张前进一行。Unix 简化为只用 LF，而 DOS/Windows 保留了 CRLF 传统。

### 3. 错误处理模式

```rust
// Rust: 使用 Result 类型进行显式错误处理
fn convert_file(input: &Path, output: &Path) -> Result<(), ConvertError> {
    let input_file = File::open(input)
        .map_err(|e| ConvertError::InputOpen(input.to_path_buf(), e))?;
    let output_file = File::create(output)
        .map_err(|e| ConvertError::OutputCreate(output.to_path_buf(), e))?;
    // ...
}
```

**Rust vs Go 错误处理对比**：
- **Rust**：`Result<T, E>` 类型，编译器强制处理错误，`?` 运算符简化传播
- **Go**：`error` 接口，`if err != nil` 模式，显式但冗长

### 4. CLI 参数解析

dos2unix 的命令行接口设计：

```bash
# 基本用法
dos2unix file.txt           # 原地转换
dos2unix -n input.txt output.txt  # 指定输出文件

# 选项
dos2unix -r file.txt        # unix2dos（反向转换）
dos2unix -i file.txt        # 显示文件信息
dos2unix -l                 # 从标准输入读取
```

## Rust 实现解析

本项目的 dos2unix 使用 Rust 实现，主要技术选型：

- **CLI 解析**：使用 `clap` 库，声明式定义命令行参数
- **文件操作**：标准库 `std::fs` 和 `std::io`
- **错误处理**：自定义 `ConvertError` 枚举，实现 `From` trait 自动转换

::: details 查看核心架构

```
dos2unix/src/
├── main.rs           # 入口：CLI 解析和流程编排
├── converter.rs      # 核心：流式转换逻辑
├── error.rs          # 错误类型定义
└── cli.rs            # CLI 参数定义
```

:::

## 练习

1. **基础**：阅读源码，理解流式处理的核心循环
2. **进阶**：实现 `--info` 选项，统计文件中的换行符类型
3. **挑战**：添加对大文件（>1GB）的优化支持，考虑内存映射

## 下一步

完成本模块后，继续前往 [模块二：gzip](/zh/academy/module-02-gzip/) —— 你将学习压缩算法和双语言对比。

---

**参考资料**：
- [dos2unix 官方项目](https://waterlan.home.xs4all.nl/dos2unix.html)
- [RFC 3629 - UTF-8 编码标准](https://www.rfc-editor.org/rfc/rfc3629)
- [The Rust Programming Language - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
