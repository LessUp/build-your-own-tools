use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

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

fn process_stdin_stdout(quiet: bool, check_only: bool) -> io::Result<bool> {
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

fn process_file(path: &Path, quiet: bool, check_only: bool) -> io::Result<bool> {
    let data = fs::read(path)?;
    let converted = convert_crlf_to_lf(&data);
    let has_crlf = converted != data;

    if check_only {
        if has_crlf && !quiet {
            println!("{}", path.display());
        }
        return Ok(has_crlf);
    }

    if has_crlf {
        fs::write(path, &converted)?;
        if !quiet {
            println!("转换: {}", path.display());
        }
    } else if !quiet {
        println!("未改变: {}", path.display());
    }
    Ok(has_crlf)
}

fn main() {
    let mut args = env::args().skip(1);
    let mut files: Vec<String> = Vec::new();
    let mut quiet = false;
    let mut check_only = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" | "-V" => {
                println!("{} {}", NAME, VERSION);
                return;
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
                    eprintln!("未知选项: {}", arg);
                    eprintln!("使用 --help 查看用法。");
                    std::process::exit(2);
                } else {
                    files.push(arg);
                }
            }
        }
    }

    if files.is_empty() {
        match process_stdin_stdout(quiet, check_only) {
            Ok(has_crlf) => {
                if has_crlf && check_only {
                    std::process::exit(2);
                }
            }
            Err(e) => {
                eprintln!("错误: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    let mut had_error = false;
    let mut found_crlf = false;

    for f in files {
        if f == "-" {
            match process_stdin_stdout(quiet, check_only) {
                Ok(has_crlf) => {
                    if has_crlf {
                        found_crlf = true;
                    }
                }
                Err(e) => {
                    eprintln!("错误(标准流): {}", e);
                    had_error = true;
                }
            }
        } else {
            let p = std::path::Path::new(&f);
            match process_file(p, quiet, check_only) {
                Ok(has_crlf) => {
                    if has_crlf {
                        found_crlf = true;
                    }
                }
                Err(e) => {
                    eprintln!("错误({}): {}", f, e);
                    had_error = true;
                }
            }
        }
    }

    if had_error {
        std::process::exit(1);
    }

    if check_only && found_crlf {
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::convert_crlf_to_lf;

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
}
