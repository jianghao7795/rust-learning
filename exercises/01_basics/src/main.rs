use std::env;
use std::process::ExitCode;

use stage01_basics::calculate;

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.len() != 3 {
        eprintln!("用法：stage01-basics <数字> <+|-|*|/> <数字>");
        return ExitCode::FAILURE;
    }

    let Ok(left) = arguments[0].parse::<f64>() else {
        eprintln!("第一个参数不是有效数字");
        return ExitCode::FAILURE;
    };
    let Some(operator) = arguments[1].chars().next() else {
        eprintln!("运算符不能为空");
        return ExitCode::FAILURE;
    };
    let Ok(right) = arguments[2].parse::<f64>() else {
        eprintln!("第三个参数不是有效数字");
        return ExitCode::FAILURE;
    };

    match calculate(left, operator, right) {
        Some(result) => {
            println!("{left} {operator} {right} = {result}");
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("不支持的运算，或者除数为零");
            ExitCode::FAILURE
        }
    }
}
