//! gzip 压缩/解压库
//!
//! 提供核心压缩和解压功能，不包含 CLI 相关逻辑。

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::time::UNIX_EPOCH;

use flate2::read::GzDecoder;
use flate2::{Compression, GzBuilder};

pub const MIN_LEVEL: u32 = 0;
pub const MAX_LEVEL: u32 = 9;

/// 限制压缩级别在 0~9 之间。
pub fn sanitize_level(level: u32) -> u32 {
    level.clamp(MIN_LEVEL, MAX_LEVEL)
}

/// 将指定文件压缩为 gzip 文件。
pub fn compress_path(input: &Path, output: &Path, level: u32) -> io::Result<()> {
    let level = sanitize_level(level);
    let mut in_file = File::open(input)?;

    let meta = in_file.metadata().ok();
    let mtime = meta
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as u32)
        .unwrap_or(0);

    let out_file = File::create(output)?;

    let filename_in_header = input
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let mut encoder = GzBuilder::new()
        .mtime(mtime)
        .filename(filename_in_header)
        .write(out_file, Compression::new(level));

    io::copy(&mut in_file, &mut encoder)?;
    let _ = encoder.finish()?;
    Ok(())
}

/// 将任意读取器的内容压缩到指定文件。
pub fn compress_reader_to_path<R: Read>(reader: R, output: &Path, level: u32) -> io::Result<()> {
    let file = File::create(output)?;
    let _ = compress_reader_to_writer(reader, file, level)?;
    Ok(())
}

/// 将任意读取器的内容压缩到任意写入器。
pub fn compress_reader_to_writer<R: Read, W: Write>(
    mut reader: R,
    writer: W,
    level: u32,
) -> io::Result<W> {
    let level = sanitize_level(level);
    let mut encoder = GzBuilder::new().write(writer, Compression::new(level));
    io::copy(&mut reader, &mut encoder)?;
    encoder.finish()
}

/// 将 gzip 文件解压到指定文件。
pub fn decompress_path(input: &Path, output: &Path) -> io::Result<()> {
    let file = File::open(input)?;
    decompress_reader_to_path(file, output)
}

/// 将任意 gzip 流解压到指定文件。
pub fn decompress_reader_to_path<R: Read>(reader: R, output: &Path) -> io::Result<()> {
    let file = File::create(output)?;
    decompress_reader_to_writer(reader, file)
}

/// 将任意 gzip 流解压到任意写入器。
pub fn decompress_reader_to_writer<R: Read, W: Write>(reader: R, mut writer: W) -> io::Result<()> {
    let mut decoder = GzDecoder::new(reader);
    io::copy(&mut decoder, &mut writer)?;
    writer.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_sanitize_level_clamp() {
        assert_eq!(sanitize_level(0), 0);
        assert_eq!(sanitize_level(6), 6);
        assert_eq!(sanitize_level(9), 9);
        assert_eq!(sanitize_level(100), 9);
    }

    #[test]
    fn test_compress_decompress_roundtrip() {
        let dir = std::env::temp_dir().join("rgzip_test_roundtrip");
        let _ = fs::create_dir_all(&dir);
        let input = dir.join("input.txt");
        let compressed = dir.join("input.txt.gz");
        let decompressed = dir.join("output.txt");

        let data = b"Hello, gzip roundtrip test!\n";
        fs::write(&input, data).unwrap();

        compress_path(&input, &compressed, 6).unwrap();
        assert!(compressed.exists());

        decompress_path(&compressed, &decompressed).unwrap();
        assert_eq!(fs::read(&decompressed).unwrap(), data);

        // cleanup
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_compress_reader_to_writer() {
        let input = b"test data for stream compression";
        let mut compressed = Vec::new();
        compress_reader_to_writer(&input[..], &mut compressed, 6).unwrap();
        assert!(!compressed.is_empty());

        let mut decompressed = Vec::new();
        decompress_reader_to_writer(&compressed[..], &mut decompressed).unwrap();
        assert_eq!(decompressed, input);
    }
}
