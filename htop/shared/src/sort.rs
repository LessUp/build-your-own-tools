//! Internal sorting implementation
//!
//! This module is not publicly exposed. The public interface is
//! `sort_process_list` in the parent module.

use crate::{ProcRow, SortKey};
use std::cmp::Ordering;

pub fn sort_impl(
    processes: &mut [ProcRow],
    key: SortKey,
    descending: bool,
    anchor_pid: Option<u32>,
    fallback_index: usize,
) -> usize {
    processes.sort_by(|a, b| compare(a, b, key));

    if descending {
        processes.reverse();
    }

    resolve_anchor(processes, anchor_pid, fallback_index)
}

fn compare(a: &ProcRow, b: &ProcRow, key: SortKey) -> Ordering {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pid: u32, name: &str, cpu: f32, mem_mb: u64) -> ProcRow {
        ProcRow::new(pid, name, cpu, mem_mb)
    }

    #[test]
    fn test_sort_by_cpu_descending() {
        let mut procs = vec![
            row(1, "a", 1.0, 100),
            row(2, "b", 5.0, 200),
            row(3, "c", 3.0, 300),
        ];

        let selected = sort_impl(&mut procs, SortKey::Cpu, true, None, 0);

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

        let selected = sort_impl(&mut procs, SortKey::Cpu, false, None, 0);

        assert_eq!(procs[0].pid, 2); // lowest CPU first
        assert_eq!(procs[1].pid, 3);
        assert_eq!(procs[2].pid, 1);
        assert_eq!(selected, 0);
    }

    #[test]
    fn test_sort_by_mem() {
        let mut procs = vec![row(1, "a", 1.0, 300), row(2, "b", 5.0, 100)];

        sort_impl(&mut procs, SortKey::Mem, true, None, 0);

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

        sort_impl(&mut procs, SortKey::Pid, false, None, 0);

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

        sort_impl(&mut procs, SortKey::Name, false, None, 0);

        assert_eq!(procs[0].name, "alice");
        assert_eq!(procs[1].name, "bob");
        assert_eq!(procs[2].name, "charlie");
    }

    #[test]
    fn test_sort_preserves_anchor() {
        let mut procs = vec![row(1, "a", 1.0, 100), row(2, "b", 5.0, 200)];

        // Anchor on PID 1, but after sort it should be at index 1
        let selected = sort_impl(&mut procs, SortKey::Cpu, true, Some(1), 0);

        assert_eq!(selected, 1); // PID 1 is now at index 1
        assert_eq!(procs[selected].pid, 1);
    }

    #[test]
    fn test_sort_anchor_missing_uses_fallback() {
        let mut procs = vec![row(1, "a", 1.0, 100), row(2, "b", 5.0, 200)];

        let selected = sort_impl(&mut procs, SortKey::Cpu, true, Some(999), 1);

        assert_eq!(selected, 1); // fallback used
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
        assert_eq!(compare(&a, &b, SortKey::Cpu), Ordering::Greater);

        // Same CPU and MEM, different PID
        let a = row(1, "a", 5.0, 100);
        let b = row(2, "b", 5.0, 100);
        assert_eq!(compare(&a, &b, SortKey::Cpu), Ordering::Less);
    }
}
