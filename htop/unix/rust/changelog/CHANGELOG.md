# Changelog

所有对本项目的修改都会记录在此文件中。

## [0.1.0] - 2025-09-25
- 初始化项目结构：`Cargo.toml`、`src/main.rs`、`README.md`、`.gitignore`、`changelog/CHANGELOG.md`
- 实现最小可用的 TUI：
  - 上方摘要：CPU 平均使用率、内存使用/总量与百分比
  - 下方表格：进程 PID、NAME、CPU%、MEM(MB)
  - 支持滚动（↑/↓、PgUp/PgDn）与排序键循环（s），升/降序切换（r）
- 依赖选择：`ratatui = 0.29`、`crossterm = 0.29`、`sysinfo = 0.29`
- 兼容性修正：
  - 引入 `PidExt` 获取 `pid.as_u32()`
  - 进程名 `p.name().to_string()`
  - `ratatui` 的 `Table` 使用 `.widths(&[...])` 方式设置列宽
  - `Frame` 显式生命周期参数 `Frame<'_>`

## [0.1.1] - 2025-09-25
- 修复构建错误：将 `Table` 的 `highlight_style`/`highlight_symbol` 更新为 `row_highlight_style`/`row_highlight_symbol`
- 微调：保持 `Frame<'_>` 与 `.widths(&...)` 用法以适配 ratatui 0.29

## [0.1.2] - 2025-09-25
- 新增：进程搜索/过滤（/ 进入、Backspace 删除、Enter 确认、Esc 清除并退出，名称包含匹配，大小写不敏感）
- 新增：暂停与手动刷新（p 切换暂停、F5 手动刷新）
- 新增：刷新间隔可调（- 减少、+ 或 = 增加，范围 100ms~5000ms）
- UI：在摘要栏显示 Filter/Mode/Paused 信息与快捷键提示
- 重构：抽取 `do_refresh()` 统一刷新与过滤/排序逻辑

## [0.1.3] - 2025-09-25
- 新增：进程详情面板（Enter 或 d 显示/隐藏），展示名称、状态、CPU%、内存、可执行路径、命令行
- 新增：结束选中进程（k，Windows 下调用 `taskkill`，先普通再 `/F` 强制）
- 改进：表头显示当前排序列与方向箭头（↑/↓），支持 Home/End 快速跳至首/末行
- 改进：摘要键位提示同步更新
- 文档：更新 README 说明与键位

## [0.1.4] - 2025-10-20
- 微调：在非 Windows 分支中使用 `pid`（赋值给 `_`）以消除未使用警告

## [0.1.5] - 2026-02-13
- 修复：在 Unix 平台实现 `kill_process()`，支持 SIGTERM → SIGKILL 逐级终止
- 更新 README：反映跨平台 kill 功能
