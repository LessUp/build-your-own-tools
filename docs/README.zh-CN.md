# 文档

> build-your-own-tools 的完整文档

[English](README.md) | **简体中文**

---

## 欢迎

欢迎阅读 build-your-own-tools 文档。本文档提供了全面的指南、API参考和教程，帮助你理解、构建和贡献本项目。

## 文档结构

遵循 [GitHub 社区最佳实践](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file)：

```
docs/
├── README.md                 # 本文档 - 文档索引
├── README.zh-CN.md           # 本文档（中文）
├── setup/                    # 环境搭建指南
│   ├── GETTING-STARTED.md    # 快速开始指南（英文）
│   └── GETTING-STARTED.zh-CN.md  # 快速开始（中文）
├── tutorials/                # 用户教程和对比
│   ├── COMPARISON.md         # Rust vs Go 对比（英文）
│   └── COMPARISON.zh-CN.md   # Rust vs Go 对比（中文）
├── architecture/             # 高层架构规范
│   ├── ARCHITECTURE.md       # 系统架构（英文）
│   ├── ARCHITECTURE.zh-CN.md # 系统架构（中文）
│   ├── API.md                # API 参考（英文）
│   └── API.zh-CN.md          # API 参考（中文）
├── assets/                   # 图片、UML 图、静态资源
└── changelogs/               # 变更日志资源
    ├── INDEX.md              # 所有变更日志索引
    ├── INDEX.zh-CN.md        # 变更日志索引（中文）
    └── MIGRATION.md          # 版本迁移指南
```

## 快速开始

初次接触 build-your-own-tools？从这里开始：

1. **[快速开始](setup/GETTING-STARTED.zh-CN.md)** - 设置环境并构建你的第一个工具
2. **[架构指南](architecture/ARCHITECTURE.zh-CN.md)** - 理解系统设计和项目结构
3. **[语言对比](tutorials/COMPARISON.zh-CN.md)** - 了解 Rust 和 Go 实现的区别

## API 参考

查找函数文档？查看 API 参考：

- **[API 参考](architecture/API.zh-CN.md)** - 所有库模块的完整 API 文档

## 变更日志

跟踪项目变化和迁移：

- **[变更日志索引](changelogs/INDEX.zh-CN.md)** - 所有变更日志的索引
- **[迁移指南](changelogs/MIGRATION.md)** - 版本迁移说明

## 语言选择 / Language Selection

选择你的首选语言：

- 🇺🇸 **English** - [Architecture](architecture/ARCHITECTURE.md) | [Comparison](tutorials/COMPARISON.md) | [API](architecture/API.md) | [Getting Started](setup/GETTING-STARTED.md)
- 🇨🇳 **简体中文** - [架构指南](architecture/ARCHITECTURE.zh-CN.md) | [语言对比](tutorials/COMPARISON.zh-CN.md) | [API参考](architecture/API.zh-CN.md) | [快速开始](setup/GETTING-STARTED.zh-CN.md)

## 快速链接

### 项目链接

- [GitHub 仓库](https://github.com/LessUp/build-your-own-tools)
- [在线文档](https://lessup.github.io/build-your-own-tools/)
- [CHANGELOG.md](../CHANGELOG.md) - 根目录变更日志
- [CONTRIBUTING.md](../CONTRIBUTING.md) - 贡献指南
- [LICENSE](../LICENSE) - MIT 或 Apache-2.0 许可证

### 子项目

| 项目 | 路径 | 说明 |
|---------|------|-------------|
| dos2unix | `dos2unix/` | CRLF 转 LF 转换器 |
| gzip | `gzip/go/`, `gzip/rust/` | 文件压缩工具 |
| htop | `htop/` | 系统监控 TUI |

### 外部资源

- [Rust 权威指南](https://rustwiki.org/zh-CN/book/)
- [Go 文档](https://golang.org/doc/)
- [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)
- [语义化版本](https://semver.org/lang/zh-CN/)

---

## 贡献文档

我们欢迎文档改进！请参阅 [CONTRIBUTING.md](../CONTRIBUTING.md) 了解：

- 报告文档问题
- 建议改进
- 提交文档 PR
- 翻译贡献

---

**术语中英文对照表**

| 英文 | 中文 |
|------|------|
| Architecture | 架构 |
| Comparison | 对比/比较 |
| Documentation | 文档 |
| Guide | 指南 |
| Index | 索引 |
| Migration | 迁移 |
| Quick Start | 快速开始 |
| Reference | 参考 |

---

**最后更新**: 2026-04-17  
**文档版本**: 2.1
