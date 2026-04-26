# Design: Phase 3 AI Governance & Tooling

## Decisions

### Decision: AGENTS.md + CLAUDE.md consolidation

Keep both files but make them strictly complementary, not overlapping:
- `AGENTS.md` = shared project workflow rules (all tools/agents benefit)
- `CLAUDE.md` = Claude-specific tactics and conventions

Eliminate duplication by having CLAUDE.md reference AGENTS.md for shared context.

### Decision: Copilot-instructions.md as the execution entry point

`.github/copilot-instructions.md` should be the **first document an AI agent reads** when working on this repo. It must:
- Summarize the 4-phase governance model
- Point to AGENTS.md for workflow rules
- Provide quick reference for the three tools (dos2unix, gzip, htop) and their language/structure
- Flag any project-specific security or architectural constraints

### Decision: MCP vs CLI Skills strategy

- **MCP candidates**: Things that need long-lived context or stateful interaction (e.g., a persistent project analyzer)
- **CLI Skills candidates**: Discrete, reusable workflows that operate on files/repos independently (e.g., the brainstorming or reviewing patterns already in use)
- **Default**: Prefer CLI skills to keep token overhead minimal; add MCP only if the benefit clearly justifies complexity

### Decision: LSP configuration as a tracked artifact

Create `.vscode/settings.json` that documents:
- Rust: `rust-analyzer` with clippy warning level, formatting rules
- Go: `gopls` configuration for workspace layout recognition
- TypeScript/JavaScript: tsconfig alignment with VitePress

This becomes a shared baseline that helps any editor / agent understand the project.

## Validation

- Copilot-instructions.md reads clearly without external context
- AGENTS.md / CLAUDE.md are internally consistent (no contradictory rules)
- `.vscode/settings.json` is syntactically valid
- No duplicate reasoning across the four instruction files
