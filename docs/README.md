# Documentation

> Supporting docs for the durable parts of Build Your Own Tools.

**English** | [简体中文](README.zh-CN.md)

## Documentation structure

```text
docs/
├── README.md
├── README.zh-CN.md
├── setup/
├── tutorials/
└── architecture/
```

## Read in this order

1. **[Getting Started](setup/GETTING-STARTED.md)** - set up the toolchains and run the project
2. **[Architecture](architecture/ARCHITECTURE.md)** - understand repository layout and engineering surfaces
3. **[Comparison](tutorials/COMPARISON.md)** - compare Rust and Go implementation choices
4. **[API Reference](architecture/API.md)** - inspect the documented program surfaces when you need more detail

## Canonical project surfaces

- [README.md](../README.md) - repository overview and navigation
- [index.md](../index.md) - Pages landing page
- [CHANGELOG.md](../CHANGELOG.md) - project release history
- [CONTRIBUTING.md](../CONTRIBUTING.md) - contribution and collaboration rules

## Notes

- `docs/` should explain durable topics, not mirror every release note or temporary cleanup decision.
- If a page mainly repeats README, Pages, or the root changelog, it should be pruned or merged.
