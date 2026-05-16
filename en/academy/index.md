---
title: Academy
---

# Build Your Own Tools Academy

Welcome to the **BYOT Academy** — a progressive learning path to master systems programming by re-implementing real CLI tools.

## Learning Philosophy

We believe **learning by doing** is the best approach. This academy selects three representative command-line tools, from simple to complex, guiding you step by step through the core concepts of systems programming.

More importantly, each tool provides **dual implementations in both Rust and Go**, allowing you to directly compare the design philosophy differences between the two languages when solving the same problem.

## Learning Path

```mermaid
graph TD
    Start([Start Learning]) --> M1[Module 1: dos2unix]
    M1 --> M2[Module 2: gzip]
    M2 --> M3[Module 3: htop]
    
    M1 --> |"⭐"| L1[Streaming I/O]
    M1 --> |"⭐"| L2[Newline Handling]
    
    M2 --> |"⭐⭐"| L3[Compression Algorithms]
    M2 --> |"⭐⭐"| L4[CLI Design Patterns]
    M2 --> |"⭐⭐"| L5[Error Handling Strategies]
    
    M3 --> |"⭐⭐⭐"| L6[TUI Frameworks]
    M3 --> |"⭐⭐⭐"| L7[System APIs]
    M3 --> |"⭐⭐⭐"| L8[Cross-Platform Architecture]
    
    classDef primary fill:#f59e0b,color:#fff,stroke:#d97706,stroke-width:2px
    classDef module fill:#3b82f6,color:#fff,stroke:#2563eb,stroke-width:2px
    classDef concept fill:#22c55e,color:#fff,stroke:#16a34a,stroke-width:2px
    
    class Start primary
    class M1,M2,M3 module
    class L1,L2,L3,L4,L5,L6,L7,L8 concept
```

## Module Overview

| Module | Tool | Core Concepts | Complexity | Rust | Go |
|--------|------|---------------|------------|------|-----|
| 1 | dos2unix | Streaming I/O, newline handling, basic error handling | ⭐ | ✅ | — |
| 2 | gzip | DEFLATE algorithm, CLI design, dual-language comparison | ⭐⭐ | ✅ | ✅ |
| 3 | htop | TUI development, system APIs, cross-platform architecture | ⭐⭐⭐ | ✅ | ✅ |

## Prerequisites

- **Required**: Basic knowledge of at least one programming language
- **Recommended**: Basic command-line experience
- **Bonus**: Familiarity with Rust or Go syntax

## Study Tips

1. **Follow the order**: Modules increase in difficulty, complete them in sequence
2. **Compare implementations**: Focus on the differences between Rust and Go
3. **Hands-on practice**: Try implementing or modifying features yourself
4. **Reference specs**: Use the [Specifications](/en/specs/) to understand requirement-driven development

## Recommended Reading

- [Whitepaper: System Architecture](/en/whitepaper/architecture) — Understand the overall design
- [Comparison: Memory Model](/en/comparison/memory) — Rust vs Go memory management
- [References](/en/reference/) — Related papers and projects
