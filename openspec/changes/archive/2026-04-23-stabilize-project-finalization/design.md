# Design: Stabilize Project Finalization

## Context

The codebase already passes core linting and test gates, but the repository model around the code is inconsistent:

- OpenSpec exists, but active changes do not reflect the close-out goal.
- Documentation is duplicated across README, docs pages, changelog indexes, and architecture notes.
- AI workflow guidance is split between `AGENTS.md`, local Claude settings, and OpenCode commands without a single project-level instruction set.
- GitHub Actions workflows are numerous and partly overdesigned for a project that is nearing completion.
- GitHub Pages is technically functional but not yet operating as a strong landing page for new visitors.
- Docs tooling is broken because `vue-tsc` is used but not installed.

The design goal is to turn the repository into a coherent, low-maintenance project without introducing extra operational burden.

## Goals

1. Re-establish OpenSpec as the canonical planning and scope-control layer.
2. Replace generic or redundant docs with a smaller set of project-specific, high-signal documents.
3. Make AI-assisted development workflow explicit across AGENTS, Claude, Copilot, and OpenCode.
4. Reduce CI/CD and automation noise while preserving meaningful quality gates.
5. Improve the Pages landing experience and align GitHub repository metadata with it.
6. Produce an archive-ready working model for the remaining maintenance window.

## Non-Goals

1. Shipping new end-user tools during the stabilization pass.
2. Maintaining multiple competing project narratives across README, Pages, and docs.
3. Adding context-heavy MCPs or plugins without a clear ongoing benefit.

## Decisions

### Decision: Use one umbrella OpenSpec change

**Decision**: Create a single `stabilize-project-finalization` change and route repository-wide cleanup through it.

**Rationale**:

- The work is cross-cutting and should be tracked as one coherent finish-line program.
- It gives later GLM/autopilot execution a single source of scope and task ordering.

### Decision: Remove feature drift from the active backlog

**Decision**: Treat `add-cat-tool` as deferred/out of scope and remove it from the active change list.

**Rationale**:

- The repository goal is closure, not expansion.
- Feature growth would compete with stabilization work and delay archival readiness.

### Decision: Keep only one canonical project narrative

**Decision**: Rework the root README, Pages homepage, core architecture/process docs, and changelog policy so each has a clear job and avoids duplicating whole sections from the others.

**Rationale**:

- The current content is readable but repetitive.
- Archive-ready repos benefit from small, durable, high-signal docs.

### Decision: Prefer instruction files and repo config over heavy integration sprawl

**Decision**: Add project-level Claude/Copilot guidance and minimal editor/LSP settings; document MCP/plugin trade-offs instead of pre-adding many integrations.

**Rationale**:

- The repo is near completion, so low-maintenance conventions matter more than ecosystem breadth.
- Repo-local instructions travel better than tool-specific experiments.

### Decision: Reduce workflow noise

**Decision**: Keep the workflows that protect correctness or release quality, tighten their triggers, move generated config into tracked files, and downgrade secondary workflows to manual/scheduled usage where appropriate.

**Rationale**:

- The current workflow set is larger than necessary for a close-out phase.
- Narrower triggers reduce meaningless runs and maintenance fatigue.

## File Organization

### OpenSpec and governance

- `openspec/changes/stabilize-project-finalization/`
- `openspec/config.yaml`
- `openspec/specs/project/spec.md`
- `AGENTS.md`
- `CLAUDE.md`
- `.github/copilot-instructions.md`

### Automation and engineering

- `.github/workflows/*.yml`
- `.githooks/pre-commit`
- `.github/pull_request_template.md`
- `.github/ISSUE_TEMPLATE/*`
- `package.json`
- `deny.toml`
- `.vscode/*`

### Site and presentation

- `README.md`
- `README.zh-CN.md`
- `index.md`
- `.vitepress/config.mts`
- selected `docs/**` pages that remain authoritative

## Risks / Trade-offs

1. **Dirty worktree overlap**: some target files already contain local edits, so cleanup must reconcile rather than overwrite blindly.
2. **Docs pruning risk**: aggressive deletion can break links or remove helpful detail if not carefully redirected.
3. **Workflow simplification risk**: removing noisy jobs too aggressively could also drop useful safety checks.
4. **Metadata sequencing**: GitHub About/homepage should be updated only after the site narrative is stable.

## Testing Strategy

1. Use the existing project gates as the hard floor: `make lint-all`, `make test-all`, and docs build/type checks.
2. Re-run the Pages build after site/navigation changes.
3. Re-check GitHub metadata after `gh` updates.
4. Use a code review pass near the end to catch logic regressions in workflow or doc structure changes.

## Open Questions

1. Which existing changelog files should remain user-facing versus be collapsed into a smaller release history model?
2. Whether the repo should keep both detailed docs and a marketing-style Pages site, or push deeper content behind fewer entry points.
3. Whether a minimal `copilot-setup-steps.yml` provides enough benefit to justify another workflow file.
