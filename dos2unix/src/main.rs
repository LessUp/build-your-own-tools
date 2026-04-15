use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

use anyhow::Result;

const NAME: &str = "dos2unix-rust";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    let help = format!(
        "{name} {version}\n\
使用方法:\n\
  {name} [OPTIONS] [FILES...]\n\
\n\
说明:\n\
  将 CRLF 转换为 LF。若未提供 FILES 或 FILES 包含 '-'，则从标准输入读取并写到标准输出。\n\
\n\
选项:\n\
  -h, --help       显示帮助\n\
  -v, --version    显示版本\n\
  -c, --check      检测模式，仅报告含 CRLF 的目标\n\
  -q, --quiet      静默模式，减少输出\n\
\n\
示例:\n\
  {name} file.txt\n\
  type file.txt | {name} > out.txt\n\
  {name} - file1.txt file2.txt\n",
        name = NAME,
        version = VERSION
    );
    println!("{}", help);
}

/// 流式转换，避免加载整个文件到内存
/// 返回 (是否包含CRLF, 转换后的字节数)
fn convert_crlf_to_lf_stream<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
) -> io::Result<(bool, u64)> {
    let mut has_crlf = false;
    let mut bytes_written: u64 = 0;
    let mut prev_was_cr = false;

    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            // 如果最后一个字符是 CR，需要写出
            if prev_was_cr {
                writer.write_all(b"\r")?;
                bytes_written += 1;
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
            has_crlf = true;
            prev_was_cr = false;
        }

        while i < n {
            if buf[i] == b'\r' && i + 1 < n && buf[i + 1] == b'\n' {
                buf[write_pos] = b'\n';
                write_pos += 1;
                i += 2;
                has_crlf = true;
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
        bytes_written += write_pos as u64;
    }

    Ok((has_crlf, bytes_written))
}

/// 内存转换（用于小文件或 stdin）
fn convert_crlf_to_lf(input: &[u8]) -> Vec<u8> {
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
fn contains_crlf(data: &[u8]) -> bool {
    let mut i = 0;
    while i < data.len() {
        if data[i] == b'\r' && i + 1 < data.len() && data[i + 1] == b'\n' {
            return true;
        }
        i += 1;
    }
    false
}

fn process_stdin_stdout(quiet: bool, check_only: bool) -> Result<bool> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let converted = convert_crlf_to_lf(&buf);
    let has_crlf = converted != buf;

    if check_only {
        if has_crlf && !quiet {
            println!("-");
        }
        return Ok(has_crlf);
    }

    if !quiet && !has_crlf {
        eprintln!("提示: 标准输入未发现 CRLF。");
    }
    let mut stdout = io::stdout();
    stdout.write_all(&converted)?;
    stdout.flush()?;
    Ok(has_crlf)
}

/// 使用临时文件进行流式转换，支持大文件
fn process_file(path: &Path, quiet: bool, check_only: bool) -> Result<bool> {
    // 先检测是否包含 CRLF（快速路径）
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // 使用小缓冲区检测 CRLF
    let mut detect_buf = [0u8; 4096];
    let n = reader.read(&mut detect_buf)?;
    let has_crlf_in_sample = contains_crlf(&detect_buf[..n]);

    if check_only {
        if has_crlf_in_sample && !quiet {
            println!("{}", path.display());
        }
        return Ok(has_crlf_in_sample);
    }

    // 如果样本中没有 CRLF，需要检查整个文件
    if !has_crlf_in_sample {
        let mut full_check = Vec::new();
        full_check.extend_from_slice(&detect_buf[..n]);
        reader.read_to_end(&mut full_check)?;
        let has_crlf = contains_crlf(&full_check);

        if !has_crlf {
            if !quiet {
                println!("未改变: {}", path.display());
            }
            return Ok(false);
        }

        // 有 CRLF，进行转换
        let converted = convert_crlf_to_lf(&full_check);
        fs::write(path, &converted)?;
        if !quiet {
            println!("转换: {}", path.display());
        }
        return Ok(true);
    }

    // 样本中有 CRLF，使用流式转换
    // 创建临时文件
    let temp_path = path.with_extension("tmp");
    let temp_file = File::create(&temp_path)?;
    let mut writer = BufWriter::new(temp_file);

    // 重新打开原文件进行完整读取
    let original_file = File::open(path)?;
    let mut full_reader = BufReader::new(original_file);

    let (has_crlf, _) = convert_crlf_to_lf_stream(&mut full_reader, &mut writer)?;
    writer.flush()?;

    if has_crlf {
        // 用临时文件替换原文件
        fs::rename(&temp_path, path)?;
        if !quiet {
            println!("转换: {}", path.display());
        }
    } else {
        // 删除临时文件
        fs::remove_file(&temp_path)?;
        if !quiet {
            println!("未改变: {}", path.display());
        }
    }

    Ok(has_crlf)
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    let mut args = env::args().skip(1);
    let mut files: Vec<String> = Vec::new();
    let mut quiet = false;
    let mut check_only = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "-v" | "--version" | "-V" => {
                println!("{} {}", NAME, VERSION);
                return Ok(());
            }
            "-q" | "--quiet" => quiet = true,
            "-c" | "--check" => check_only = true,
            "--" => {
                files.extend(args);
                break;
            }
            _ => {
                if arg == "-" {
                    files.push(arg);
                } else if arg.starts_with('-') {
                    anyhow::bail!("未知选项: {}，使用 --help 查看用法", arg);
                } else {
                    files.push(arg);
                }
            }
        }
    }

    if files.is_empty() {
        let has_crlf = process_stdin_stdout(quiet, check_only)?;
        if has_crlf && check_only {
            std::process::exit(2);
        }
        return Ok(());
    }

    let mut found_crlf = false;

    for f in files {
        if f == "-" {
            let has_crlf = process_stdin_stdout(quiet, check_only)?;
            if has_crlf {
                found_crlf = true;
            }
        } else {
            let p = std::path::Path::new(&f);
            let has_crlf = process_file(p, quiet, check_only)?;
            if has_crlf {
                found_crlf = true;
            }
        }
    }

    if check_only && found_crlf {
        std::process::exit(2);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{convert_crlf_to_lf, convert_crlf_to_lf_stream};
    use std::io::Cursor;

    #[test]
    fn test_empty() {
        assert_eq!(convert_crlf_to_lf(b""), b"".to_vec());
    }

    #[test]
    fn test_only_lf() {
        assert_eq!(convert_crlf_to_lf(b"a\nb\n"), b"a\nb\n".to_vec());
    }

    #[test]
    fn test_only_crlf() {
        assert_eq!(convert_crlf_to_lf(b"a\r\nb\r\n"), b"a\nb\n".to_vec());
    }

    #[test]
    fn test_mixed() {
        assert_eq!(convert_crlf_to_lf(b"a\n\r\nb\r\n"), b"a\n\nb\n".to_vec());
    }

    #[test]
    fn test_trailing_crlf() {
        assert_eq!(convert_crlf_to_lf(b"a\r\n"), b"a\n".to_vec());
    }

    #[test]
    fn test_lone_cr_not_converted() {
        assert_eq!(convert_crlf_to_lf(b"a\rb"), b"a\rb".to_vec());
    }

    #[test]
    fn test_trailing_cr_not_converted() {
        assert_eq!(convert_crlf_to_lf(b"a\r"), b"a\r".to_vec());
    }

    #[test]
    fn test_consecutive_crlf() {
        assert_eq!(convert_crlf_to_lf(b"\r\n\r\n\r\n"), b"\n\n\n".to_vec());
    }

    #[test]
    fn test_no_newlines() {
        assert_eq!(convert_crlf_to_lf(b"hello world"), b"hello world".to_vec());
    }

    // Streaming tests
    #[test]
    fn test_stream_empty() {
        let input = Cursor::new(b"");
        let mut output = Vec::new();
        let (has_crlf, bytes) = convert_crlf_to_lf_stream(input, &mut output).unwrap();
        assert_eq!(output, b"");
        assert!(!has_crlf);
        assert_eq!(bytes, 0);
    }

    #[test]
    fn test_stream_no_crlf() {
        let input = Cursor::new(b"hello\nworld\n");
        let mut output = Vec::new();
        let (has_crlf, bytes) = convert_crlf_to_lf_stream(input, &mut output).unwrap();
        assert_eq!(output, b"hello\nworld\n");
        assert!(!has_crlf);
        assert_eq!(bytes, 12);
    }

    #[test]
    fn test_stream_with_crlf() {
        let input = Cursor::new(b"hello\r\nworld\r\n");
        let mut output = Vec::new();
        let (has_crlf, bytes) = convert_crlf_to_lf_stream(input, &mut output).unwrap();
        assert_eq!(output, b"hello\nworld\n");
        assert!(has_crlf);
        assert_eq!(bytes, 12);
    }

    #[test]
    fn test_stream_mixed() {
        let input = Cursor::new(b"a\n\r\nb\r\n");
        let mut output = Vec::new();
        let (has_crlf, _) = convert_crlf_to_lf_stream(input, &mut output).unwrap();
        assert_eq!(output, b"a\n\nb\n");
        assert!(has_crlf);
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
        let (has_crlf, bytes) = convert_crlf_to_lf_stream(input, &mut output).unwrap();

        // Should have converted the CRLF
        assert!(has_crlf);
        // Output should be 1 byte shorter (CRLF -> LF)
        assert_eq!(bytes, 9999);
        // Verify the conversion happened correctly
        assert_eq!(output[8191], b'\n');
    }

    #[test]
    fn test_stream_lone_cr_preserved() {
        let input = Cursor::new(b"hello\rworld");
        let mut output = Vec::new();
        let (has_crlf, bytes) = convert_crlf_to_lf_stream(input, &mut output).unwrap();
        assert_eq!(output, b"hello\rworld");
        assert!(!has_crlf);
        assert_eq!(bytes, 11);
    }
}
