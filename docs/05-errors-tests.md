# 阶段 5：错误处理与测试

目标：正确区分可恢复与不可恢复错误，编写能说明程序行为的测试。本阶段建议用时一周。

创建项目：

```bash
cargo new stage05_file_stats
cd stage05_file_stats
```

## 1. `panic!` 与 `Result`

- 程序进入无法继续且通常表示代码缺陷的状态时，可以 `panic!`。
- 文件不存在、输入错误、网络失败等预期内失败，应返回 `Result<T, E>`。

读取文件：

```rust
use std::fs;
use std::io;

fn read_text(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
```

`Result` 有两种情况：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 2. 使用 `?` 传播错误

```rust
use std::fs;
use std::io;

fn first_line(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    let line = content.lines().next().unwrap_or_default();
    Ok(line.to_string())
}
```

如果读取失败，`?` 会提前返回错误；成功时则取出 `Ok` 内的值继续运行。`?` 不是忽略错误，而是把错误交给调用者。

## 3. 自定义错误

当函数可能失败于多种原因时，可以建立领域错误：

```rust
use std::fmt;
use std::io;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    EmptyFile,
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "文件操作失败：{error}"),
            Self::EmptyFile => write!(formatter, "文件内容为空"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
```

实现 `From<io::Error>` 后，返回 `AppError` 的函数可以对 I/O 结果使用 `?`。

应用程序也可以使用成熟的错误处理 crate，但应先理解标准库的 `Result`、`Error` 和 `From`。

## 4. 不要滥用 `unwrap`

`unwrap()` 和 `expect()` 在 `Err` 或 `None` 时会 panic。适用场景主要是：

- 测试代码。
- 已由程序逻辑严格保证不可能失败的情况，并且原因清晰。
- 快速原型，之后需要改成正式错误处理。

处理用户输入、文件和网络时，通常应该传播或显式处理错误。

## 5. 单元测试

```rust
fn divide(left: f64, right: f64) -> Result<f64, &'static str> {
    if right == 0.0 {
        Err("除数不能为零")
    } else {
        Ok(left / right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_numbers() {
        assert_eq!(divide(8.0, 2.0), Ok(4.0));
    }

    #[test]
    fn rejects_zero_divisor() {
        assert_eq!(divide(8.0, 0.0), Err("除数不能为零"));
    }
}
```

测试应包括：正常情况、边界情况、错误情况。测试名称要说明行为，而不是只叫 `test1`。

## 6. 集成测试

把公共行为测试放在项目根目录的 `tests/` 中：

```text
tests/
└── file_stats_test.rs
```

集成测试只能调用 library crate 的公共 API。项目同时包含可执行程序和可测试逻辑时，推荐结构：

```text
src/
├── lib.rs
└── main.rs
```

把业务逻辑放进 `lib.rs` 或其子模块，`main.rs` 只负责参数、输出和退出码。

## 阶段项目：文件文本统计器

把阶段 4 的文本统计器升级为命令行文件工具：

```bash
cargo run -- notes.txt
```

要求：

1. 从第一个命令行参数获得文件路径。
2. 未提供路径时显示用法并返回非零退出码。
3. 文件不存在或无权限时显示具体错误。
4. 核心分析函数不直接读文件，保持为可独立测试的纯逻辑。
5. 单元测试覆盖统计逻辑，集成测试覆盖至少一个文件读取流程。
6. 正常输出行数、单词数、字节数和最高频单词。

可以使用标准库：

```rust
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = env::args().nth(1) else {
        eprintln!("用法：stage05_file_stats <文件路径>");
        return ExitCode::FAILURE;
    };

    // 调用库中的函数并处理 Result
    println!("分析文件：{path}");
    ExitCode::SUCCESS
}
```

## 完成标准

- 能判断失败应该 panic 还是返回 `Result`。
- 会用 `?` 传播错误，并能读懂错误类型转换。
- 正常、边界和错误路径都有测试。
- 能把业务逻辑与命令行 I/O 分离。
- 项目通过格式化、Clippy 和全部测试。

完成后进入[阶段 6：智能指针与项目工程化](06-engineering.md)。
