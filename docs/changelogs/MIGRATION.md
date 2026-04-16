# Migration Guide | 迁移指南

> Instructions for migrating between versions / 版本之间迁移的说明

---

## English

### v0.1.x to v0.2.0

**Release Date**: 2026-03-13

#### Breaking Changes

None. This is a minor release focused on infrastructure improvements.

#### Migration Steps

1. **Update your local repository**:
   ```bash
   git pull origin main
   ```

2. **Verify workflows** (if you've forked):
   - Rename `docs.yml` to `pages.yml` if applicable
   - Update GitHub Actions configuration

3. **No code changes required** for users of the tools.

#### What's Changed

- **CI/CD**: Pages workflow installation strategy updated
  - Changed from `npm ci` to `npm install --no-package-lock`
  - Fixed dead links in `README.zh-CN.md`

- **Workflows**: rustfmt line ending rules fixed for Windows
  - Added `*.rs text eol=lf` to `.gitattributes`

### v0.1.0 Initial Release

**Release Date**: 2026-02-13

No migration needed - this is the initial release.

#### Included Tools

| Tool | Version | Description |
|------|---------|-------------|
| dos2unix | v0.1.0 | CRLF to LF converter |
| gzip (Go) | v0.1.0 | File compression with parallel processing |
| gzip (Rust) | v0.1.0 | File compression with library crate |
| htop (Unix/Rust) | v0.1.0 | System monitor for Unix |
| htop (Win/Rust) | v0.1.0 | System monitor for Windows |
| htop (Win/Go) | v0.1.0 | System monitor for Windows (Go) |

---

## 简体中文

### v0.1.x 升级到 v0.2.0

**发布日期**: 2026-03-13

#### 破坏性变更

无。这是一个专注于基础设施改进的次版本发布。

#### 迁移步骤

1. **更新你的本地仓库**:
   ```bash
   git pull origin main
   ```

2. **验证工作流**（如果你fork了）:
   - 如适用，将 `docs.yml` 重命名为 `pages.yml`
   - 更新 GitHub Actions 配置

3. **无需代码变更** - 对于工具用户而言。

#### 变更内容

- **CI/CD**: Pages 工作流安装策略更新
  - 从 `npm ci` 更改为 `npm install --no-package-lock`
  - 修复 `README.zh-CN.md` 中的失效链接

- **工作流**: 修复 Windows 的 rustfmt 换行符规则
  - 添加 `*.rs text eol=lf` 到 `.gitattributes`

### v0.1.0 初始发布

**发布日期**: 2026-02-13

无需迁移 - 这是初始发布。

#### 包含的工具

| 工具 | 版本 | 说明 |
|------|---------|-------------|
| dos2unix | v0.1.0 | CRLF 转 LF 转换器 |
| gzip (Go) | v0.1.0 | 带并行处理的文件压缩 |
| gzip (Rust) | v0.1.0 | 带库 crate 的文件压缩 |
| htop (Unix/Rust) | v0.1.0 | Unix 系统监控器 |
| htop (Win/Rust) | v0.1.0 | Windows 系统监控器 |
| htop (Win/Go) | v0.1.0 | Windows 系统监控器 (Go) |

---

## Future Migration Plans

### Upcoming v0.3.0 (Planned)

#### Breaking Changes Preview

- **htop/shared**: API refactoring for better modularity
- **dos2unix**: Potential library extraction

#### Preparation

Watch for deprecation warnings in current versions and follow migration guides as they're published.

---

## General Migration Best Practices | 通用迁移最佳实践

### Before Upgrading | 升级前

1. **Read the changelog** / 阅读变更日志
2. **Backup your data** / 备份你的数据
3. **Test in a non-production environment** / 在非生产环境测试

### During Upgrade | 升级期间

1. **Follow the specific migration guide** / 遵循特定的迁移指南
2. **Update one minor version at a time** / 一次更新一个次版本
3. **Run the full test suite** / 运行完整的测试套件

### After Upgrade | 升级后

1. **Verify functionality** / 验证功能
2. **Check for deprecation warnings** / 检查弃用警告
3. **Update your own code** / 更新你自己的代码

---

## Getting Help | 获取帮助

- 📖 [Documentation](../README.md) / [文档](../README.zh-CN.md)
- 🐛 [GitHub Issues](https://github.com/LessUp/build-your-own-tools/issues)
- 💬 [GitHub Discussions](https://github.com/LessUp/build-your-own-tools/discussions)

---

## Version Compatibility Matrix | 版本兼容性矩阵

| Version | dos2unix | gzip (Go) | gzip (Rust) | htop (Unix) | htop (Win/Rust) | htop (Win/Go) |
|---------|----------|-----------|-------------|-------------|-----------------|---------------|
| v0.2.0 | v0.2.1 | v0.3.0 | v0.1.1 | v0.1.5 | v0.1.0 | v0.1.3 |
| v0.1.1 | v0.1.0+ | v0.2.0+ | v0.1.0+ | v0.1.0+ | v0.1.0 | v0.1.0+ |
| v0.1.0 | v0.1.0 | v0.1.0 | v0.1.0 | v0.1.0 | v0.1.0 | v0.1.0 |

---

## Terminology | 术语

| English | 中文 | Description |
|---------|------|-------------|
| Breaking Change | 破坏性变更 | Changes that require code updates |
| Deprecation | 弃用 | Feature marked for future removal |
| Migration | 迁移 | Process of upgrading to a new version |
| Semantic Versioning | 语义化版本 | Versioning scheme (MAJOR.MINOR.PATCH) |

---

**Last Updated**: 2026-04-16  
**Version**: 1.0
