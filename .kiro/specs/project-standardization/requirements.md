# Requirements Document

## Introduction

本文档定义 Build-Your-Own-Tools 项目的需求规范。项目是一个用 Rust/Go 手写常用命令行工具的学习仓库。

## Requirements

### REQ-1: 项目文档体系

| ID | 需求 | 状态 |
|----|------|------|
| REQ-1.1 | 根目录 README.md 包含徽章、特性列表、快速开始 | ✅ |
| REQ-1.2 | CONTRIBUTING.md 贡献指南 | ✅ |
| REQ-1.3 | CODE_OF_CONDUCT.md 行为准则 | ✅ |
| REQ-1.4 | SECURITY.md 安全政策 | ✅ |
| REQ-1.5 | CHANGELOG.md 变更记录 (Keep a Changelog) | ✅ |
| REQ-1.6 | 每个子项目有标准化 README.md | 🔄 |

### REQ-2: CI/CD 流水线

| ID | 需求 | 状态 |
|----|------|------|
| REQ-2.1 | CI 工作流包含格式检查 (cargo fmt, gofmt) | ✅ |
| REQ-2.2 | CI 工作流包含 Lint 检查 (clippy, go vet) | ✅ |
| REQ-2.3 | CI 工作流包含单元测试 | ✅ |
| REQ-2.4 | CI 工作流支持跨平台构建 | ✅ |
| REQ-2.5 | Release 工作流自动构建发布二进制 | ✅ |
| REQ-2.6 | Pages 工作流部署文档站 | ✅ |

### REQ-3: 代码质量

| ID | 需求 | 状态 |
|----|------|------|
| REQ-3.1 | .editorconfig 编辑器统一配置 | ✅ |
| REQ-3.2 | rustfmt.toml Rust 格式化配置 | ✅ |
| REQ-3.3 | .golangci.yml Go Lint 配置 | ✅ |
| REQ-3.4 | 所有代码通过 `cargo clippy -- -D warnings` | ✅ |
| REQ-3.5 | 所有代码通过 `cargo fmt --check` | ✅ |

### REQ-4: 子项目规范

| ID | 需求 | 状态 |
|----|------|------|
| REQ-4.1 | 每个子项目有 README.md | 🔄 |
| REQ-4.2 | 每个子项目有 changelog/CHANGELOG.md | ✅ |
| REQ-4.3 | 每个子项目有测试文件 | ✅ |
| REQ-4.4 | 子项目遵循语言命名规范 | ✅ |

### REQ-5: 文档站

| ID | 需求 | 状态 |
|----|------|------|
| REQ-5.1 | VitePress 文档站配置 | ✅ |
| REQ-5.2 | docs/ 目录架构文档 | ✅ |
| REQ-5.3 | Rust vs Go 对比文档 | ✅ |

## Status Legend

- ✅ 完成
- 🔄 进行中
- ❌ 未开始
