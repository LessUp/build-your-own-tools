# htop Specification

## Purpose

Interactive system monitoring TUI that displays process information, resource usage, and allows process management in real-time.

## Requirements

### Requirement: Process List Display

The system SHALL display a list of running processes with key metrics.

#### Scenario: Display process information
- GIVEN htop is running
- WHEN the process list is rendered
- THEN each process shows PID, name, CPU%, memory, and status
- AND CPU and memory usage bars are visible
- AND the list updates in real-time

#### Scenario: Process metrics
- GIVEN a running process
- WHEN displayed in htop
- THEN PID is accurate
- AND CPU percentage is calculated correctly
- AND memory usage is accurate (RSS on Unix, Working Set on Windows)

### Requirement: Process Sorting

The system SHALL support sorting processes by column.

#### Scenario: Sort by CPU
- GIVEN a process list is displayed
- WHEN sorted by CPU column
- THEN processes are ordered by CPU usage (highest first)

#### Scenario: Sort by memory
- GIVEN a process list is displayed
- WHEN sorted by memory column
- THEN processes are ordered by memory usage (highest first)

#### Scenario: Sort by PID or name
- GIVEN a process list is displayed
- WHEN sorted by PID or name column
- THEN processes are ordered accordingly (ascending)

### Requirement: Process Search/Filter

The system SHALL support searching and filtering processes.

#### Scenario: Filter by name
- GIVEN a process list is displayed
- WHEN user searches for "firefox"
- THEN only processes matching "firefox" are displayed
- AND matching is case-insensitive

### Requirement: Process Termination

The system SHALL allow terminating processes.

#### Scenario: Kill process
- GIVEN a process with PID 1234 is selected
- WHEN user presses "k" and confirms
- THEN SIGTERM is sent to PID 1234 (Unix)
- OR WM_CLOSE is sent (Windows)
- AND the process list updates

### Requirement: Adjustable Refresh Interval

The system SHALL support configurable refresh intervals.

#### Scenario: Custom interval
- GIVEN htop starts with --interval 500
- WHEN running
- THEN the display refreshes every 500ms

### Requirement: Terminal Resize Handling

The system SHALL adapt to terminal size changes.

#### Scenario: Resize terminal
- GIVEN htop is running in an 80x24 terminal
- WHEN terminal is resized to 120x40
- THEN the layout adapts to new dimensions
- AND no content is lost

### Requirement: Clean Exit

The system SHALL exit cleanly on user request.

#### Scenario: Quit
- GIVEN htop is running
- WHEN user presses "q"
- THEN htop exits with code 0
- AND terminal is restored to normal state

## CLI Interface

```
htop [OPTIONS]

Options:
  -h, --help              Print help information
  -V, --version           Print version information
  -s, --sort <COLUMN>     Sort column (cpu, mem, pid, name)
  -i, --interval <MS>     Refresh interval in milliseconds (default: 1000)
  -p, --pid <PID>         Show only the given PID(s)

Keyboard Shortcuts:
  ↑/↓     Navigate process list
  /       Search/filter processes
  k       Kill selected process
  F5      Refresh
  F6      Sort by column
  q       Quit

Exit Codes:
  0  Normal exit
  1  Error occurred
```

## Keyboard Interface

| Key | Action |
|-----|--------|
| ↑/↓ | Navigate process list |
| / | Enter search mode |
| k | Kill selected process |
| F5 | Force refresh |
| F6 | Change sort column |
| q | Quit |

## Performance Requirements

| Metric | Target |
|--------|--------|
| CPU overhead | < 1% |
| Rendering | Stable 60 FPS |
| Startup time | < 100ms |

## Language Differences

| Feature | Unix (Rust) | Windows (Rust) | Windows (Go) |
|---------|-------------|----------------|--------------|
| Platform support | ✅ | ✅ | ✅ |
| Sparkline history | ❌ | ✅ | ✅ |

## Library API

The shared library (`htop-shared`) provides common functionality for all htop implementations.

### Data Structures

```rust
/// Process information for display
pub struct ProcRow {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_mib: u64,
    pub status: String,
    pub command: String,
    pub exe_path: String,
}

/// Sort column keys
pub enum SortKey {
    PID,
    Name,
    CPU,
    Memory,
}
```

### Functions

```rust
/// Compare two process rows for sorting
pub fn compare_proc_rows(
    a: &ProcRow,
    b: &ProcRow,
    sort_key: SortKey,
    ascending: bool,
) -> Ordering

/// Filter processes by search query (case-insensitive)
pub fn filter_processes(processes: &[ProcRow], query: &str) -> Vec<ProcRow>

/// Get color for usage ratio (0.0 - 1.0)
/// Thresholds: 0.0-0.6 Green, 0.6-0.85 Yellow, 0.85-1.0 Red
pub fn color_for_ratio(ratio: f32) -> Color

/// Get highlight style for selected rows
pub fn highlight_style() -> Style

/// Resolve selected index to valid range
pub fn resolve_selected_index(current: usize, filtered_count: usize) -> usize

/// Get PID at selected index
pub fn selected_pid(processes: &[ProcRow], selected_index: usize) -> Option<u32>
```

---

**Implementation Languages**: Rust (Unix, Windows) + Go (Unix, Windows)
**Complexity Level**: Advanced (⭐⭐⭐)
