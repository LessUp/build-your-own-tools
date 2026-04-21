## Release v0.3.0

### 🎉 What's New

#### Documentation Overhaul

- **Bilingual Documentation**: Complete documentation now available in both English and Chinese
  - English: `docs/setup/`, `docs/architecture/`, `docs/tutorials/` - comprehensive guides
  - 中文: Same directories with `.zh-CN.md` suffix - comprehensive guides
  - Documentation index with language switching

- **New Documentation**:
  - [Getting Started Guide](docs/setup/GETTING-STARTED.md) - Environment setup, debugging, troubleshooting
  - [API Reference](docs/architecture/API.md) - Complete library API documentation
  - [Architecture Guide](docs/architecture/ARCHITECTURE.md) - Mermaid diagrams, system design details
  - [Enhanced Comparison](docs/tutorials/COMPARISON.md) - Performance benchmarks, code statistics

#### Changelog Improvements

- Centralized changelog index at `docs/changelogs/`
- Migration guide for version upgrades
- Professional formatting with categories (Added/Changed/Fixed/Performance)
- Performance metrics and benchmarks

### 📖 Documentation | 文档

- ✅ English Documentation (4 guides + index)
- ✅ 简体中文文档 (4 指南 + 索引)
- ✅ Changelog Index & Migration Guide
- ✅ API Reference Documentation
- ✅ Architecture Diagrams (Mermaid)

### 🛠️ Infrastructure | 基础设施

- Updated VitePress configuration for new doc structure
- Enhanced navigation and search
- Cross-linking between language versions

### 📦 Assets | 下载

| Platform | Rust Binaries | Go Binaries |
|----------|---------------|-------------|
| Linux AMD64 | ✅ dos2unix-rust, rgzip, htop-unix-rust | ✅ gzip-go |
| macOS AMD64 | ✅ dos2unix-rust, rgzip, htop-unix-rust | ✅ gzip-go |
| macOS ARM64 | ✅ dos2unix-rust, rgzip, htop-unix-rust | ✅ gzip-go |
| Windows AMD64 | ✅ dos2unix-rust, rgzip, htop-win-rust | ✅ gzip-go, htop-win-go |

### 🚀 Quick Start | 快速开始

```bash
# Download the latest release
curl -LO https://github.com/LessUp/build-your-own-tools/releases/latest/download/dos2unix-rust-linux-amd64
chmod +x dos2unix-rust-linux-amd64
mv dos2unix-rust-linux-amd64 /usr/local/bin/dos2unix-rust
```

---

## 发布 v0.3.0（中文版）

### 🎉 新特性

#### 文档全面重构

- **双语文档**：现在提供完整的中英文双语文档
  - 英文：`docs/setup/`, `docs/architecture/`, `docs/tutorials/` - 全面指南
  - 中文：同一目录下 `.zh-CN.md` 后缀 - 全面指南
  - 文档索引支持语言切换

- **新增文档**：
  - [快速开始指南](docs/setup/GETTING-STARTED.zh-CN.md) - 环境搭建、调试、故障排除
  - [API 参考](docs/architecture/API.zh-CN.md) - 完整库 API 文档
  - [架构指南](docs/architecture/ARCHITECTURE.zh-CN.md) - Mermaid 图表、系统设计详情
  - [增强对比](docs/tutorials/COMPARISON.zh-CN.md) - 性能基准、代码统计

#### 变更日志改进

- 集中式变更日志索引位于 `docs/changelogs/`
- 版本升级迁移指南
- 专业格式化（新增/变更/修复/性能）
- 性能指标和基准测试

### 🛠️ 基础设施

- 更新 VitePress 配置以支持新文档结构
- 增强导航和搜索功能
- 语言版本之间的交叉链接

### 🚀 快速开始

```bash
# 下载最新版本
curl -LO https://github.com/LessUp/build-your-own-tools/releases/latest/download/dos2unix-rust-linux-amd64
chmod +x dos2unix-rust-linux-amd64
mv dos2unix-rust-linux-amd64 /usr/local/bin/dos2unix-rust
```

---

**Full Changelog**: [v0.2.0...v0.3.0](../../compare/v0.2.0...v0.3.0)  
**完整更新日志**: [v0.2.0...v0.3.0](../../compare/v0.2.0...v0.3.0)
