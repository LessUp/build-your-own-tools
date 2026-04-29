# Proposal: Phase 1 Repo Normalization

## Why

The repository is in close-out mode, but its governance and public surfaces still drift in ways that make later cleanup harder:

- several instruction files still imply that an archived cleanup change is the current active change
- changelog and migration surfaces are broader than a near-finished learning repository needs
- Pages navigation still exposes more surface area than the final public story requires
- version and configuration anchors are not yet documented as a single normalization baseline

Before workflow, GitHub, and AI-tooling cleanup can proceed safely, the repository needs one truthful normalization phase that resets scope and reduces documentation noise.

## What

### Modified capabilities

- **project** - normalize OpenSpec close-out workflow, reduce low-value documentation surface, simplify public navigation, and establish the baseline inventory for later phases

### Scope

- create a new active change for Phase 1 instead of treating archived history as active work
- correct instruction files that still reference the archived cleanup change as current
- prune low-value changelog and migration entry points
- simplify VitePress navigation and sidebars so the public docs focus on durable surfaces
- tighten the stable project spec around single-phase close-out execution
- establish the initial version-anchor and cleanup baseline for subsequent phases

### Out of scope

- workflow trigger cleanup and GitHub metadata updates
- AGENTS/CLAUDE/Copilot role redesign beyond the minimal drift fixes required to remove false active-change references
- new tooling, new MCP dependencies, or new end-user features
