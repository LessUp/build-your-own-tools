# 文档

> Build Your Own Tools 的补充文档入口，只保留仍然有长期价值的说明面。

[English](README.md) | **简体中文**

## 文档结构

```text
docs/
├── README.md
├── README.zh-CN.md
├── setup/
├── tutorials/
└── architecture/
```

## 建议阅读顺序

1. **[快速开始](setup/GETTING-STARTED.zh-CN.md)** - 配置环境并跑通项目
2. **[架构指南](architecture/ARCHITECTURE.zh-CN.md)** - 了解仓库结构和工程表面
3. **[语言对比](tutorials/COMPARISON.zh-CN.md)** - 对比 Rust 与 Go 的实现选择
4. **[API 参考](architecture/API.zh-CN.md)** - 需要更细节时再深入查看

## 项目级核心文档

- [README.md](../README.md) - 仓库入口和导航
- [index.md](../index.md) - GitHub Pages 首页
- [CHANGELOG.md](../CHANGELOG.md) - 项目发布历史
- [CONTRIBUTING.md](../CONTRIBUTING.md) - 协作与贡献规则

## 说明

- `docs/` 负责解释长期有效的主题，不再承担 changelog 索引或迁移手册这类低价值辅助层。
- 如果某一页主要是在重复 README、Pages 或根目录 changelog，就应该被裁剪或并入 canonical surface。
