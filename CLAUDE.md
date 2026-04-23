# CLAUDE.md

## Working mode

This repository is in an **archive-ready stabilization phase**. Optimize for clarity, consistency, and reduced maintenance burden.

## First steps for any non-trivial task

1. Run `openspec list`.
2. Read the active change artifacts that match the task.
3. Prefer implementing against the current active change instead of inventing parallel scope.

## Repository-specific priorities

- Finish and harden the current tool set; avoid new tool expansion.
- Treat documentation as product surface area: remove duplication, dead structure, and generic filler.
- Keep GitHub Pages distinct from the README; the site should sell the project quickly and route readers deeper.
- Keep GitHub metadata aligned with the final public narrative.

## Tooling guidance

- Use repo-tracked hooks, workflows, and config instead of local-only conventions whenever the rule should be shared.
- Prefer minimal editor/LSP settings and explicit instruction files over adding many plugins or MCP integrations.
- Use `gh` for repository metadata, workflow inspection, and GitHub-side cleanup.
- Use `/review` before merge-ready workflow, docs-governance, or cross-cutting automation changes.

## Validation commands

```bash
make lint-all
make test-all
npm run docs:check
npm run docs:build
```

## Local overrides

`CLAUDE.local.md` is for maintainer-specific or machine-local preferences only. Shared project rules belong here or in `AGENTS.md`.
