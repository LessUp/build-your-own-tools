# AGENTS.md

## Project intent

Build Your Own Tools is a learning-focused repository for three existing tool families:

- `dos2unix` - Rust
- `gzip` - Rust + Go
- `htop` - Rust (Unix/Windows) + Go (Windows)

The repository is now in a **stabilization / close-out phase**. Prefer finishing, clarifying, and hardening the existing project over adding new surface area.

## Canonical sources of truth

| Topic | Canonical location |
| --- | --- |
| Stable requirements | `openspec/specs/` |
| Active change work | `openspec/changes/<change>/` |
| Current repo-wide cleanup work | `openspec/changes/<current-phase-change>/` (create the active phase change if none exists) |
| Public repo entry point | `README.md` |
| Public project landing page | `index.md` + `.vitepress/` |
| AI workflow rules | `AGENTS.md`, `CLAUDE.md`, `.github/copilot-instructions.md` |

## Required workflow

1. **Start from OpenSpec**
   - Run `openspec list`.
   - If the task belongs to an active change, read `proposal.md`, `specs/`, `design.md`, and `tasks.md` first.
   - If the task changes repo behavior and no active change exists, create the current phase change before implementing.
   - Do not treat archived cleanup changes as active scope.

2. **Keep scope aligned with close-out**
   - Prefer bug fixes, documentation consolidation, workflow simplification, and release/readiness work.
   - Do **not** add new tools or major feature expansion unless explicitly approved.
   - Remove drift instead of layering yet another parallel system.

3. **Keep one canonical explanation per topic**
   - `README.md` explains what the project is and how to navigate it.
   - Pages explains why the project is worth exploring and where to go next.
   - Deep docs explain details; they should not re-copy the whole README.
   - Changelogs should record meaningful release history, not every minor doc churn.

4. **Use AI tools deliberately**
   - Use `/opsx:propose`, `/opsx:apply`, and `/opsx:archive` for OpenSpec lifecycle work.
   - Use `/review` before merge-ready or workflow-heavy changes.
   - Use subagents/autopilot for long sequential work.
   - Avoid `/fleet` unless the work is clearly independent and the speedup is worth the quota cost.

5. **Preserve user work**
   - The worktree may already contain in-progress edits.
   - Integrate with them carefully; do not revert or overwrite unrelated local changes.

## Project structure

```text
build-your-own-tools/
├── .github/workflows/         CI, Pages, release, maintenance automation
├── .githooks/                 tracked git hooks
├── .opencode/                 OpenCode slash commands and skills
├── .vitepress/                Pages site configuration
├── docs/                      supporting documentation
├── openspec/                  specs, schemas, and tracked changes
├── dos2unix/                  Rust implementation
├── gzip/go/                   Go implementation
├── gzip/rust/                 Rust implementation
├── htop/shared/               shared Rust logic
├── htop/unix/rust/            Unix Rust UI
├── htop/win/rust/             Windows Rust UI
├── htop/win/go/               Windows Go UI
├── README.md                  repo entry point
└── index.md                   Pages landing page
```

## Quality gates

Run the relevant existing checks before closing work:

```bash
make lint-all
make test-all
npm run docs:check
npm run docs:build
```

Use `npm run docs:check` for docs/site/config changes and `npm run docs:build` for landing-page or navigation changes.

## Engineering rules

- Keep workflow triggers narrow and purposeful.
- Prefer tracked config files over generating config inline inside workflows.
- Keep project-level editor/LSP config minimal and useful.
- Prefer removing stale docs/config over retaining low-signal history.
- When changing GitHub metadata or Pages, keep repository description/topics/homepage aligned with the final public narrative.

## Close-out checklist

Before declaring a stabilization task done:

1. OpenSpec artifacts reflect the actual scope.
2. README, Pages, and process docs tell the same story.
3. CI/workflow changes reduce noise instead of adding it.
4. AI instruction files agree on the same workflow.
5. The checks relevant to the modified area pass.
