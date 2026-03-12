# Rustfmt 换行规则修正

日期：2026-03-13

## 变更内容

- 为 `htop/win/rust/.gitattributes` 补充 `*.rs text eol=lf`
- 保留该 Windows 子项目其余文本文件默认使用 `CRLF` 的约定
- 修复 GitHub Actions checkout 后 `src/main.rs` 被转换成 `CRLF`，从而触发 `cargo fmt --check` 失败的问题

## 背景

该子目录原本用 `* text eol=crlf` 覆盖了根目录 `.gitattributes` 中针对 Rust 源码的 LF 规则，导致 Hosted Runner 检出时把 `main.rs` 也转成 `CRLF`。`rustfmt` 对换行风格敏感，因此需要在子目录里显式把 `*.rs` 拉回 `LF`。
