---
title: 参考文献
---

# 参考文献

本页面整理了项目中引用的学术论文、技术文档和相关资源。

## 学术论文

### 压缩算法

1. **"A Technique for High Performance Data Compression"** — Jacob Ziv, Abraham Lempel (1977)
   - 引入了 LZ77 算法，奠定了现代压缩技术的基础
   - [DOI: 10.1109/COM.1977.1097347](https://doi.org/10.1109/COM.1977.1097347)

2. **"A Method for the Construction of Minimum-Redundancy Codes"** — David A. Huffman (1952)
   - 霍夫曼编码的原始论文
   - [DOI: 10.1145/390202.391004](https://doi.org/10.1145/390202.391004)

### 系统编程

3. **"The Rust Programming Language"** — Steve Klabnik, Carol Nichols
   - Rust 官方教材，详细阐述所有权系统和内存安全
   - [在线阅读](https://doc.rust-lang.org/book/)

4. **"Effective Go"** — Google
   - Go 语言最佳实践指南
   - [在线阅读](https://go.dev/doc/effective_go)

### 并发模型

5. **"Communicating Sequential Processes"** — C.A.R. Hoare (1978)
   - Go 并发模型的理论基础
   - [PDF](https://spinroot.com/courses/summer/Papers/hoare_1978.pdf)

6. **"Fearless Concurrency"** — Niko Matsakis
   - Rust 并发安全性设计理念
   - [Rust Blog](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)

## 技术规范

### RFC 文档

| RFC | 标题 | 相关模块 |
|-----|------|----------|
| [RFC 1951](https://www.rfc-editor.org/rfc/rfc1951) | DEFLATE Compressed Data Format Specification | gzip |
| [RFC 1952](https://www.rfc-editor.org/rfc/rfc1952) | GZIP File Format Specification | gzip |
| [RFC 3629](https://www.rfc-editor.org/rfc/rfc3629) | UTF-8, a transformation format of Unicode | dos2unix |

### POSIX 标准

- [POSIX.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/) — 系统接口规范
- [Open Group Base Specifications](https://pubs.opengroup.org/onlinepubs/7908799/) — Shell 和工具规范

## 经典书籍

| 书籍 | 作者 | 主题 |
|------|------|------|
| "The C Programming Language" | Kernighan & Ritchie | 系统编程基础 |
| "Advanced Programming in the UNIX Environment" | W. Richard Stevens | Unix 系统编程 |
| "Operating Systems: Three Easy Pieces" | Remzi Arpaci-Dusseau | 操作系统原理 |
| "Programming Rust" | Jim Blandy, Jason Orendorff | Rust 实战 |
| "The Go Programming Language" | Donovan & Kernighan | Go 实战 |

## 在线资源

### Rust

- [Rust Documentation](https://doc.rust-lang.org/) — 官方文档门户
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — 实例学习
- [Rustlings](https://github.com/rust-lang/rustlings) — 官方练习题

### Go

- [Go Documentation](https://go.dev/doc/) — 官方文档
- [Go by Example](https://gobyexample.com/) — 实例学习
- [A Tour of Go](https://go.dev/tour/) — 快速入门

### 终端与 TUI

- [Terminal Guide](https://terminalguide.namepedia.de/) — 终端控制序列参考
- [Text-User Interface Design](https://www.linusakesson.net/programming/tui/) — TUI 设计原则

## 引用格式

如需在学术写作中引用本项目，建议使用以下格式：

```bibtex
@misc{byot2025,
  author = {LessUp},
  title = {Build Your Own Tools: A Systems Programming Learning Repository},
  year = {2025},
  publisher = {GitHub},
  url = {https://github.com/LessUp/build-your-own-tools}
}
```

---

> **贡献**：如果你发现其他值得引用的资源，欢迎通过 [GitHub Issues](https://github.com/LessUp/build-your-own-tools/issues) 提交建议。