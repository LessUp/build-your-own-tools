# Tasks: Phase 4 Close-Out Backlog & Handoff

## 1. Audit remaining issues and convert to backlog

- [ ] 1.1 Scan all code for `TODO`, `FIXME`, `XXX` comments and catalog them
- [ ] 1.2 Review all test output for skipped or conditional tests
- [ ] 1.3 Check GitHub issues/PRs for any outstanding bugs or feature requests
- [ ] 1.4 Review all Phase 1-3 change documents for captured-but-incomplete items
- [ ] 1.5 Cross-check docs build output for warnings or broken references

## 2. Classify issues and create the final backlog

- [ ] 2.1 Create a structured backlog table (id, title, type, executor, blocking?)
- [ ] 2.2 Mark each task as must-finish or optional-polish
- [ ] 2.3 Document dependencies between tasks
- [ ] 2.4 Name the recommended executor (GLM, Copilot, Code Review, Human)
- [ ] 2.5 Add acceptance criteria for each must-finish task

## 3. Test full suite and gather baseline metrics

- [ ] 3.1 Run `make lint-all` and capture any non-critical warnings
- [ ] 3.2 Run `make test-all` and document any flaky tests
- [ ] 3.3 Run `npm run docs:check && npm run docs:build` and confirm success
- [ ] 3.4 Test GitHub workflows locally or review their syntax
- [ ] 3.5 Document any version or platform constraints discovered

## 4. Verify handoff readiness

- [ ] 4.1 Confirm all must-finish tasks are done or clearly documented
- [ ] 4.2 Update AGENTS.md / CLAUDE.md with any new patterns discovered
- [ ] 4.3 Verify openspec/specs/project/spec.md reflects final repo state
- [ ] 4.4 Create a README.GLM.md with quick start for GLM model
- [ ] 4.5 Prepare final checklist for archival decision

## 5. Consolidate and finalize Phase 4 change

- [ ] 5.1 Write final summary in this change's proposal/design/tasks
- [ ] 5.2 Ensure no contradictions between backlog and prior phases
- [ ] 5.3 Get `/review` sign-off on final backlog and handoff readiness
- [ ] 5.4 Archive Phase 4 and transition to handoff state
