---
title: 模块三 - htop
---

# 模块三：htop — TUI 开发与跨平台架构

> ⭐⭐⭐ 复杂度：高级
>
> 综合运用 TUI 框架、系统 API 调用、跨平台设计等高级技术。

## 模块概述

htop 是本学院最复杂的项目，它综合了多个系统编程的高级主题：

1. **TUI（终端用户界面）**：实时渲染、键盘交互、颜色管理
2. **系统 API**：进程信息、CPU/内存统计、网络状态
3. **跨平台架构**：Unix 和 Windows 的差异抽象

## 核心知识点

### 1. TUI 框架

#### Rust: Ratatui

```rust
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

fn draw_ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &App) {
    terminal.draw(|f| {
        // 布局分割
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // CPU 使用率
                Constraint::Length(3),  // 内存使用率
                Constraint::Min(10),    // 进程列表
            ])
            .split(f.size());
        
        // CPU 使用率条
        let cpu_gauge = Gauge::default()
            .block(Block::default().title("CPU").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(app.cpu_usage);
        f.render_widget(cpu_gauge, chunks[0]);
        
        // 进程列表...
    }).unwrap();
}
```

#### Go: tview

```go
import (
    "github.com/rivo/tview"
)

func createUI(app *tview.Application) {
    // 创建主布局
    flex := tview.NewFlex().
        SetDirection(tview.FlexRow).
        AddItem(createCPUGauge(), 3, 0, false).
        AddItem(createMemGauge(), 3, 0, false).
        AddItem(createProcessTable(), 0, 1, true)
    
    app.SetRoot(flex, true).EnableMouse(true)
}
```

**对比**：
- **Ratatui**：函数式风格，widget 组合，即时模式渲染
- **tview**：面向对象风格，widget 树结构，保留模式渲染

### 2. 系统 API 抽象

跨平台系统信息获取是 htop 的核心挑战：

```mermaid
graph TB
    subgraph "Unix 平台"
        UnixAPI[Unix API]
        ProcFS[/proc 文件系统]
        Syscall[系统调用]
    end
    
    subgraph "Windows 平台"
        WinAPI[Windows API]
        PDH[Performance Data Helper]
        WMI[Windows Management Instrumentation]
    end
    
    subgraph "抽象层"
        Trait[Platform Trait]
        Sysinfo[sysinfo 库]
        Gopsutil[gopsutil 库]
    end
    
    UnixAPI --> ProcFS
    UnixAPI --> Syscall
    WinAPI --> PDH
    WinAPI --> WMI
    
    Trait --> Sysinfo
    Trait --> Gopsutil
    
    Sysinfo --> UnixAPI
    Sysinfo --> WinAPI
    Gopsutil --> UnixAPI
    Gopsutil --> WinAPI
    
    classDef api fill:#3b82f6,color:#fff
    classDef abstract fill:#f59e0b,color:#fff
    
    class UnixAPI,WinAPI api
    class Trait,Sysinfo,Gopsutil abstract
```

#### Unix: /proc 文件系统

```rust
// 读取进程信息
fn read_process_info(pid: u32) -> Result<ProcessInfo> {
    let stat = fs::read_to_string(format!("/proc/{}/stat", pid))?;
    // 解析 stat 格式: pid (comm) state ppid ...
    let parts: Vec<&str> = stat.split_whitespace().collect();
    Ok(ProcessInfo {
        pid,
        name: parts[1].trim_matches(|c| c == '(' || c == ')').to_string(),
        state: parts[2].chars().next().unwrap(),
        // ...
    })
}
```

#### Windows: Performance Counters

```rust
// 使用 windows-rs 调用 Windows API
use windows::Win32::System::PerformanceInformation::*;

fn get_cpu_usage() -> f64 {
    let mut info: SYSTEM_PERFORMANCE_INFORMATION = unsafe { std::mem::zeroed() };
    unsafe {
        NtQuerySystemInformation(
            SystemPerformanceInformation,
            &mut info as *mut _ as *mut c_void,
            std::mem::size_of::<SYSTEM_PERFORMANCE_INFORMATION>() as u32,
            std::ptr::null_mut(),
        );
    }
    // 计算使用率...
}
```

### 3. 跨平台架构设计

```
htop/
├── shared/              # 共享代码（平台无关）
│   ├── app.rs           # 应用状态
│   ├── ui.rs            # UI 渲染逻辑
│   └── config.rs        # 配置管理
├── unix/                # Unix 特定实现
│   ├── platform.rs      # 平台抽象实现
│   └── process.rs       # 进程信息获取
├── win/                 # Windows 特定实现
│   ├── platform.rs      # 平台抽象实现
│   └── process.rs       # 进程信息获取
└── src/
    └── main.rs          # 条件编译入口
```

**条件编译**：

```rust
// main.rs
#[cfg(unix)]
mod platform {
    pub use super::unix::platform::*;
}

#[cfg(windows)]
mod platform {
    pub use super::win::platform::*;
}

fn main() {
    let mut app = App::new();
    loop {
        // 使用平台抽象
        let processes = platform::get_processes();
        app.update(processes);
        app.draw();
    }
}
```

### 4. 实时更新与事件处理

```rust
use crossterm::event::{self, Event, KeyCode, KeyEvent};

fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => {
                return Ok(true); // 退出
            }
            Event::Key(KeyEvent { code: KeyCode::Up, .. }) => {
                app.select_previous();
            }
            Event::Key(KeyEvent { code: KeyCode::Down, .. }) => {
                app.select_next();
            }
            // ...
        }
    }
    Ok(false)
}
```

## 实现对比

| 特性 | Rust (Ratatui) | Go (tview) |
|------|----------------|------------|
| 渲染模式 | 即时模式 | 保留模式 |
| 布局系统 | 约束求解 | Flex 布局 |
| 事件处理 | 手动轮询 | 回调绑定 |
| 学习曲线 | 较陡 | 较平缓 |
| 性能 | 更优 | 良好 |

## 练习

1. **基础**：运行 htop，观察 CPU/内存/进程列表的实时更新
2. **进阶**：添加一个新列显示进程的启动时间
3. **跨平台**：在 Windows 和 Linux 上分别运行，对比行为差异
4. **挑战**：实现进程树视图（父子进程关系）

## 总结

恭喜你完成了 BYOT 学院的学习！通过三个模块，你已经：

- ✅ 理解了流式 I/O 和文本处理
- ✅ 掌握了压缩算法的实际应用
- ✅ 学会了 TUI 开发和跨平台设计
- ✅ 对比了 Rust 和 Go 的设计哲学差异

## 下一步

- 阅读 [白皮书](/zh/whitepaper/) 深入理解架构设计
- 查看 [对比研究](/zh/comparison/) 了解更多语言差异
- 参考 [相关项目](/zh/reference/projects) 探索更多实现

---

**参考资料**：
- [Ratatui 文档](https://docs.rs/ratatui/)
- [tview 文档](https://pkg.go.dev/github.com/rivo/tview)
- [Linux /proc 文件系统](https://man7.org/linux/man-pages/man5/proc.5.html)
- [Windows Performance Counters](https://docs.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-portal)
