---
layout: home
hero:
  name: Build Your Own Tools
  text: 系统编程技术白皮书
  tagline: Rust × Go 双实现架构对比研究
  image:
    src: /logo.svg
    alt: Build Your Own Tools
  actions:
    - theme: brand
      text: 查看架构全景
      link: /whitepaper/architecture
    - theme: alt
      text: 技术规范
      link: /specs/
    - theme: alt
      text: GitHub
      link: https://github.com/LessUp/build-your-own-tools

features:
  - icon: 🏗️
    title: 架构对比
    details: 同一问题的 Rust 与 Go 实现，深入分析两种语言的设计哲学差异
  - icon: 📋
    title: OpenSpec 规范
    details: Gherkin 风格的需求规格，从需求到实现的完整追踪
  - icon: 📊
    title: 性能分析
    details: 跨语言性能基准，内存模型与并发模型对比研究
  - icon: 🤖
    title: AI 协作
    details: AGENTS.md + CLAUDE.md 治理层设计，AI 辅助工程实践
---

## 技术白皮书概览

本项目是一个**系统编程学习仓库**，通过重新实现三个真实的 CLI 工具（dos2unix、gzip、htop）来展示 Rust 和 Go 两种语言的系统编程风格差异。

### 核心特性

```mermaid
graph LR
    A[Monorepo] --> B[dos2unix]
    A --> C[gzip]
    A --> D[htop]
    
    B --> B1[Rust]
    C --> C1[Rust]
    C --> C2[Go]
    D --> D1[Rust]
    D --> D2[Go]
    
    style A fill:#f59e0b,color:#fff
    style B fill:#3b82f6,color:#fff
    style C fill:#3b82f6,color:#fff
    style D fill:#3b82f6,color:#fff
```

### 学习路径

| 阶段 | 工具 | 学习重点 | 复杂度 |
|------|------|----------|--------|
| 1 | dos2unix | 流式 I/O、换行符处理 | ⭐ |
| 2 | gzip | 压缩流程、CLI 设计、错误处理 | ⭐⭐ |
| 3 | htop | TUI、系统 API、跨平台架构 | ⭐⭐⭐ |

### 技术栈

- **Rust**: 系统编程、内存安全、零成本抽象
- **Go**: 并发模型、简洁语法、快速开发
- **VitePress**: 文档站点、Mermaid 图表、LLM 友好输出
- **OpenSpec**: 需求规范、变更管理、Gherkin 场景

## 快速导航

<div class="quick-links">

[白皮书](/whitepaper/){.VPButton}
[技术规范](/specs/){.VPButton .alt}
[对比研究](/comparison/){.VPButton .alt}
[工程实践](/engineering/){.VPButton .alt}

</div>

<style>
.quick-links {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin: 2rem 0;
}
</style>
