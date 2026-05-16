# 架构说明

> 这份文档用来快速理解仓库结构、工具分布，以及围绕代码的工程化表面。

[English](ARCHITECTURE.md) | **简体中文**

## 这个仓库是什么

Build Your Own Tools 是一个用 Rust 和 Go 重做少量 CLI 工具的学习型 monorepo。除了工具实现本身，这个仓库也刻意保留了完整工程表面，方便你同时学习：

- 工具本身如何实现
- Rust 与 Go 在同一问题上的取舍
- 规格、CI、发布、文档站点等工程化部分如何协同

## 核心设计原则

1. **通过实现来教学**：每个工具都足够真实，能体现有价值的设计权衡。
2. **在同一问题上比较语言**：gzip 和 htop 可以直接看出 Rust / Go 的风格差异。
3. **让工程化可见**：构建脚本、OpenSpec、CI 和文档站点都是仓库学习内容的一部分。
4. **保持小而完整的工具集**：与其铺很多浅层示例，不如把少量工具做深做清楚。

## 仓库地图

```text
build-your-own-tools/
├── openspec/                  仓库级规格与活跃变更
├── docs/                      支撑文档
├── .vitepress/                文档站点配置
├── .github/workflows/         CI、Pages、发布与维护型 workflow
├── .githooks/                 版本管理钩子
├── dos2unix/                  Rust 换行符转换工具
├── gzip/go/                   Go 版 gzip
├── gzip/rust/                 Rust 版 gzip
├── htop/shared/               htop 的共享 Rust 逻辑
├── htop/unix/rust/            Unix Rust 版 htop
├── htop/win/rust/             Windows Rust 版 htop
├── htop/win/go/               Windows Go 版 htop
├── README.md                  仓库入口
└── index.md                   Pages 首页
```

## 工具地图

| 工具 | 语言 | 主要学习点 |
| --- | --- | --- |
| `dos2unix` | Rust | 流式 I/O、换行符转换、缓冲区边界 |
| `gzip` | Rust + Go | 压缩流程、CLI 行为、错误处理 |
| `htop` | Rust + Go | 终端 UI、进程指标、跨平台系统 API |

## 构建与验证表面

仓库保留了少量共享命令入口：

```bash
make build-all
make lint-all
make test-all
npm run docs:check
npm run docs:build
```

- `make` 统一覆盖 Rust 与 Go 实现。
- npm 脚本负责文档站点构建和 VitePress 的类型/构建检查。

## OpenSpec 与工作流模型

仓库级改动通过 OpenSpec 跟踪：

- `openspec/specs/` 保存稳定要求
- `openspec/changes/` 保存活跃或历史变更
- `.opencode/commands/` 提供对应的 `/opsx:*` 命令

对一个非平凡改动，推荐流程是：

1. 先提出变更
2. 更新规格与设计
3. 按任务实现
4. 用现有命令做验证
5. 完成后归档

## 对外表面

仓库刻意把几个公共入口拆开，各自承担不同职责：

| 表面 | 作用 |
| --- | --- |
| `README.md` | 仓库导览与快速上手 |
| `index.md` | 面向新访客的首页 |
| `docs/` | 细节说明与深入文档 |
| `CHANGELOG.md` | 项目版本历史 |

这样可以避免站点只是 README 的简单镜像。
