# Design: Phase 4 Close-Out Backlog & Handoff

## Archive-Readiness Criteria

A repository is archive-ready when:

1. ✅ **Code quality**: Full test suite passes, no warnings from lint/clippy/golangci-lint
2. ✅ **Documentation**: All public surfaces (README, Pages, ARCHITECTURE, CONTRIBUTING) are current and consistent
3. ✅ **Governance**: OpenSpec stable, instruction files aligned, no archived changes treated as active
4. ✅ **Workflows**: CI/CD is lean and efficient; only runs when necessary; no dead jobs
5. ✅ **Metadata**: GitHub About page (description, topics, homepage) reflects the actual project state
6. ✅ **Version pinning**: All major dependencies are anchored (Rust, Go, Node versions)
7. ✅ **Known issues**: All discovered bugs are either fixed or documented in the backlog

## Task classification

### Must-finish (blocks archival)

Tasks that, if left incomplete, would mean the repository is NOT archive-ready.

Examples:
- Broken links or docs build failures
- Test suite failures or flaky tests
- Workflow syntax errors or security misconfigurations
- Contradictory instructions or governance layer issues
- Untracked dependencies or version mismatches

### Optional polish (nice-to-have)

Tasks that improve presentation, completeness, or minor UX but don't block archival.

Examples:
- Additional examples or tutorial content
- Performance optimizations
- Code style cleanups that don't affect correctness
- Extra validation or error messages

## Executor recommendation model

- **GLM**: Autonomous execution of well-defined, self-contained tasks
- **Copilot CLI**: Verification, validation, quick fixes, orchestration
- **Code review**: Cross-file changes, governance layer updates, sensitive edits
- **Human review**: Architecture decisions, scope ambiguity, judgment calls

## Handoff gate

Before declaring Phase 4 complete and handing off to GLM:

1. All must-finish tasks are done
2. All optional tasks are filed but may be deferred
3. Known risks and open questions are documented
4. Success criteria for handoff readiness are met
