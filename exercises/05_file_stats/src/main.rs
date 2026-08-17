use std::env;
use std::process::ExitCode;

use stage04_text_stats::Report;
use stage05_file_stats::analyze_file;

fn main() -> ExitCode {
    let Some(path) = env::args().nth(1) else {
        eprintln!("用法：stage05-file-stats <文件路径>");
        return ExitCode::FAILURE;
    };

    match analyze_file(&path) {
        Ok(stats) => {
            println!("文件：{path}\n{}", stats.report());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
