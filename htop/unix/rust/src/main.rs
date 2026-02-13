use std::{cmp::Ordering, error::Error, io, time::{Duration, Instant}};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
    Terminal,
};
use sysinfo::{CpuExt, Pid, PidExt, ProcessExt, System, SystemExt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SortKey {
    Cpu,
    Mem,
    Pid,
}

fn draw_process_details(frame: &mut ratatui::Frame<'_>, area: Rect, sys: &System, app: &App) {
    if app.processes.is_empty() {
        let para = Paragraph::new("No process selected").block(Block::default().borders(Borders::ALL).title("Details"));
        frame.render_widget(para, area);
        return;
    }
    let row = match app.processes.get(app.selected) { Some(r) => r, None => {
        let para = Paragraph::new("No process selected").block(Block::default().borders(Borders::ALL).title("Details"));
        frame.render_widget(para, area);
        return;
    }};
    let pid = Pid::from_u32(row.pid as u32);
    let (name, status, cpu, mem_mb, exe, cmd) = if let Some(p) = sys.process(pid) {
        let name = p.name().to_string();
        let status = format!("{:?}", p.status());
        let cpu = format!("{:.1}", p.cpu_usage());
        let mem_mb = format!("{:.1}", p.memory() as f64 / 1024.0);
        let exe = format!("{}", p.exe().display());
        let cmd = if p.cmd().is_empty() { String::from("") } else { p.cmd().join(" ") };
        (name, status, cpu, mem_mb, exe, cmd)
    } else {
        (row.name.clone(), String::from("Unknown"), format!("{:.1}", row.cpu), format!("{:.1}", row.mem_kb as f64 / 1024.0), String::from(""), String::from(""))
    };

    let lines = vec![
        Line::from(vec![Span::styled(format!(" PID: {}  ", row.pid), Style::default().fg(Color::Yellow)), Span::raw(format!("Status: {}", status))]),
        Line::from(format!(" Name: {}", name)),
        Line::from(format!(" CPU%: {}  Mem: {} MB", cpu, mem_mb)),
        Line::from(format!(" Exe: {}", exe)),
        Line::from(format!(" Cmd: {}", cmd)),
    ];
    let para = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Details"));
    frame.render_widget(para, area);
}

fn kill_process(pid: i32) -> Result<(), String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        // 尝试正常结束
        let out = Command::new("taskkill").args(["/PID", &pid.to_string()]).output()
            .map_err(|e| format!("spawn taskkill failed: {}", e))?;
        if out.status.success() {
            return Ok(());
        }
        // 强制结束
        let out = Command::new("taskkill").args(["/PID", &pid.to_string(), "/F"]).output()
            .map_err(|e| format!("spawn taskkill /F failed: {}", e))?;
        if out.status.success() { return Ok(()); }
        Err(format!("taskkill exit code {:?}", out.status.code()))
    }
    #[cfg(not(windows))]
    {
        use std::process::Command;
        // 先尝试 SIGTERM
        let out = Command::new("kill")
            .arg(pid.to_string())
            .output()
            .map_err(|e| format!("spawn kill failed: {}", e))?;
        if out.status.success() {
            return Ok(());
        }
        // 强制 SIGKILL
        let out = Command::new("kill")
            .args(["-9", &pid.to_string()])
            .output()
            .map_err(|e| format!("spawn kill -9 failed: {}", e))?;
        if out.status.success() {
            return Ok(());
        }
        Err(format!("kill exit code {:?}", out.status.code()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InputMode {
    Normal,
    Searching,
}

#[derive(Clone, Debug)]
struct ProcRow {
    pid: i32,
    name: String,
    cpu: f32,
    mem_kb: u64,
}

impl ProcRow {
    fn as_row(&self) -> Row<'_> {
        let cpu = format!("{:.1}", self.cpu);
        let mem_mb = (self.mem_kb as f64) / 1024.0;
        Row::new(vec![
            Cell::from(self.pid.to_string()),
            Cell::from(self.name.clone()),
            Cell::from(cpu),
            Cell::from(format!("{mem_mb:.1}")),
        ])
    }
}

struct App {
    sort: SortKey,
    desc: bool,
    selected: usize,
    processes: Vec<ProcRow>,
    filter: String,
    mode: InputMode,
    paused: bool,
    status: String,
    show_details: bool,
}

impl App {
    fn new() -> Self {
        Self {
            sort: SortKey::Cpu,
            desc: true,
            selected: 0,
            processes: Vec::new(),
            filter: String::new(),
            mode: InputMode::Normal,
            paused: false,
            status: String::new(),
            show_details: false,
        }
    }

    fn cycle_sort(&mut self) {
        self.sort = match self.sort {
            SortKey::Cpu => SortKey::Mem,
            SortKey::Mem => SortKey::Pid,
            SortKey::Pid => SortKey::Cpu,
        };
    }

    fn sort_processes(&mut self) {
        match self.sort {
            SortKey::Cpu => self.processes.sort_by(|a, b| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal)),
            SortKey::Mem => self.processes.sort_by_key(|p| p.mem_kb),
            SortKey::Pid => self.processes.sort_by_key(|p| p.pid),
        }
        if self.desc {
            self.processes.reverse();
        }
        if self.selected >= self.processes.len() {
            self.selected = self.processes.len().saturating_sub(1);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal init
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();

    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let mut sys = System::new_all();
    // 预热，保证 CPU 使用率有参考基线
    sys.refresh_all();
    std::thread::sleep(Duration::from_millis(150));

    let mut tick_rate = Duration::from_millis(500);
    let mut last_tick = Instant::now();
    let mut table_state = TableState::default();

    loop {
        if last_tick.elapsed() >= tick_rate {
            if !app.paused {
                do_refresh(&mut sys, &mut app);
            }
            last_tick = Instant::now();
        }

        // 绘制
        table_state.select(if app.processes.is_empty() { None } else { Some(app.selected) });
        terminal.draw(|f| ui(f, &sys, &app, &mut table_state))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release { continue; }
                match app.mode {
                    InputMode::Normal => {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Up => app.selected = app.selected.saturating_sub(1),
                            KeyCode::Down => {
                                if app.selected + 1 < app.processes.len() { app.selected += 1; }
                            }
                            KeyCode::PageUp => app.selected = app.selected.saturating_sub(10),
                            KeyCode::PageDown => {
                                app.selected = (app.selected + 10).min(app.processes.len().saturating_sub(1));
                            }
                            KeyCode::Home => { app.selected = 0; }
                            KeyCode::End => { app.selected = app.processes.len().saturating_sub(1); }
                            KeyCode::Char('s') => { app.cycle_sort(); app.sort_processes(); }
                            KeyCode::Char('r') => { app.desc = !app.desc; app.sort_processes(); }
                            KeyCode::Char('/') => { app.mode = InputMode::Searching; }
                            KeyCode::Char('p') => { app.paused = !app.paused; }
                            KeyCode::F(5) => { do_refresh(&mut sys, &mut app); }
                            KeyCode::Char('k') => {
                                if let Some(row) = app.processes.get(app.selected) {
                                    match kill_process(row.pid) {
                                        Ok(()) => {
                                            app.status = format!("killed PID {}", row.pid);
                                            do_refresh(&mut sys, &mut app);
                                        }
                                        Err(e) => {
                                            app.status = format!("kill PID {} failed: {}", row.pid, e);
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('-') => {
                                let ms = tick_rate.as_millis().saturating_sub(100) as u64;
                                let ms = ms.clamp(100, 5000);
                                tick_rate = Duration::from_millis(ms);
                            }
                            KeyCode::Char('+') | KeyCode::Char('=') => {
                                let ms = (tick_rate.as_millis() as u64).saturating_add(100);
                                let ms = ms.clamp(100, 5000);
                                tick_rate = Duration::from_millis(ms);
                            }
                            KeyCode::Enter | KeyCode::Char('d') => {
                                app.show_details = !app.show_details;
                            }
                            KeyCode::Esc => {
                                if !app.filter.is_empty() {
                                    app.filter.clear();
                                    app.selected = 0;
                                    app.processes = collect_processes(&sys);
                                    app.sort_processes();
                                }
                            }
                            _ => {}
                        }
                    }
                    InputMode::Searching => {
                        match key.code {
                            KeyCode::Enter => { app.mode = InputMode::Normal; }
                            KeyCode::Esc => {
                                app.filter.clear();
                                app.mode = InputMode::Normal;
                                app.selected = 0;
                                app.processes = collect_processes(&sys);
                                app.sort_processes();
                            }
                            KeyCode::Backspace => { app.filter.pop(); }
                            KeyCode::Char(c) => {
                                // 只接受可显示字符
                                if !c.is_control() { app.filter.push(c); }
                            }
                            _ => {}
                        }

                        // 根据当前过滤词即时过滤
                        app.selected = 0;
                        app.processes = collect_processes(&sys);
                        if !app.filter.is_empty() {
                            app.processes = filter_processes(app.processes, &app.filter);
                        }
                        app.sort_processes();
                    }
                }
            }
        }
    }
}

fn collect_processes(sys: &System) -> Vec<ProcRow> {
    let mut v = Vec::new();
    for (pid, p) in sys.processes() {
        v.push(ProcRow {
            pid: pid.as_u32() as i32, // compatible display
            name: p.name().to_string(),
            cpu: p.cpu_usage(),
            mem_kb: p.memory(),
        });
    }
    v
}

fn filter_processes(v: Vec<ProcRow>, filter: &str) -> Vec<ProcRow> {
    if filter.is_empty() { return v; }
    let needle = filter.to_lowercase();
    v.into_iter()
        .filter(|p| p.name.to_lowercase().contains(&needle))
        .collect()
}

fn do_refresh(sys: &mut System, app: &mut App) {
    // 刷新系统与进程信息并应用过滤与排序
    sys.refresh_cpu();
    sys.refresh_processes();
    let v = collect_processes(sys);
    app.processes = if app.filter.is_empty() { v } else { filter_processes(v, &app.filter) };
    app.sort_processes();
}

fn ui(frame: &mut ratatui::Frame<'_>, sys: &System, app: &App, table_state: &mut TableState) {
    let chunks = if app.show_details {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(7),
            ])
            .split(frame.area())
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
            ])
            .split(frame.area())
    };

    draw_summary(frame, chunks[0], sys, app);
    draw_process_table(frame, chunks[1], app, table_state);
    if app.show_details {
        draw_process_details(frame, chunks[2], sys, app);
    }
}

fn draw_summary(frame: &mut ratatui::Frame<'_>, area: Rect, sys: &System, app: &App) {
    // CPU 平均
    let (cpu_avg, cores) = if sys.cpus().is_empty() {
        (0.0_f64, 0_usize)
    } else {
        let sum: f64 = sys.cpus().iter().map(|c| c.cpu_usage() as f64).sum();
        (sum / sys.cpus().len() as f64, sys.cpus().len())
    };

    // 内存
    let total = sys.total_memory().max(1);
    let used = sys.used_memory().min(total);
    let mem_pct = (used as f64) * 100.0 / (total as f64);
    let used_gb = (used as f64) / (1024.0 * 1024.0);
    let total_gb = (total as f64) / (1024.0 * 1024.0);

    let sort = match app.sort {
        SortKey::Cpu => "CPU",
        SortKey::Mem => "MEM",
        SortKey::Pid => "PID",
    };
    let order = if app.desc { "desc" } else { "asc" };
    let mode = match app.mode { InputMode::Normal => "NORMAL", InputMode::Searching => "SEARCH" };
    let filter_shown: String = if app.filter.is_empty() { "—".into() } else { app.filter.clone() };
    let paused = if app.paused { "PAUSED" } else { "RUN" };

    let text = Line::from(vec![
        Span::styled(" htop-rust  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!("CPU {cpu_avg:.1}% ({cores} cores)  ")),
        Span::raw(format!("Mem {used_gb:.2}/{total_gb:.2} GiB ({mem_pct:.1}%)  ")),
        Span::raw(format!("Sort: {sort} ({order})  ")),
        Span::raw(format!("Filter: {}  ", filter_shown)),
        Span::raw(format!("Mode: {}  ", mode)),
        Span::raw(format!("Paused: {}  ", paused)),
        Span::raw("Keys: q quit, ↑/↓ move, PgUp/PgDn fast, Home/End, s sort, r reverse, / search, Esc clear, p pause, F5 refresh, -/+ interval, k kill, Enter/d details"),
        if app.status.is_empty() { Span::raw("") } else { Span::raw(format!("  Msg: {}", app.status)) },
    ]);

    let para = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Summary"));
    frame.render_widget(para, area);
}

fn draw_process_table(frame: &mut ratatui::Frame<'_>, area: Rect, app: &App, table_state: &mut TableState) {
    let arrow = if app.desc { "↓" } else { "↑" };
    let pid_h = if matches!(app.sort, SortKey::Pid) { format!("PID {arrow}") } else { "PID".into() };
    let name_h = "NAME".to_string();
    let cpu_h = if matches!(app.sort, SortKey::Cpu) { format!("CPU% {arrow}") } else { "CPU%".into() };
    let mem_h = if matches!(app.sort, SortKey::Mem) { format!("MEM(MB) {arrow}") } else { "MEM(MB)".into() };
    let header = Row::new(vec![pid_h, name_h, cpu_h, mem_h]) 
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let rows = app.processes.iter().map(|p| p.as_row());

    let widths = [
        Constraint::Length(8),
        Constraint::Percentage(55),
        Constraint::Length(8),
        Constraint::Length(12),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Processes"))
        .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(table, area, table_state);
}
