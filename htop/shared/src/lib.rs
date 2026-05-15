//! Shared library for htop implementations
//!
//! Provides pure data types and high-leverage operations.
//! No UI dependencies in the core module.

use std::cmp::Ordering;

/// Pure process data — zero external dependencies on UI or system libraries.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcRow {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub mem_mb: u64,
}

impl ProcRow {
    /// Create a new process row.
    pub fn new(pid: u32, name: impl Into<String>, cpu: f32, mem_mb: u64) -> Self {
        Self {
            pid,
            name: name.into(),
            cpu,
            mem_mb,
        }
    }
}

/// Sort key for process list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SortKey {
    #[default]
    Cpu,
    Mem,
    Pid,
    Name,
}

impl SortKey {
    /// Cycle to next sort key: Cpu → Mem → Pid → Name → Cpu
    pub fn next(self) -> Self {
        match self {
            Self::Cpu => Self::Mem,
            Self::Mem => Self::Pid,
            Self::Pid => Self::Name,
            Self::Name => Self::Cpu,
        }
    }

    /// Get display label for the sort key.
    pub fn label(self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Mem => "MEM",
            Self::Pid => "PID",
            Self::Name => "NAME",
        }
    }
}

/// Sort a process list with optional anchor preservation.
///
/// Returns the new selected index. If `anchor_pid` is provided and found
/// in the sorted list, returns its index; otherwise returns `fallback_index`.
///
/// # Example
/// ```
/// use htop_shared::{ProcRow, SortKey, sort_process_list};
///
/// let mut processes = vec![
///     ProcRow::new(1, "bash", 1.0, 100),
///     ProcRow::new(2, "python", 5.0, 200),
/// ];
/// let selected = sort_process_list(&mut processes, SortKey::Cpu, true, None, 0);
/// assert_eq!(processes[0].pid, 2);  // python has higher CPU
/// ```
pub fn sort_process_list(
    processes: &mut [ProcRow],
    key: SortKey,
    descending: bool,
    anchor_pid: Option<u32>,
    fallback_index: usize,
) -> usize {
    processes.sort_by(|a, b| compare_processes(a, b, key));

    if descending {
        processes.reverse();
    }

    resolve_anchor(processes, anchor_pid, fallback_index)
}

/// Compare two processes by the given sort key.
fn compare_processes(a: &ProcRow, b: &ProcRow, key: SortKey) -> Ordering {
    match key {
        SortKey::Cpu => a
            .cpu
            .partial_cmp(&b.cpu)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.pid.cmp(&b.pid)),
        SortKey::Mem => a
            .mem_mb
            .cmp(&b.mem_mb)
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.pid.cmp(&b.pid)),
        SortKey::Pid => a
            .pid
            .cmp(&b.pid)
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.mem_mb.cmp(&b.mem_mb)),
        SortKey::Name => a
            .name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.pid.cmp(&b.pid)),
    }
}

/// Resolve anchor PID to index after sorting.
fn resolve_anchor(processes: &[ProcRow], anchor_pid: Option<u32>, fallback: usize) -> usize {
    if processes.is_empty() {
        return 0;
    }
    if let Some(pid) = anchor_pid {
        if let Some(idx) = processes.iter().position(|p| p.pid == pid) {
            return idx;
        }
    }
    fallback.min(processes.len() - 1)
}

/// Filter processes by name (case-insensitive substring match).
///
/// Returns a new vector containing processes whose name contains the query
/// string (case-insensitive). Empty query returns all processes.
pub fn filter_processes<'a>(
    processes: impl IntoIterator<Item = &'a ProcRow>,
    query: &str,
) -> Vec<ProcRow> {
    if query.is_empty() {
        return processes.into_iter().cloned().collect();
    }
    let needle = query.to_lowercase();
    processes
        .into_iter()
        .filter(|p| p.name.to_lowercase().contains(&needle))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pid: u32, name: &str, cpu: f32, mem_mb: u64) -> ProcRow {
        ProcRow::new(pid, name, cpu, mem_mb)
    }

    #[test]
    fn test_proc_row_new() {
        let r = row(1234, "test", 50.0, 1024);
        assert_eq!(r.pid, 1234);
        assert_eq!(r.name, "test");
        assert_eq!(r.cpu, 50.0);
        assert_eq!(r.mem_mb, 1024);
    }

    #[test]
    fn test_sort_key_cycle() {
        assert_eq!(SortKey::Cpu.next(), SortKey::Mem);
        assert_eq!(SortKey::Mem.next(), SortKey::Pid);
        assert_eq!(SortKey::Pid.next(), SortKey::Name);
        assert_eq!(SortKey::Name.next(), SortKey::Cpu);
    }

    #[test]
    fn test_sort_key_labels() {
        assert_eq!(SortKey::Cpu.label(), "CPU");
        assert_eq!(SortKey::Mem.label(), "MEM");
        assert_eq!(SortKey::Pid.label(), "PID");
        assert_eq!(SortKey::Name.label(), "NAME");
    }

    #[test]
    fn test_sort_by_cpu_descending() {
        let mut procs = vec![
            row(1, "a", 1.0, 100),
            row(2, "b", 5.0, 200),
            row(3, "c", 3.0, 300),
        ];

        let selected = sort_process_list(&mut procs, SortKey::Cpu, true, None, 0);

        assert_eq!(procs[0].pid, 2); // highest CPU first
        assert_eq!(procs[1].pid, 3);
        assert_eq!(procs[2].pid, 1);
        assert_eq!(selected, 0);
    }

    #[test]
    fn test_sort_by_cpu_ascending() {
        let mut procs = vec![
            row(1, "a", 5.0, 100),
            row(2, "b", 1.0, 200),
            row(3, "c", 3.0, 300),
        ];

        let selected = sort_process_list(&mut procs, SortKey::Cpu, false, None, 0);

        assert_eq!(procs[0].pid, 2); // lowest CPU first
        assert_eq!(procs[1].pid, 3);
        assert_eq!(procs[2].pid, 1);
        assert_eq!(selected, 0);
    }

    #[test]
    fn test_sort_by_mem() {
        let mut procs = vec![row(1, "a", 1.0, 300), row(2, "b", 5.0, 100)];

        sort_process_list(&mut procs, SortKey::Mem, true, None, 0);

        assert_eq!(procs[0].pid, 1); // highest MEM first
        assert_eq!(procs[1].pid, 2);
    }

    #[test]
    fn test_sort_by_pid() {
        let mut procs = vec![
            row(3, "c", 1.0, 100),
            row(1, "a", 1.0, 100),
            row(2, "b", 1.0, 100),
        ];

        sort_process_list(&mut procs, SortKey::Pid, false, None, 0);

        assert_eq!(procs[0].pid, 1);
        assert_eq!(procs[1].pid, 2);
        assert_eq!(procs[2].pid, 3);
    }

    #[test]
    fn test_sort_by_name() {
        let mut procs = vec![
            row(1, "charlie", 1.0, 100),
            row(2, "alice", 1.0, 100),
            row(3, "bob", 1.0, 100),
        ];

        sort_process_list(&mut procs, SortKey::Name, false, None, 0);

        assert_eq!(procs[0].name, "alice");
        assert_eq!(procs[1].name, "bob");
        assert_eq!(procs[2].name, "charlie");
    }

    #[test]
    fn test_sort_preserves_anchor() {
        let mut procs = vec![row(1, "a", 1.0, 100), row(2, "b", 5.0, 200)];

        // Anchor on PID 1, but after sort it should be at index 1
        let selected = sort_process_list(&mut procs, SortKey::Cpu, true, Some(1), 0);

        assert_eq!(selected, 1); // PID 1 is now at index 1
        assert_eq!(procs[selected].pid, 1);
    }

    #[test]
    fn test_sort_anchor_missing_uses_fallback() {
        let mut procs = vec![row(1, "a", 1.0, 100), row(2, "b", 5.0, 200)];

        let selected = sort_process_list(&mut procs, SortKey::Cpu, true, Some(999), 1);

        assert_eq!(selected, 1); // fallback used
    }

    #[test]
    fn test_filter_processes_case_insensitive() {
        let procs = vec![
            row(1, "bash", 1.0, 100),
            row(2, "Python", 2.0, 200),
            row(3, "nginx", 3.0, 300),
        ];

        let filtered = filter_processes(&procs, "py");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].pid, 2);
    }

    #[test]
    fn test_filter_processes_empty_query() {
        let procs = vec![row(1, "bash", 1.0, 100), row(2, "python", 2.0, 200)];
        let filtered = filter_processes(&procs, "");
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_processes_no_match() {
        let procs = vec![row(1, "bash", 1.0, 100), row(2, "python", 2.0, 200)];
        let filtered = filter_processes(&procs, "xyz");
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_sort_empty_list() {
        let mut procs: Vec<ProcRow> = vec![];
        let selected = sort_process_list(&mut procs, SortKey::Cpu, true, None, 0);
        assert_eq!(selected, 0);
    }

    #[test]
    fn test_sort_single_element() {
        let mut procs = vec![row(1, "a", 1.0, 100)];
        let selected = sort_process_list(&mut procs, SortKey::Cpu, true, None, 0);
        assert_eq!(selected, 0);
        assert_eq!(procs[0].pid, 1);
    }

    #[test]
    fn test_resolve_anchor_empty_list() {
        let procs: Vec<ProcRow> = vec![];
        assert_eq!(resolve_anchor(&procs, Some(1), 5), 0);
    }

    #[test]
    fn test_compare_cpu_with_tiebreakers() {
        // Same CPU, different MEM
        let a = row(1, "a", 5.0, 200);
        let b = row(2, "b", 5.0, 100);
        assert_eq!(compare_processes(&a, &b, SortKey::Cpu), Ordering::Greater);

        // Same CPU and MEM, different PID
        let a = row(1, "a", 5.0, 100);
        let b = row(2, "b", 5.0, 100);
        assert_eq!(compare_processes(&a, &b, SortKey::Cpu), Ordering::Less);
    }
}
