# Tasks: Phase 3 AI Governance & Tooling

## 1. Instruction consolidation

- [ ] 1.1 Tighten AGENTS.md to shared workflow rules only
- [ ] 1.2 Consolidate CLAUDE.md as Claude-specific supplement
- [ ] 1.3 Rewrite `.github/copilot-instructions.md` as the execution entry point
- [ ] 1.4 Cross-verify no contradiction between the three files

## 2. LSP and editor configuration

- [ ] 2.1 Create or update `.vscode/settings.json` with Rust/Go/TypeScript guidance
- [ ] 2.2 Document any project-specific linting rules or conventions
- [ ] 2.3 Validate LSP configuration against tool installations in CI

## 3. MCP and CLI skills strategy

- [ ] 3.1 Document which capabilities should live in MCP vs CLI skills
- [ ] 3.2 Review current skills installation and remove redundant entries
- [ ] 3.3 Create a reusable project-context template for subagent prompts

## 4. Agent consistency validation

- [ ] 4.1 Test that a fresh agent reading copilot-instructions.md can reason about the codebase
- [ ] 4.2 Verify that agent workflows respect the 4-phase governance without contradictions
