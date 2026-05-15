use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

use anyhow::Result;
use dos2unix::{contains_crlf, memory, stream};

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

fn process_stdin_stdout(quiet: bool, check_only: bool) -> Result<bool> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let converted = memory(&buf);
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
        let converted = memory(&full_check);
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

    let result = stream(&mut full_reader, &mut writer)?;
    writer.flush()?;

    if result.has_crlf {
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

    Ok(result.has_crlf)
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
