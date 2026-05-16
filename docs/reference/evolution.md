---
title: 演进思考
---

# 演进思考

本页面记录项目的设计决策演进、未来改进方向和社区贡献指南。

## 设计决策演进

### v0.1 → v0.2：从单语言到双语言

**背景**：最初项目只使用 Rust 实现。

**决策**：引入 Go 作为第二实现语言。

**理由**：
1. **对比学习价值**：同一问题的双实现能直观展示语言差异
2. **覆盖更广受众**：Rust 和 Go 是当前系统编程的主流选择
3. **工程实践**：多语言项目是真实场景中的常见需求

**代价**：
- 维护成本增加
- 构建复杂度上升

### v0.2 → v0.3：OpenSpec 规范引入

**背景**：需求散落在各处，缺乏结构化追踪。

**决策**：引入 OpenSpec 规范驱动开发。

**理由**：
1. **可追溯性**：从需求到实现的完整链路
2. **AI 协作友好**：结构化规格便于 AI 理解和辅助
3. **文档即代码**：规格本身是高质量的文档

### 当前：归档准备阶段

**目标**：稳定化、清晰化、降低维护负担。

**原则**：
- 不添加新功能
- 修复关键 Bug
- 完善文档
- 确保构建通过

## 未来改进方向

### 短期（如果解冻开发）

| 优先级 | 改进项 | 描述 |
|--------|--------|------|
| P0 | 性能基准测试 | 添加自动化基准测试和对比报告 |
| P0 | CI/CD 完善 | 添加跨平台构建和测试 |
| P1 | 文档国际化 | 完善英文文档 |
| P1 | 代码覆盖率 | 添加测试覆盖率报告 |

### 长期（社区驱动）

| 方向 | 可能性 | 描述 |
|------|--------|------|
| 新工具 | 中 | 添加更多 CLI 工具实现 |
| 第三语言 | 低 | 添加 Zig 或其他语言实现 |
| Web 版本 | 低 | htop 的 Web TUI 版本 |

## 架构改进思考

### 潜在重构点

1. **共享代码提取**
   - 当前：各工具独立实现
   - 改进：提取通用 CLI 解析、日志、错误处理模块

2. **构建系统统一**
   - 当前：Makefile + Cargo + go.work
   - 改进：考虑使用 Just 或 Task 统一任务运行

3. **测试策略优化**
   - 当前：各工具独立测试
   - 改进：添加跨工具的集成测试

### 不改进的理由

在"归档准备"阶段，我们选择**不进行**以下改进：

- **大规模重构**：风险大于收益
- **新功能添加**：违背归档目标
- **依赖升级**：可能引入破坏性变更

## 社区贡献指南

### 欢迎的贡献

- **文档改进**：修正错别字、补充说明、翻译
- **Bug 修复**：修复已知问题
- **测试补充**：增加测试覆盖

### 贡献流程

```mermaid
graph LR
    Fork[Fork 仓库] --> Branch[创建分支]
    Branch --> Change[进行修改]
    Change --> Test[运行测试]
    Test --> PR[提交 PR]
    PR --> Review[代码审查]
    Review --> Merge[合并]
    
    classDef step fill:#3b82f6,color:#fff
    class Fork,Branch,Change,Test,PR,Review,Merge step
```

### 代码风格

- **Rust**：遵循 `rustfmt` 和 `clippy` 建议
- **Go**：遵循 `gofmt` 和 `golangci-lint` 建议
- **文档**：遵循 `markdownlint` 规范

### 提交信息规范

使用 [Conventional Commits](https://www.conventionalcommits.org/)：

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

类型：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

## 致谢

感谢以下项目和资源对本项目的启发：

- [Build Your Own X](https://github.com/danistefanovic/build-your-own-x) — 项目灵感来源
- [htop](https://github.com/htop-dev/htop) — htop 实现参考
- [GNU gzip](https://www.gnu.org/software/gzip/) — gzip 规范参考
- [Rust](https://www.rust-lang.org/) & [Go](https://go.dev/) — 语言生态

---

> **联系**：如有问题或建议，请通过 [GitHub Issues](https://github.com/LessUp/build-your-own-tools/issues) 联系。