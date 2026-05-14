# OpenSpec 工作流

本文档描述 OpenSpec 规范框架的工作流程和最佳实践。

## 工作流概览

```mermaid
flowchart TB
    A[识别需求] --> B[创建变更目录]
    B --> C[编写 proposal.md]
    C --> D{评审通过?}
    D -->|否| C
    D -->|是| E[编写 design.md]
    E --> F{设计通过?}
    F -->|否| E
    F -->|是| G[编写 tasks.md]
    G --> H[开始实现]
    H --> I{实现完成?}
    I -->|否| H
    I -->|是| J[代码审查]
    J --> K{审查通过?}
    K -->|否| H
    K -->|是| L[归档到 archive/]
    
    style A fill:#f59e0b,color:#fff
    style L fill:#10b981,color:#fff
```

## 目录结构

```
openspec/changes/
├── archive/                    # 已完成变更
│   ├── phase-1-init/
│   │   ├── proposal.md
│   │   ├── design.md
│   │   └── tasks.md
│   └── phase-2-features/
│       └── ...
└── active/                     # 当前活跃变更
    └── phase-3-optimization/
        ├── proposal.md
        ├── design.md
        └── tasks.md
```

## 文档模板

### proposal.md

```markdown
# 提案标题

## 背景
描述为什么需要这个变更。

## 目标
- 目标 1
- 目标 2

## 范围
描述变更的边界。

## 非目标
明确不在范围内的事项。

## 影响评估
- 性能影响: ?
- 兼容性影响: ?
- 文档影响: ?
```

### design.md

```markdown
# 设计文档

## 概述
设计方案的总体描述。

## 架构变更
描述对现有架构的影响。

## API 设计
```rust
// 新 API 示例
pub fn new_function() -> Result<(), Error>;
```

## 数据结构
```rust
struct NewStruct {
    field: String,
}
```

## 实现计划
1. 步骤一
2. 步骤二
3. 步骤三
```

### tasks.md

```markdown
# 任务列表

## 必须完成
- [ ] 任务一
- [ ] 任务二

## 可选优化
- [ ] 优化一

## 验收标准
- [ ] 所有测试通过
- [ ] 文档已更新
```

## 变更生命周期

```mermaid
stateDiagram-v2
    [*] --> Draft: 创建 proposal
    Draft --> Review: 提交评审
    Review --> Approved: 通过
    Review --> Draft: 需要修改
    Approved --> InProgress: 开始实现
    InProgress --> Testing: 完成实现
    Testing --> Merged: 测试通过
    Merged --> Archived: 归档
    Archived --> [*]
```

## 最佳实践

### 1. 单一职责

每个变更应该只解决一个问题：

```mermaid
graph LR
    A[大需求] --> B[拆分]
    B --> C[变更1]
    B --> D[变更2]
    B --> E[变更3]
    
    style C fill:#10b981,color:#fff
    style D fill:#10b981,color:#fff
    style E fill:#10b981,color:#fff
```

### 2. 原子提交

每次提交应该是一个完整的逻辑单元：

```bash
# 好的提交
git commit -m "feat(dos2unix): add UTF-16 BOM detection"

# 不好的提交
git commit -m "fix some bugs and add features"
```

### 3. 可测试需求

每个需求都应该可验证：

```gherkin
# 好: 可测试
Then 输出文件大小应小于输入文件大小的 50%

# 不好: 不可测试
Then 性能应该提升
```

## 工具支持

### OpenSpec CLI

```bash
# 创建新变更
openspec new phase-4-new-feature

# 检查规范完整性
openspec validate

# 归档完成的变更
openspec archive phase-4-new-feature
```

### Git Hooks

```bash
# pre-commit hook
#!/bin/bash
openspec validate || exit 1
```

## 相关文档

- [技术规范概览](/specs/) — 规范总览
- [CI/CD 设计](/engineering/cicd) — 工作流集成
- [文档策略](/engineering/documentation) — 文档维护
