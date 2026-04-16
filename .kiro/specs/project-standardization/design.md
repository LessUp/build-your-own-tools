# Design Document: Project Standardization

## Overview

本文档描述 Build-Your-Own-Tools 项目的架构设计和规范化方案。项目是一个用 Rust/Go 手写常用命令行工具的学习仓库。

## Architecture

### 项目结构

```
Build-Your-Own-Tools/
├── .github/
│   ├── ISSUE_TEMPLATE/       # Issue 模板
│   ├── workflows/            # CI/CD 工作流
│   │   ├── ci.yml            # 持续集成
│   │   ├── release.yml       # 自动发布
│   │   └── pages.yml         # 文档部署
│   └── pull_request_template.md
├── docs/                     # 项目文档
│   ├── ARCHITECTURE.md       # 架构说明
│   └── COMPARISON.md         # Rust vs Go 对比
├── dos2unix/                 # CRLF → LF 转换工具
├── gzip/                     # 压缩/解压工具
│   ├── go/                   # Go 实现
│   └── rust/                 # Rust 实现
├── htop/                     # 系统监控工具
│   ├── shared/               # 共享库
│   ├── unix/rust/            # Unix Rust 实现
│   └── win/
│       ├── go/               # Windows Go 实现
│       └── rust/             # Windows Rust 实现
├── .editorconfig             # 编辑器配置
├── .gitattributes            # Git 属性
├── .gitignore                # Git 忽略
├── CHANGELOG.md              # 变更日志
├── CODE_OF_CONDUCT.md        # 行为准则
├── CONTRIBUTING.md           # 贡献指南
├── Cargo.toml                # Rust workspace
├── LICENSE                   # 许可证
├── Makefile                  # 构建脚本
├── README.md                 # 项目主文档
├── SECURITY.md               # 安全政策
└── go.work                   # Go workspace
```

## Components

### 1. 文档体系

| 文件 | 用途 |
|------|------|
| README.md | 项目概述、快速开始、特性列表 |
| CHANGELOG.md | 版本变更记录 (Keep a Changelog 格式) |
| CONTRIBUTING.md | 贡献流程、代码规范 |
| CODE_OF_CONDUCT.md | 行为准则 (Contributor Covenant) |
| SECURITY.md | 安全政策 |

### 2. CI/CD 流水线

| 工作流 | 触发条件 | 功能 |
|--------|----------|------|
| ci.yml | push/PR | 格式检查、Lint、测试、跨平台构建 |
| release.yml | tag push | 构建发布二进制、创建 GitHub Release |
| pages.yml | main/master push | 部署文档站到 GitHub Pages |

### 3. 代码质量

| 工具 | 配置文件 | 用途 |
|------|----------|------|
| rustfmt | rustfmt.toml | Rust 代码格式化 |
| clippy | - | Rust Lint |
| gofmt | - | Go 代码格式化 |
| go vet | - | Go 静态分析 |
| EditorConfig | .editorconfig | 编辑器统一配置 |

### 4. 子项目结构

每个子项目包含：
- `README.md` - 使用说明和示例
- `changelog/CHANGELOG.md` - 版本变更记录
- 语言特定配置 (Cargo.toml / go.mod)
- 测试文件

## Versioning

遵循 [Semantic Versioning](https://semver.org/):
- MAJOR: 不兼容的 API 变更
- MINOR: 向后兼容的功能新增
- PATCH: 向后兼容的问题修复

## Changelog Format

遵循 [Keep a Changelog](https://keepachangelog.com/):

```markdown
## [Unreleased]
### Added / Changed / Fixed / Security

## [0.1.0] - YYYY-MM-DD
### Added
- Initial release
```

## Cross-Platform Support

| 平台 | Rust | Go |
|------|------|-----|
| Linux (x86_64) | ✅ | ✅ |
| macOS (x86_64, ARM64) | ✅ | ✅ |
| Windows (x86_64) | ✅ | ✅ |
