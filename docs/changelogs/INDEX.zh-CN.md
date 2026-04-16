# 变更日志索引

> build-your-own-tools 项目中所有变更日志的索引

[English](INDEX.md) | **简体中文**

---

## 概述

本项目在多个级别维护变更日志：

- **根目录变更日志**：项目范围的变更
- **子项目变更日志**：单个工具的变更

所有变更日志遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 格式和 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## 变更日志位置

### 根目录级别

| 文件 | 说明 | 最后更新 |
|------|-------------|--------------|
| [CHANGELOG.md](../../CHANGELOG.md) | 项目范围的变更和发布 | 2026-04-16 |

### 子项目变更日志

#### dos2unix

| 文件 | 说明 | 当前版本 |
|------|-------------|-----------------|
| [dos2unix/changelog/CHANGELOG.md](../../dos2unix/changelog/CHANGELOG.md) | CRLF 转换器变更 | v0.2.1 |

#### gzip

| 实现 | 文件 | 当前版本 |
|----------------|------|-----------------|
| Go | [gzip/go/changelog/CHANGELOG.md](../../gzip/go/changelog/CHANGELOG.md) | v0.3.0 |
| Rust | [gzip/rust/changelog/CHANGELOG.md](../../gzip/rust/changelog/CHANGELOG.md) | v0.1.1 |

#### htop

| 实现 | 文件 | 当前版本 |
|----------------|------|-----------------|
| Unix/Rust | [htop/unix/rust/changelog/CHANGELOG.md](../../htop/unix/rust/changelog/CHANGELOG.md) | v0.1.5 |
| Windows/Rust | [htop/win/rust/changelog/CHANGELOG.md](../../htop/win/rust/changelog/CHANGELOG.md) | v0.1.0 |
| Windows/Go | [htop/win/go/changelog/CHANGELOG.md](../../htop/win/go/changelog/CHANGELOG.md) | v0.1.3 |
| Shared | [htop/changelog/CHANGELOG.md](../../htop/changelog/CHANGELOG.md) | v0.1.0 |

---

## 版本历史

### 根项目版本

```
v0.2.0 (2026-03-13)  ───┬───  基础设施改进
                        │
v0.1.1 (2026-03-10)  ───┼───  工作流标准化
                        │
v0.1.0 (2026-02-13)  ───┘───  初始发布
```

### 子项目最新版本

| 项目 | 版本 | 状态 |
|---------|---------|--------|
| dos2unix | v0.2.1 | ✅ 稳定 |
| gzip/go | v0.3.0 | ✅ 稳定 |
| gzip/rust | v0.1.1 | ✅ 稳定 |
| htop/unix/rust | v0.1.5 | ✅ 稳定 |
| htop/win/rust | v0.1.0 | ✅ 稳定 |
| htop/win/go | v0.1.3 | ✅ 稳定 |

---

## 变更日志类别

### 变更类型

所有变更日志使用以下类别：

| 英文 | 中文 | 说明 |
|------|------|------|
| Added | 新增 | 新功能 |
| Changed | 变更 | 现有功能的变更 |
| Deprecated | 废弃 | 即将移除的功能 |
| Removed | 移除 | 已移除的功能 |
| Fixed | 修复 | Bug 修复 |
| Security | 安全 | 安全改进 |
| Performance | 性能 | 性能改进 |
| Documentation | 文档 | 文档变更 |
| Infrastructure | 基础设施 | CI/CD 和工具变更 |

### 版本格式

```
## [版本号] - YYYY-MM-DD

### 类别

- 变更描述及详情
  - 额外上下文
  - 破坏性变更指示
```

---

## 迁移指南

详见 [MIGRATION.md](MIGRATION.md) 了解版本之间的详细迁移说明。

### 快速迁移链接

| 从版本 | 到版本 | 指南 |
|--------------|------------|-------|
| v0.1.x | v0.2.0 | [迁移指南](MIGRATION.md#v010-到-v020) |
| 初始版本 | v0.1.0 | [快速开始](../zh-CN/GETTING-STARTED.md) |

---

## 贡献变更日志

贡献变更时：

1. 将条目添加到适当的 `Unreleased` 部分
2. 使用正确的类别（新增/变更/修复等）
3. 适用时包含 issue/PR 引用
4. 清楚标记破坏性变更

示例：
```markdown
### 新增

- **dos2unix**: 新增 `--check` 标志用于检测 CRLF 而不修改
  - 发现 CRLF 时返回退出码 2
  - 适用于 CI/CD 流水线 ([#123](../../issues/123))

### 修复

- **htop**: 修复进程刷新循环中的内存泄漏
  - **BREAKING**: 变更了刷新 API 签名
```

---

## 发布计划

| 发布类型 | 频率 | 说明 |
|--------------|-----------|-------------|
| 主版本 | 按需 | 破坏性变更 |
| 次版本 | 每月 | 新功能 |
| 补丁版本 | 双周 | Bug 修复 |

---

## 相关资源

- [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)
- [语义化版本](https://semver.org/lang/zh-CN/)
- [约定式提交](https://www.conventionalcommits.org/zh-hans/)
- [迁移指南](MIGRATION.md)

---

**最后更新**: 2026-04-16  
**版本**: 1.0
