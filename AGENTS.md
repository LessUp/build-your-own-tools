# AGENTS.md

## Project intent

Build Your Own Tools is a learning-focused repository for three tool implementations:

- `dos2unix` - Rust
- `gzip` - Rust + Go
- `htop` - Rust (Unix/Windows) + Go (Unix/Windows)

The repository is in **close-out / archive-ready phase** (governed by OpenSpec phases). All work prioritizes **stabilization, clarity, and reduced maintenance burden** over new features.

## Shared workflow rules for all AI tools

### 1. OpenSpec-first discipline

- **Always** run `openspec list` before starting work.
- Read the active phase change's `proposal.md`, `design.md`, and `tasks.md` if one exists.
- Do not treat archived changes as active scope or reference points; they are historical only.
- If a task requires scope changes and no active change exists, **create the current phase change first**. Do not work in the void.

### 2. Close-out scope discipline

- **Prefer**: bug fixes, documentation simplification, workflow consolidation, version anchoring, and readiness work.
- **Avoid**: new tools, major feature expansion, adding parallel systems or governance layers.
- **Principle**: Remove drift instead of layering. Simplify instead of explaining away complexity.

### 3. Documentation ownership model

- `README.md` = repo orientation and quick routing (not a duplicate of everything)
- `index.md` / GitHub Pages = public landing page, project value, and calls to action
- `openspec/specs/` = canonical requirements; stable, long-lived
- `docs/architecture/`, `docs/setup/` = deep content for readers who want detail
- `CHANGELOG.md` = minimal release history
- Do **not** maintain parallel copies of the same explanation across multiple surfaces.

### 4. Review gates

Use `/review` **before merge** for any of:
- Workflow or CI/CD changes (risk of cascading failures)
- Cross-file refactoring or governance changes (risk of inconsistency)
- Large documentation rewrites or information architecture changes
- Changes to AGENTS.md, CLAUDE.md, or .github/copilot-instructions.md (governance layer changes)

### 5. Execution model

- Use **Copilot CLI**, **Claude**, or other agents with **long context retention** for sequential, stateful work.
- Use `/opsx:propose`, `/opsx:apply`, `/opsx:archive` for OpenSpec change lifecycle.
- Prefer **autopilot** or **subagent-driven-development** for long runs; use `/fleet` only if work is truly independent and quota trade-off is justified.
- Preserve existing local edits; integrate carefully rather than reverting.

### 6. Validation always, everywhere

After any code / workflow / docs / config edits:

```bash
# For code/config changes
make lint-all
make test-all

# For docs/site changes
npm run docs:check
npm run docs:build
```

**Never commit without running the relevant checks.**

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
├── htop/unix/go/              Unix Go UI
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
