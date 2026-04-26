# MCP vs CLI Skills Strategy

This document records the tooling strategy for this repository and explains which capabilities should live in MCP vs lightweight CLI skills.

## Context

- **MCP (Model Context Protocol)**: Long-lived, stateful integrations with external systems or complex analyzers
- **CLI Skills**: Discrete, reusable workflows that operate on files or command output; lightweight text transformations
- **Instruction files**: Procedural guidance (AGENTS.md, CLAUDE.md, copilot-instructions.md)
- **Token cost vs benefit**: MCP has higher upfront token cost; CLI skills are lighter but less capable

## Decisions for this repository

### DO use CLI skills for:

- **Brainstorming workflows**: Generate ideas from requirements, explore design options
- **Code review patterns**: Parallel review layers (blind hunter, edge case, acceptance auditor)
- **Documentation strategies**: Outlining, editing, cross-linking validation
- **OpenSpec lifecycle**: Propose, apply, archive phase changes
- **Git workflow helpers**: Worktree creation, branch management, cleanup
- **Validation orchestration**: Run multiple checks, aggregate results, report findings

*Rationale*: These are stateless, event-driven, and the interaction is captured in the user's session. Adding state or long-lived context would not materially improve the outcomes.

### DO NOT use MCP for:

- **Project analyzers**: Do not add persistent code indexing or semantic analysis via MCP
- **Remote databases**: Do not integrate external knowledge bases or artifact stores
- **Authentication systems**: Do not add OAuth, GitHub App integrations, or credential managers
- **Notification services**: Do not add Slack, email, or webhook integrations

*Rationale*: The repository is in close-out phase; reducing external dependencies and keeping the tooling minimal supports the stability goal. All necessary context fits in the copilot-instructions.md + phase change files.

## Current tooling inventory

### Active CLI skills

- `brainstorming`: Generate design options interactively
- `code-review` (custom): Adversarial review with parallel layers
- `requesting-code-review`: Dispatch code-reviewer subagent
- `verification-before-completion`: Run validation gates before claiming done
- `test-driven-development`: Write tests before implementation
- `systematic-debugging`: Structured debug workflows
- `subagent-driven-development`: Task-by-task execution with checkpoints

### Not added (and should not be)

- MCP for persistent project indexing
- MCP for external knowledge bases
- MCP for GitHub App integrations beyond standard `gh` CLI
- Copilot plugins (unless a clear project-wide benefit emerges; currently none exists)

## When to revisit

This strategy is stable unless:

1. The repository gains a substantial new tool family requiring deep cross-tool analysis
2. A new class of workflow (e.g., distributed CI orchestration) requires persistent state
3. External integrations (e.g., documentation mirrors, release automation) become mission-critical

Current trajectory: **Finalization**. No new integrations are anticipated before handoff to GLM.
