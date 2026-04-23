---
layout: home

hero:
  name: Build Your Own Tools
  text: Learn systems programming in Rust and Go
  tagline: Three real CLI tools covering streaming I/O, compression, terminal UI, and cross-platform implementation trade-offs.
  image:
    src: /logo.svg
    alt: Build Your Own Tools
  actions:
    - theme: brand
      text: Explore the docs
      link: /en/docs/setup/GETTING-STARTED
    - theme: alt
      text: Compare Rust and Go
      link: /en/docs/tutorials/COMPARISON
    - theme: alt
      text: GitHub
      link: https://github.com/LessUp/build-your-own-tools

features:
  - icon: "🔧"
    title: dos2unix
    details: Start with a small but real streaming utility and learn file I/O, buffer boundaries, and newline handling.
  - icon: "📦"
    title: gzip
    details: Compare Rust and Go on the same compression/decompression problem and see how the trade-offs surface in code.
  - icon: "📊"
    title: htop
    details: Learn terminal UI architecture, process metrics, refresh models, and platform-specific system APIs.
  - icon: "🔀"
    title: Rust × Go
    details: This repo is built for implementation comparison, not just syntax comparison.
  - icon: "🧭"
    title: Progressive learning path
    details: Move from simple stream processing to a cross-platform TUI without jumping across unrelated examples.
  - icon: "📚"
    title: Full project surface
    details: Specs, CI, releases, and the docs site stay in the repo so you can study the engineering around the code too.
---

## Why explore this repository

| If you want to study... | You will find... |
| --- | --- |
| real Rust vs Go trade-offs | dual implementations, dual workspaces, and side-by-side design choices |
| how CLI tools are built | file processing, compression pipelines, terminal interfaces, and system metrics |
| how a learning repo is engineered | OpenSpec, Makefile automation, GitHub Actions, and VitePress |

## Tool map

| Tool | Languages | Main learning focus | Suggested order |
| --- | --- | --- | --- |
| `dos2unix` | Rust | streaming I/O and newline handling | 1 |
| `gzip` | Rust + Go | compression pipelines, CLI design, error handling | 2 |
| `htop` | Rust + Go | terminal UI, system APIs, cross-platform architecture | 3 |

## Where to go next

<div class="quick-links">

[Getting Started](/en/docs/setup/GETTING-STARTED){.VPButton}
[Architecture](/en/docs/architecture/ARCHITECTURE){.VPButton .alt}
[Comparison](/en/docs/tutorials/COMPARISON){.VPButton .alt}
[GitHub Repository](https://github.com/LessUp/build-your-own-tools){.VPButton .alt}

</div>

<style>
.quick-links {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin: 2rem 0;
}
</style>
