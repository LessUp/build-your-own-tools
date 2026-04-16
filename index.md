---
layout: home

hero:
  name: Build Your Own Tools
  text: 用 Rust / Go 手写命令行工具
  tagline: 底层实现 · 命令行设计 · 跨语言对比 · 全平台支持
  image:
    src: /logo.svg
    alt: Build Your Own Tools
  actions:
    - theme: brand
      text: 🚀 快速开始
      link: /docs/GETTING-STARTED
    - theme: alt
      text: 📖 架构指南
      link: /docs/ARCHITECTURE
    - theme: alt
      text: 🔍 语言对比
      link: /docs/COMPARISON
    - theme: sponsor
      text: ⭐ GitHub
      link: https://github.com/LessUp/build-your-own-tools

features:
  - icon: <span class="emoji">🔧</span>
    title: dos2unix
    details: Rust 实现的 CRLF → LF 转换器。学习文件 I/O、流处理、缓冲区管理，以及跨平台换行符处理。
    link: /dos2unix/
    linkText: 了解详情 →
  
  - icon: <span class="emoji">📦</span>
    title: gzip
    details: Go + Rust 双语言 DEFLATE 压缩/解压。对比两种语言的流处理、错误处理、并发模式差异。
    link: /gzip/
    linkText: 了解详情 →
  
  - icon: <span class="emoji">📊</span>
    title: htop
    details: 跨平台 TUI 系统监控器。Ratatui × tview，实时 CPU/内存/进程监控，掌握系统编程。
    link: /htop/
    linkText: 了解详情 →
  
  - icon: <span class="emoji">🎯</span>
    title: 学习驱动设计
    details: 代码清晰优先于微优化，详细注释解释"为什么"，从简单到复杂渐进式学习路径。
  
  - icon: <span class="emoji">⚡</span>
    title: 现代工具链
    details: GitHub Actions 多平台 CI/CD，Makefile 统一构建，Cargo × Go Workspaces，自动发布。
  
  - icon: <span class="emoji">🔀</span>
    title: 深度对比
    details: 所有权 vs GC、Result vs error、RAII vs defer、编译时 vs 运行时，亲眼见证差异。
    link: /docs/COMPARISON
    linkText: 查看对比 →

head:
  - - meta
    - name: description
      content: 用 Rust 和 Go 手写常用命令行工具的学习仓库。包含 dos2unix、gzip、htop 的实现，对比两种语言在系统编程中的差异。
  - - meta
    - name: keywords
      content: Rust, Go, CLI, 命令行工具, 系统编程, TUI, dos2unix, gzip, htop, 学习, 教程
  - - meta
    - property: og:title
      content: Build Your Own Tools - 用 Rust / Go 手写命令行工具
  - - meta
    - property: og:description
      content: 从零开始实现常用 CLI 工具，学习底层系统编程和跨语言对比
  - - meta
    - property: og:type
      content: website
  - - meta
    - property: og:url
      content: https://lessup.github.io/build-your-own-tools/
  - - meta
    - name: twitter:card
      content: summary_large_image
  - - meta
    - name: twitter:title
      content: Build Your Own Tools
  - - meta
    - name: twitter:description
      content: 用 Rust / Go 手写命令行工具，学习系统编程
---

<script setup>
import { onMounted } from 'vue'

onMounted(() => {
  // Add smooth scroll behavior
  document.documentElement.style.scrollBehavior = 'smooth'
})
</script>

<style>
/* Custom styles for the hero */
.VPHero {
  .name {
    background: linear-gradient(120deg, #f59e0b 30%, #ef4444);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }
}

.emoji {
  font-size: 2.5em;
  display: block;
  margin-bottom: 0.5rem;
}

/* Feature cards hover effect */
.VPFeature {
  transition: all 0.3s ease;
  border: 1px solid var(--vp-c-divider);
}

.VPFeature:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 24px -8px rgba(0, 0, 0, 0.15);
  border-color: var(--vp-c-brand-1);
}
</style>

## 🎓 学习目标矩阵

<table>
  <thead>
    <tr>
      <th style="width: 20%">技术领域</th>
      <th style="width: 26%">⭐ dos2unix</th>
      <th style="width: 26%">⭐⭐ gzip</th>
      <th style="width: 26%">⭐⭐⭐ htop</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>文件 I/O</strong></td>
      <td>✅ 流式读写</td>
      <td>✅ 流式压缩</td>
      <td>—</td>
    </tr>
    <tr>
      <td><strong>CLI 设计</strong></td>
      <td>✅ 手动参数</td>
      <td>✅ clap/pflag</td>
      <td>✅ 交互式 TUI</td>
    </tr>
    <tr>
      <td><strong>错误处理</strong></td>
      <td>✅ anyhow/Result</td>
      <td>✅ anyhow/error</td>
      <td>✅ anyhow/error</td>
    </tr>
    <tr>
      <td><strong>算法</strong></td>
      <td>—</td>
      <td>✅ DEFLATE</td>
      <td>—</td>
    </tr>
    <tr>
      <td><strong>系统 API</strong></td>
      <td>—</td>
      <td>—</td>
      <td>✅ 进程/CPU/内存</td>
    </tr>
    <tr>
      <td><strong>TUI 开发</strong></td>
      <td>—</td>
      <td>—</td>
      <td>✅ ratatui/tview</td>
    </tr>
    <tr>
      <td><strong>并发</strong></td>
      <td>—</td>
      <td>✅ goroutines</td>
      <td>✅ 异步刷新</td>
    </tr>
  </tbody>
</table>

## 🛠️ 技术栈对比

<div class="tech-stack">

| 特性 | Rust | Go |
|------|:--:|:--:|
| **CLI 框架** | clap | 标准库 flag |
| **TUI 框架** | ratatui + crossterm | tview |
| **压缩库** | flate2 (DEFLATE) | compress/gzip (标准库) |
| **系统信息** | sysinfo | gopsutil |
| **测试框架** | cargo test | go test |
| **错误处理** | anyhow + ? | error + if err != nil |
| **内存管理** | 所有权 + 借用检查 | GC |
| **二进制大小** | ~2MB | ~4MB |

</div>

## 🚀 快速开始

::: code-group

```bash [Rust]
# 构建全部 Rust 项目
make build-rust

# 或使用 cargo
cargo build --release -p dos2unix-rust
cargo build --release -p rgzip
cargo build --release -p htop-rust
```

```bash [Go]
# 构建全部 Go 项目
make build-go

# 或使用 go build
cd gzip/go && go build -o bin/gzip-go ./cmd/gzip-go
cd htop/win/go && go build -o bin/htop-win-go ./cmd/htop-win-go
```

```bash [测试]
# 运行所有测试
make test-all

# Rust 测试
cargo test --all

# Go 测试
go test ./...
```

:::

## 📊 项目统计

<div class="stats-grid">

| 指标 | 数值 |
|------|------|
| 实现语言 | **Rust + Go** |
| 子项目 | **3** (dos2unix, gzip, htop) |
| 实现版本 | **6** (跨平台) |
| 文档页面 | **14+** |
| 支持平台 | Linux, macOS, Windows |
| CI/CD 平台 | **12** 种组合 |
| 测试覆盖率 | **> 80%** |

</div>

## 🤝 参与贡献

我们欢迎各种形式的贡献！无论你是想修复 bug、添加新功能，还是改进文档。

<div class="actions">

[查看贡献指南](./CONTRIBUTING.md){.VPButton}
[阅读行为准则](./CODE_OF_CONDUCT.md){.VPButton .alt}
[报告问题](https://github.com/LessUp/build-your-own-tools/issues){.VPButton .alt}

</div>

## 📜 许可证

本项目采用 **MIT OR Apache-2.0** 双许可证。您可以选择其中任意一种。

<div class="license-links">

[查看 LICENSE](./LICENSE)
[查看 Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0)
[查看 MIT](http://opensource.org/licenses/MIT)

</div>

---

<div align="center">

**Made with ❤️ by [LessUp](https://github.com/LessUp)**

[English](README.md) | **简体中文** | [GitHub](https://github.com/LessUp/build-your-own-tools)

</div>

<style>
.tech-stack {
  overflow-x: auto;
}

.tech-stack th:first-child {
  text-align: left;
}

.tech-stack td:not(:first-child),
.tech-stack th:not(:first-child) {
  text-align: center;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin: 2rem 0;
}

.stats-grid table {
  margin: 0;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin: 2rem 0;
  justify-content: center;
}

.license-links {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin: 1rem 0;
  justify-content: center;
  font-size: 0.9rem;
}

.license-links a {
  color: var(--vp-c-text-2);
  text-decoration: none;
}

.license-links a:hover {
  color: var(--vp-c-brand-1);
}
</style>
