# 阶段 2：所有权、借用与生命周期

目标：理解 Rust 如何在没有垃圾回收器的情况下保证内存安全。这是整套教程最重要的阶段，建议用时一到两周。

创建练习项目：

```bash
cargo new stage02_ownership
cd stage02_ownership
```

## 1. 所有权规则

先记住三条规则：

1. Rust 中每个值都有一个所有者。
2. 同一时间只能有一个所有者。
3. 所有者离开作用域时，值会被释放。

```rust
fn main() {
    {
        let message = String::from("hello");
        println!("{message}");
    } // message 离开作用域，String 的内存被释放
}
```

## 2. Move、Copy 与 Clone

`String` 的赋值会转移所有权：

```rust,compile_fail
let first = String::from("hello");
let second = first;
println!("{first}"); // 错误：first 的值已经移动
```

如果确实需要两份独立数据，可以显式复制：

```rust
let first = String::from("hello");
let second = first.clone();
println!("{first}, {second}");
```

整数、布尔值和 `char` 等简单栈上类型通常实现了 `Copy`：

```rust
let first = 10;
let second = first;
println!("{first}, {second}");
```

不要为了绕过编译器而到处使用 `clone()`。先判断函数是否只需要借用数据。

## 3. 借用

函数只读取字符串时，接收 `&str` 通常比接收 `String` 更灵活：

```rust
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() {
    let text = String::from("learn rust step by step");
    let count = word_count(&text);

    println!("{text}");
    println!("单词数：{count}");
}
```

`&text` 创建引用，不取得 `text` 的所有权，所以调用后仍能使用原值。

## 4. 可变借用

```rust
fn add_period(text: &mut String) {
    text.push('。');
}

fn main() {
    let mut text = String::from("正在学习 Rust");
    add_period(&mut text);
    println!("{text}");
}
```

同一段有效使用区间内：

- 可以有多个不可变引用。
- 或者只能有一个可变引用。
- 不能同时使用可变引用和不可变引用。

这些限制防止数据竞争和迭代器失效等问题。

## 5. 切片

切片是对连续数据的借用：

```rust
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

fn main() {
    let sentence = String::from("hello rust");
    let word = first_word(&sentence);
    println!("{word}");
}
```

数组切片的写法类似：

```rust
fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}
```

参数使用 `&str` 和 `&[T]`，通常既能接收完整值的引用，也能接收其中一部分。

## 6. 生命周期

大多数生命周期可以由编译器推断。只有引用之间的关系不明确时才需要标注：

```rust
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}
```

`'a` 表示返回引用的有效期不能超过两个输入引用中较短的那个。它描述引用之间的关系，不会让任何数据活得更久。

下面的函数无法成立：

```rust
fn invalid_reference() -> &str {
    let text = String::from("temporary");
    &text
}
```

`text` 会在函数结束时释放，不能返回指向它的引用。此时应该直接返回 `String`。

## 7. 函数参数如何选择

| 需求 | 推荐参数 |
| --- | --- |
| 只读取文本 | `&str` |
| 修改调用者的文本 | `&mut String` |
| 函数需要保存或取得文本所有权 | `String` |
| 只读取一组元素 | `&[T]` |
| 修改一组已有元素 | `&mut [T]` |

## 必做练习

独立实现：

```rust
fn first_word(text: &str) -> &str {
    todo!()
}

fn append_world(text: &mut String) {
    todo!()
}

fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    todo!()
}

fn largest(values: &[i32]) -> Option<&i32> {
    todo!()
}
```

还要故意制造并理解四种错误：

1. 值移动后再次使用。
2. 两个可变引用的有效使用区间重叠。
3. 可变引用和不可变引用的有效使用区间重叠。
4. 返回指向局部变量的引用。

每次只保留一个错误，运行 `cargo check`，阅读完整提示后再修复。

## 完成标准

- 能画出 `String` move 前后的所有者变化。
- 能解释 `Copy` 和 `Clone` 的区别。
- 能根据函数需求选择 `String`、`&str` 或 `&mut String`。
- 知道生命周期标注约束引用关系，不负责延长数据寿命。
- 所有练习通过测试，并且没有用不必要的 `clone()`。

完成后进入[阶段 3：结构体、枚举与模块](03-data-modeling.md)。
