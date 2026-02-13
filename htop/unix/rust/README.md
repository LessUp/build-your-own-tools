# htop-rust

一个用 Rust 编写的极简版 htop（终端 TUI），遵循 KISS 原则：
- 显示 CPU 平均使用率、内存使用/总量
- 进程表（PID、NAME、CPU%、MEM(MB)）
- 支持滚动、排序、搜索/过滤、暂停/手动刷新、刷新间隔调节
- 进程详情面板、结束进程（Windows）

## 快速开始

要求：Rust 稳定版（建议最新 stable）。

构建与运行：
```bash
cargo run
```
Windows PowerShell：
```powershell
cargo run
```

首次运行会自动下载依赖，请保持联网。

## 键位
- q：退出
- ↑/↓：选择行
- PgUp/PgDn：快速翻页
- s：切换排序键（CPU → MEM → PID）
- r：切换升/降序
- /：进入搜索模式，输入关键字进行过滤（不区分大小写）
- Enter：退出搜索模式（保留过滤）
- Backspace：删除一个字符
- Esc：清除过滤并返回普通模式
- p：暂停/继续自动刷新
- F5：手动刷新一次
- - / + / =：调整刷新间隔（100ms ~ 5000ms）
- Enter 或 d：显示/隐藏进程详情面板
- k：结束选中进程（Windows 调用 taskkill，Unix 调用 kill；请谨慎使用）

## 功能说明
- CPU：显示所有核心的平均使用率与核心数量
- 内存：显示已用/总量（GiB）和百分比
- 进程表：
  - CPU% 来自 sysinfo，周期性刷新
  - MEM(MB) 基于 KiB 转 MiB 的简单换算
- 搜索/过滤：
  - 在顶部摘要显示当前 Filter 与 Mode
  - 过滤条件为进程名包含（大小写不敏感）
- 暂停与刷新：
  - 可暂停自动刷新以稳定观测
  - 支持手动刷新与刷新间隔动态调整
- 详情面板：
  - 显示选中进程的名称、状态、CPU%、内存、可执行路径、命令行
- 结束进程：
  - Windows：调用 taskkill，先普通后 /F 强制
  - Unix：先 SIGTERM，失败后 SIGKILL

## 已知注意事项
- CPU 使用率需要刷新间隔才能稳定，程序内已做初始预热（150ms）
- 进程列表默认刷新间隔为 500ms，可按需动态调整
- Windows 下正在运行的可执行文件会锁定，若需要重新构建请先在 TUI 中按 q 退出

## 路线图
- 颜色高亮与主题
- CPU/内存历史曲线
- 导出进程列表（CSV/JSON）

## 许可
MIT OR Apache-2.0（遵循仓库根目录 LICENSE）。
