# README.GLM — Quick-Start Handoff Guide

**To GLM Model**: This document is your entry point. Read this first; it tells you everything you need to know to pick up Phase 4 execution.

---

## What you're inheriting

A **learning-focused CLI tools repository** in **close-out / archive-ready phase**.

Three tool implementations:
- **dos2unix** - Rust only (line-ending converter)
- **gzip** - Rust + Go implementations (DEFLATE compressor)
- **htop** - Rust (Unix/Windows) + Go (Unix/Windows) (system monitor)

Phases 1-3 have **stabilized the repository**: fixed documentation narrative, simplified GitHub workflows, unified AI agent governance.

**Your job (Phase 4)**: Complete the final backlog and archive the repository for long-term storage.

---

## How to get oriented (30-minute quickstart)

### Step 1: Understand the repository layout (5 min)

```
build-your-own-tools/
├── dos2unix/              # Rust implementation
├── gzip/
│   ├── rust/              # Rust implementation
│   └── go/                # Go implementation
├── htop/
│   ├── shared/            # Shared Rust logic
│   ├── unix/rust/         # Unix Rust UI
│   ├── win/rust/          # Windows Rust UI
│   └── win/go/            # Windows Go UI
├── docs/                  # VitePress markdown source
├── openspec/              # Governance & change tracking
├── README.md              # Repo entry point
├── index.md               # GitHub Pages home
├── Cargo.toml             # Rust workspace
├── go.work                # Go workspace
├── package.json           # Node/docs tooling
├── Makefile               # Validation and build commands
└── .github/
    ├── workflows/         # GitHub Actions CI/CD
    ├── copilot-instructions.md    # AI agent entry point
    └── ISSUE_TEMPLATE/    # GitHub forms
```

### Step 2: Read the governance layer (10 min)

In order:

1. `.github/copilot-instructions.md` — Your project domain quick reference
2. `AGENTS.md` — Shared workflow rules for all AI tools
3. `CLAUDE.md` — Claude-specific reasoning and conventions (if you are Claude)
4. `openspec/specs/project/spec.md` — Canonical requirements (durable, stable)

### Step 3: Check current status (5 min)

```bash
openspec list
# Expected: phase-4-closeout-backlog (0/24 tasks)

make lint-all
make test-all
npm run docs:check && npm run docs:build
# Expected: all passing
```

### Step 4: Read your mission (5 min)

```bash
cat openspec/changes/phase-4-closeout-backlog/BACKLOG.md
```

This file contains:
- Archive-readiness checklist (all green ✅)
- Must-finish tasks (A, B, C sections)
- Optional polish tasks (O section)
- Executor recommendations
- Handoff gate criteria

---

## Your mandate (Phase 4)

**Goal**: Move the repository to "archive-ready" state and hand off to whoever makes the archival decision.

**Scope**:
- ✅ Complete all Section A tasks (verification, no drift, archive the phase)
- ✅ Verify all Section B tasks (docs coherent, links active)
- ✅ Verify all Section C checks (GitHub metadata aligned, Pages live)
- ⏭️ Skip Section O tasks unless specifically requested

**Success criteria**:
- All must-finish tasks in A-C are done
- `make lint-all && make test-all && npm run docs:check && npm run docs:build` passes
- No open blockers
- `openspec list` returns "No active changes found"
- Code review sign-off received

---

## Daily workflow

### Every morning (or start of session)

```bash
# 1. Check OpenSpec status
openspec list

# 2. Read the active phase change's tasks
cat openspec/changes/phase-4-closeout-backlog/tasks.md
cat openspec/changes/phase-4-closeout-backlog/BACKLOG.md

# 3. Pick the next unfinished task from Section A
# (then Section B, then Section C)
```

### For each task

1. Create a git branch: `git checkout -b phase-4-task-<task-id>`
2. Implement the task following the rules in `.github/copilot-instructions.md`
3. Run validation: `make lint-all && make test-all && npm run docs:check && npm run docs:build`
4. Commit with a clear message: `git commit -m "docs: complete task A1"`
5. If cross-file or governance-heavy, request `/review` before merge
6. Merge to main and update task status in `openspec/changes/phase-4-closeout-backlog/BACKLOG.md`

### After all Section A, B, C tasks

```bash
# Archive the phase
mkdir -p openspec/changes/archive/$(date +%Y-%m-%d)-phase-4-closeout-backlog
cp -r openspec/changes/phase-4-closeout-backlog/* openspec/changes/archive/$(date +%Y-%m-%d)-phase-4-closeout-backlog/
rm -rf openspec/changes/phase-4-closeout-backlog

# Verify no active changes remain
openspec list

# Final validation
make lint-all && make test-all && npm run docs:check && npm run docs:build

# Commit
git commit -m "chore: archive phase 4 and finalize repository for handoff"

# Signal handoff readiness
echo "Repository is archive-ready. Awaiting archival decision."
```

---

## Critical rules (do not skip)

1. **Run validation checks before every commit.** No exceptions.
   ```bash
   make lint-all && make test-all && npm run docs:check && npm run docs:build
   ```

2. **Never treat archived changes as active.** Only work within `phase-4-closeout-backlog`.

3. **Use `/review` for cross-file changes.** Especially governance layer edits.

4. **Ask for help early.** Don't spin on blockers; document and escalate.

5. **Keep the worktree clean.** No local changes outside the active phase.

---

## What you should NOT do

❌ Add new tools or features  
❌ Major refactoring outside the backlog  
❌ Invent new governance layers or processes  
❌ Change the 4-phase structure or OpenSpec model  
❌ Commit without running the validation suite  
❌ Skip `/review` for governance or cross-file edits  

---

## Emergency reference

**Something breaks?**
→ Run `make lint-all` and `make test-all` to isolate the issue.
→ Check the error message carefully; most issues are simple (syntax, import, etc).
→ If unsure, document the blocker and request human judgment.

**Need to understand a tool?**
→ Read `<tool>/README.md` (if exists) or the main README.md architecture section.
→ Run the tool with `--help` to see command-line options.
→ Check `openspec/specs/project/spec.md` for tool requirements.

**Need to understand the docs?**
→ Edit markdown in `docs/` (source).
→ Run `npm run docs:build` to see HTML output.
→ Check `.vitepress/config.mts` for nav/sidebar structure.

**Need to update workflows?**
→ Edit `.github/workflows/*.yml`.
→ Validate syntax: `gh workflow list` should succeed.
→ Request `/review` before merge.

---

## Handoff checklist (copy this into your first commit message)

```markdown
Phase 4 execution checklist:
- [ ] All Section A tasks complete (final verification, no drift, phase archived)
- [ ] All Section B items verified (docs coherent, links active)
- [ ] All Section C checks pass (GitHub metadata, Pages live)
- [ ] Code review sign-off received
- [ ] No unresolved blockers remain
- [ ] openspec list returns "No active changes found"
- [ ] Final validation suite passes
- [ ] Repository declared archive-ready
```

---

## Questions?

If something is unclear:
1. Re-read `.github/copilot-instructions.md`
2. Check `AGENTS.md` for shared rules
3. Review the active phase change's `proposal.md` and `design.md`
4. Document the question and request human judgment

**You are not expected to invent context.** The repository's instruction files are your single source of truth.

---

## Good luck! 🚀

You've inherited a stable, well-documented codebase. The hard work (Phases 1-3) is done. Your job is to finish the checklist and hand it off cleanly.

See you at archive-ready. 📦
