# Implementation Tasks

## Overview

Build-Your-Own-Tools 项目规范化实施任务清单。

## Completed Tasks

### Phase 1: 项目文档体系 ✅

- [x] 1.1 增强 README.md
  - 添加 CI、Docs、License、Rust、Go 徽章
  - 完善特性列表和快速开始指南
  - 添加项目结构树
- [x] 1.2 创建 CONTRIBUTING.md
- [x] 1.3 创建 CODE_OF_CONDUCT.md
- [x] 1.4 创建 SECURITY.md
- [x] 1.5 创建根目录 CHANGELOG.md

### Phase 2: 代码质量配置 ✅

- [x] 2.1 创建 .editorconfig
- [x] 2.2 创建 rustfmt.toml
- [x] 2.3 创建 .golangci.yml

### Phase 3: GitHub 配置 ✅

- [x] 3.1 创建 Issue 模板 (bug_report.md, feature_request.md)
- [x] 3.2 创建 PR 模板
- [x] 3.3 增强 CI 工作流 (跨平台构建)
- [x] 3.4 创建 Release 工作流
- [x] 3.5 创建 Pages 工作流

### Phase 4: 文档目录 ✅

- [x] 4.1 创建 docs/ARCHITECTURE.md
- [x] 4.2 创建 docs/COMPARISON.md

### Phase 5: Changelog 整理 ✅

- [x] 5.1 整合根目录 CHANGELOG.md
- [x] 5.2 整合 dos2unix/changelog/CHANGELOG.md
- [x] 5.3 整合 gzip/go/changelog/CHANGELOG.md
- [x] 5.4 整合 gzip/rust/changelog/CHANGELOG.md
- [x] 5.5 整合 htop/changelog/CHANGELOG.md
- [x] 5.6 整合 htop/unix/rust/changelog/CHANGELOG.md
- [x] 5.7 整合 htop/win/rust/changelog/CHANGELOG.md
- [x] 5.8 整合 htop/win/go/changelog/CHANGELOG.md
- [x] 5.9 删除分散的日期命名 changelog 文件

## In Progress

### Phase 6: 子项目 README 🔄

- [ ] 6.1 更新 gzip/go/README.md
- [ ] 6.2 更新 htop/README.md
- [ ] 6.3 更新 htop/unix/rust/README.md
- [ ] 6.4 更新 htop/win/rust/README.md
- [ ] 6.5 更新 htop/win/go/README.md

## Pending

### Phase 7: 文档优化

- [ ] 7.1 更新 docs/ARCHITECTURE.md
- [ ] 7.2 更新 docs/COMPARISON.md

## Verification

```bash
# Rust
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all --all-features

# Go
cd gzip/go && gofmt -l . && go vet ./... && go test ./...
cd htop/win/go && gofmt -l . && go vet ./... && go test ./...
```
