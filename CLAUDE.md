# CLAUDE.md

## Working principles

Claude's role in this repository: **cross-file reasoning, process design, governance coherence, and high-context problem-solving.**

This is a **close-out / archive-ready** repository. All decisions prioritize **clarity, reduced maintenance, and stabilization** over adding new complexity.

## How Claude works here: first steps

1. **Read the governance layer first**
   - Run `openspec list` to find any active phase change.
   - Read `.github/copilot-instructions.md` for project-domain quick reference (tool structure, language stacks, conventions).
   - Read `AGENTS.md` for shared workflow rules.

2. **Understand the phase context**
   - If an active phase change exists, read its `proposal.md`, `design.md`, and `tasks.md` before starting.
   - Work **within** the phase scope; do not invent parallel changes.
   - If no active change exists and your work changes repo behavior, create the phase change first.

3. **Never treat archived changes as active**
   - Archived changes under `openspec/changes/archive/` are historical reference only.
   - The **current active phase** is always the authoritative scope.

## Repository-specific Claude reasoning

## Archive-ready checklist

Before declaring the repository archive-ready, verify:

- [x] No duplicate changelog files (only root `CHANGELOG.md`)
- [x] No duplicate CI workflows (removed `htop/.github/workflows/ci.yml`)
- [x] All `checkout` actions use the same version (v5)
- [x] VitePress navigation has fewer than 12 top-level items
- [x] No dead links in documentation
- [x] `make lint-all` passes
- [x] `make test-all` passes
- [x] `npm run docs:build` succeeds

## Validation commands

```bash
make lint-all
make test-all
npm run docs:check
npm run docs:build
```

**Never claim a fix or feature is "done" without running the relevant checks.**

### When to request `/review`

Before merging any of:
- Workflow, CI/CD, or security changes (high cascade risk)
- Cross-file refactoring or governance changes (consistency risk)
- Documentation architecture or Pages rewrites (user experience risk)
- Changes to AGENTS.md, CLAUDE.md, or copilot-instructions.md (governance layer risk)

### When Claude should defer

- Local or machine-specific preferences belong in `CLAUDE.local.md`, not here.
- Shared project rules belong in AGENTS.md or project-level documentation.
- User authentication, secrets, or sensitive config: **never commit; always flag**.

## Remote work and sub-agent prompting

When delegating to other agents (e.g., code-reviewer, explore, general-purpose):

1. **Provide complete context** (do not rely on them reading this session's history).
2. **Quote the relevant phase change** if one is active.
3. **State the validation gates** (e.g., "verify with `npm run docs:build`").
4. **Specify the expected output** (e.g., "return a list of issues by severity").

Example template:

```
Task: Review Phase 2 workflow changes
Context: Active change is phase-2-github-workflow-alignment
Scope: Simplified .github/workflows/security.yml, narrowed ci.yml triggers
Validation: Confirm changes pass make lint-all and do not break GitHub Actions syntax
Expected output: Issues categorized as Critical/Important/Minor with reproduction steps
```
