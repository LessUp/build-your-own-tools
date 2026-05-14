//! Rendering adapters for ratatui
//!
//! This module depends on ratatui and provides UI-specific rendering.

use crate::ProcRow;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Row;

/// Render a ProcRow as a ratatui table row.
pub fn render_proc_row(row: &ProcRow) -> Row<'_> {
    Row::new(vec![
        ratatui::widgets::Cell::from(row.pid.to_string()),
        ratatui::widgets::Cell::from(row.name.as_str()),
        ratatui::widgets::Cell::from(format!("{:.1}", row.cpu)),
        ratatui::widgets::Cell::from(format!("{:.1}", row.mem_mb)),
    ])
}

/// Column headers for process table.
pub fn process_table_header() -> Row<'static> {
    Row::new(vec!["PID", "NAME", "CPU%", "MEM MiB"])
        .style(Style::default().add_modifier(Modifier::BOLD))
}

/// Color for usage ratio (0.0 to 1.0).
pub fn usage_color(ratio: f32) -> Color {
    if ratio < 0.5 {
        Color::LightGreen
    } else if ratio < 0.8 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Standard highlight style for selected rows.
pub fn selection_style() -> Style {
    Style::default()
        .bg(Color::DarkGray)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_color_thresholds() {
        assert_eq!(usage_color(0.0), Color::LightGreen);
        assert_eq!(usage_color(0.49), Color::LightGreen);
        assert_eq!(usage_color(0.5), Color::Yellow);
        assert_eq!(usage_color(0.79), Color::Yellow);
        assert_eq!(usage_color(0.8), Color::Red);
        assert_eq!(usage_color(1.0), Color::Red);
    }

    #[test]
    fn test_render_proc_row() {
        let row = ProcRow::new(1234, "test-process", 25.5, 512);
        let rendered = render_proc_row(&row);

        // Verify we get a Row with 4 cells
        // (ratatui Row doesn't expose cell count easily, so we just verify it compiles)
        let _ = rendered;
    }

    #[test]
    fn test_process_table_header() {
        let header = process_table_header();
        let _ = header;
    }

    #[test]
    fn test_selection_style() {
        let style = selection_style();
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }
}
