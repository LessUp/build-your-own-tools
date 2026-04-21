# Changelog Index

> Index of all changelogs in the build-your-own-tools project

**English** | [简体中文](INDEX.zh-CN.md)

---

## Overview

This project maintains changelogs at multiple levels:

- **Root Changelog**: Project-wide changes
- **Sub-project Changelogs**: Individual tool changes

All changelogs follow the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## Changelog Locations

### Root Level

| File | Description | Last Updated |
|------|-------------|--------------|
| [CHANGELOG.md](../../CHANGELOG.md) | Project-wide changes and releases | 2026-04-16 |

### Sub-project Changelogs

#### dos2unix

| File | Description | Current Version |
|------|-------------|-----------------|
| [dos2unix/changelog/CHANGELOG.md](../../dos2unix/changelog/CHANGELOG.md) | CRLF converter changes | v0.2.1 |

#### gzip

| Implementation | File | Current Version |
|----------------|------|-----------------|
| Go | [gzip/go/changelog/CHANGELOG.md](../../gzip/go/changelog/CHANGELOG.md) | v0.3.0 |
| Rust | [gzip/rust/changelog/CHANGELOG.md](../../gzip/rust/changelog/CHANGELOG.md) | v0.1.1 |

#### htop

| Implementation | File | Current Version |
|----------------|------|-----------------|
| Unix/Rust | [htop/unix/rust/changelog/CHANGELOG.md](../../htop/unix/rust/changelog/CHANGELOG.md) | v0.1.5 |
| Windows/Rust | [htop/win/rust/changelog/CHANGELOG.md](../../htop/win/rust/changelog/CHANGELOG.md) | v0.1.0 |
| Windows/Go | [htop/win/go/changelog/CHANGELOG.md](../../htop/win/go/changelog/CHANGELOG.md) | v0.1.3 |
| Shared | [htop/changelog/CHANGELOG.md](../../htop/changelog/CHANGELOG.md) | v0.1.0 |

---

## Version History

### Root Project Versions

```
v0.2.0 (2026-03-13)  ───┬───  Infrastructure improvements
                        │
v0.1.1 (2026-03-10)  ───┼───  Workflow standardization
                        │
v0.1.0 (2026-02-13)  ───┘───  Initial release
```

### Sub-project Latest Versions

| Project | Version | Status |
|---------|---------|--------|
| dos2unix | v0.2.1 | ✅ Stable |
| gzip/go | v0.3.0 | ✅ Stable |
| gzip/rust | v0.1.1 | ✅ Stable |
| htop/unix/rust | v0.1.5 | ✅ Stable |
| htop/win/rust | v0.1.0 | ✅ Stable |
| htop/win/go | v0.1.3 | ✅ Stable |

---

## Changelog Categories

### Change Types

All changelogs use the following categories:

- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security improvements
- **Performance** - Performance improvements
- **Documentation** - Documentation changes
- **Infrastructure** - CI/CD and tooling changes

### Version Format

```
## [VERSION] - YYYY-MM-DD

### Category

- Change description with details
  - Additional context
  - Breaking change indicators
```

---

## Migration Guides

See [MIGRATION.md](MIGRATION.md) for detailed migration instructions between versions.

### Quick Migration Links

| From Version | To Version | Guide |
|--------------|------------|-------|
| v0.1.x | v0.2.0 | [Migration Guide](MIGRATION.md#v01x-to-v020) |
| Initial | v0.1.0 | [Getting Started](../setup/GETTING-STARTED.md) |

---

## Contributing to Changelogs

When contributing changes:

1. Add entries to the appropriate `Unreleased` section
2. Use the correct category (Added/Changed/Fixed/etc.)
3. Include references to issues/PRs when applicable
4. Mark breaking changes clearly

Example:
```markdown
### Added

- **dos2unix**: New `--check` flag for detecting CRLF without modification
  - Returns exit code 2 when CRLF found
  - Useful for CI/CD pipelines ([#123](../../issues/123))

### Fixed

- **htop**: Fixed memory leak in process refresh loop
  - **BREAKING**: Changed refresh API signature
```

---

## Release Schedule

| Release Type | Frequency | Description |
|--------------|-----------|-------------|
| Major | As needed | Breaking changes |
| Minor | Monthly | New features |
| Patch | Bi-weekly | Bug fixes |

---

## Related Resources

- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Migration Guide](MIGRATION.md)

---

**Last Updated**: 2026-04-16  
**Version**: 1.0
