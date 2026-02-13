# 2026-02-13 补充测试与路径修复

- `same_path()` 改为使用 `fs::canonicalize` 进行路径规范化比较，避免 `./foo` 与 `foo` 视为不同路径
- 补充 6 个单元测试：
  - `test_sanitize_level_clamp`：压缩级别限制
  - `test_default_output_for_compress`：默认压缩输出路径
  - `test_default_output_for_decompress_gz`：去 `.gz` 后缀
  - `test_default_output_for_decompress_no_gz`：追加 `.out`
  - `test_compress_decompress_roundtrip`：文件级压缩/解压往返
  - `test_ensure_writable_no_force`：覆盖检测
  - `test_compress_reader_to_writer`：流式压缩/解压往返
