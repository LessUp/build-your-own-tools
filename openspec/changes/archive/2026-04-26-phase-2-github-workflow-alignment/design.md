# Design: Phase 2 GitHub Workflow Alignment

## Decisions

### Decision: Keep a small workflow set

`ci.yml`, `pages.yml`, and `release.yml` remain because they map directly to correctness, public presentation, and releases. Secondary workflows must justify their ongoing cost.

### Decision: Remove or downgrade low-value automation

Manual-only coverage is optional but not essential for close-out. Security automation should focus on dependency and module security rather than stacking multiple scanners with overlapping surface.

### Decision: Metadata follows the public narrative

GitHub About changes happen only after README and Pages already tell the intended story. Metadata is alignment work, not a separate narrative surface.

## Validation

- relevant workflow syntax remains valid
- docs/site commands still pass after workflow and template edits
- GitHub metadata reads back correctly through `gh repo view`
