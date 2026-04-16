# AGENTS.md - AI Agent Workflow Configuration

## Project Philosophy: Spec-Driven Development (SDD)

本项目严格遵循**规范驱动开发（Spec-Driven Development）**范式。所有的代码实现必须以 `/specs` 目录下的规范文档为唯一事实来源（Single Source of Truth）。

---

## Directory Context (目录说明)

```
build-your-own-tools/
├── specs/                      # 核心：规范文档存放地
│   ├── product/                # 产品功能定义与验收标准 (PRD)
│   ├── rfc/                    # 技术设计文档 (RFCs)
│   ├── api/                    # API/CLI 接口定义（机器可读与人类可读）
│   ├── db/                     # 数据库/数据模型定义
│   └── testing/                # BDD 测试用例规范
├── docs/                       # 用户文档和教程
│   ├── setup/                  # 环境搭建指南
│   ├── architecture/           # 架构说明文档
│   ├── tutorials/              # 用户使用教程
│   └── changelogs/             # 变更日志索引
├── dos2unix/                   # CRLF → LF 转换器 (Rust)
├── gzip/                       # 压缩/解压工具 (Rust + Go)
│   ├── go/                     # Go 实现
│   └── rust/                   # Rust 实现
├── htop/                       # 系统监控 TUI (Rust + Go)
│   ├── shared/                 # 共享 Rust 库
│   ├── unix/rust/              # Unix Rust 实现
│   └── win/                    # Windows 实现
│       ├── go/                 # Go 实现
│       └── rust/               # Rust 实现
├── .github/workflows/          # CI/CD 工作流
├── AGENTS.md                   # 本文件：AI Agent 工作流配置
├── CHANGELOG.md                # 变更日志
├── CONTRIBUTING.md             # 贡献指南
├── README.md                   # 项目文档 (默认英文)
└── README.zh-CN.md             # 项目文档 (中文版)
```

---

## AI Agent Workflow Instructions (AI 工作流指令)

当你（AI）被要求开发一个新功能、修改现有功能或修复 Bug 时，**必须严格按照以下工作流执行，不可跳过任何步骤**：

### Step 1: 审查与分析 (Review Specs)

- 在编写任何代码之前，首先阅读 `/specs` 目录下相关的产品文档、RFC 和 API 定义。
- 如果用户指令与现有 Spec 冲突，应立即停止编码，并指出冲突点，询问用户是否需要先更新 Spec。

**必须阅读的文档**：
```
/specs/product/project-standardization.md   # 产品需求
/specs/rfc/0001-project-architecture.md     # 架构设计
/specs/api/cli-interfaces.md                # 接口定义
/specs/testing/test-specifications.md       # 测试规范
```

### Step 2: 规范优先 (Spec-First Update)

- 如果这是一个新功能，或者需要改变现有的接口/数据结构，**必须首先提议修改或创建相应的 Spec 文档**。
- 等待用户确认 Spec 的修改后，才能进入代码编写阶段。

**Spec 更新流程**：
1. 新功能 → 先更新 `/specs/product/`
2. 接口变更 → 先更新 `/specs/api/`
3. 架构变更 → 先创建 `/specs/rfc/NNNN-xxx.md`
4. 数据模型变更 → 先更新 `/specs/db/`

### Step 3: 代码实现 (Implementation)

- 编写代码时，必须 100% 遵守 Spec 中的定义（包括变量命名、API 路径、数据类型、状态码等）。
- 不要在代码中擅自添加 Spec 中未定义的功能（No Gold-Plating）。
- 遵循项目现有的代码风格和约定。

**代码风格要求**：
- Rust: 使用 `cargo fmt` 格式化，通过 `cargo clippy -- -D warnings` 检查
- Go: 使用 `gofmt` 格式化，通过 `go vet` 检查

### Step 4: 测试验证 (Test against Spec)

- 根据 `/specs/testing/` 中的验收标准（Acceptance Criteria）编写单元测试和集成测试。
- 确保测试用例覆盖了 Spec 中描述的所有边界情况。

**测试要求**：
```bash
# 运行所有测试
make test-all

# 或者分别运行
cargo test --all          # Rust 测试
go test -v ./...          # Go 测试
```

---

## Code Generation Rules

### 通用规则

1. **任何对外部暴露的 API/CLI 变更，必须同步修改 `/specs/api/cli-interfaces.md`**
2. **如果遇到不确定的技术细节，请查阅 `/specs/rfc/` 下的架构约定，不要自行捏造设计模式**
3. **所有新增代码必须遵循项目现有的代码风格和测试规范**
4. **文档更新应与代码更新同步**

### Rust 代码规范

```rust
// 使用 anyhow 进行错误处理
use anyhow::Result;

// 公共 API 必须有文档注释
/// Converts CRLF line endings to LF.
///
/// # Arguments
///
/// * `input` - Input string potentially containing CRLF sequences
///
/// # Returns
///
/// String with all CRLF replaced by LF
pub fn convert_crlf_to_lf(input: &str) -> String {
    input.replace("\r\n", "\n")
}
```

### Go 代码规范

```go
// Exported functions must have documentation comments.
// ConvertCRLFToLF converts Windows line endings to Unix line endings.
func ConvertCRLFToLF(input string) string {
    return strings.ReplaceAll(input, "\r\n", "\n")
}
```

---

## Documentation Standards

### README 规范

- `README.md` 默认使用英文
- 在顶部提供中文版链接：`**English** | [简体中文](README.zh-CN.md)`
- 中文版在顶部提供英文链接：`[English](README.md) | **简体中文**`
- 所有文档遵循一致的命名约定（大写英文，如 `GETTING-STARTED.md`）

### 文档更新同步

当代码发生变更时，需要同步更新以下文档：

| 变更类型 | 需要更新的文档 |
|----------|----------------|
| 新功能 | `CHANGELOG.md`, 相关 `README.md`, `/specs/product/` |
| API 变更 | `/specs/api/`, `docs/architecture/API.md` |
| Bug 修复 | 相关 `changelog/CHANGELOG.md` |
| 架构变更 | `/specs/rfc/`, `docs/architecture/ARCHITECTURE.md` |

---

## Project-Specific Guidelines

### 多语言实现对比

本项目包含 Rust 和 Go 两种语言的实现。当修改其中一个实现时：

1. **考虑是否需要在另一个语言中做对应修改**
2. **如果是对等修改，参考另一个实现的模式**
3. **如果是语言特定的优化，在注释中说明原因**

### 工具复杂度分级

| 工具 | 复杂度 | 学习重点 |
|------|--------|----------|
| dos2unix | ⭐ 初级 | 文件 I/O、流处理、错误处理 |
| gzip | ⭐⭐ 中级 | 压缩算法、库设计、并发 |
| htop | ⭐⭐⭐ 高级 | TUI 开发、系统 API、异步 |

### 构建命令参考

```bash
# 构建所有项目
make build-all

# 仅构建 Rust 项目
make build-rust

# 仅构建 Go 项目
make build-go

# 运行所有测试
make test-all

# 代码检查
make lint-all

# 代码格式化
make fmt-all
```

---

## Why This Matters

### 防范 AI 幻觉

AI 很容易在没有上下文的情况下"自由发挥"。强制它第一步读取 `/specs` 可以锚定其思考范围。

### 约束修改路径

声明了"修改代码前先改 Spec"，保证了文档与代码永远同步（Document-Code Synchronization）。

### 提高 PR 质量

当 AI 帮你生成 Pull Request 时，它的实现会与业务逻辑高度一致，因为它是根据你在 Spec 中定义的验收标准来进行开发的。

---

## Quick Reference

### 文档位置速查

| 需要查找的内容 | 文档位置 |
|----------------|----------|
| 产品需求 | `/specs/product/project-standardization.md` |
| 架构设计 | `/specs/rfc/0001-project-architecture.md` |
| CLI 接口定义 | `/specs/api/cli-interfaces.md` |
| 数据模型 | `/specs/db/schema.md` |
| 测试规范 | `/specs/testing/test-specifications.md` |
| 快速开始 | `docs/setup/GETTING-STARTED.md` |
| 架构说明 | `docs/architecture/ARCHITECTURE.md` |
| 语言对比 | `docs/tutorials/COMPARISON.md` |
| API 参考 | `docs/architecture/API.md` |
| 变更日志 | `CHANGELOG.md` |
| 贡献指南 | `CONTRIBUTING.md` |

### 关键命令速查

```bash
# 开发流程
make fmt-all && make lint-all && make test-all && make build-all

# 运行特定工具
./target/release/dos2unix-rust file.txt
./target/release/rgzip file.txt
./target/release/htop-unix-rust

# 查看 API 文档
cargo doc --open  # Rust 文档
go doc -all       # Go 文档
```

---

**Last Updated**: 2026-04-17
