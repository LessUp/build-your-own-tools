# Contributing Guide

Thank you for your interest in contributing to Build-Your-Own-Tools! We welcome contributions of all kinds.

**English** | [简体中文](CONTRIBUTING.zh-CN.md)

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Spec-Driven Development](#spec-driven-development)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Commit Conventions](#commit-conventions)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

This project adheres to our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

1. Search the [Issues](https://github.com/LessUp/build-your-own-tools/issues) to see if the bug has already been reported
2. If not, create a new issue using the Bug Report template
3. Provide detailed reproduction steps and environment information

### Suggesting Features

1. Search existing issues for similar suggestions
2. Create a new issue using the Feature Request template
3. Describe the use case and expected behavior

### Submitting Code

1. Fork the repository
2. Create a feature branch
3. Write code and tests
4. Submit a Pull Request

## Spec-Driven Development

This project follows **Spec-Driven Development (SDD)** methodology. All implementations must be based on the specification documents in the `/specs` directory.

### Spec Directory Structure

```
specs/
├── product/        # Product requirements and acceptance criteria
├── rfc/            # Technical design documents (RFCs)
├── api/            # API/CLI interface definitions
├── db/             # Data model specifications
└── testing/        # BDD test case specifications
```

### AI Agent Workflow

When developing new features, modifying existing functionality, or fixing bugs:

1. **Review Specs First**: Read relevant product docs, RFCs, and API definitions in `/specs`
2. **Spec-First Update**: For new features or interface changes, propose spec modifications first
3. **Implementation**: Code must 100% comply with specs (no gold-plating)
4. **Test Verification**: Write tests based on acceptance criteria in `/specs/testing/`

See [AGENTS.md](AGENTS.md) for the complete SDD workflow.

## Development Workflow

### Environment Setup

```bash
# Clone your fork
git clone https://github.com/<your-username>/build-your-own-tools.git
cd build-your-own-tools

# Add upstream repository
git remote add upstream https://github.com/LessUp/build-your-own-tools.git

# Install dependencies
# Rust: https://rustup.rs/
# Go: https://golang.org/dl/
```

### Creating a Branch

```bash
# Sync with upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/your-feature-name
```

### Local Testing

```bash
# Rust projects
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all

# Go projects
gofmt -w .
go vet ./...
go test ./...
```

## Code Standards

### Rust

- Use `rustfmt` for code formatting
- Use `clippy` for static analysis
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Public APIs must have documentation

```rust
/// Converts CRLF line endings to LF
///
/// # Arguments
///
/// * `input` - Input string
///
/// # Returns
///
/// String with all CRLF replaced by LF
pub fn convert_crlf_to_lf(input: &str) -> String {
    input.replace("\r\n", "\n")
}
```

### Go

- Use `gofmt` for code formatting
- Use `go vet` for static analysis
- Follow [Effective Go](https://golang.org/doc/effective_go)
- Exported functions must have documentation

```go
// ConvertCRLFToLF converts Windows line endings to Unix line endings.
func ConvertCRLFToLF(input string) string {
    return strings.ReplaceAll(input, "\r\n", "\n")
}
```

### General Standards

- Use UTF-8 encoding
- Use LF line endings
- Preserve trailing newline at end of files
- Remove trailing whitespace

## Commit Conventions

We use [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation updates
- `style`: Code formatting (no functional change)
- `refactor`: Refactoring (not a feature or bug fix)
- `perf`: Performance optimization
- `test`: Test related
- `chore`: Build/tooling related

### Scope

- `dos2unix`: dos2unix sub-project
- `gzip`: gzip sub-project
- `htop`: htop sub-project
- `ci`: CI/CD related
- `docs`: Documentation related
- `specs`: Specifications related

### Examples

```
feat(dos2unix): add support for recursive directory processing

Add -r/--recursive flag to process all files in a directory tree.

Closes #123
```

```
fix(gzip): handle empty input files correctly

Previously, empty files would cause a panic. Now they are handled
gracefully with an appropriate error message.
```

## Pull Request Process

### Pre-submission Checklist

- [ ] Code passes all tests
- [ ] Code passes formatting checks
- [ ] Code passes lint checks
- [ ] Related documentation is updated
- [ ] Changelog entries added (if applicable)

### PR Description

Please include in your PR description:

1. Purpose and background for the changes
2. Summary of changes
3. Testing methodology
4. Related issues (if any)

### Code Review

- All PRs require review by at least one maintainer
- Please respond to review comments promptly
- Update code as needed based on feedback

### Merging

- PRs are merged by maintainers after review
- Use Squash and Merge to keep history clean

## 📝 Changelog

For feature changes, please add entries to the corresponding sub-project's `changelog/` directory:

```markdown
## [Unreleased]
### Added
- Brief description of the change

### Fixed
- Brief description of the fix
```

## 🙏 Thank You

Thank you for taking the time to read this guide. We look forward to your contributions!

---

## 中文版本

请访问 [CONTRIBUTING.zh-CN.md](CONTRIBUTING.zh-CN.md) 查看中文版本的贡献指南。
