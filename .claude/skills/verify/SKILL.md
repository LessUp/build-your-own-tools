---
name: verify
description: Run all quality checks (format, lint, test) to verify changes are ready. Use before committing or when you want to validate your work.
---

Run all quality checks for this project.

## Steps

1. **Format all code**
   ```bash
   make fmt-all
   ```

2. **Run all linters**
   ```bash
   make lint-all
   ```

3. **Run all tests**
   ```bash
   make test-all
   ```

## Success Criteria

All three commands must pass with no errors:
- `make fmt-all` — Code is properly formatted
- `make lint-all` — No clippy warnings (treated as errors), no go vet issues
- `make test-all` — All Rust and Go tests pass

## If Checks Fail

- **Format issues**: Run `make fmt-all` to auto-fix
- **Lint issues**: Fix the reported problems. Clippy warnings are treated as errors.
- **Test failures**: Investigate and fix the failing tests

## After Success

Changes are ready for commit. The pre-commit hook will run these checks again.
