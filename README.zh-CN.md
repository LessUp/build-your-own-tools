# Build-Your-Own-Tools

[![CI](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/LessUp/build-your-own-tools/actions/workflows/ci.yml)
[![Docs](https://github.com/LessUp/build-your-own-tools/actions/workflows/pages.yml/badge.svg)](https://lessup.github.io/build-your-own-tools/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/Go-1.21+-00ADD8.svg)](https://golang.org/)

[English](README.md) | **简体中文**

一个用 **Rust** 和 **Go** 手写常用命令行工具的学习仓库，用于练习底层系统编程、命令行设计模式以及跨语言实现对比。

[📖 在线文档](https://lessup.github.io/build-your-own-tools/) | [🚀 快速开始](docs/zh-CN/GETTING-STARTED.md) | [📋 架构说明](docs/zh-CN/ARCHITECTURE.md) | [🔍 语言对比](docs/zh-CN/COMPARISON.md)

---

## 🚀 快速开始

```bash
# 克隆仓库
git clone https://github.com/LessUp/build-your-own-tools.git
cd build-your-own-tools

# 构建所有 Rust 项目
make build-rust

# 构建所有 Go 项目
make build-go

# 运行所有测试
make test-all

# 运行工具
echo "Hello World" | ./target/release/dos2unix-rust
```

---

## 📦 项目

| 项目 | 语言 | 描述 | 状态 | 版本 |
|---------|----------|-------------|--------|---------|
| [dos2unix](./dos2unix/) | Rust | CRLF → LF 换行符转换器 | ✅ 稳定 | v0.2.1 |
| [gzip](./gzip/) | Rust, Go | Gzip 压缩/解压 CLI | ✅ 稳定 | v0.3.0 |
| [htop](./htop/) | Rust, Go | 跨平台 TUI 系统监控器 | ✅ 稳定 | v0.1.5 |

### 项目概览

```
⭐ 初级 ──▶ dos2unix ──▶ 学习文件 I/O 和流处理
              │
⭐⭐ 中级 ──▶ gzip ────▶ 学习压缩算法和并发
              │
⭐⭐⭐ 高级 ──▶ htop ────▶ 学习 TUI 和系统 API
```

---

## 🎯 特性

- **多语言实现** — 相同工具用 Rust 和 Go 实现，可并排对比
- **跨平台支持** — 支持 Linux、macOS、Windows
- **生产级质量** — 单元测试、CI/CD、自动化发布
- **完善的文档** — 架构文档、API 参考、对比指南
- **渐进式学习** — 从简单到高级的难度递进

---

## 🏗️ 项目结构

```
build-your-own-tools/
├── 📁 docs/                      # 文档
│   ├── en/                      # 英文文档
│   ├── zh-CN/                   # 中文文档
│   └── changelogs/              # 变更日志索引
│
├── 📁 dos2unix/                  # CRLF 转 LF 转换器
│   ├── src/main.rs
│   └── changelog/CHANGELOG.md
│
├── 📁 gzip/
│   ├── go/                      # Go 实现
│   └── rust/                    # Rust 实现
│
├── 📁 htop/
│   ├── shared/                  # 共享库
│   ├── unix/rust/               # Unix 实现
│   └── win/                     # Windows 实现
│
├── Cargo.toml                   # Rust 工作区
├── go.work                      # Go 工作区
└── Makefile                     # 构建自动化
```

---

## 🛠️ 开发

### 前置条件

| 工具 | 版本 | 安装 |
|------|---------|---------|
| Rust | 1.70+ | [rustup.rs](https://rustup.rs/) |
| Go | 1.21+ | [golang.org](https://golang.org/dl/) |
| make | 任意 | 通常已预装 |

### 构建命令

```bash
# 构建所有项目
make build-all

# 构建特定项目
make build-dos2unix
make build-gzip-rust
make build-gzip-go
make build-htop-unix-rust
make build-htop-win-rust

# 运行测试
make test-all
make test-rust
make test-go

# 代码检查
make lint-all

# 代码格式化
make fmt-all
```

### 开发工作流

```bash
# 1. 格式化代码
make fmt-all

# 2. 运行代码检查
make lint-all

# 3. 运行测试
make test-all

# 4. 构建发布版本
make build-all
```

---

## 📖 文档

我们提供完整的中英文双语文档：

| 文档 | 中文 | 英文 | 描述 |
|------|------|------|----------|
| 快速开始 | [zh-CN](docs/zh-CN/GETTING-STARTED.md) | [en](docs/en/GETTING-STARTED.md) | 环境搭建和首次构建 |
| 架构指南 | [zh-CN](docs/zh-CN/ARCHITECTURE.md) | [en](docs/en/ARCHITECTURE.md) | 系统设计和模式 |
| 语言对比 | [zh-CN](docs/zh-CN/COMPARISON.md) | [en](docs/en/COMPARISON.md) | 语言权衡和基准测试 |
| API 参考 | [zh-CN](docs/zh-CN/API.md) | [en](docs/en/API.md) | 库函数文档 |
| 变更日志 | [zh-CN](docs/changelogs/INDEX.zh-CN.md) | [en](docs/changelogs/INDEX.md) | 版本历史和变更 |
| 迁移指南 | [双语](docs/changelogs/MIGRATION.md) | | 版本升级说明 |

---

## 🧪 测试

```bash
# 运行所有测试并显示详细输出
cargo test --all -- --nocapture
go test -v ./...

# 运行特定测试
cargo test -p dos2unix-rust test_stream_large_data
go test -C gzip/go -run TestGzipStream

# 带覆盖率测试
cargo tarpaulin --all
go test -cover ./...
```

---

## 📊 学习目标

每个子项目都教授特定的概念：

| 主题 | dos2unix | gzip | htop |
|-------|----------|------|------|
| 文件 I/O | ✅ 流式处理 | ✅ 流式处理 | - |
| CLI 设计 | ✅ 手动参数 | ✅ clap/pflag | ✅ 交互式 |
| 错误处理 | ✅ anyhow | ✅ anyhow | ✅ anyhow |
| 压缩算法 | - | ✅ DEFLATE | - |
| 系统 API | - | - | ✅ 进程信息 |
| TUI | - | - | ✅ ratatui/tview |
| 并发 | - | ✅ goroutines | ✅ 异步刷新 |

**推荐学习路径**:
1. 从 **dos2unix** 开始（⭐ 初级）— 学习基本概念
2. 进阶到 **gzip**（⭐⭐ 中级）— 学习算法和并发
3. 完成 **htop**（⭐⭐⭐ 高级）— 学习系统编程和 TUI

---

## 🤝 贡献

我们欢迎各种形式的贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解贡献指南。

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交变更 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

---

## 📋 路线图

### 2026 Q2

- [ ] 新工具: `cat` 实现
- [ ] 新工具: `wc` (字数统计)
- [ ] 增强教程文档

### 2026 Q3

- [ ] htop 网络监控
- [ ] 磁盘 I/O 监控
- [ ] 插件系统探索

### 未来

- [ ] 容器感知的进程分组
- [ ] 远程系统监控
- [ ] 其他语言实现 (Zig?)

---

## 📄 许可证

本项目的许可证为以下之一：

- Apache License, Version 2.0 ([LICENSE](LICENSE) 或 http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE](LICENSE) 或 http://opensource.org/licenses/MIT)

您可以选择其中任意一种许可证。

---

## 🙏 致谢

- [ratatui](https://github.com/ratatui-org/ratatui) — Rust TUI 框架
- [tview](https://github.com/rivo/tview) — Go TUI 框架
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) — Rust 系统信息
- [gopsutil](https://github.com/shirou/gopsutil) — Go 系统信息
- [flate2](https://github.com/rust-lang/flate2-rs) — Rust DEFLATE 压缩
- [clap](https://github.com/clap-rs/clap) — Rust CLI 解析器

---

## 相关链接

- 📖 [在线文档](https://lessup.github.io/build-your-own-tools/)
- 🐛 [GitHub Issues](https://github.com/LessUp/build-your-own-tools/issues)
- 💬 [GitHub Discussions](https://github.com/LessUp/build-your-own-tools/discussions)
- 📝 [变更日志](CHANGELOG.md)

---

**术语中英文对照表**

| 英文 | 中文 |
|------|------|
| Build | 构建 |
| CLI | 命令行界面 |
| Compression | 压缩 |
| Concurrency | 并发 |
| Documentation | 文档 |
| Error Handling | 错误处理 |
| Stream | 流 |
| TUI | 终端用户界面 |
| Workspace | 工作区 |

---

**最后更新**: 2026-04-16  
**版本**: v0.2.0+
