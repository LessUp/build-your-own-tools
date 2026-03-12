# Workflow Pages 安装策略修正

日期：2026-03-13

## 变更内容

- 将 Pages workflow 的依赖安装从 `npm ci` 改为 `npm install --no-package-lock`
- 修正 `README.zh-CN.md` 中会被 VitePress 判定为 dead link 的站内链接
- 保持 VitePress 文档构建与 Pages 部署结构不变
- 适配仓库当前未提交 `package-lock.json` 的实际状态，避免文档构建在安装阶段失败

## 背景

该仓库的文档站只维护 `package.json`，未跟随提交 npm lockfile。继续使用 `npm ci` 会在 GitHub Hosted Runner 上稳定失败，因此本次将 Pages workflow 收敛到与仓库现状一致的安装方式。
