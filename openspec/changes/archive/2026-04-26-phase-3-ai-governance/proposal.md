# Proposal: Phase 3 AI Governance & Tooling

## Why

Phases 1-2 fixed repository narrative and engineering noise. The next phase unlocks **AI agent capability** through coherent, deep configuration:

- `AGENTS.md` and `CLAUDE.md` are broad process guides but lack tactical role clarity
- Copilot instructions are procedural but don't reflect project-specific domain knowledge
- MCP vs CLI Skills trade-off is unresolved — adding capabilities without clarity leads to bloat
- LSP configuration is minimal and doesn't guide code editors / language servers on project conventions
- Tooling choices (Claude / Copilot / other) lack documented rationale

Phase 3 establishes **AI agent consistency** by:
1. Making AGENTS / CLAUDE / Copilot instructions unified and deep, not scattered
2. Documenting tooling strategy (which capabilities belong to MCP vs lightweight CLI skills)
3. Configuring LSP baselines that reflect the project's tech stack reality
4. Establishing a "golden path" template for how agents should reason about this codebase

## What

### Modified capabilities

- **project** - unify and deepen AI agent configuration across all entry points

### Scope

- consolidate AGENTS.md / CLAUDE.md role clarity
- refine `.github/copilot-instructions.md` with project-domain knowledge
- document MCP and CLI skill strategy for this project
- configure or recommend LSP rules matching project tech stack
- create a project-context reusable template for subagent prompting

### Out of scope

- changing the 4-phase structure or governance model itself
- adding new end-user tools or substantial feature expansion
- wholesale rebuild of existing AI agent implementations
