//! CRLF 到 LF 的转换模块
//!
//! 提供流式和内存两种转换方式，支持大文件处理。

use std::io::{self, Read, Write};

/// 转换结果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConvertResult {
    /// 是否检测到 CRLF
    pub has_crlf: bool,
    /// 输出的字节数
    pub bytes_written: u64,
}

/// 流式转换，避免加载整个文件到内存
///
/// 读取输入流，将 CRLF (`\r\n`) 转换为 LF (`\n`)，写入输出流。
/// 返回转换结果，包含是否检测到 CRLF 和写入的字节数。
///
/// # 示例
///
/// ```
/// use std::io::Cursor;
/// use dos2unix::convert::{stream, ConvertResult};
///
/// let input = Cursor::new(b"hello\r\nworld\r\n");
/// let mut output = Vec::new();
/// let result = stream(input, &mut output).unwrap();
/// assert!(result.has_crlf);
/// assert_eq!(output, b"hello\nworld\n");
/// ```
pub fn stream<R: Read, W: Write>(mut reader: R, mut writer: W) -> io::Result<ConvertResult> {
    let mut result = ConvertResult::default();
    let mut prev_was_cr = false;

    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            // 如果最后一个字符是 CR，需要写出
            if prev_was_cr {
                writer.write_all(b"\r")?;
                result.bytes_written += 1;
            }
            break;
        }

        // 处理 CRLF：遍历并写入，跳过 \r 后面的 \n
        let mut write_pos = 0;
        let mut i = 0;

        // 如果上一个 buffer 结尾是 CR，检查当前 buffer 开头是否是 LF
        if prev_was_cr && n > 0 && buf[0] == b'\n' {
            buf[write_pos] = b'\n';
            write_pos += 1;
            i = 1; // 跳过 LF
            result.has_crlf = true;
            prev_was_cr = false;
        }

        while i < n {
            if buf[i] == b'\r' && i + 1 < n && buf[i + 1] == b'\n' {
                buf[write_pos] = b'\n';
                write_pos += 1;
                i += 2;
                result.has_crlf = true;
            } else if buf[i] == b'\r' && i + 1 == n {
                // CR 在 buffer 结尾，留到下一个 buffer 处理
                prev_was_cr = true;
                i += 1;
            } else {
                buf[write_pos] = buf[i];
                write_pos += 1;
                i += 1;
            }
        }
        writer.write_all(&buf[..write_pos])?;
        result.bytes_written += write_pos as u64;
    }

    Ok(result)
}

/// 内存转换（用于小文件或 stdin）
///
/// 将输入数据中的 CRLF (`\r\n`) 转换为 LF (`\n`)。
///
/// # 示例
///
/// ```
/// use dos2unix::convert::memory;
///
/// assert_eq!(memory(b"a\r\nb\r\n"), b"a\nb\n".to_vec());
/// assert_eq!(memory(b"a\nb\n"), b"a\nb\n".to_vec()); // 无变化
/// ```
pub fn memory(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] == b'\r' && i + 1 < input.len() && input[i + 1] == b'\n' {
            out.push(b'\n');
            i += 2;
        } else {
            out.push(input[i]);
            i += 1;
        }
    }
    out
}

/// 检测数据中是否包含 CRLF
///
/// # 示例
///
/// ```
/// use dos2unix::convert::contains_crlf;
///
/// assert!(contains_crlf(b"a\r\nb"));
/// assert!(!contains_crlf(b"a\nb"));
/// ```
pub fn contains_crlf(data: &[u8]) -> bool {
    let mut i = 0;
    while i < data.len() {
        if data[i] == b'\r' && i + 1 < data.len() && data[i + 1] == b'\n' {
            return true;
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_memory_empty() {
        assert_eq!(memory(b""), b"".to_vec());
    }

    #[test]
    fn test_memory_only_lf() {
        assert_eq!(memory(b"a\nb\n"), b"a\nb\n".to_vec());
    }

    #[test]
    fn test_memory_only_crlf() {
        assert_eq!(memory(b"a\r\nb\r\n"), b"a\nb\n".to_vec());
    }

    #[test]
    fn test_memory_mixed() {
        assert_eq!(memory(b"a\n\r\nb\r\n"), b"a\n\nb\n".to_vec());
    }

    #[test]
    fn test_memory_trailing_crlf() {
        assert_eq!(memory(b"a\r\n"), b"a\n".to_vec());
    }

    #[test]
    fn test_memory_lone_cr_not_converted() {
        assert_eq!(memory(b"a\rb"), b"a\rb".to_vec());
    }

    #[test]
    fn test_memory_trailing_cr_not_converted() {
        assert_eq!(memory(b"a\r"), b"a\r".to_vec());
    }

    #[test]
    fn test_memory_consecutive_crlf() {
        assert_eq!(memory(b"\r\n\r\n\r\n"), b"\n\n\n".to_vec());
    }

    #[test]
    fn test_memory_no_newlines() {
        assert_eq!(memory(b"hello world"), b"hello world".to_vec());
    }

    // Streaming tests
    #[test]
    fn test_stream_empty() {
        let input = Cursor::new(b"");
        let mut output = Vec::new();
        let result = stream(input, &mut output).unwrap();
        assert_eq!(output, b"");
        assert!(!result.has_crlf);
        assert_eq!(result.bytes_written, 0);
    }

    #[test]
    fn test_stream_no_crlf() {
        let input = Cursor::new(b"hello\nworld\n");
        let mut output = Vec::new();
        let result = stream(input, &mut output).unwrap();
        assert_eq!(output, b"hello\nworld\n");
        assert!(!result.has_crlf);
        assert_eq!(result.bytes_written, 12);
    }

    #[test]
    fn test_stream_with_crlf() {
        let input = Cursor::new(b"hello\r\nworld\r\n");
        let mut output = Vec::new();
        let result = stream(input, &mut output).unwrap();
        assert_eq!(output, b"hello\nworld\n");
        assert!(result.has_crlf);
        assert_eq!(result.bytes_written, 12);
    }

    #[test]
    fn test_stream_mixed() {
        let input = Cursor::new(b"a\n\r\nb\r\n");
        let mut output = Vec::new();
        let result = stream(input, &mut output).unwrap();
        assert_eq!(output, b"a\n\nb\n");
        assert!(result.has_crlf);
    }

    #[test]
    fn test_stream_large_data() {
        // Create data larger than the 8KB buffer
        let mut input_data = vec![b'x'; 10000];
        // Add CRLF at position 8192 (buffer boundary)
        input_data[8191] = b'\r';
        input_data[8192] = b'\n';

        let input = Cursor::new(input_data.clone());
        let mut output = Vec::new();
        let result = stream(input, &mut output).unwrap();

        // Should have converted the CRLF
        assert!(result.has_crlf);
        // Output should be 1 byte shorter (CRLF -> LF)
        assert_eq!(result.bytes_written, 9999);
        // Verify the conversion happened correctly
        assert_eq!(output[8191], b'\n');
    }

    #[test]
    fn test_stream_lone_cr_preserved() {
        let input = Cursor::new(b"hello\rworld");
        let mut output = Vec::new();
        let result = stream(input, &mut output).unwrap();
        assert_eq!(output, b"hello\rworld");
        assert!(!result.has_crlf);
        assert_eq!(result.bytes_written, 11);
    }

    // contains_crlf tests
    #[test]
    fn test_contains_crlf_true() {
        assert!(contains_crlf(b"a\r\nb"));
        assert!(contains_crlf(b"\r\n"));
        assert!(contains_crlf(b"a\r\nb\r\nc"));
    }

    #[test]
    fn test_contains_crlf_false() {
        assert!(!contains_crlf(b""));
        assert!(!contains_crlf(b"a\nb"));
        assert!(!contains_crlf(b"a\rb"));
        assert!(!contains_crlf(b"a\r"));
    }
}
