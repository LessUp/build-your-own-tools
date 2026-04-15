//! Shared library for htop implementations
//!
//! Provides common types and functions used by both Unix and Windows htop.

use ratatui::style::{Color, Style};
use ratatui::widgets::Row;
use sysinfo::{Pid, PidExt};

/// Sort key for process list
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortKey {
    Cpu,
    Mem,
    Pid,
}

/// A row in the process table
#[derive(Clone, Debug)]
pub struct ProcRow {
    pub pid: Pid,
    pub name: String,
    pub cpu: f32,
    pub mem_mb: u64,
}

impl ProcRow {
    /// Convert to a ratatui table row
    pub fn as_row(&self) -> Row<'_> {
        let cpu = format!("{:.1}", self.cpu);
        Row::new(vec![
            ratatui::widgets::Cell::from(self.pid.as_u32().to_string()),
            ratatui::widgets::Cell::from(self.name.clone()),
            ratatui::widgets::Cell::from(cpu),
            ratatui::widgets::Cell::from(format!("{:.1}", self.mem_mb)),
        ])
    }
}

impl PartialOrd for ProcRow {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProcRow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Default comparison by PID
        self.pid.as_u32().cmp(&other.pid.as_u32())
    }
}

impl PartialEq for ProcRow {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Eq for ProcRow {}

/// Compare two process rows by the given sort key
pub fn compare_proc_rows(a: &ProcRow, b: &ProcRow, sort: SortKey) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match sort {
        SortKey::Cpu => a
            .cpu
            .partial_cmp(&b.cpu)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.pid.as_u32().cmp(&b.pid.as_u32()))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortKey::Mem => a
            .mem_mb
            .cmp(&b.mem_mb)
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.pid.as_u32().cmp(&b.pid.as_u32()))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortKey::Pid => a
            .pid
            .as_u32()
            .cmp(&b.pid.as_u32())
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
    }
}

/// Get the selected PID from a process list
pub fn selected_pid(processes: &[ProcRow], selected: usize) -> Option<Pid> {
    processes.get(selected).map(|row| row.pid)
}

/// Resolve the selected index after a list update
pub fn resolve_selected_index(
    processes: &[ProcRow],
    preferred_pid: Option<Pid>,
    fallback_index: usize,
) -> usize {
    if processes.is_empty() {
        return 0;
    }
    if let Some(pid) = preferred_pid {
        if let Some(index) = processes.iter().position(|row| row.pid == pid) {
            return index;
        }
    }
    fallback_index.min(processes.len() - 1)
}

/// Filter processes by name (case-insensitive)
pub fn filter_processes(processes: Vec<ProcRow>, filter: &str) -> Vec<ProcRow> {
    if filter.is_empty() {
        return processes;
    }
    let needle = filter.to_lowercase();
    processes
        .into_iter()
        .filter(|p| p.name.to_lowercase().contains(&needle))
        .collect()
}

/// Get color based on usage ratio (0.0 to 1.0)
pub fn color_for_ratio(ratio: f32) -> Color {
    if ratio < 0.5 {
        Color::LightGreen
    } else if ratio < 0.8 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Create a highlighted style for selected rows
pub fn highlight_style() -> Style {
    Style::default()
        .bg(Color::DarkGray)
        .fg(Color::White)
        .add_modifier(ratatui::style::Modifier::BOLD)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proc_row(pid: u32, name: &str, cpu: f32, mem_mb: u64) -> ProcRow {
        ProcRow {
            pid: Pid::from_u32(pid),
            name: name.to_string(),
            cpu,
            mem_mb,
        }
    }

    #[test]
    fn test_filter_processes_case_insensitive() {
        let rows = vec![
            proc_row(1, "bash", 1.0, 100),
            proc_row(2, "Python", 2.0, 200),
            proc_row(3, "nginx", 3.0, 300),
        ];

        let filtered = filter_processes(rows, "PY");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].pid.as_u32(), 2);
    }

    #[test]
    fn test_filter_processes_empty_returns_all() {
        let rows = vec![
            proc_row(1, "bash", 1.0, 100),
            proc_row(2, "python", 2.0, 200),
        ];
        let filtered = filter_processes(rows.clone(), "");
        assert_eq!(filtered.len(), rows.len());
    }

    #[test]
    fn test_resolve_selected_index_prefers_existing_pid() {
        let rows = vec![proc_row(1, "a", 1.0, 100), proc_row(2, "b", 2.0, 200)];
        assert_eq!(resolve_selected_index(&rows, Some(Pid::from_u32(2)), 0), 1);
    }

    #[test]
    fn test_resolve_selected_index_falls_back_when_pid_missing() {
        let rows = vec![proc_row(1, "a", 1.0, 100), proc_row(2, "b", 2.0, 200)];
        assert_eq!(resolve_selected_index(&rows, Some(Pid::from_u32(99)), 1), 1);
        assert_eq!(resolve_selected_index(&rows, Some(Pid::from_u32(99)), 8), 1);
    }

    #[test]
    fn test_resolve_selected_index_empty_rows() {
        let rows: Vec<ProcRow> = Vec::new();
        assert_eq!(resolve_selected_index(&rows, Some(Pid::from_u32(99)), 3), 0);
    }

    #[test]
    fn test_compare_proc_rows_cpu() {
        let a = proc_row(1, "a", 10.0, 100);
        let b = proc_row(2, "b", 5.0, 200);
        assert_eq!(
            compare_proc_rows(&a, &b, SortKey::Cpu),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_color_for_ratio_thresholds() {
        assert_eq!(color_for_ratio(0.0), Color::LightGreen);
        assert_eq!(color_for_ratio(0.49), Color::LightGreen);
        assert_eq!(color_for_ratio(0.5), Color::Yellow);
        assert_eq!(color_for_ratio(0.79), Color::Yellow);
        assert_eq!(color_for_ratio(0.8), Color::Red);
        assert_eq!(color_for_ratio(1.0), Color::Red);
    }
}
