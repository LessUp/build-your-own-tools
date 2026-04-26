# Design: Phase 1 Repo Normalization

## Context

The repository already has a broadly improved README, Pages home, and project spec, but Phase 1 still needs to fix foundational inconsistencies before later close-out work can build on them:

- `openspec list` currently reports no active changes
- multiple instruction files still point to `stabilize-project-finalization` as if it were active
- changelog index and migration pages add maintenance burden without supporting the final public narrative
- VitePress navigation and sidebars still overexpose changelog surfaces and secondary pages

## Decisions

### Decision: Use a new phase-scoped change

Phase 1 creates `phase-1-repo-normalization` as the active OpenSpec change. Archived changes remain historical input only.

### Decision: Keep a smaller changelog model

The repository keeps `CHANGELOG.md` as the only canonical project release-history surface. Auxiliary changelog index and migration pages are removed.

### Decision: Reduce Pages navigation pressure

Pages will continue to route readers to setup, architecture, comparisons, project landing pages, and key project documents, but it will stop foregrounding changelog depth from docs sidebars and per-tool sidebars.

### Decision: Fix durable guidance now, deeper AI/tooling redesign later

Phase 1 only corrects workflow guidance that is objectively false today. Broader AI/tooling role design is deferred to Phase 3.

## Baseline inventory for later phases

### Version anchors

- **Rust workspace**: `Cargo.toml` uses workspace package `edition = "2021"` and shared workspace dependencies; CI uses the stable Rust toolchain.
- **Go workspace**: `go.work` is anchored at `go 1.23.0`; workflows currently use `go-version: stable`.
- **Docs toolchain**: `package.json` is version `0.3.0` with engines `node >=18` and `npm >=9`; Pages workflow uses Node 22.
- **Current install reality**: this branch does not provide a tracked `package-lock.json`, so baseline setup in the isolated worktree requires `npm install` rather than `npm ci`.

### Documentation and navigation baseline

- `CHANGELOG.md` is now the canonical project history surface.
- `docs/changelogs/INDEX.md`, `docs/changelogs/INDEX.zh-CN.md`, and `docs/changelogs/MIGRATION.md` are removed.
- VitePress no longer foregrounds changelog depth from docs sidebars or per-tool sidebars.
- Root docs indexes now describe `docs/` as a durable explanation layer instead of a release-history mirror.

## Risks

1. Removing changelog helper pages could break internal links if root docs are not updated in the same batch.
2. Simplifying navigation too aggressively could hide useful pages unless the root README and landing pages still route people clearly.
3. Updating stable project rules too broadly would blur Phase 1 with later phases.

## Validation

- `npm run docs:check`
- `npm run docs:build`

Run broader repository checks later when Phase 1 lands as a coherent batch.
