# 2026-02-13 项目全面优化升级

## 根级基础设施
- 新增根级 `.editorconfig`：统一 UTF-8、LF 换行、缩进风格（Rust 4 空格、Go Tab、Makefile Tab）
- 新增根级 `.gitattributes`：统一文本行尾、标记二进制文件类型
- 增强根级 `.gitignore`：补充更多编译产物（`.a`/`.o`/`.obj`/`.lib`/`.pdb`/`.rlib`/`.wasm`）
- `LICENSE`：补充版权持有者名称

## Cargo workspace
- `Cargo.toml`：新增 `[workspace.package]` 共享 `edition` 和 `license` 字段

## Go workspace
- `gzip/go/go.mod`：Go 版本从 1.21 升级至 1.23，与 `go.work` 保持一致

## CI/CD
- 根级 `.github/workflows/ci.yml`：
  - Rust：拆分 lint（fmt + clippy）与 test 作业，新增 Cargo 缓存，多平台测试（Linux + Windows）
  - Go：补充 `go build` 步骤
- `gzip/.github/workflows/ci.yml`：Rust 作业补充 `cargo fmt`、`cargo clippy`、`cargo test`

## Makefile
- 重构根级 Makefile：新增 `fmt-rust`/`fmt-go`/`fmt-all`、`lint-rust`/`lint-go`/`lint-all`、`build-all`、`clean` 目标
- 修复 `build-htop-win-go`：从直接 `cd` 改为 `$(MAKE) -C`
- 新增 `htop/win/go/Makefile`

## dos2unix
- 补充 5 个单元测试：`lone_cr_not_converted`、`trailing_cr_not_converted`、`consecutive_crlf`、`no_newlines`

## gzip/rust
- `same_path()` 改为使用 `fs::canonicalize` 进行路径规范化比较
- 补充 6 个单元测试：压缩/解压往返、级别限制、默认路径生成、覆盖检测、流式处理

## htop/unix/rust
- 修复 `kill_process()`：在 Unix 平台实现 SIGTERM → SIGKILL 逻辑（原先为空实现）

## htop/win/rust
- 依赖版本升级：`crossterm` 0.27 → 0.29、`ratatui` 0.26 → 0.29
- 源码适配：`frame.size()` → `frame.area()`，移除不需要的 `EnableMouseCapture`/`DisableMouseCapture`
- 新增 `changelog/CHANGELOG.md`

## 文档
- 根 `README.md`：添加 CI/LICENSE 徽章、子项目表格、更新目录结构树、更新版本要求、统一使用 Makefile 命令
