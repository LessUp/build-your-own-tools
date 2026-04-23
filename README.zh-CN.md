# Build Your Own Tools

[![CI](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml)
[![Docs](https://github.com/LessUp/build-your-own-tools/actions/workflows/pages.yml/badge.svg)](https://lessup.github.io/build-your-own-tools/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/Go-1.21+-00ADD8.svg)](https://golang.org/)

[English](README.md) | **简体中文**

这是一个用 **Rust** 和 **Go** 从零重做真实 CLI 工具的学习仓库，适合系统化理解文件 I/O、压缩流程、终端 UI、跨平台实现，以及两种系统语言在设计上的取舍。

[在线站点](https://lessup.github.io/build-your-own-tools/) · [快速开始](docs/setup/GETTING-STARTED.zh-CN.md) · [架构说明](docs/architecture/ARCHITECTURE.zh-CN.md) · [Rust vs Go 对比](docs/tutorials/COMPARISON.zh-CN.md)

## 项目清单

| 工具 | 语言 | 你会学到什么 | 状态 |
| --- | --- | --- | --- |
| [`dos2unix`](./dos2unix/) | Rust | 流式文件 I/O、缓冲区边界、换行符处理 | 稳定 |
| [`gzip`](./gzip/) | Rust + Go | 压缩管线、CLI 设计、惯用错误处理 | 稳定 |
| [`htop`](./htop/) | Rust + Go | TUI 架构、进程指标、跨平台系统 API | 稳定 |

## 这个仓库的价值

- **同一问题，两种实现**：可以直接对比 Rust 和 Go 的工程风格。
- **难度循序渐进**：从小型流处理工具一路走到跨平台终端 UI。
- **Spec 驱动开发**：仓库级改动通过 OpenSpec 管理，避免需求和实现漂移。
- **工程化也可学习**：统一构建、CI、发布和文档站点都是学习内容的一部分。

## 快速开始

```bash
git clone https://github.com/LessUp/build-your-own-tools.git
cd build-your-own-tools

make build-all
make test-all

# 示例：从 stdin 转换 CRLF
printf 'hello\r\nworld\r\n' | ./target/release/dos2unix-rust
```

## 开发流程

仓库级改动遵循 OpenSpec：

```bash
openspec list
/opsx:propose "描述这次改动"
/opsx:apply
/opsx:archive
```

实现和验证常用命令：

```bash
make lint-all
make test-all
npm run docs:check
npm run docs:build
```

共享的 AI 协作规范见 [AGENTS.md](AGENTS.md)、[CLAUDE.md](CLAUDE.md) 和 [.github/copilot-instructions.md](.github/copilot-instructions.md)。

## 文档导览

| 文档 | 作用 |
| --- | --- |
| [快速开始](docs/setup/GETTING-STARTED.zh-CN.md) | 配置环境并跑通项目 |
| [架构说明](docs/architecture/ARCHITECTURE.zh-CN.md) | 了解仓库结构与系统设计 |
| [语言对比](docs/tutorials/COMPARISON.zh-CN.md) | 对比 Rust 与 Go 的设计选择 |
| [项目规范](openspec/specs/project/spec.md) | 项目级标准与流程要求 |
| [CHANGELOG.md](CHANGELOG.md) | 项目版本历史 |

## 许可证

本项目采用以下任一许可证：

- Apache License, Version 2.0（[LICENSE](LICENSE) 或 <http://www.apache.org/licenses/LICENSE-2.0>）
- MIT License（[LICENSE](LICENSE) 或 <http://opensource.org/licenses/MIT>）
