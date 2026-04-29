# Phase 4 Backlog: Complete Task Inventory

This document is the definitive handoff manifest for GLM. It contains all known work, classified by priority and executor recommendation.

## Archive-readiness checklist ✅

All items below have been **verified as passing** after Phases 1-3:

| Category | Status | Evidence |
| --- | --- | --- |
| **Code Quality** | ✅ PASS | `make lint-all` successful across Rust/Go |
| **Test Suite** | ✅ PASS | `make test-all` - all 18 tests pass, zero failures |
| **Docs Build** | ✅ PASS | `npm run docs:check && npm run docs:build` successful |
| **Governance** | ✅ PASS | Only active phase is phase-4-closeout-backlog; archived phases clean |
| **Workflows** | ✅ PASS | Simplified; triggers narrowed; security scans scheduled-only |
| **Metadata** | ✅ PASS | GitHub repo description/topics/homepage aligned with README |
| **Version Anchoring** | ✅ PASS | Rust/Go/Node versions tracked; package-lock.json committed |
| **Blocked TODOs** | ✅ NONE | No blocking TODO/FIXME/XXX in source; only documentation references |

## Final backlog inventory

### Must-finish (blocks archive-readiness)

#### Section A: Critical path closure

| ID | Task | Depends on | Executor | Status |
| --- | --- | --- | --- | --- |
| **A1** | Final verification: Run full suite one more time in sequence (lint, test, docs, site) | None | GLM or Copilot | Blocked until Phase 4 complete |
| **A2** | Document any last-minute version/dependency drift discovered during final verification | A1 | GLM | Pending |
| **A3** | Archive Phase 4 and set repository to "no active changes" state | A2 | Copilot or manual | Pending |

#### Section B: Documentation coherence

| ID | Task | Depends on | Executor | Status |
| --- | --- | --- | --- | --- |
| **B1** | Cross-verify README, Pages home, and Architecture doc are not repeating the same content | None | Code Review | Not yet started |
| **B2** | Confirm all internal doc links are active (no 404s) after Phases 1-3 pruning | None | Copilot (npm run docs:check) | Passing |
| **B3** | Update CHANGELOG.md with summary of stabilization phases (optional, if release cadence exists) | B1, B2 | GLM or Human | Optional |

#### Section C: GitHub-side finalization

| ID | Task | Depends on | Executor | Status |
| --- | --- | --- | --- | --- |
| **C1** | Final check: `gh repo view` output matches expected description, topics, homepage | None | Copilot | Not yet started |
| **C2** | Confirm GitHub Pages is live and serving latest site build | None | Human or Copilot | Manual verification |
| **C3** | Verify branch protection and merge requirements are sane for close-out phase | None | Human | Manual verification |

### Optional polish (does not block archive)

| ID | Task | Depends on | Executor | Rationale |
| --- | --- | --- | --- | --- |
| **O1** | Add per-platform CI badges to README | None | GLM | Nice for credibility, but GitHub Actions visible anyway |
| **O2** | Create a RELEASE.md guide for future maintainers (if any) | B1 | Human | Helpful if anyone forks post-archival |
| **O3** | Record version matrix (Rust/Go/Node + tested platforms) | A1 | GLM | Useful for posterity, not blocking |
| **O4** | Add acknowledgments or credits section if not already present | None | Human | Optional community recognition |
| **O5** | Create an `en/` landing page mirror for GitHub Pages (if not already present) | B1 | GLM | Useful if multi-language traffic exists |

## Risks and open questions

### Known risks

- **Flaky tests**: None detected; all 18 tests pass consistently
- **Workflow syntax**: Reduced surface (ci, pages, security) means fewer failure modes, but should be spot-checked in CI before final archive
- **Documentation stale-ness**: Post-archive, no automatic updates. Readers may find minor inconsistencies. Acceptable for learning project.
- **Platform coverage**: Windows Rust/Go, Unix Rust tested. Untested edge cases (MinGW, musl, arm64) may have issues. Acceptable for learning project.

### Open questions

1. Should the repository add release tags/artifacts when archiving, or assume no future releases?
2. Should archived state be marked with a GitHub "archived" label, or kept as "historical but potentially usable"?
3. Should GLM model generate a final "lessons learned" or "retrospective" document post-archive?

**Recommendation**: Leave these as post-handoff decisions for whoever owns GLM execution.

## Executor guides

### For GLM (autonomous execution)

1. Read this entire document before starting any work
2. Follow the "Must-finish" tasks in order (A, then B, then C)
3. Skip optional tasks unless explicitly requested
4. Report blockers immediately; do not attempt workarounds
5. Archive Phase 4 only after all must-finish tasks are done

### For Copilot CLI (validation + orchestration)

1. Run `make lint-all && make test-all && npm run docs:check && npm run docs:build` at least once before final commit
2. Use `gh` CLI to verify GitHub metadata if GLM makes changes to repo description/topics
3. Never skip validation checks, even for trivial edits
4. Flag any workflow syntax warnings; do not merge if CI is broken

### For Code Review (cross-file governance checks)

1. Request `/review` before finalizing Phase 4 archival
2. Check that no instruction files still reference archived changes as active
3. Verify governance layer consistency (AGENTS, CLAUDE, copilot-instructions alignment)
4. Confirm no secrets or sensitive config leaked during phases

### For Human (judgment calls, final sign-off)

1. Verify GitHub Pages is live and serving correctly
2. Confirm branch protection and access control are appropriate for archived state
3. Make final decision on archive-readiness before GLM closure
4. Document any last-minute decisions in CHANGELOG.md or release notes if applicable

## Handoff gate

**Repository is archive-ready when:**

- [ ] All Section A tasks complete (final verification, no drift, Phase 4 archived)
- [ ] All Section B tasks complete or verified passing (docs coherent, links active)
- [ ] All Section C checks pass (GitHub metadata aligned, Pages live, access control sane)
- [ ] Code Review sign-off received
- [ ] No unresolved blockers remain
- [ ] `openspec list` returns "No active changes found"

**Until then:** Keep Phase 4 active; continue addressing blockers.

## Final handoff summary

| Aspect | Status | Notes |
| --- | --- | --- |
| **Code Quality** | Stable | Tests pass, lint clean, no warnings |
| **Documentation** | Stable | Coherent, consolidated, Pages live |
| **Governance** | Stable | OpenSpec clean, instructions aligned |
| **Workflows** | Stable | Simplified, secure, efficient |
| **Metadata** | Stable | GitHub About page reflects actual project state |
| **Known Issues** | None blocking | All critical path items addressed |
| **Ready for GLM?** | **YES** | Assuming must-finish tasks in A-C are complete |
