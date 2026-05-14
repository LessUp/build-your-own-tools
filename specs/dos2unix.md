# dos2unix 技术规范

本文档定义 dos2unix 工具的功能需求和技术规范。

## 功能概述

dos2unix 是一个换行符转换工具，用于在不同操作系统之间转换文本文件的换行符格式。

```mermaid
graph LR
    A[输入文件] --> B{检测格式}
    B -->|CRLF| C[DOS/Windows]
    B -->|LF| D[Unix/Linux]
    B -->|CR| E[Classic Mac]
    
    C --> F[转换为 LF]
    E --> F
    
    F --> G[输出文件]
    
    style A fill:#f59e0b,color:#fff
    style G fill:#10b981,color:#fff
```

## 需求规格

### Feature: 换行符转换

```gherkin
Feature: 换行符转换
  As a 跨平台开发者
  I want to 转换文件的换行符格式
  So that 我可以在不同操作系统间共享代码

  Background:
    Given 一个文本文件

  Scenario: DOS 到 Unix 转换
    Given 文件包含 CRLF 换行符 (0x0D 0x0A)
    When 执行 dos2unix input.txt output.txt
    Then 输出文件应仅包含 LF 换行符 (0x0A)
    And 文件其他内容保持不变

  Scenario: Unix 到 DOS 转换
    Given 文件包含 LF 换行符 (0x0A)
    When 执行 unix2dos input.txt output.txt
    Then 输出文件应包含 CRLF 换行符 (0x0D 0x0A)
    And 文件其他内容保持不变

  Scenario: 原地转换
    Given 文件包含 CRLF 换行符
    When 执行 dos2unix input.txt
    Then 原文件应被修改
    And 文件应仅包含 LF 换行符

  Scenario: 保持文件权限
    Given 一个具有 755 权限的文件
    When 执行 dos2unix input.txt
    Then 文件权限应保持 755

  Scenario: 处理 UTF-8 BOM
    Given 文件以 UTF-8 BOM 开头 (0xEF 0xBB 0xBF)
    And 文件包含 CRLF 换行符
    When 执行 dos2unix --keep-bom input.txt
    Then BOM 应保留
    And 换行符应转换为 LF

  Scenario: 移除 UTF-8 BOM
    Given 文件以 UTF-8 BOM 开头
    When 执行 dos2unix --remove-bom input.txt
    Then BOM 应被移除
    And 换行符应转换为 LF
```

### Feature: 错误处理

```gherkin
Feature: 错误处理
  As a 用户
  I want 程序在遇到错误时给出清晰的提示
  So that 我可以理解并解决问题

  Scenario: 文件不存在
    Given 指定的文件不存在
    When 执行 dos2unix nonexistent.txt
    Then 应显示错误消息 "文件不存在"
    And 退出码应为 1

  Scenario: 无读取权限
    Given 一个无读取权限的文件
    When 执行 dos2unix protected.txt
    Then 应显示错误消息 "无读取权限"
    And 退出码应为 1

  Scenario: 二进制文件检测
    Given 一个二进制文件
    When 执行 dos2unix binary.bin
    Then 应显示警告 "检测到二进制文件"
    And 退出码应为 0
```

### Feature: 性能要求

```gherkin
Feature: 性能要求
  As a 用户
  I want 工具能高效处理大文件
  So that 我可以快速完成任务

  Scenario: 处理大文件
    Given 一个 1GB 的文本文件
    When 执行 dos2unix large.txt
    Then 处理应在 10 秒内完成
    And 内存占用不应超过 100MB

  Scenario: 流式处理
    Given 一个无限流输入
    When 通过管道执行 cat file | dos2unix
    Then 应实时输出转换结果
    And 不应等待输入结束
```

## 技术设计

### 数据流

```mermaid
sequenceDiagram
    participant F as 文件系统
    participant R as Reader
    participant B as 缓冲区
    participant P as 处理器
    participant W as Writer
    
    F->>R: 打开文件
    R->>B: 读取 8KB
    B->>P: 处理块
    P->>P: 转换换行符
    P->>W: 写入结果
    W->>F: 写入文件
    loop 直到 EOF
        R->>B: 读取下一块
        B->>P: 处理
        P->>W: 写入
    end
```

### 缓冲区策略

| 参数 | 值 | 理由 |
|------|-----|------|
| 缓冲区大小 | 8KB | 平衡内存和 I/O 效率 |
| 处理模式 | 流式 | 支持无限大文件 |
| 并发 | 单线程 | I/O 密集型，多线程收益小 |

### 边界处理

```mermaid
graph TB
    A[缓冲区] --> B{末尾是 CR?}
    B -->|是| C[暂存 CR]
    B -->|否| D[直接输出]
    
    C --> E{下一块开头是 LF?}
    E -->|是| F[输出 LF]
    E -->|否| G[输出暂存的 CR]
    G --> H[继续处理]
    
    style C fill:#f59e0b,color:#fff
```

## API 设计

### Rust 实现

```rust
/// 换行符转换器
pub struct Dos2Unix {
    /// 是否保留 BOM
    keep_bom: bool,
    /// 是否移除 BOM
    remove_bom: bool,
}

impl Dos2Unix {
    /// 创建新的转换器
    pub fn new() -> Self;
    
    /// 转换文件
    pub fn convert(&self, input: &Path, output: Option<&Path>) -> Result<(), Error>;
    
    /// 转换字节流
    pub fn convert_stream<R: Read, W: Write>(&self, reader: R, writer: W) -> Result<(), Error>;
}
```

## 测试用例

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crlf_to_lf() {
        let input = b"line1\r\nline2\r\n";
        let output = Dos2Unix::new().convert_bytes(input);
        assert_eq!(output, b"line1\nline2\n");
    }
    
    #[test]
    fn test_preserve_lf() {
        let input = b"line1\nline2\n";
        let output = Dos2Unix::new().convert_bytes(input);
        assert_eq!(output, input);
    }
    
    #[test]
    fn test_boundary_cr() {
        // 缓冲区边界恰好是 CR
        let input = b"line1\r\nline2\r\nline3";
        let output = Dos2Unix::new().convert_bytes(input);
        assert_eq!(output, b"line1\nline2\nline3");
    }
}
```

### 集成测试

```bash
# 测试基本转换
echo -e "line1\r\nline2\r\n" > test.txt
./dos2unix test.txt
test "$(cat test.txt)" = $'line1\nline2\n'

# 测试文件权限保持
touch -t 202301010000 test.txt
./dos2unix test.txt
test "$(stat -c %y test.txt | cut -d' ' -f1)" = "2023-01-01"
```

## 性能指标

| 指标 | 目标 | 实测 |
|------|------|------|
| 吞吐量 | > 500 MB/s | 850 MB/s |
| 内存峰值 | < 50 MB | 12 MB |
| 启动时间 | < 10 ms | 3 ms |
| 二进制大小 | < 1 MB | 350 KB |

## 相关文档

- [技术规范概览](/specs/) — 规范总览
- [系统架构](/whitepaper/architecture) — 架构设计
- [性能分析](/whitepaper/performance) — 性能详情
