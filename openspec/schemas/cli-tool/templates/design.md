# Design: <change-name>

## Context

<!-- Background, current state, constraints, stakeholders -->

## Goals / Non-Goals

**Goals:**

<!-- What this design achieves -->

- ...

**Non-Goals:**

<!-- What is explicitly excluded -->

- ...

---

## Decisions

### Decision: <decision-name>

**Context**: <!-- Why this decision was needed -->

**Decision**: <!-- What was decided -->

**Rationale**: <!-- Why this approach was chosen -->

**Alternatives Considered**:

1. <alternative 1> - Rejected because...
2. <alternative 2> - Rejected because...

---

## Data Structures

```rust
// Rust
struct Example {
    // ...
}
```

```go
// Go
type Example struct {
    // ...
}
```

---

## Algorithms

<!-- Key algorithms and their complexity -->

---

## File Organization

### Rust

```
<tool>/
├── src/
│   ├── main.rs      # CLI entry point
│   ├── lib.rs       # Library exports
│   └── module.rs    # ...
├── tests/
└── Cargo.toml
```

### Go (if applicable)

```
<tool>/
├── cmd/<tool>/
│   └── main.go      # CLI entry point
├── <tool>.go        # Core implementation
├── go.mod
└── go.sum
```

---

## Error Handling

| Error Type | Rust | Go |
|------------|------|-----|
| FileNotFound | `io::ErrorKind::NotFound` | `os.ErrNotExist` |
| PermissionDenied | `io::ErrorKind::PermissionDenied` | `os.ErrPermission` |
| ... | ... | ... |

---

## Testing Strategy

### Unit Tests

<!-- What to test at unit level -->

### Integration Tests

<!-- What to test at integration level -->

### Performance Tests

<!-- Benchmarks and targets -->

---

## Dependencies

| Dependency | Rust Crate | Go Package | Purpose |
|------------|------------|------------|---------|
| CLI parsing | clap | flag | Argument parsing |
| ... | ... | ... | ... |

---

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| ... | ... |

---

## Open Questions

<!-- Outstanding decisions or unknowns to resolve -->

- [ ] Question 1?
- [ ] Question 2?
