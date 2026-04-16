# Documentation

> Complete documentation for build-your-own-tools

**English** | [简体中文](README.zh-CN.md)

---

## Welcome

Welcome to the build-your-own-tools documentation. This documentation provides comprehensive guides, API references, and tutorials to help you understand, build, and contribute to the project.

## Documentation Structure

Following [GitHub Community Best Practices](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file):

```
docs/
├── README.md                 # This file - Documentation index
├── README.zh-CN.md           # 中文文档索引
├── setup/                    # Environment setup guides
│   ├── GETTING-STARTED.md    # Quick start guide (EN)
│   └── GETTING-STARTED.zh-CN.md  # 快速开始（中文）
├── tutorials/                # User tutorials and comparisons
│   ├── COMPARISON.md         # Rust vs Go comparison (EN)
│   └── COMPARISON.zh-CN.md   # Rust vs Go 对比（中文）
├── architecture/             # High-level architecture specs
│   ├── ARCHITECTURE.md       # System architecture (EN)
│   ├── ARCHITECTURE.zh-CN.md # 系统架构（中文）
│   ├── API.md                # API reference (EN)
│   └── API.zh-CN.md          # API 参考（中文）
├── assets/                   # Images, UML diagrams, static resources
└── changelogs/               # Changelog resources
    ├── INDEX.md              # All changelogs index
    ├── INDEX.zh-CN.md        # 变更日志索引
    └── MIGRATION.md          # Version migration guide
```

## Getting Started

New to build-your-own-tools? Start here:

1. **[Getting Started](setup/GETTING-STARTED.md)** - Set up your environment and build your first tool
2. **[Architecture](architecture/ARCHITECTURE.md)** - Understand the system design and project structure
3. **[Comparison](tutorials/COMPARISON.md)** - Learn the differences between Rust and Go implementations

## API Reference

Looking for function documentation? Check the API reference:

- **[API Reference](architecture/API.md)** - Complete API documentation for all library modules

## Changelogs

Track project changes and migrations:

- **[Changelog Index](changelogs/INDEX.md)** - Index of all changelogs
- **[Migration Guide](changelogs/MIGRATION.md)** - Version upgrade instructions

## Language Selection / 语言选择

Choose your preferred language:

- 🇺🇸 **English** - [Architecture](architecture/ARCHITECTURE.md) | [Comparison](tutorials/COMPARISON.md) | [API](architecture/API.md) | [Getting Started](setup/GETTING-STARTED.md)
- 🇨🇳 **简体中文** - [架构指南](architecture/ARCHITECTURE.zh-CN.md) | [语言对比](tutorials/COMPARISON.zh-CN.md) | [API参考](architecture/API.zh-CN.md) | [快速开始](setup/GETTING-STARTED.zh-CN.md)

## Quick Links

### Project Links

- [GitHub Repository](https://github.com/LessUp/build-your-own-tools)
- [Live Documentation](https://lessup.github.io/build-your-own-tools/)
- [CHANGELOG.md](../CHANGELOG.md) - Root changelog
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guide
- [LICENSE](../LICENSE) - MIT OR Apache-2.0 License

### Sub-Projects

| Project | Path | Description |
|---------|------|-------------|
| dos2unix | `dos2unix/` | CRLF to LF converter |
| gzip | `gzip/go/`, `gzip/rust/` | File compression tool |
| htop | `htop/` | System monitor TUI |

### External Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Go Documentation](https://golang.org/doc/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)

---

## Contributing to Documentation

We welcome documentation improvements! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on:

- Reporting documentation issues
- Suggesting improvements
- Submitting documentation PRs
- Translation contributions

---

**Last Updated**: 2026-04-17  
**Documentation Version**: 2.1
