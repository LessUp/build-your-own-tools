use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::bar::NINE_LEVELS;
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, Clear, Gauge, Row, Sparkline, Table, TableState,
};
use ratatui::{Frame, Terminal};
use sysinfo::{CpuExt, Pid, PidExt, ProcessExt, System, SystemExt};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // 清理终端状态
    disable_raw_mode().ok();
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen).ok();

    // 将可能的错误返回
    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();
    // 清空启动前残留按键（例如在 shell 中按 Enter 启动命令的回车）
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // 事件处理，带超时让 UI 定期刷新
        let timeout = Duration::from_millis(200);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if app.handle_key(key) {
                    break;
                }
            }
        }
        app.on_tick();
    }

    Ok(())
}

#[derive(Clone, Copy)]
enum SortKey {
    Cpu,
    Mem,
    Pid,
    Name,
}

struct ProcRow {
    pid: Pid,
    name: String,
    cpu: f32,
    mem_mb: u64,
}

struct App {
    sys: System,
    rows: Vec<ProcRow>,
    table_state: TableState,
    sort_key: SortKey,
    sort_desc: bool,
    last_refresh: Instant,
    cpu_usage: f32,
    mem_used_mb: u64,
    mem_total_mb: u64,
    should_quit: bool,
    // 过滤与输入
    filter_text: Option<String>,
    filter_input: String,
    filter_active: bool,
    // 刷新控制
    paused: bool,
    refresh_interval_ms: u64,
    // 详情弹窗
    show_details: bool,
    // 历史曲线（0-100 百分比）
    cpu_hist: VecDeque<u64>,
    mem_hist: VecDeque<u64>,
    hist_capacity: usize,
}

impl App {
    fn new() -> Self {
        let mut sys = System::new_all();
        // 第一次刷新，注意进程 CPU 使用率需要至少两次刷新才准确
        sys.refresh_all();
        let mut app = Self {
            sys,
            rows: Vec::new(),
            table_state: TableState::default(),
            sort_key: SortKey::Cpu,
            sort_desc: true,
            last_refresh: Instant::now(),
            cpu_usage: 0.0,
            mem_used_mb: 0,
            mem_total_mb: 0,
            should_quit: false,
            filter_text: None,
            filter_input: String::new(),
            filter_active: false,
            paused: false,
            refresh_interval_ms: 1000,
            show_details: false,
            cpu_hist: VecDeque::with_capacity(120),
            mem_hist: VecDeque::with_capacity(120),
            hist_capacity: 120,
        };
        // 立刻刷新一次数据用于首帧显示
        app.refresh_data();
        // 选中第一行（如果有）
        if !app.rows.is_empty() {
            app.table_state.select(Some(0));
        }
        app
    }

    fn on_tick(&mut self) {
        if self.paused {
            return;
        }
        // 按可调间隔刷新
        if self.last_refresh.elapsed() >= Duration::from_millis(self.refresh_interval_ms) {
            self.refresh_data();
        }
    }

    fn refresh_data(&mut self) {
        // 为了更准确的 CPU% 需要连续两次刷新间隔一段时间，但我们采用周期性刷新简化
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.sys.refresh_processes();

        self.cpu_usage = self.sys.global_cpu_info().cpu_usage();
        self.mem_total_mb = self.sys.total_memory() / 1024;
        self.mem_used_mb = self.sys.used_memory() / 1024;

        // 维护历史（百分比 0-100）
        let cpu_pct = self.cpu_usage.clamp(0.0, 100.0) as u64;
        let mem_pct = if self.mem_total_mb == 0 {
            0
        } else {
            ((self.mem_used_mb * 100) / self.mem_total_mb).min(100)
        };
        self.cpu_hist.push_back(cpu_pct);
        self.mem_hist.push_back(mem_pct);
        while self.cpu_hist.len() > self.hist_capacity {
            self.cpu_hist.pop_front();
        }
        while self.mem_hist.len() > self.hist_capacity {
            self.mem_hist.pop_front();
        }

        let mut new_rows: Vec<ProcRow> = self
            .sys
            .processes()
            .values()
            .map(|p| ProcRow {
                pid: p.pid(),
                name: p.name().to_string(),
                cpu: p.cpu_usage(),
                mem_mb: p.memory() / 1024, // KiB -> MiB
            })
            .collect();

        self.sort_rows(&mut new_rows);
        // 应用过滤
        if let Some(ft) = &self.filter_text {
            let q = ft.to_lowercase();
            new_rows.retain(|r| {
                r.name.to_lowercase().contains(&q) || r.pid.as_u32().to_string().contains(&q)
            });
        }
        self.rows = new_rows;

        // 修正选中索引
        if let Some(i) = self.table_state.selected() {
            if self.rows.is_empty() {
                self.table_state.select(None);
            } else if i >= self.rows.len() {
                self.table_state.select(Some(self.rows.len() - 1));
            }
        }

        self.last_refresh = Instant::now();
    }

    fn sort_rows(&self, rows: &mut [ProcRow]) {
        match self.sort_key {
            SortKey::Cpu => rows.sort_by(|a, b| {
                a.cpu
                    .partial_cmp(&b.cpu)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            SortKey::Mem => rows.sort_by_key(|r| r.mem_mb),
            SortKey::Pid => rows.sort_by_key(|r| r.pid.as_u32()),
            SortKey::Name => rows.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        }
        if self.sort_desc {
            rows.reverse();
        }
    }

    fn cycle_sort_key(&mut self) {
        self.sort_key = match self.sort_key {
            SortKey::Cpu => SortKey::Mem,
            SortKey::Mem => SortKey::Pid,
            SortKey::Pid => SortKey::Name,
            SortKey::Name => SortKey::Cpu,
        };
        self.refresh_data();
    }

    fn toggle_sort_order(&mut self) {
        self.sort_desc = !self.sort_desc;
        self.refresh_data();
    }

    fn selected_pid(&self) -> Option<Pid> {
        self.table_state
            .selected()
            .and_then(|i| self.rows.get(i))
            .map(|r| r.pid)
    }

    fn page_down(&mut self) {
        if self.rows.is_empty() {
            self.table_state.select(None);
            return;
        }
        let cur = self.table_state.selected().unwrap_or(0);
        let step = 10usize;
        let i = (cur + step).min(self.rows.len() - 1);
        self.table_state.select(Some(i));
    }

    fn page_up(&mut self) {
        if self.rows.is_empty() {
            self.table_state.select(None);
            return;
        }
        let cur = self.table_state.selected().unwrap_or(0);
        let step = 10usize;
        let i = cur.saturating_sub(step);
        self.table_state.select(Some(i));
    }

    fn go_home(&mut self) {
        if self.rows.is_empty() {
            self.table_state.select(None);
        } else {
            self.table_state.select(Some(0));
        }
    }

    fn go_end(&mut self) {
        if self.rows.is_empty() {
            self.table_state.select(None);
        } else {
            self.table_state.select(Some(self.rows.len() - 1));
        }
    }

    fn force_refresh(&mut self) {
        self.refresh_data();
    }

    fn set_interval_faster(&mut self) {
        // 更快：减小间隔，最小 200ms
        let cur = self.refresh_interval_ms as i64;
        let next = (cur - 200).max(200) as u64;
        self.refresh_interval_ms = next;
    }

    fn set_interval_slower(&mut self) {
        // 更慢：增大间隔，最大 5000ms
        let cur = self.refresh_interval_ms as i64;
        let next = (cur + 200).min(5000) as u64;
        self.refresh_interval_ms = next;
    }

    fn select_next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => (i + 1).min(self.rows.len().saturating_sub(1)),
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn select_prev(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn kill_selected(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if let Some(row) = self.rows.get(i) {
                if let Some(proc_) = self.sys.process(row.pid) {
                    // 尝试结束进程
                    let _ = proc_.kill();
                }
            }
            // 结束进程后刷新进程列表
            self.sys.refresh_processes();
            self.refresh_data();
        }
    }

    // 处理键盘事件；返回 true 表示退出
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        // 详情弹窗模式优先处理
        if self.show_details {
            match key.code {
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('i') => {
                    self.show_details = false;
                    return false;
                }
                _ => return false,
            }
        }
        // 处于过滤输入模式时优先处理输入
        if self.filter_active {
            match key.code {
                KeyCode::Esc => {
                    // 取消输入，恢复之前的过滤
                    self.filter_active = false;
                    return false;
                }
                KeyCode::Enter => {
                    let trimmed = self.filter_input.trim().to_string();
                    if trimmed.is_empty() {
                        self.filter_text = None;
                    } else {
                        self.filter_text = Some(trimmed);
                    }
                    self.filter_active = false;
                    self.refresh_data();
                    return false;
                }
                KeyCode::Backspace => {
                    self.filter_input.pop();
                    return false;
                }
                KeyCode::Char(c) => {
                    self.filter_input.push(c);
                    return false;
                }
                _ => return false,
            }
        }
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
                true
            }
            KeyCode::Down => {
                self.select_next();
                false
            }
            KeyCode::Up => {
                self.select_prev();
                false
            }
            KeyCode::PageDown => {
                self.page_down();
                false
            }
            KeyCode::PageUp => {
                self.page_up();
                false
            }
            KeyCode::Home => {
                self.go_home();
                false
            }
            KeyCode::End => {
                self.go_end();
                false
            }
            KeyCode::Char('s') => {
                self.cycle_sort_key();
                false
            }
            KeyCode::Char('r') => {
                self.toggle_sort_order();
                false
            }
            KeyCode::Char('k') => {
                self.kill_selected();
                false
            }
            KeyCode::Char('i') => {
                // 打开详情（需已选中）
                if self.selected_pid().is_some() {
                    self.show_details = true;
                }
                false
            }
            KeyCode::Char('/') => {
                // 进入过滤输入
                self.filter_active = true;
                self.filter_input = self.filter_text.clone().unwrap_or_default();
                false
            }
            KeyCode::Char('p') => {
                self.paused = !self.paused;
                false
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.set_interval_faster();
                false
            }
            KeyCode::Char('-') => {
                self.set_interval_slower();
                false
            }
            KeyCode::F(5) => {
                self.force_refresh();
                false
            }
            _ => false,
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // 顶部 CPU/内存 + 历史曲线
            Constraint::Min(8),     // 进程表
            Constraint::Length(1),  // 底部帮助
        ])
        .split(frame.area());

    draw_top_panel(frame, chunks[0], app);
    draw_process_table(frame, chunks[1], app);
    draw_footer(frame, chunks[2], app);

    if app.show_details {
        draw_details_popup(frame, app);
    }
}

fn draw_top_panel(frame: &mut Frame, area: Rect, app: &App) {
    // 顶部区域：CPU/内存仪表 + 历史曲线
    let lines = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(3),
        ])
        .split(area);

    let cpu_ratio = (app.cpu_usage / 100.0).clamp(0.0, 1.0);
    let cpu_color = color_for_ratio(cpu_ratio);
    let cpu_gauge = Gauge::default()
        .block(
            Block::default()
                .title("CPU")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .gauge_style(Style::default().fg(cpu_color).bg(Color::Black))
        .ratio(cpu_ratio as f64)
        .label(format!("{:.1}%", app.cpu_usage));
    frame.render_widget(cpu_gauge, lines[0]);

    let mem_ratio = if app.mem_total_mb == 0 {
        0.0
    } else {
        app.mem_used_mb as f32 / app.mem_total_mb as f32
    }
    .clamp(0.0, 1.0);
    let mem_color = color_for_ratio(mem_ratio);
    let mem_gauge = Gauge::default()
        .block(
            Block::default()
                .title("内存")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .gauge_style(Style::default().fg(mem_color).bg(Color::Black))
        .ratio(mem_ratio as f64)
        .label(format!(
            "{} / {} MiB ({:.1}%)",
            app.mem_used_mb,
            app.mem_total_mb,
            mem_ratio * 100.0
        ));
    frame.render_widget(mem_gauge, lines[1]);

    // 历史曲线区域再拆成两行：CPU 与 内存
    let hist_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(2)])
        .split(lines[2]);

    let cpu_data: Vec<u64> = app.cpu_hist.iter().copied().collect();
    let mem_data: Vec<u64> = app.mem_hist.iter().copied().collect();

    let cpu_spark = Sparkline::default()
        .block(
            Block::default()
                .title("CPU历史")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(cpu_color))
        .data(&cpu_data)
        .max(100)
        .bar_set(NINE_LEVELS);
    frame.render_widget(cpu_spark, hist_rows[0]);

    let mem_spark = Sparkline::default()
        .block(
            Block::default()
                .title("内存历史")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(mem_color))
        .data(&mem_data)
        .max(100)
        .bar_set(NINE_LEVELS);
    frame.render_widget(mem_spark, hist_rows[1]);
}

fn draw_process_table(frame: &mut Frame, area: Rect, app: &mut App) {
    let header = Row::new(vec![
        Cell::from("PID"),
        Cell::from("进程"),
        Cell::from("CPU%"),
        Cell::from("内存(MiB)"),
    ])
    .style(
        Style::default()
            .bg(Color::DarkGray)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let cpu_cell = Cell::from(format!("{:>6.1}", r.cpu))
                .style(Style::default().fg(color_for_ratio((r.cpu / 100.0).clamp(0.0, 1.0))));
            let row = Row::new(vec![
                Cell::from(r.pid.as_u32().to_string()),
                Cell::from(r.name.clone()),
                cpu_cell,
                Cell::from(format!("{:>10}", r.mem_mb)),
            ]);
            if i % 2 == 0 {
                row.style(Style::default().bg(Color::Rgb(32, 32, 32)))
            } else {
                row
            }
        })
        .collect();

    let sort_text = match app.sort_key {
        SortKey::Cpu => "CPU",
        SortKey::Mem => "内存",
        SortKey::Pid => "PID",
        SortKey::Name => "名称",
    };
    let order_text = if app.sort_desc { "↓" } else { "↑" };
    let widths = vec![
        Constraint::Length(8),
        Constraint::Percentage(55),
        Constraint::Length(8),
        Constraint::Length(12),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(format!("进程（排序: {} {}）", sort_text, order_text)),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    let filter_display = if app.filter_active {
        format!("/{}_", app.filter_input)
    } else if let Some(t) = &app.filter_text {
        format!("filter: {}", t)
    } else {
        "filter: (none)".to_string()
    };
    let paused = if app.paused { "暂停" } else { "运行" };
    let interval = format!("{}ms", app.refresh_interval_ms);
    let shown = app.rows.len();
    let total = app.sys.processes().len();

    let help = Line::from(vec![
        Span::styled(" q ", Style::default().bg(Color::DarkGray)),
        Span::raw("退出  "),
        Span::styled(
            " ↑/↓ PgUp/PgDn Home/End ",
            Style::default().bg(Color::DarkGray),
        ),
        Span::raw("导航  "),
        Span::styled(" s/r ", Style::default().bg(Color::DarkGray)),
        Span::raw("切换/反转排序  "),
        Span::styled(" / ", Style::default().bg(Color::DarkGray)),
        Span::raw("搜索  "),
        Span::styled(" i ", Style::default().bg(Color::DarkGray)),
        Span::raw("详情  "),
        Span::styled(" p ", Style::default().bg(Color::DarkGray)),
        Span::raw("暂停  "),
        Span::styled(" +/- ", Style::default().bg(Color::DarkGray)),
        Span::raw("调整刷新  "),
        Span::styled(" F5 ", Style::default().bg(Color::DarkGray)),
        Span::raw("强制刷新  |  "),
        Span::raw(format!(
            "{} | {} | {} | 进程 {} / {}",
            filter_display, paused, interval, shown, total
        )),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let paragraph = ratatui::widgets::Paragraph::new(help).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_details_popup(frame: &mut Frame, app: &App) {
    // 计算弹窗区域（居中 70%x60%）
    let area = centered_rect(70, 60, frame.area());
    frame.render_widget(Clear, area); // 清理背景

    let mut lines: Vec<Line> = Vec::new();
    if let Some(pid) = app.selected_pid() {
        if let Some(p) = app.sys.process(pid) {
            let name = p.name();
            let parent = p.parent().map(|pp| pp.as_u32()).unwrap_or(0);
            let cpu = p.cpu_usage();
            let mem = p.memory() / 1024; // MiB
            let exe = p.exe().to_string_lossy().to_string();
            let cwd = p.cwd().to_string_lossy().to_string();
            let cmd: String = if p.cmd().is_empty() {
                String::new()
            } else {
                p.cmd()
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let status = format!("{:?}", p.status());

            lines.push(Line::raw(format!("名称: {}", name)));
            lines.push(Line::raw(format!(
                "PID: {}    PPID: {}",
                pid.as_u32(),
                parent
            )));
            lines.push(Line::raw(format!("状态: {}", status)));
            lines.push(Line::raw(format!("CPU%: {:.1}    内存: {} MiB", cpu, mem)));
            lines.push(Line::raw(format!("Exe: {}", exe)));
            lines.push(Line::raw(format!("Cwd: {}", cwd)));
            if !cmd.is_empty() {
                lines.push(Line::raw(format!("Cmd: {}", cmd)));
            }
        } else {
            lines.push(Line::raw("未找到进程详情（可能已退出）"));
        }
    } else {
        lines.push(Line::raw("未选中任何进程"));
    }

    let block = Block::default()
        .title("进程详情（Esc/Enter 关闭）")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let paragraph = ratatui::widgets::Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);
    horizontal[1]
}

fn color_for_ratio(ratio: f32) -> Color {
    if ratio < 0.5 {
        Color::LightGreen
    } else if ratio < 0.8 {
        Color::Yellow
    } else {
        Color::Red
    }
}
