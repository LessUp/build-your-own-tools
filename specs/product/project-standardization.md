# Product Requirements: Project Standardization

## Introduction

This document defines the product requirements for the Build-Your-Own-Tools project. The project is a learning-focused repository for re-implementing common CLI tools from scratch in **Rust** and **Go**.

## Requirements

### REQ-1: Project Documentation System

| ID | Requirement | Status |
|----|-------------|--------|
| REQ-1.1 | Root README.md includes badges, feature list, quick start | ✅ |
| REQ-1.2 | CONTRIBUTING.md contribution guidelines | ✅ |
| REQ-1.3 | CODE_OF_CONDUCT.md code of conduct | ✅ |
| REQ-1.4 | SECURITY.md security policy | ✅ |
| REQ-1.5 | CHANGELOG.md version history (Keep a Changelog format) | ✅ |
| REQ-1.6 | Each sub-project has standardized README.md | 🔄 |

### REQ-2: CI/CD Pipeline

| ID | Requirement | Status |
|----|-------------|--------|
| REQ-2.1 | CI workflow includes format checks (cargo fmt, gofmt) | ✅ |
| REQ-2.2 | CI workflow includes lint checks (clippy, go vet) | ✅ |
| REQ-2.3 | CI workflow includes unit tests | ✅ |
| REQ-2.4 | CI workflow supports cross-platform builds | ✅ |
| REQ-2.5 | Release workflow auto-builds and publishes binaries | ✅ |
| REQ-2.6 | Pages workflow deploys documentation site | ✅ |

### REQ-3: Code Quality

| ID | Requirement | Status |
|----|-------------|--------|
| REQ-3.1 | .editorconfig editor统一 configuration | ✅ |
| REQ-3.2 | rustfmt.toml Rust formatting configuration | ✅ |
| REQ-3.3 | .golangci.yml Go lint configuration | ✅ |
| REQ-3.4 | All code passes `cargo clippy -- -D warnings` | ✅ |
| REQ-3.5 | All code passes `cargo fmt --check` | ✅ |

### REQ-4: Sub-Project Standards

| ID | Requirement | Status |
|----|-------------|--------|
| REQ-4.1 | Each sub-project has README.md | 🔄 |
| REQ-4.2 | Each sub-project has changelog/CHANGELOG.md | ✅ |
| REQ-4.3 | Each sub-project has test files | ✅ |
| REQ-4.4 | Sub-projects follow language naming conventions | ✅ |

### REQ-5: Documentation Site

| ID | Requirement | Status |
|----|-------------|--------|
| REQ-5.1 | VitePress documentation site configuration | ✅ |
| REQ-5.2 | docs/ directory architecture documentation | ✅ |
| REQ-5.3 | Rust vs Go comparison document | ✅ |

## Status Legend

- ✅ Complete
- 🔄 In Progress
- ❌ Not Started
