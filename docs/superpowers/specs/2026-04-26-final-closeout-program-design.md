# Final Close-Out Program Design

## Context

Build Your Own Tools is already functionally substantial, but its repository governance has drifted away from its intended archive-ready finish line.

Current analysis shows five concrete problems:

1. OpenSpec instructions repeatedly refer to `stabilize-project-finalization` as the active cleanup change, but `openspec list` shows no active change and the change now lives under `openspec/changes/archive/2026-04-23-stabilize-project-finalization/`.
2. README, Pages, architecture docs, changelog surfaces, and migration docs still carry overlapping or low-value narrative.
3. GitHub Pages is directionally distinct from the README, but the site structure remains oversized for a close-out repository and still exposes unnecessary informational surface.
4. Workflow inventory is larger than needed for a low-maintenance stabilization phase, especially around coverage, security, and setup automation.
5. AI workflow guidance is stronger than before but still depends on multiple files that can drift unless roles and sequencing are tightened further.

The project goal is not feature expansion. The goal is to drive the repository into a stable, low-noise, high-signal final state that can be handed off cleanly for the remaining close-out work.

## Goals

1. Restore a truthful OpenSpec execution model for the finalization program.
2. Aggressively reduce documentation drift and redundant public/internal narratives.
3. Reframe GitHub Pages as a compact, attractive project landing surface instead of a large mirrored doc shell.
4. Simplify GitHub automation and repository metadata so they match the close-out posture.
5. Reconfigure AI workflow, editor/LSP conventions, and lightweight tooling around project-specific leverage rather than ecosystem sprawl.
6. Produce a structured, dependency-aware backlog that another model can execute with minimal additional context.

## Non-Goals

1. Adding new tools or major new product scope.
2. Introducing heavyweight MCP/plugin surface without strong repository-specific payoff.
3. Keeping broad historical or migration documentation that no longer serves current users.
4. Running multiple unrelated active OpenSpec changes in parallel during close-out.

## Design Decisions

### Decision 1: Use one umbrella program design and four sequential OpenSpec changes

The finalization effort will be governed by one umbrella program design and executed through four sequential phase changes. The umbrella layer defines the overall finish-line model, dependencies, and acceptance bars. Implementation work is then split into four OpenSpec changes aligned to the four phases.

Only one phase change should be active at a time. When a phase finishes, it is archived before the next phase starts.

This keeps scope legible, reduces active-backlog drift, and makes the execution stream easier to hand over to another model.

### Decision 2: Treat the archived stabilization change as historical input, not as the live source of truth

The archived `2026-04-23-stabilize-project-finalization` change remains useful as prior art, but it must not continue to be referenced as the current active cleanup change.

Phase 1 must remove or rewrite all instructions that falsely describe it as active.

### Decision 3: Use strict phase boundaries

The four phases are sequential and intentionally asymmetric:

1. **Phase 1: Aggressive review and normalization**
   - Fix OpenSpec governance drift.
   - Normalize repository/document structure.
   - Prune low-value docs and changelog layers.
   - Rebuild Pages information architecture and public narrative.
   - Scan for bugs and configuration breakage directly coupled to stabilization work.
   - Establish version-anchor inventory across Rust, Go, Node, npm, Actions, docs tooling, and lint tooling.

2. **Phase 2: Engineering workflow and GitHub integration**
   - Simplify workflows to the smallest meaningful set.
   - Narrow trigger scopes and remove low-value automation.
   - Align GitHub About metadata with the final public narrative using `gh`.
   - Codify the lightweight OpenSpec-first workflow, including when `/review` is mandatory.

3. **Phase 3: AI tooling and Vibe Coding configuration**
   - Redesign `AGENTS.md`, `CLAUDE.md`, and `.github/copilot-instructions.md` as a coordinated system.
   - Audit `.opencode/`, LSP/editor config, Copilot/Claude behavior, and other repo-tracked tooling.
   - Decide which capabilities belong in repo-local instructions or skills versus which are not worth adding as MCP/plugin dependencies.

4. **Phase 4: Final backlog and execution runway**
   - Convert findings and decisions from phases 1-3 into a structured backlog.
   - Mark dependencies, ordering, and recommended execution ownership.
   - Define what must be finished for archive readiness versus what remains optional polish.

### Decision 4: Keep one canonical explanation per surface

The close-out model uses a small set of durable surfaces with non-overlapping roles:

| Surface | Role |
| --- | --- |
| `README.md` | quick repository orientation |
| `index.md` + `.vitepress/` | attractive landing page and routing surface |
| selected `docs/**` pages | deeper explanation only where depth is still useful |
| `openspec/specs/project/spec.md` | stable governance rules |
| per-phase OpenSpec change artifacts | temporary execution detail |

Any document that mainly repeats another document without adding durable value is a candidate for deletion, consolidation, or demotion.

### Decision 5: Use gated execution instead of free-form cleanup

The program will not be executed as unconstrained repo-wide churn. It uses three hard gates:

1. OpenSpec scope control.
2. Existing shared validation commands.
3. `/review` before merge-ready workflow, docs-governance, or AI-governance changes.

This matches the user's preference to avoid `/fleet` and favors long-context sequential execution instead.

## Current-State Findings That Shape the Plan

### OpenSpec

- `openspec list` reports no active changes.
- Repository instructions still reference a supposedly active `stabilize-project-finalization` change.
- The archived change already covered much of this territory, so the new program must explicitly prevent stale references from reappearing.

### Documentation

- README and architecture documentation are directionally better than generic boilerplate, but the overall doc surface is still too wide for a finish-line repo.
- `CHANGELOG.md`, changelog index pages, and migration docs still expose unnecessary maintenance burden for a near-complete learning repository.
- The project needs a stronger rule for which docs stay public, which become bilingual, and which are removed.

### Pages

- The current landing page is materially better than a README mirror.
- The VitePress configuration still carries a large navigation and sidebar surface, including items that may not justify ongoing maintenance in close-out mode.

### Automation and configuration

- CI and Pages are clearly purposeful.
- Coverage, security, release, and setup workflows need a fresh value review under the low-maintenance objective.
- The repository already has several shared configuration files, but version anchors and tool-selection rationale are not yet consolidated into a single finish-line policy.

### AI workflow

- `AGENTS.md`, `CLAUDE.md`, and Copilot instructions already point in roughly the same direction.
- Their roles still need to be tightened so future agents do not inherit contradictory or stale operational guidance.

## Risks

1. **Over-pruning documentation** can remove useful detail or break navigation if deletion is not paired with a canonical replacement.
2. **Workflow simplification** can accidentally cut meaningful safety checks if "noise" is judged too aggressively.
3. **Metadata sequencing mistakes** can make GitHub About or Pages tell a new story before the repository surfaces are actually aligned.
4. **Tooling sprawl disguised as modernization** can increase maintenance cost instead of reducing it.

## Validation Model

The program will keep the existing commands as the hard floor:

- `make lint-all`
- `make test-all`
- `npm run docs:check`
- `npm run docs:build`

Validation will be applied per touched surface rather than indiscriminately on every tiny step, but the final acceptance bar for the close-out program assumes those commands remain working.

## Planned Deliverables

1. A new umbrella implementation plan for the full close-out program.
2. Four sequential phase changes with explicit scope and task lists.
3. A pruned and role-driven docs/site/governance structure.
4. A simplified GitHub workflow and metadata model.
5. A final dependency-aware backlog suitable for GLM handoff.
