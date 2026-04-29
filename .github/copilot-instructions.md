# GitHub Copilot instructions for this repository

**Start here if you're a fresh AI agent working on this repository.**

---

## Project snapshot

**Build Your Own Tools**: A learning repository where you rebuild three real CLI tools from scratch using Rust and/or Go.

- `dos2unix` - Rust implementation
- `gzip` - Rust + Go implementations
- `htop` - Rust (Unix/Windows) + Go (Windows)

**Current phase**: Close-out / archive-ready (final stabilization, not feature expansion).

**Tech stack overview**:
- **Rust**: workspace with `Cargo.toml`, shared logic, tool-specific crates
- **Go**: workspace with `go.work`, per-platform implementations
- **Docs**: VitePress site (`index.md` + `.vitepress/`), markdown docs in `docs/`
- **Config**: `.editorconfig`, `.golangci.yml`, `deny.toml`, `.markdownlint-cli2.yaml` (all tracked, no local overrides)
- **Verification**: `make lint-all`, `make test-all`, `npm run docs:check`, `npm run docs:build`

---

## Required workflow: The 5-step mandate

### 1. Check OpenSpec first

```bash
openspec list
```

- If an active phase change exists → read its `proposal.md`, `design.md`, `tasks.md`
- If no active change exists and your work changes repo behavior → **create the phase change first**
- **Never** treat archived changes as active scope; they are historical only

### 2. Understand the governance model

Read in this order:
1. `.github/copilot-instructions.md` (this file)
2. `AGENTS.md` (shared workflow rules for all tools)
3. `CLAUDE.md` (Claude-specific reasoning and conventions)
4. `openspec/specs/project/spec.md` (canonical requirements)

### 3. Make small, coherent edits

- One batch = related changes touching 2-4 files, addressing one clear goal
- Not: "refactor the whole `os` module"; instead: "fix broken error handling in os/unix.rs"
- Use `git diff` to confirm your batch stays focused

### 4. Run the validation gates

**For code/config changes:**
```bash
make lint-all    # Rust clippy, Go golangci-lint, cargo-deny, govulncheck
make test-all    # Full test suite
```

**For docs/site changes:**
```bash
npm run docs:check  # TypeScript, link checking, prose validation
npm run docs:build  # VitePress build to HTML
```

**Never commit without running the relevant checks.** Broken builds waste everyone's time.

### 5. Request review before merge

Use `/review` if your changes touch:
- `.github/workflows/` or any CI/CD logic
- `AGENTS.md`, `CLAUDE.md`, or governance files
- Documentation architecture or GitHub Pages
- Cross-file refactoring or config changes

---

## Key decisions & constraints

### Documentation model (non-negotiable)

| Surface | Purpose | Canonical |
| --- | --- | --- |
| `README.md` / `README.zh-CN.md` | Repo entry point, routing | YES |
| `index.md` / GitHub Pages | Landing page, value pitch | YES |
| `docs/architecture/` | Deep systems design | YES |
| `docs/setup/` | Getting started | YES |
| `docs/tutorials/` | Comparison, examples | YES |
| `CHANGELOG.md` | Release history | YES |
| Sub-project changelogs | Redundant; avoid | NO |

**Rule**: Do not maintain the same explanation across multiple surfaces. One explanation wins; others link to it.

### Tooling decisions (inform your work)

- **LSP**: Minimal repo-tracked config; editors use `.editorconfig`, `.vscode/settings.json` (see below)
- **MCP vs CLI Skills**: Prefer lightweight CLI skills; add MCP only if long-lived context or stateful interaction is essential
- **Copilot**: Use for execution, validation, and gh CLI integration; use `/review` for cross-cutting changes
- **Claude**: Use for reasoning about architecture, governance, and multi-file refactoring

### Close-out scope

✅ Finish:
- Bug fixes, version anchoring, workflow consolidation
- Documentation simplification and clarity
- Test coverage, security, and edge cases

❌ Avoid:
- New tools, major features, parallel governance systems
- Adding complexity; prefer removing it

---

## The three tools: what you're working with

### dos2unix

- **Rust only** (no Go)
- Converts line endings between DOS (CRLF) and Unix (LF)
- Structure: `dos2unix/rust/` with separate `dos2unix` crate
- Simple but good for learning I/O, error handling, platform abstractions

### gzip

- **Both Rust and Go**
- Compresses/decompresses using DEFLATE algorithm
- Structure: `gzip/rust/` and `gzip/go/` (separate implementations)
- More complex: requires understanding compression algorithms

### htop

- **Rust (Unix/Windows) + Go (Windows)**
- System process monitor with real-time terminal UI
- Structure: `htop/shared/` (Rust common), `htop/unix/rust/`, `htop/win/rust/`, `htop/win/go/`
- Most complex: terminal UI, platform abstractions, real-time data

---

## Common tasks: quick reference

### Add or fix a feature in a tool

1. Identify which tool/language: `dos2unix`, `gzip/{rust,go}`, or `htop/{unix,go,shared}`
2. Run relevant tests: `cargo test` (Rust) or `go test ./...` (Go)
3. Run `make lint-all` to catch style issues before committing
4. If cross-platform or architectural, open with `openspec` phase change

### Update documentation

1. Edit the canonical location (not a duplicate)
2. Run `npm run docs:check && npm run docs:build`
3. Verify links are not broken; site structure is correct
4. Request `/review` if architecture or Pages IA changed

### Change workflows or GitHub config

1. Create a phase change (usually `phase-N-github-alignment` pattern)
2. Edit `.github/workflows/` or repository metadata
3. Validate syntax: `gh workflow list`, test locally if possible
4. Request `/review` before merge

### Fix or add tests

1. Identify the failing/missing test
2. Run `make test-all` to see full test suite status
3. Add/fix the test; verify it passes
4. Run `make lint-all` to catch any linting issues

---

## Avoid these mistakes

❌ **Mistake**: Committing without running `make lint-all`, `make test-all`, or docs checks
→ **Why**: CI breaks; other agents/humans waste time; builds fail on merge

❌ **Mistake**: Treating an archived OpenSpec change as the active scope
→ **Why**: Work gets lost or conflicts when the real phase change is created; scope becomes incoherent

❌ **Mistake**: Maintaining the same explanation in README, docs, AND Pages landing page
→ **Why**: One gets updated; others become stale; readers get conflicting information

❌ **Mistake**: Adding MCP, plugins, or tooling without justifying why CLI skills won't work
→ **Why**: Token overhead increases; maintenance burden grows; the repo becomes less lean

❌ **Mistake**: Asking for help or creating PR without linking to the active OpenSpec change
→ **Why**: Reviewers don't understand scope; work gets mixed into wrong backlog; priorities become unclear

---

## When in doubt

1. Check `AGENTS.md` for shared workflow
2. Check `CLAUDE.md` for reasoning guidance
3. Check `.github/copilot-instructions.md` (this file) for project snapshot
4. Run `openspec list` and read the active phase change
5. Ask: *"Is this work inside the active phase change scope?"*

If the answer is no, create the phase change first.
