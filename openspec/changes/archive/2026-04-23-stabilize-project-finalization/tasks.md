# Tasks: Stabilize Project Finalization

## 1. OpenSpec Governance

- [x] 1.1 Create the `stabilize-project-finalization` OpenSpec change artifacts
- [x] 1.2 Remove or archive `add-cat-tool` from the active change backlog
- [x] 1.3 Rewrite the project-level OpenSpec standards so they govern stabilization and close-out work
- [x] 1.4 Align OpenCode workflow docs with the OpenSpec-first process

## 2. Core Documentation

- [x] 2.1 Rewrite the root README to present the project clearly and concisely
- [x] 2.2 Rewrite the Chinese README to match the final project narrative
- [x] 2.3 Simplify or remove stale/duplicated docs and changelog layers
- [x] 2.4 Rewrite the core architecture/process documentation around the final repo structure

## 3. AI Workflow Documentation

- [x] 3.1 Rewrite `AGENTS.md` with project-specific OpenSpec guidance
- [x] 3.2 Add a project-level `CLAUDE.md`
- [x] 3.3 Add/update GitHub Copilot instruction files
- [x] 3.4 Document review, subagent, autopilot, and limited `/fleet` usage rules

## 4. Engineering Configuration

- [x] 4.1 Fix broken docs tooling and version anchors
- [x] 4.2 Simplify hooks and quality-check entry points
- [x] 4.3 Add tracked canonical config for security/license checks where needed
- [x] 4.4 Add minimal editor/LSP recommendations for Rust, Go, Markdown, and workflows

## 5. GitHub Automation

- [x] 5.1 Simplify CI workflow triggers and job intent
- [x] 5.2 Rework Pages workflow around meaningful build inputs only
- [x] 5.3 Downgrade secondary workflows to manual/scheduled usage where appropriate
- [x] 5.4 Decide whether to add `copilot-setup-steps.yml` and implement it if beneficial

## 6. Site and Repository Presentation

- [x] 6.1 Redesign the Pages homepage for project positioning instead of README mirroring
- [x] 6.2 Simplify VitePress navigation around the final information architecture
- [x] 6.3 Update GitHub About description, homepage, and topics via `gh`

## 7. Close-Out Workflow

- [x] 7.1 Rewrite PR/issue templates for a finish-line maintenance model
- [x] 7.2 Codify the final branch/review/merge workflow
- [x] 7.3 Ensure the remaining repo docs consistently reflect the archive-ready maintenance posture

## 8. Verification

- [x] 8.1 Run `make lint-all`
- [x] 8.2 Run `make test-all`
- [x] 8.3 Run docs checks/build
- [x] 8.4 Review the final change set for unnecessary files, drift, and workflow noise
