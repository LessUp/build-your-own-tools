# Specification: <domain>

## Purpose

<!-- High-level description of this spec's domain -->

---

## Requirements

### Requirement: <requirement-name>

The system SHALL/MUST/SHOULD <behavior>.

#### Scenario: <scenario-name>

- **GIVEN** <precondition>
- **WHEN** <action>
- **THEN** <expected result>
- **AND** <additional assertions>

#### Scenario: <edge-case-name>

- **GIVEN** <edge case precondition>
- **WHEN** <action>
- **THEN** <expected handling>

---

## CLI Interface

```
<tool-name> [OPTIONS] [ARGUMENTS]

Options:
  -h, --help       Print help information
  -V, --version    Print version information
  ...

Exit Codes:
  0  Success
  1  Error
  ...
```

---

## Library API

<!-- If this tool provides a library, document the public API -->

### Functions

```rust
// Rust
pub fn function_name(arg: Type) -> ReturnType
```

```go
// Go
func FunctionName(arg Type) ReturnType
```

---

## Performance Requirements

| Metric | Target |
|--------|--------|
| Throughput | > X MB/s |
| Memory | < X MB |
| Latency | < X ms |

---

## Language Differences

| Feature | Rust | Go |
|---------|------|-----|
| ... | ... | ... |

---

## Cross-Platform Notes

<!-- Unix vs Windows differences -->

---

# Delta Operations (for modifications)

When modifying existing specs, use these delta operations:

## ADDED Requirements

Use for new capabilities being introduced.

```markdown
## ADDED Requirements

### Requirement: New Feature

The system SHALL <new behavior>.

#### Scenario: New feature scenario
- **GIVEN** ...
- **WHEN** ...
- **THEN** ...
```

## MODIFIED Requirements

Use for changed behavior. **MUST include FULL updated content**, not just diffs.

```markdown
## MODIFIED Requirements

### Requirement: Changed Feature

The system SHALL <updated behavior>.

#### Scenario: Updated scenario
- **GIVEN** ...
- **WHEN** ...
- **THEN** ...
```

## REMOVED Requirements

Use for deprecated features. MUST include Reason and Migration.

```markdown
## REMOVED Requirements

### Requirement: Deprecated Feature

**Reason**: Why this is being removed

**Migration**: How to migrate to the new approach
```

## RENAMED Requirements

Use for name changes only.

```markdown
## RENAMED Requirements

### Requirement: Old Name

**FROM**: old-name
**TO**: new-name
```
