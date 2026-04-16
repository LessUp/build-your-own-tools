# Product Requirements Document (PRD)

> Build-Your-Own-Tools 项目产品需求文档

| Metadata | Value |
|----------|-------|
| **Version** | 1.0 |
| **Status** | Active |
| **Last Updated** | 2026-04-17 |

---

## 1. Project Overview

### 1.1 Project Vision

**Build-Your-Own-Tools** 是一个以学习为中心的开源项目，使用 **Rust** 和 **Go** 从零开始重新实现常见的 CLI 工具。

### 1.2 Target Users

- 系统编程学习者
- Rust/Go 语言学习者
- 想要理解底层实现原理的开发者
- 对跨语言对比感兴趣的技术爱好者

### 1.3 Core Value Proposition

- **学习优先**：代码可读性优先于微优化
- **多语言对比**：相同功能用 Rust 和 Go 实现，便于对比学习
- **生产级质量**：完整的测试、CI/CD、文档体系
- **渐进式学习**：从简单工具到复杂工具的递进路径

---

## 2. Feature Requirements

### REQ-1: Project Documentation System

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| REQ-1.1 | Root README.md includes badges, feature list, quick start | High | ✅ Done |
| REQ-1.2 | CONTRIBUTING.md contribution guidelines | High | ✅ Done |
| REQ-1.3 | CODE_OF_CONDUCT.md code of conduct | Medium | ✅ Done |
| REQ-1.4 | SECURITY.md security policy | Medium | ✅ Done |
| REQ-1.5 | CHANGELOG.md version history (Keep a Changelog format) | High | ✅ Done |
| REQ-1.6 | Each sub-project has standardized README.md | Medium | 🔄 In Progress |
| REQ-1.7 | README.md defaults to English with CN link | High | 🔄 In Progress |

### REQ-2: CI/CD Pipeline

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| REQ-2.1 | CI workflow includes format checks (cargo fmt, gofmt) | High | ✅ Done |
| REQ-2.2 | CI workflow includes lint checks (clippy, go vet) | High | ✅ Done |
| REQ-2.3 | CI workflow includes unit tests | High | ✅ Done |
| REQ-2.4 | CI workflow supports cross-platform builds | High | ✅ Done |
| REQ-2.5 | Release workflow auto-builds and publishes binaries | High | ✅ Done |
| REQ-2.6 | Pages workflow deploys documentation site | Medium | ✅ Done |

### REQ-3: Code Quality

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| REQ-3.1 | .editorconfig editor unified configuration | Medium | ✅ Done |
| REQ-3.2 | rustfmt.toml Rust formatting configuration | Medium | ✅ Done |
| REQ-3.3 | .golangci.yml Go lint configuration | Medium | ✅ Done |
| REQ-3.4 | All code passes `cargo clippy -- -D warnings` | High | ✅ Done |
| REQ-3.5 | All code passes `cargo fmt --check` | High | ✅ Done |

### REQ-4: Sub-Project Standards

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| REQ-4.1 | Each sub-project has README.md | Medium | 🔄 In Progress |
| REQ-4.2 | Each sub-project has changelog/CHANGELOG.md | High | ✅ Done |
| REQ-4.3 | Each sub-project has test files | High | ✅ Done |
| REQ-4.4 | Sub-projects follow language naming conventions | High | ✅ Done |

### REQ-5: Documentation Site

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| REQ-5.1 | VitePress documentation site configuration | Medium | ✅ Done |
| REQ-5.2 | docs/ directory architecture documentation | High | ✅ Done |
| REQ-5.3 | Rust vs Go comparison document | Medium | ✅ Done |

---

## 3. Tool Requirements

### TOOL-1: dos2unix

**Purpose**: CRLF to LF line ending converter  
**Complexity**: ⭐ (Beginner)  
**Languages**: Rust only

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| TOOL-1.1 | Convert CRLF line endings to LF | High | ✅ Done |
| TOOL-1.2 | Support streaming processing for large files | High | ✅ Done |
| TOOL-1.3 | Check mode (detect CRLF without modification) | Medium | ✅ Done |
| TOOL-1.4 | stdin/stdout support for pipeline usage | Medium | ✅ Done |
| TOOL-1.5 | Handle CRLF at buffer boundaries correctly | High | ✅ Done |

### TOOL-2: gzip

**Purpose**: File compression/decompression tool  
**Complexity**: ⭐⭐ (Intermediate)  
**Languages**: Rust, Go

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| TOOL-2.1 | Compress files using DEFLATE algorithm | High | ✅ Done |
| TOOL-2.2 | Decompress .gz files | High | ✅ Done |
| TOOL-2.3 | Support compression levels (0-9) | Medium | ✅ Done |
| TOOL-2.4 | stdin/stdout support | Medium | ✅ Done |
| TOOL-2.5 | Keep source file option (-k) | Low | ✅ Done |
| TOOL-2.6 | Recursive directory processing (-r) [Go only] | Low | ✅ Done |
| TOOL-2.7 | Parallel file processing [Go only] | Medium | ✅ Done |

### TOOL-3: htop

**Purpose**: Interactive system monitoring TUI  
**Complexity**: ⭐⭐⭐ (Advanced)  
**Languages**: Rust (Unix, Windows), Go (Windows)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| TOOL-3.1 | Display process list with PID, name, CPU, memory | High | ✅ Done |
| TOOL-3.2 | Real-time CPU and memory monitoring | High | ✅ Done |
| TOOL-3.3 | Process sorting by column | Medium | ✅ Done |
| TOOL-3.4 | Process search/filter | Medium | ✅ Done |
| TOOL-3.5 | Kill process functionality | Medium | ✅ Done |
| TOOL-3.6 | Adjustable refresh interval | Low | ✅ Done |
| TOOL-3.7 | Sparkline history visualization [Windows only] | Low | ✅ Done |

---

## 4. Quality Attributes

### 4.1 Performance

- dos2unix: > 100 MB/s throughput on large files
- gzip: Compression speed > 50 MB/s
- htop: < 1% CPU overhead, stable 60 FPS rendering

### 4.2 Reliability

- All unit tests pass on all supported platforms
- No known critical bugs
- Graceful error handling with informative messages

### 4.3 Maintainability

- Code coverage > 80%
- All public APIs documented
- Consistent code style across all projects

### 4.4 Portability

- Support Linux (x86_64)
- Support macOS (x86_64, ARM64)
- Support Windows (x86_64)

---

## 5. Roadmap

### 2026 Q2

- [ ] New tool: `cat` implementation
- [ ] New tool: `wc` (word count)
- [ ] Enhanced documentation with tutorials

### 2026 Q3

- [ ] htop network monitoring
- [ ] Disk I/O monitoring
- [ ] Plugin system exploration

### Future

- [ ] Container-aware process grouping
- [ ] Remote system monitoring
- [ ] Additional language implementations (Zig?)

---

## Status Legend

- ✅ Done
- 🔄 In Progress
- ❌ Not Started
- ⏸️ On Hold

---

**Last Updated**: 2026-04-17
