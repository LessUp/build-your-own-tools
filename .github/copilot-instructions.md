# GitHub Copilot instructions for this repository

## Start from OpenSpec

- Check `openspec/changes/` for an active change before editing repo behavior.
- For repo-wide cleanup, stabilization, or workflow changes, use the active change as the starting point.
- Keep the active backlog aligned with current goals; do not keep abandoned feature changes active.

## Project priorities

- This repository is in archive-ready stabilization phase. Prefer stabilization, cleanup, documentation quality, and public presentation over new features.
- Keep only one canonical explanation per topic. Avoid copying the same content across README, docs pages, and changelogs.
- Treat GitHub Pages as the public landing page and keep it aligned with repository metadata.

## Editing guidance

- Prefer `rg`, `glob`, and file reads over broad shell exploration.
- Use `gh` for repository description, homepage, topics, and workflow inspection.
- Preserve existing local work; do not revert unrelated changes.
- For workflow files, keep triggers narrow and configuration explicit.

## Recommended review flow

1. Update OpenSpec artifacts if scope or behavior changes.
2. Make the smallest coherent batch of edits.
3. Run the existing checks relevant to the touched area.
4. Use `/review` before merge-ready or workflow-heavy changes.

## Validation commands

```bash
make lint-all
make test-all
npm run docs:check
npm run docs:build
```
