# 阶段 1：变量、类型、函数与流程控制

目标：掌握 Rust 基础语法，完成一个命令行计算器。本阶段建议用时一周。

创建练习项目：

```bash
cargo new stage01_basics
cd stage01_basics
```

## 1. 变量与可变性

Rust 变量默认不可修改：

```rust
fn main() {
    let name = "Rust";
    let mut days = 1;
    days += 1;

    const HOURS_PER_DAY: u32 = 24;
    println!("学习 {name} 的第 {days} 天，一天有 {HOURS_PER_DAY} 小时");
}
```

- `let` 声明变量。
- `mut` 允许修改变量的值。
- `const` 声明编译期常量，必须标注类型，名称通常使用大写下划线形式。
- 可以使用同名的 `let` 创建新变量，这叫变量遮蔽。

```rust
let spaces = "   ";
let spaces = spaces.len();
```

遮蔽可以改变类型，`mut` 修改值时不能改变类型。

## 2. 常用数据类型

```rust
fn main() {
    let count: i32 = 42;
    let price: f64 = 19.9;
    let enabled: bool = true;
    let letter: char = '中';

    let point: (i32, i32) = (10, 20);
    let numbers: [i32; 4] = [1, 2, 3, 4];

    println!("{} {} {} {}", count, price, enabled, letter);
    println!("x={}, y={}, first={}", point.0, point.1, numbers[0]);
}
```

先掌握：

- 有符号整数：`i8`、`i16`、`i32`、`i64`、`i128`、`isize`。
- 无符号整数：对应的 `u8` 到 `usize`。
- 浮点数：`f32` 和通常使用的 `f64`。
- 复合类型：固定长度数组和元组。
- `char` 表示一个 Unicode 标量值，不等同于单字节字符。

## 3. 函数、语句和表达式

```rust
fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    let answer = add(20, 22);
    println!("answer = {answer}");
}
```

函数参数必须写类型。返回值写在 `->` 后面。`left + right` 没有分号，是一个表达式；加分号后会变成语句，不再返回这个值。

## 4. 流程控制

```rust
fn describe(number: i32) -> &'static str {
    if number > 0 {
        "正数"
    } else if number < 0 {
        "负数"
    } else {
        "零"
    }
}

fn main() {
    for number in 1..=5 {
        println!("{number}: {}", describe(number));
    }

    let mut count = 3;
    while count > 0 {
        println!("{count}");
        count -= 1;
    }

    let result = loop {
        break 42;
    };
    println!("result = {result}");
}
```

`if` 和 `loop` 都可以产生值。`1..5` 不包含 5，`1..=5` 包含 5。

## 5. `String` 与 `&str` 初识

```rust
fn print_length(text: &str) {
    println!("{text} 的字节长度是 {}", text.len());
}

fn main() {
    let literal: &str = "hello";
    let mut owned: String = String::from("hello");
    owned.push_str(" rust");

    print_length(literal);
    print_length(&owned);
}
```

- 字符串字面量通常是 `&str`。
- `String` 拥有可增长的 UTF-8 文本。
- `len()` 返回字节数，不是人眼看到的字符数。

所有权区别会在阶段 2 详细学习。

## 6. 阶段项目：命令行计算器

先实现纯函数，不急着读取用户输入：

```rust
fn calculate(left: f64, operator: char, right: f64) -> Option<f64> {
    match operator {
        '+' => Some(left + right),
        '-' => Some(left - right),
        '*' => Some(left * right),
        '/' if right != 0.0 => Some(left / right),
        _ => None,
    }
}

fn main() {
    match calculate(12.0, '*', 3.0) {
        Some(result) => println!("结果：{result}"),
        None => println!("运算无效"),
    }
}
```

继续完成这些要求：

1. 支持加、减、乘、除。
2. 除数为零时返回 `None`。
3. 无效运算符返回 `None`。
4. 给 `calculate` 添加至少 4 个测试。

测试示例：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(calculate(2.0, '+', 3.0), Some(5.0));
    }
}
```

## 额外练习

1. 摄氏温度和华氏温度互转。
2. 输出第 `n` 个 Fibonacci 数。
3. 输出 1～100 的 FizzBuzz。
4. 计算一个整数数组中的最大值。

## 完成标准

- 能解释 `let`、`mut`、`const` 和变量遮蔽。
- 能写带参数和返回值的函数。
- 会使用 `if`、`match`、`for`、`while` 和 `loop`。
- 计算器通过 `cargo fmt --check`、`cargo clippy -- -D warnings` 和 `cargo test`。

完成后进入[阶段 2：所有权、借用与生命周期](02-ownership.md)。

## 配套项目

[命令行计算器](../exercises/01_basics/)综合使用变量、类型、函数、流程控制和 `Option`。
