# 架构指南

> build-your-own-tools 的系统设计、模式和实现细节

[English](ARCHITECTURE.md) | **简体中文**

---

## 目录

- [概述](#概述)
- [设计理念](#设计理念)
- [项目结构](#项目结构)
- [架构图](#架构图)
- [子项目详情](#子项目详情)
- [构建系统](#构建系统)
- [CI/CD 流水线](#cicd-流水线)
- [跨平台策略](#跨平台策略)
- [扩展指南](#扩展指南)
- [故障排除](#故障排除)
- [参考](#参考)

---

## 概述

**build-your-own-tools** 是一个以学习为中心的仓库，使用 **Rust** 和 **Go** 从零开始重新实现常见的 CLI 工具。本项目展示了：

- 底层系统编程概念
- CLI 设计模式和最佳实践
n- 跨语言实现对比
- 跨平台开发技术

### 关键统计

| 指标 | 数值 |
|------|-------|
| 编程语言 | Rust, Go |
| 项目数量 | 3 (dos2unix, gzip, htop) |
| 支持平台 | Linux, macOS, Windows |
| 实现总数 | 6 个 |
| 测试覆盖 | 单元测试 + 集成测试 |

---

## 设计理念

### 1. 学习优先

- **可读性** 优先于微优化
- **全面的注释** 解释"为什么"
- **最小依赖** 以理解核心概念
- **渐进式复杂度** 从简单工具到高级工具

### 2. 多语言实现

```
概念/需求 → Rust实现 → 并排对比 → 学习洞察
          ↘ Go实现   ↗
```

### 3. 生产级质量

- 全面的测试覆盖
- CI/CD 自动化
- 跨平台兼容性
- 适当的错误处理

### 4. 模块化设计

每个工具都是自包含的，包含：
- 独立目录
- 独立的构建配置
- 独立的文档
- 独立的版本控制

---

## 项目结构

```
build-your-own-tools/
├── 📁 docs/                      # 文档
│   ├── setup/                   # 环境搭建指南
│   ├── architecture/            # 架构文档
│   ├── tutorials/               # 教程和对比
│   └── changelogs/              # 变更日志索引
│
├── 📁 dos2unix/                  # CRLF → LF 转换器
│   ├── src/main.rs             # Rust 实现
│   ├── Cargo.toml              # Rust 配置
│   ├── README.md               # 项目文档
│   └── changelog/              # 版本历史
│
├── 📁 gzip/                      # 文件压缩工具
│   ├── go/                     # Go 实现
│   │   ├── cmd/gzip-go/        # CLI 入口
│   │   └── changelog/          # 版本历史
│   └── rust/                   # Rust 实现
│       ├── src/lib.rs          # 库 crate
│       ├── src/main.rs         # CLI 入口
│       └── changelog/          # 版本历史
│
├── 📁 htop/                      # 系统监控 TUI
│   ├── shared/                 # 共享 Rust 库
│   ├── unix/rust/              # Unix 实现
│   ├── win/rust/               # Windows Rust 实现
│   └── win/go/                 # Windows Go 实现
│
├── 📁 .github/                   # GitHub 配置
│   ├── workflows/              # CI/CD 流水线
│   └── ISSUE_TEMPLATE/         # Issue 模板
│
├── Cargo.toml                  # Rust 工作区
├── go.work                     # Go 工作区
├── Makefile                    # 构建自动化
└── README.md                   # 项目概览
```

---

## 架构图

### 高层系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                         工作区 (Workspace)                    │
├─────────────────────────────────────────────────────────────┤
│  Cargo Workspace (Rust 项目)  │  Go Workspace (Go 项目)       │
└──────────────────┬──────────────────────────┬───────────────┘
                   │                          │
           ┌───────┴───────┐          ┌───────┴───────┐
           │     工具       │          │   构建系统     │
           ├───────────────┤          ├───────────────┤
           │  dos2unix     │          │    Make       │
           │  gzip         │          │ GitHub Actions│
           │  htop         │          └───────────────┘
           └───────────────┘
```

### 数据流 - dos2unix

```
┌─────────────┐    ┌───────────────┐
│  输入文件    │───▶│ CRLF 检测     │
└─────────────┘    └───────┬───────┘
                           │
            ┌──────────────┼──────────────┐
            │ 是                          │ 否
            ▼                             ▼
    ┌───────────────┐            ┌───────────────┐
    │ CRLF → LF 转换 │            │    直通       │
    └───────┬───────┘            └───────┬───────┘
            │                            │
            └──────────────┬─────────────┘
                           ▼
                    ┌───────────────┐
                    │   输出文件     │
                    └───────────────┘
```

### 数据流 - gzip

```
┌───────────────┐    ┌───────────┐    ┌─────────────┐
│  输入文件(们)  │───▶│  压缩?    │───▶│ DEFLATE 压缩 │───▶ .gz 输出
└───────────────┘    └───────────┘    └─────────────┘
                            │
                            ▼ 否
                    ┌───────────────┐    ┌─────────────┐
                    │ DEFLATE 解压  │───▶│  原始文件    │
                    └───────────────┘    └─────────────┘
```

---

## 子项目详情

### dos2unix

**目的**: 文本文件换行符转换器  
**复杂度**: ⭐ (初级)  
**核心概念**: 文件 I/O、流式处理、缓冲区管理

**关键特性**:

| 特性 | 实现方式 |
|------|---------|
| 流式处理 | 缓冲读取 (8KB 块) |
| 跨缓冲区 CRLF | 状态跟踪 (`prev_was_cr`) |
| 检查模式 | 仅检测，不修改 |
| stdin/stdout | 管道友好设计 |

**依赖**:
- `anyhow` - 错误处理

---

### gzip

**目的**: 文件压缩/解压  
**复杂度**: ⭐⭐ (中级)  
**核心概念**: DEFLATE 算法、流式处理、并发

**特性对比**:

| 特性 | Rust (rgzip) | Go (gzip-go) |
|------|-------------|--------------|
| 库 crate | ✅ | ❌ |
| 并行处理 | ❌ (需 rayon) | ✅ goroutines |
| 递归目录 | ❌ | ✅ -r 标志 |
| 压缩级别 | 0-9 | 0-9 |
| stdin/stdout | ✅ | ✅ |
| 保留源文件 | ✅ -k | ✅ 默认 |

**依赖**:
- Rust: `flate2`, `clap`
- Go: 仅标准库

---

### htop

**目的**: 交互式系统监控器  
**复杂度**: ⭐⭐⭐ (高级)  
**核心概念**: TUI、系统 API、实时更新、异步

**关键特性**:

| 特性 | Unix/Rust | Win/Rust | Win/Go |
|------|-----------|----------|--------|
| 进程列表 | ✅ | ✅ | ✅ |
| CPU 监控 | ✅ | ✅ | ✅ |
| 内存监控 | ✅ | ✅ | ✅ |
| 进程终止 | ✅ (SIGTERM→SIGKILL) | ✅ | ✅ |
| 搜索/过滤 | ✅ | ✅ | ✅ |
| 列排序 | ✅ | ✅ | ✅ |
| 火花线历史 | ❌ | ✅ | ❌ |
| 刷新间隔 | ✅ | ✅ | ✅ |

**依赖**:
- Rust: `ratatui`, `crossterm`, `sysinfo`
- Go: `tview`, `gopsutil`

---

## 构建系统

### Cargo 工作区 (Rust)

```toml
[workspace]
resolver = "2"
members = [
    "dos2unix",
    "gzip/rust",
    "htop/shared",
    "htop/unix/rust",
    "htop/win/rust",
]

[workspace.dependencies]
crossterm = "0.29"
ratatui = { version = "0.29", ... }
sysinfo = "0.29"
clap = { version = "4.5", ... }
flate2 = "1.0"
anyhow = "1.0"
```

**优点**:
- 共享依赖解析
- 统一构建命令
- 跨 crate 优化

---

## CI/CD 流水线

### 持续集成

```
推送/PR ──▶ CI 流水线
              │
              ├── Rust CI
              │   ├── cargo fmt --check
              │   ├── cargo clippy
              │   ├── cargo test
              │   └── cargo build --release
              │
              └── Go CI
                  ├── gofmt
                  ├── go vet
                  ├── go test
                  └── go build
```

**矩阵策略**:
| 操作系统 | Rust 目标 | Go 平台 |
|----|-------------|--------------|
| Ubuntu | x86_64-linux | linux/amd64 |
| macOS | x86_64-darwin, aarch64-darwin | darwin/amd64, darwin/arm64 |
| Windows | x86_64-windows | windows/amd64 |

### 发布流水线

```
Tag 推送 v* ──▶ 构建所有二进制文件
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
   Linux AMD64  macOS AMD64  Windows AMD64
   macOS ARM64
        │           │           │
        └───────────┴───────────┘
                    │
              GitHub Release
```

---

## 跨平台策略

### 条件编译

**Rust**:
```rust
#[cfg(target_os = "windows")]
fn windows_specific() { }

#[cfg(target_os = "unix")]
fn unix_specific() { }

#[cfg(not(target_os = "windows"))]
fn non_windows() { }
```

**Go**:
```go
//go:build windows
// +build windows

package main

func windowsSpecific() { }
```

### 平台抽象

| 特性 | Linux | macOS | Windows |
|---------|-------|-------|---------|
| 进程信息 | /proc | sysctl/libproc | WMI/NT API |
| 终端 | ANSI | ANSI | Windows API |
| 路径分隔符 | / | / | \\ |
| 换行符 | LF | LF | CRLF |

---

## 扩展指南

### 添加新工具

1. **创建目录结构**:
   ```bash
   mkdir mytool/
   cd mytool/
   cargo init  # 或 go mod init
   mkdir changelog
   touch README.md
   ```

2. **添加到工作区**:
   - Rust: 添加到根目录 `Cargo.toml` 工作区成员
   - Go: 添加到 `go.work`

3. **创建变更日志**:
   ```bash
   touch changelog/CHANGELOG.md
   ```

4. **更新文档**:
   - 添加到根目录 README.md 项目表
   - 添加到 docs/architecture/API.md 和 docs/architecture/API.zh-CN.md
   - 如需，添加 CI 工作流

5. **添加 Makefile 目标**:
   ```makefile
   build-mytool:
       cd mytool && cargo build --release
   ```

### 添加语言实现

1. 在现有工具中创建语言子目录
2. 实现相同的功能
3. 添加功能对等测试
4. 更新对比文档

---

## 故障排除

### 常见构建问题

| 问题 | 解决方案 |
|-------|----------|
| `cargo: not found` | 安装 Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `go: not found` | 安装 Go: https://golang.org/dl/ |
| 缺少依赖 | 运行 `cargo build` 自动获取 |
| 换行符问题 | 检查 `.gitattributes` 设置 |

### 平台特定问题

**Windows**:
- 使用 Git Bash 或 PowerShell
- 为符号链接启用开发者模式
- 安装 Visual Studio Build Tools

**macOS**:
- 安装 Xcode 命令行工具: `xcode-select --install`
- 可能需要在安全设置中允许终端

**Linux**:
- 安装 build-essential: `sudo apt install build-essential`

### 调试技巧

```bash
# 详细构建输出
cargo build --release -vv

# 使用回溯运行
RUST_BACKTRACE=1 cargo run

# 仅检查格式
cargo fmt -- --check

# 自动修复 lint 问题
cargo clippy --fix
```

---

## 参考

### 官方文档

- [Rust 权威指南](https://doc.rust-lang.org/book/) (中文: https://rustwiki.org/zh-CN/book/)
- [Go 文档](https://golang.org/doc/)
- [Cargo 手册](https://doc.rust-lang.org/cargo/)
- [Go 模块](https://golang.org/ref/mod)

### 库和框架

- [ratatui](https://github.com/ratatui-org/ratatui) - Rust TUI 框架
- [tview](https://github.com/rivo/tview) - Go TUI 框架
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Rust 系统信息
- [gopsutil](https://github.com/shirou/gopsutil) - Go 系统信息
- [flate2](https://github.com/rust-lang/flate2-rs) - Rust DEFLATE 压缩
- [clap](https://github.com/clap-rs/clap) - Rust CLI 解析器

### 标准

- [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)
- [语义化版本](https://semver.org/lang/zh-CN/)
- [约定式提交](https://www.conventionalcommits.org/zh-hans/)

---

**术语中英文对照表**

| 英文 | 中文 |
|------|------|
| Architecture | 架构 |
| Build System | 构建系统 |
| Conditional Compilation | 条件编译 |
| Concurrency | 并发 |
| Dependency | 依赖 |
| Error Handling | 错误处理 |
| Library | 库 |
| Memory Management | 内存管理 |
| Runtime | 运行时 |
| Workspace | 工作区 |

---

**最后更新**: 2026-04-16  
**版本**: 2.0
