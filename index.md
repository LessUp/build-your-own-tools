---
layout: home

hero:
  name: Build Your Own Tools
  text: 用 Rust / Go 手写命令行工具
  tagline: 底层实现 · 命令行设计 · 跨语言对比 · Linux / macOS / Windows
  actions:
    - theme: brand
      text: 架构说明
      link: /docs/ARCHITECTURE
    - theme: alt
      text: Rust vs Go 对比
      link: /docs/COMPARISON
    - theme: alt
      text: GitHub
      link: https://github.com/LessUp/build-your-own-tools

features:
  - icon: 🔧
    title: dos2unix
    details: Rust 实现的 CRLF → LF 转换工具，演示文件 I/O、流处理与检测模式
    link: /dos2unix/
  - icon: 📦
    title: gzip
    details: Go + Rust 双语言实现的 DEFLATE 压缩/解压，对比流处理与错误处理差异
    link: /gzip/
  - icon: 📊
    title: htop
    details: 跨平台 TUI 系统监控（Unix + Windows），Rust ratatui / Go tview，实时 CPU/内存/进程
    link: /htop/
  - icon: 🎯
    title: 学习驱动
    details: 代码清晰易读，详细注释，每个工具对标生产级 CLI 标准（clap / cobra、彩色输出、进度条）
  - icon: ⚡
    title: 跨平台 CI
    details: GitHub Actions 多平台构建矩阵（Linux / macOS / Windows），Rust fmt+clippy+test、Go vet+test
  - icon: 🔀
    title: Rust vs Go
    details: 相同功能的双语言实现，对比所有权 vs GC、Result vs error、RAII vs defer、编译与运行时性能
    link: /docs/COMPARISON
---

## 技术栈

| | Rust | Go |
|------|------|-----|
| **CLI 框架** | clap | cobra / 标准库 |
| **TUI** | ratatui + crossterm | tview |
| **压缩** | flate2 (DEFLATE) | compress/gzip (标准库) |
| **系统信息** | sysinfo | gopsutil |
| **测试** | cargo test + 属性测试 | go test |

## 快速开始

```bash
# Rust — 构建全部子项目
make build-rust

# Go — 构建全部子项目
make build-go

# 或分别构建
cargo build --release -p dos2unix-rust
cd gzip/go && go build -o bin/gzip-go ./cmd/gzip-go
```

## 每个子项目教你什么

| 子项目 | 核心学习点 |
|--------|-----------|
| **dos2unix** | 文件 I/O、流式读写、换行符编码、标准输入输出 |
| **gzip** | DEFLATE 压缩算法、流处理大文件、文件元数据保持 |
| **htop** | 系统 API（CPU/内存/进程）、终端 UI、实时刷新、跨平台差异 |
