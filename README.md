# Build-Your-Own-Tools

[![CI](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/Docs-GitHub%20Pages-blue?logo=github)](https://lessup.github.io/build-your-own-tools/)

简体中文 | [English](README.en.md)

一个用 **Rust / Go** 手写常用命令行工具的学习仓库，用于练习底层实现、命令行设计与跨语言对比。

| 子项目 | 语言 | 描述 |
|--------|------|------|
| `dos2unix/` | Rust | 轻量 dos2unix 工具：CRLF → LF 转换 |
| `gzip/` | Go, Rust | 极简 gzip 压缩/解压命令行 |
| `htop/` | Rust, Go | 跨平台 htop TUI 任务管理器（Unix + Windows） |

## ✨ 特性

```text
./
├── .github/workflows/   # 统一 CI（Rust lint/test + Go vet/test）
├── dos2unix/             # Rust 实现的 dos2unix
├── gzip/
│   ├── go/               # Go 版 gzip
│   └── rust/             # Rust 版 gzip（rgzip）
├── htop/
│   ├── unix/rust/        # Unix htop（Rust + ratatui）
│   ├── win/rust/         # Windows htop（Rust + ratatui）
│   └── win/go/           # Windows htop（Go + tview）
├── Cargo.toml            # Rust workspace
├── go.work               # Go workspace
├── Makefile              # 统一构建/检查/测试入口
└── LICENSE               # MIT OR Apache-2.0
```

## 🚀 快速开始

### 前置依赖

- **Rust** 工具链（建议 stable 最新版）
- **Go** 工具链（建议 1.23+）
- Linux / macOS / Windows（全平台支持，htop 部分子项目有平台限制）

## 快速开始

```bash
# 构建所有 Rust 子项目
make build-rust

# 构建所有 Go 子项目
make build-go

# 或构建全部
make build-all
```

单独构建某个子项目：

```bash
# dos2unix
cargo build --release -p dos2unix-rust

# gzip（Rust 版）
cargo build --release -p rgzip

# gzip（Go 版）
cd gzip/go && make build

# htop（Unix Rust 版）
cargo run -p htop-rust

# htop（Windows Rust 版）
cargo run -p htop-win-rust

# htop（Windows Go 版）
cd htop/win/go && go run ./cmd/htop-win-go
```

## 📖 使用示例

### dos2unix

```bash
# 转换单个文件
dos2unix-rust file.txt

# 检测文件是否包含 CRLF
dos2unix-rust --check file.txt

# 从标准输入读取
cat file.txt | dos2unix-rust > output.txt
```

### gzip

```bash
# 压缩文件 (Go 版)
gzip-go file.txt

# 解压文件 (Rust 版)
rgzip -d file.txt.gz
```

### htop

```bash
# 运行系统监控 (Unix)
htop-unix-rust

# 运行系统监控 (Windows)
htop-win-go
```

## 🛠️ 开发

### 代码检查

```bash
# Rust
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all

# Go
gofmt -w .
go vet ./...
go test ./...
```

### 运行所有测试

```bash
make test-all
```

## 📁 目录结构

```
build-your-own-tools/
├── dos2unix/           # Rust 实现的 dos2unix
├── gzip/
│   ├── go/             # Go 实现
│   └── rust/           # Rust 实现
├── htop/
│   ├── unix/rust/      # Unix Rust 实现
│   └── win/
│       ├── go/         # Windows Go 实现
│       └── rust/       # Windows Rust 实现
├── docs/               # 项目文档
├── .github/            # GitHub 配置
├── CHANGELOG.md        # 变更日志
├── CONTRIBUTING.md     # 贡献指南
├── CODE_OF_CONDUCT.md  # 行为准则
└── SECURITY.md         # 安全政策
```

## 🤝 贡献

欢迎贡献！请阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 了解如何参与项目开发。

## 📄 许可证

本项目采用 [MIT](LICENSE) 或 [Apache-2.0](LICENSE) 双许可证，你可以选择其中之一。

## 🙏 致谢

感谢所有贡献者和开源社区的支持！
