//! CLI 辅助函数
//!
//! 提供 CLI 特定的路径处理和检查逻辑，不属于核心库接口。

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// 生成压缩输出文件的默认路径：`<input>.gz`。
pub fn default_output_for_compress(input: &Path) -> PathBuf {
    let mut p = input.to_path_buf();
    let new_name = match input.file_name().and_then(|n| n.to_str()) {
        Some(s) => format!("{}.gz", s),
        None => format!("{}.gz", input.display()),
    };
    p.set_file_name(new_name);
    p
}

/// 生成解压输出文件的默认路径：移除 `.gz` 或追加 `.out`。
pub fn default_output_for_decompress(input: &Path) -> PathBuf {
    let mut p = input.to_path_buf();
    let new_name = match input.file_name().and_then(|n| n.to_str()) {
        Some(s) => s
            .strip_suffix(".gz")
            .map(|x| x.to_string())
            .unwrap_or_else(|| format!("{}.out", s)),
        None => format!("{}.out", input.display()),
    };
    p.set_file_name(new_name);
    p
}

/// 确保输出路径可写；若父目录不存在则创建。
pub fn ensure_writable(output: &Path, force: bool) -> io::Result<()> {
    if output.exists() && !force {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("输出文件已存在: {} (使用 -f 覆盖)", output.display()),
        ));
    }
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

/// 判断两个路径是否完全相同（尝试规范化后比较）。
pub fn same_path(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(ca), Ok(cb)) => ca == cb,
        _ => a == b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_output_for_compress() {
        let p = PathBuf::from("hello.txt");
        assert_eq!(
            default_output_for_compress(&p),
            PathBuf::from("hello.txt.gz")
        );
    }

    #[test]
    fn test_default_output_for_decompress_gz() {
        let p = PathBuf::from("hello.txt.gz");
        assert_eq!(
            default_output_for_decompress(&p),
            PathBuf::from("hello.txt")
        );
    }

    #[test]
    fn test_default_output_for_decompress_no_gz() {
        let p = PathBuf::from("hello.bin");
        assert_eq!(
            default_output_for_decompress(&p),
            PathBuf::from("hello.bin.out")
        );
    }

    #[test]
    fn test_ensure_writable_no_force() {
        let dir = std::env::temp_dir().join("rgzip_test_writable");
        let _ = fs::create_dir_all(&dir);
        let existing = dir.join("existing.txt");
        fs::write(&existing, b"x").unwrap();

        let err = ensure_writable(&existing, false);
        assert!(err.is_err());

        let ok = ensure_writable(&existing, true);
        assert!(ok.is_ok());

        let _ = fs::remove_dir_all(&dir);
    }
}
