# Documentation Strategy

This document describes the documentation maintenance strategy for the Build Your Own Tools project.

## Documentation Architecture

```mermaid
graph TB
    subgraph "User Documentation"
        A[VitePress Site]
        A --> B[Whitepaper]
        A --> C[Technical Specs]
        A --> D[Comparison Studies]
        A --> E[Engineering Practices]
    end
    
    subgraph "API Documentation"
        F[rustdoc]
        G[godoc]
    end
    
    subgraph "Change Documentation"
        H[CHANGELOG.md]
        I[OpenSpec Changes]
    end
    
    subgraph "Governance Documentation"
        J[AGENTS.md]
        K[CLAUDE.md]
        L[README.md]
    end
    
    style A fill:#f59e0b,color:#fff
    style F fill:#3b82f6,color:#fff
    style G fill:#06b6d4,color:#fff
```

## VitePress Site

### Directory Structure

```
docs/
├── index.md              # Homepage (whitepaper style)
├── whitepaper/           # Whitepaper chapters
│   ├── index.md
│   ├── overview.md
│   ├── architecture.md
│   ├── decisions.md
│   └── performance.md
├── specs/                # Technical Specifications
│   ├── index.md
│   ├── openspec-workflow.md
│   ├── dos2unix.md
│   ├── gzip.md
│   └── htop.md
├── comparison/           # Comparison Studies
│   ├── index.md
│   ├── memory.md
│   ├── concurrency.md
│   ├── errors.md
│   └── benchmarks.md
├── engineering/          # Engineering Practices
│   ├── index.md
│   ├── ai-collaboration.md
│   ├── cicd.md
│   └── documentation.md
└── en/                   # English version (symlink)
    ├── whitepaper -> ../whitepaper
    ├── specs -> ../specs
    └── ...
```

### Configuration Highlights

```typescript
// .vitepress/config.mts
export default defineConfig({
  // Mermaid diagram support
  markdown: {
    config: (md) => {
      // Custom markdown configuration
    }
  },
  
  // Search functionality
  themeConfig: {
    search: { provider: 'local' }
  },
  
  // LLM-friendly output
  vite: {
    plugins: [llmstxt()]
  }
})
```

## API Documentation

### Rust (rustdoc)

```bash
# Generate documentation
cargo doc --open --no-deps

# Generate in CI
cargo doc --no-deps --document-private-items
```

Documentation comment example:

```rust
/// Line ending converter
/// 
/// # Examples
/// 
/// ```
/// use dos2unix::Dos2Unix;
/// 
/// let converter = Dos2Unix::new();
/// let result = converter.convert("hello\r\nworld")?;
/// assert_eq!(result, "hello\nworld");
/// ```
pub struct Dos2Unix {
    // ...
}

impl Dos2Unix {
    /// Create a new converter
    /// 
    /// # Arguments
    /// 
    /// * `keep_bom` - Whether to preserve UTF-8 BOM
    pub fn new() -> Self {
        Self { keep_bom: false }
    }
}
```

### Go (godoc)

```bash
# View locally
go doc -all ./...

# Generate HTML
godoc -http=:6060
```

Documentation comment example:

```go
// Package dos2unix provides line ending conversion functionality.
// 
// Example:
//
//	converter := dos2unix.New()
//	result, err := converter.Convert("hello\r\nworld")
package dos2unix

// Converter handles line ending conversions.
type Converter struct {
	keepBOM bool
}

// New creates a new Converter instance.
//
// Example:
//
//	c := dos2unix.New()
func New() *Converter {
	return &Converter{keepBOM: false}
}

// Convert transforms line endings in the input data.
//
// Returns an error if the input contains invalid UTF-8 sequences.
func (c *Converter) Convert(input string) (string, error) {
	// ...
}
```

## Change Log

### Format (Keep a Changelog)

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New feature description

### Changed
- Change description

### Fixed
- Bug fix description

## [0.3.0] - 2025-05-14

### Added
- Add htop Windows Go implementation
- Add Mermaid diagram support to documentation

### Changed
- Upgrade VitePress to 1.5.0

### Fixed
- Fix broken symlink in en/htop/unix/go

## [0.2.0] - 2025-03-01

### Added
- Add gzip Rust implementation
- Add gzip Go implementation

[Unreleased]: https://github.com/LessUp/build-your-own-tools/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/LessUp/build-your-own-tools/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/LessUp/build-your-own-tools/compare/v0.1.0...v0.2.0
```

### Automated Generation

```yaml
# .github/workflows/release.yml
- name: Generate Changelog
  uses: orhun/git-cliff-action@v2
  with:
    config: cliff.toml
    args: --verbose
```

## Versioning Strategy

```mermaid
graph LR
    A[Semantic Versioning] --> B[MAJOR]
    A --> C[MINOR]
    A --> D[PATCH]
    
    B --> E[Breaking Changes]
    C --> F[New Features]
    D --> G[Bug Fixes]
    
    style A fill:#f59e0b,color:#fff
```

### Version Branches

| Branch | Purpose |
|--------|---------|
| `main` | Stable releases |
| `develop` | Features in development |
| `feature/*` | Feature branches |
| `hotfix/*` | Emergency fixes |

## Documentation Sync

### Mermaid Diagrams

Use Mermaid to embed diagrams in Markdown:

````markdown
```mermaid
graph TB
    A --> B
    B --> C
```
````

### Code Blocks

Use language identifiers for syntax highlighting:

````markdown
```rust
fn main() {
    println!("Hello");
}
```

```go
func main() {
    fmt.Println("Hello")
}
```
````

## Release Checklist

```markdown
## Pre-release Checklist

- [ ] All tests pass
- [ ] Update CHANGELOG.md
- [ ] Update version numbers (Cargo.toml, package.json)
- [ ] Generate API documentation
- [ ] Build documentation site
- [ ] Create Git tag
- [ ] Push tag to trigger Release
```

## Related Documents

- [Engineering Practices Overview](/engineering/) — Engineering Overview
- [CI/CD Design](/engineering/cicd) — Automated Workflow
- [System Architecture](/whitepaper/architecture) — Architecture Design
