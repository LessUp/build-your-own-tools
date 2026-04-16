# Documentation

> Complete documentation for build-your-own-tools

**English** | [简体中文](README.zh-CN.md)

---

## Welcome

Welcome to the build-your-own-tools documentation. This documentation provides comprehensive guides, API references, and tutorials to help you understand, build, and contribute to the project.

## Documentation Structure

```
docs/
├── README.md              # This file - Documentation index
├── README.zh-CN.md        # 中文文档索引
├── en/                    # English Documentation
│   ├── ARCHITECTURE.md    # System architecture and design
│   ├── COMPARISON.md      # Rust vs Go comparison
│   ├── API.md            # API reference
│   └── GETTING-STARTED.md # Quick start guide
├── zh-CN/                 # 简体中文文档
│   ├── ARCHITECTURE.md    # 系统架构
│   ├── COMPARISON.md      # Rust vs Go 对比
│   ├── API.md            # API 参考
│   └── GETTING-STARTED.md # 快速开始
└── changelogs/            # Changelog resources
    ├── INDEX.md          # All changelogs index
    ├── INDEX.zh-CN.md    # 变更日志索引
    └── MIGRATION.md      # Version migration guide
```

## Getting Started

New to build-your-own-tools? Start here:

1. **[Getting Started](en/GETTING-STARTED.md)** - Set up your environment and build your first tool
2. **[Architecture](en/ARCHITECTURE.md)** - Understand the system design and project structure
3. **[Comparison](en/COMPARISON.md)** - Learn the differences between Rust and Go implementations

## API Reference

Looking for function documentation? Check the API reference:

- **[API Reference](en/API.md)** - Complete API documentation for all library modules

## Changelogs

Track project changes and migrations:

- **[Changelog Index](changelogs/INDEX.md)** - Index of all changelogs
- **[Migration Guide](changelogs/MIGRATION.md)** - Version migration instructions

## Language Selection / 语言选择

Choose your preferred language:

- 🇺🇸 **English** - [Architecture](en/ARCHITECTURE.md) | [Comparison](en/COMPARISON.md) | [API](en/API.md) | [Getting Started](en/GETTING-STARTED.md)
- 🇨🇳 **简体中文** - [架构指南](zh-CN/ARCHITECTURE.md) | [语言对比](zh-CN/COMPARISON.md) | [API参考](zh-CN/API.md) | [快速开始](zh-CN/GETTING-STARTED.md)

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

**Last Updated**: 2026-04-16  
**Documentation Version**: 2.0
