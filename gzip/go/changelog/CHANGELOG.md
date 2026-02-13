# Changelog

## v0.1.0 - 2025-09-25
- 初始化项目结构：`go.mod`、`cmd/gzip-go/main.go`、`Makefile`、`README.md`、`changelog/CHANGELOG.md`。
- 实现基础功能：
  - 压缩（默认）与解压（`-d`）。
  - 递归处理目录（`-r`）。
  - 并行处理多个文件（`-p`，默认 CPU 核心数）。
  - 标准输入/输出支持（无参或 `-` 读取 stdin；`-stdout` 输出到 stdout）。
  - 覆盖目标（`-f`）。
  - 压缩级别（`-l 0..9`）。
- 行为遵循 KISS 原则，默认不删除源文件。

## v0.1.1 - 2025-09-25
- 修复：移除未使用的 `errors` 导入，解决编译失败问题（`cmd/gzip-go/main.go`）。

## v0.2.0 - 2025-09-25
- Makefile：
  - 新增 `BIN_DIR` 与自动创建 `bin/` 目录。
  - 新增 `build-linux` 目标（`GOOS=linux GOARCH=amd64` 交叉编译）。
  - `install` 目标依赖 `build`，确保先构建再安装。
- README：
  - 增补交叉编译指令（`make build-linux` 与显式 `GOOS/GOARCH` 示例）。

## v0.3.0 - 2026-02-13
- `go.mod`：Go 版本从 1.21 升级至 1.23，与 `go.work` 保持一致。
- 补充单元测试（`main_test.go`）：
  - 流式压缩/解压往返（`TestGzipStream`）
  - 文件级压缩（`TestGzipFile`）、解压往返（`TestGunzipFile`）
  - 输入收集逻辑（`TestCollectInputs_SkipDir`、`TestCollectInputs_SkipGz`）
  - 压缩到 Writer（`TestGzipToWriter`）
