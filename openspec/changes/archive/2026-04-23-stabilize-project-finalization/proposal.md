# Proposal: Stabilize Project Finalization

## Why

The repository has working code, but its governance, documentation, automation, and public presentation have drifted. Before moving into low-maintenance archival mode, the project needs one aggressive stabilization pass that removes scope creep, restores a single OpenSpec-driven workflow, simplifies noisy automation, and aligns the docs site and GitHub metadata with the actual value of the project.

## What

### Modified Capabilities

- **project** - tighten repository governance, documentation architecture, AI-assisted workflow, automation, and public presentation around an archive-ready finish line

### Scope

- Normalize OpenSpec structure and make active changes match the actual close-out scope
- Prune or rewrite stale, duplicate, or generic docs and changelog layers
- Redesign the core project instructions for AGENTS, Claude, Copilot, and OpenCode
- Simplify GitHub Actions, hooks, and engineering configuration to reduce maintenance noise
- Reposition GitHub Pages as a focused project landing page instead of a README mirror
- Align GitHub About/description/topics/homepage with the finalized site
- Fix repo-wide breakages discovered during the cleanup

### Explicitly Out of Scope

- New tool development for the finalization pass
- Expanding feature surface beyond what is needed to stabilize the current release line
- Adding heavyweight MCP/plugin dependencies without a concrete repo-specific payoff

## Impact

**Affected area**: project-wide governance, docs, site, automation, AI workflow, repository metadata

**Languages affected**:

- [x] Rust
- [x] Go
- [x] Both
- [ ] N/A

**Key decision**: `openspec/changes/add-cat-tool` is treated as out of scope for this finalization pass and should not remain part of the active delivery backlog.
