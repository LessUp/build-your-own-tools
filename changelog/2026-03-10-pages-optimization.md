# GitHub Pages 优化 (2026-03-10)

## 工作流优化

- **pages.yml** — 添加 sparse-checkout，仅拉取文档相关文件（.vitepress、docs、子项目 README、根 markdown），跳过 Rust/Go 源码与构建产物
- **pages.yml** — 移除不必要的 `configure-pages` 步骤

## 文档站首页重写

- **index.md** — 扩展 tagline：补充平台支持信息
- **index.md** — Feature 卡片从 3 个扩展到 6 个：新增"学习驱动""跨平台 CI""Rust vs Go"
- **index.md** — 新增 Rust vs Go 技术栈对比表
- **index.md** — 新增快速开始代码块
- **index.md** — 新增"每个子项目教你什么"学习目标表

## README 徽章

- **README.md / README.zh-CN.md** — Docs 徽章从静态徽章改为工作流状态徽章；统一顺序为 CI、Docs、License
