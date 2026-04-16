# AGENTS.md - AI Agent Workflow Configuration

## Project Philosophy: Spec-Driven Development (SDD)

本项目严格遵循**规范驱动开发（Spec-Driven Development）**范式。所有的代码实现必须以 `/specs` 目录下的规范文档为唯一事实来源（Single Source of Truth）。

## Directory Context (目录说明)

- `/specs/product/`: 产品功能定义与验收标准 (PRD)
- `/specs/rfc/`: 技术设计文档 (RFCs)
- `/specs/api/`: API/CLI 接口定义（机器可读与人类可读）
- `/specs/db/`: 数据库/数据模型定义
- `/specs/testing/`: BDD 测试用例规范
- `/docs/`: 用户文档、教程、架构说明
- `/docs/en/`: English documentation
- `/docs/zh-CN/`: 中文文档

## AI Agent Workflow Instructions (AI 工作流指令)

当你（AI）被要求开发一个新功能、修改现有功能或修复 Bug 时，**必须严格按照以下工作流执行，不可跳过任何步骤**：

### Step 1: 审查与分析 (Review Specs)

- 在编写任何代码之前，首先阅读 `/specs` 目录下相关的产品文档、RFC 和 API 定义。
- 如果用户指令与现有 Spec 冲突，应立即停止编码，并指出冲突点，询问用户是否需要先更新 Spec。

### Step 2: 规范优先 (Spec-First Update)

- 如果这是一个新功能，或者需要改变现有的接口/数据库结构，**必须首先提议修改或创建相应的 Spec 文档**（如 API 定义或 RFC 文档）。
- 等待用户确认 Spec 的修改后，才能进入代码编写阶段。

### Step 3: 代码实现 (Implementation)

- 编写代码时，必须 100% 遵守 Spec 中的定义（包括变量命名、API 路径、数据类型、状态码等）。
- 不要在代码中擅自添加 Spec 中未定义的功能（No Gold-Plating）。

### Step 4: 测试验证 (Test against Spec)

- 根据 `/specs/testing/` 中的验收标准（Acceptance Criteria）编写单元测试和集成测试。
- 确保测试用例覆盖了 Spec 中描述的所有边界情况。

## Code Generation Rules

- 任何对外部暴露的 API/CLI 变更，必须同步修改 `/specs/api/` 目录中的接口定义。
- 如果遇到不确定的技术细节，请查阅 `/specs/rfc/` 下的架构约定，不要自行捏造设计模式。
- 所有新增代码必须遵循项目现有的代码风格和测试规范。

## Documentation Standards

- README.md 默认使用英文
- 提供中文版本链接（README.zh-CN.md）
- 所有文档遵循一致的命名约定（大写，英文）
- 文档更新应与代码更新同步

## Why This Matters

- **防范 AI 幻觉**：AI 很容易在没有上下文的情况下"自由发挥"。强制它第一步读取 /specs 可以锚定其思考范围。
- **约束修改路径**：声明了"修改代码前先改 Spec"，保证了文档与代码永远同步（Document-Code Synchronization）。
- **提高 PR 质量**：当 AI 帮你生成 Pull Request 时，它的实现会与业务逻辑高度一致，因为它是根据你在 Spec 中定义的验收标准来进行开发的。
