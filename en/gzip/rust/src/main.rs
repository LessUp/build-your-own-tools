use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;
use rgzip::default_output_for_compress;
use rgzip::default_output_for_decompress;
use rgzip::ensure_writable;
use rgzip::same_path;
use rgzip::{compress_path, compress_reader_to_path, compress_reader_to_writer, sanitize_level};
use rgzip::{decompress_path, decompress_reader_to_path, decompress_reader_to_writer};

#[derive(Parser, Debug)]
#[command(name = "rgzip", version, about = "A simple gzip CLI written in Rust")]
struct Cli {
    /// 解压模式（默认压缩）
    #[arg(short = 'd', long = "decompress")]
    decompress: bool,

    /// 指定输出文件路径（默认：压缩为 <INPUT>.gz；解压为去掉 .gz 或追加 .out）
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// 覆盖已有输出文件
    #[arg(short = 'f', long = "force")]
    force: bool,

    /// 压缩级别（0-9，默认 6）
    #[arg(short = 'l', long = "level", default_value_t = 6)]
    level: u32,

    /// 保留源文件（默认成功后删除源文件）
    #[arg(short = 'k', long = "keep")]
    keep: bool,

    /// 输入文件（留空则使用标准输入）
    input: Option<PathBuf>,
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn real_main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.decompress {
        run_decompress(cli)
    } else {
        run_compress(cli)
    }
}

fn run_compress(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let level = sanitize_level(cli.level);
    match cli.input {
        Some(input) => {
            if !input.exists() {
                return Err(format!("输入文件不存在: {}", input.display()).into());
            }
            let output = match cli.output {
                Some(p) => p,
                None => default_output_for_compress(&input),
            };
            ensure_writable(&output, cli.force)?;
            if same_path(&input, &output) {
                return Err("输出路径与输入文件相同".into());
            }
            compress_path(&input, &output, level)?;
            if !cli.keep {
                fs::remove_file(&input)?;
            }
        }
        None => {
            // stdin -> (output|stdout)
            if let Some(out_path) = cli.output {
                ensure_writable(&out_path, cli.force)?;
                let stdin = io::stdin();
                let reader = stdin.lock();
                compress_reader_to_path(reader, &out_path, level)?;
            } else {
                // stdout
                let stdin = io::stdin();
                let reader = stdin.lock();
                let stdout = io::stdout();
                let handle = stdout.lock();
                let _ = compress_reader_to_writer(reader, handle, level)?;
            }
        }
    }
    Ok(())
}

fn run_decompress(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.input {
        Some(input) => {
            if !input.exists() {
                return Err(format!("输入文件不存在: {}", input.display()).into());
            }
            let output = match cli.output {
                Some(p) => p,
                None => default_output_for_decompress(&input),
            };
            ensure_writable(&output, cli.force)?;
            if same_path(&input, &output) {
                return Err("输出路径与输入文件相同".into());
            }
            decompress_path(&input, &output)?;
            if !cli.keep {
                fs::remove_file(&input)?;
            }
        }
        None => {
            // stdin -> (output|stdout)
            if let Some(out_path) = cli.output {
                ensure_writable(&out_path, cli.force)?;
                let stdin = io::stdin();
                let reader = stdin.lock();
                decompress_reader_to_path(reader, &out_path)?;
            } else {
                // stdout
                let stdin = io::stdin();
                let stdout = io::stdout();
                let out = stdout.lock();
                decompress_reader_to_writer(stdin.lock(), out)?;
            }
        }
    }
    Ok(())
}
