---
layout: home
hero:
  name: Build Your Own Tools
  text: Technical Whitepaper
  tagline: Rust × Go Dual-Implementation Architecture Comparison Study
  image:
    src: /logo.svg
    alt: Build Your Own Tools
  actions:
    - theme: brand
      text: View Architecture
      link: /en/whitepaper/architecture
    - theme: alt
      text: Specifications
      link: /en/specs/
    - theme: alt
      text: GitHub
      link: https://github.com/LessUp/build-your-own-tools

features:
  - icon: 🏗️
    title: Architecture Comparison
    details: Same problem solved in Rust and Go, revealing deep design philosophy differences between the languages
  - icon: 📋
    title: OpenSpec Specifications
    details: Gherkin-style requirements with complete traceability from spec to implementation
  - icon: 📊
    title: Performance Analysis
    details: Cross-language benchmarks comparing memory models, concurrency patterns, and runtime characteristics
  - icon: 🤖
    title: AI Collaboration
    details: AGENTS.md + CLAUDE.md governance layer design for AI-assisted engineering workflows
---

## Technical Whitepaper Overview

This project is a **systems programming learning repository** that re-implements three real CLI tools (dos2unix, gzip, htop) to demonstrate the differences between Rust and Go system programming styles.

### Core Features

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

### Learning Path

| Stage | Tool | Learning Focus | Complexity |
|-------|------|----------------|------------|
| 1 | dos2unix | Streaming I/O, newline handling | ⭐ |
| 2 | gzip | Compression pipelines, CLI design, error handling | ⭐⭐ |
| 3 | htop | TUI, system APIs, cross-platform architecture | ⭐⭐⭐ |

### Tech Stack

- **Rust**: Systems programming, memory safety, zero-cost abstractions
- **Go**: Concurrency model, simple syntax, rapid development
- **VitePress**: Documentation site, Mermaid diagrams, LLM-friendly output
- **OpenSpec**: Requirements specs, change management, Gherkin scenarios

## Quick Navigation

<div class="quick-links">

[Whitepaper](/en/whitepaper/){.VPButton}
[Specifications](/en/specs/){.VPButton .alt}
[Comparison](/en/comparison/){.VPButton .alt}
[Engineering](/en/engineering/){.VPButton .alt}

</div>

<style>
.quick-links {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin: 2rem 0;
}
</style>
