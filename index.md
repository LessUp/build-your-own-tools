---
layout: home

hero:
  name: Build Your Own Tools
  text: 用 Rust / Go 学系统编程
  tagline: 三个真实 CLI 工具，覆盖流处理、压缩、终端 UI 与跨平台实现。
  image:
    src: /logo.svg
    alt: Build Your Own Tools
  actions:
    - theme: brand
      text: 探索文档
      link: /docs/setup/GETTING-STARTED
    - theme: alt
      text: 查看语言对比
      link: /docs/tutorials/COMPARISON
    - theme: alt
      text: GitHub
      link: https://github.com/LessUp/build-your-own-tools

features:
  - icon: "🔧"
    title: dos2unix
    details: 从最小但不简单的流处理工具开始，学习文件 I/O、缓冲区边界和换行符细节。
  - icon: "📦"
    title: gzip
    details: 用 Rust 和 Go 并排实现压缩/解压 CLI，直观看到两种语言的设计差异。
  - icon: "📊"
    title: htop
    details: 通过跨平台终端 UI 掌握进程指标、刷新模型和系统 API 集成。
  - icon: "🔀"
    title: Rust × Go
    details: 同一个问题，两种实现路径，适合做设计对比而不只是语法对比。
  - icon: "🧭"
    title: 渐进式学习路径
    details: 从单一流处理程序一路走到跨平台 TUI，复杂度上升自然、可跟踪。
  - icon: "📚"
    title: 工程化可见
    details: OpenSpec、CI、发布和文档站点都保留在仓库里，便于学习完整项目形态。
---

## 为什么值得看

| 你想看什么 | 这里能看到什么 |
| --- | --- |
| Rust 与 Go 的实际取舍 | 同一工具的双实现、双工作区、双风格对比 |
| CLI 工具怎么从零实现 | 文件处理、压缩管线、终端界面、系统指标采集 |
| 一个学习仓库如何工程化 | OpenSpec、Makefile、GitHub Actions、VitePress |

## 工具地图

| 工具 | 语言 | 学习重点 | 推荐顺序 |
| --- | --- | --- | --- |
| `dos2unix` | Rust | 流式 I/O、换行符处理 | 1 |
| `gzip` | Rust + Go | 压缩流程、CLI 设计、错误处理 | 2 |
| `htop` | Rust + Go | TUI、系统 API、跨平台架构 | 3 |

## 下一步看什么

<div class="quick-links">

[快速开始](/docs/setup/GETTING-STARTED){.VPButton}
[架构说明](/docs/architecture/ARCHITECTURE){.VPButton .alt}
[语言对比](/docs/tutorials/COMPARISON){.VPButton .alt}
[项目仓库](https://github.com/LessUp/build-your-own-tools){.VPButton .alt}

</div>

<style>
.quick-links {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin: 2rem 0;
}
</style>
