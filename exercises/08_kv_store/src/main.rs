use std::env;
use std::process::ExitCode;

use stage08_kv_store::KvStore;

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("错误：{error}");
            eprintln!("用法：stage08-kv-store <文件> <set|get|remove> <键> [值]");
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let [path, command, key, rest @ ..] = arguments.as_slice() else {
        return Err("参数数量不正确".into());
    };
    let mut store = KvStore::open(path)?;

    match (command.as_str(), rest) {
        ("set", [value]) => {
            store.set(key, value)?;
            println!("OK");
        }
        ("get", []) => match store.get(key) {
            Some(value) => println!("{value}"),
            None => println!("(不存在)"),
        },
        ("remove", []) => match store.remove(key)? {
            Some(_) => println!("OK"),
            None => println!("(不存在)"),
        },
        _ => return Err("命令或参数不正确".into()),
    }
    Ok(())
}
