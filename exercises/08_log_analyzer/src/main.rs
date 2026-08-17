use std::env;
use std::fs;
use std::process::ExitCode;

use stage08_log_analyzer::{Level, analyze};

fn main() -> ExitCode {
    let Some(path) = env::args().nth(1) else {
        eprintln!("用法：stage08-log-analyzer <日志文件>");
        return ExitCode::FAILURE;
    };

    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("无法读取 {path}：{error}");
            return ExitCode::FAILURE;
        }
    };
    let summary = analyze(&text);

    println!("文件：{path}");
    println!("有效行：{}", summary.valid_lines);
    println!("无效行：{}", summary.invalid_lines);
    for level in [Level::Debug, Level::Info, Level::Warn, Level::Error] {
        println!(
            "{level}: {}",
            summary.level_counts.get(&level).unwrap_or(&0)
        );
    }
    if !summary.errors.is_empty() {
        println!("错误消息：");
        for message in summary.errors {
            println!("- {message}");
        }
    }

    ExitCode::SUCCESS
}
