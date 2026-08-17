# 阶段 0：环境与 Cargo

目标：安装 Rust，理解工具链，并独立创建、编译和运行第一个项目。

## 1. 安装 Rust

通过 Rust 官方安装页安装 `rustup`：<https://www.rust-lang.org/tools/install>

安装后检查版本：

```bash
rustc --version
cargo --version
rustup --version
```

三个工具的作用：

- `rustc`：Rust 编译器，负责把源代码编译成程序。
- `cargo`：项目管理工具，负责创建项目、管理依赖、编译和测试。
- `rustup`：工具链管理器，负责安装和切换 Rust 版本。

安装代码格式化和静态检查组件：

```bash
rustup component add rustfmt clippy
```

推荐使用 VS Code，并安装 `rust-analyzer` 扩展。

## 2. 创建第一个项目

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

项目结构：

```text
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

- `Cargo.toml` 保存项目名称、版本、Rust edition 和依赖。
- `src/main.rs` 是可执行程序的入口。
- 第一次编译后生成的 `target/` 存放构建产物，不应手动修改。

打开 `src/main.rs`，改成：

```rust
fn main() {
    let language = "Rust";
    println!("Hello, {language}!");
}
```

执行 `cargo run`，应该看到：

```text
Hello, Rust!
```

## 3. 掌握常用命令

| 命令 | 作用 |
| --- | --- |
| `cargo new demo` | 创建可执行项目 |
| `cargo new --lib demo` | 创建库项目 |
| `cargo check` | 快速检查类型和语法 |
| `cargo run` | 编译并运行 |
| `cargo build` | 构建调试版本 |
| `cargo build --release` | 构建优化后的发布版本 |
| `cargo test` | 运行测试 |
| `cargo fmt` | 格式化代码 |
| `cargo clippy` | 检查常见问题 |
| `cargo doc --open` | 生成并打开依赖文档 |

开发时优先使用 `cargo check`，因为它通常比完整构建更快。

## 4. 第一个练习

让程序输出三行内容：你的名字、学习 Rust 的原因、今天的学习时间。

要求：

- 至少声明两个变量。
- 使用变量插值输出内容。
- 执行 `cargo fmt` 和 `cargo run`。

## 完成标准

- 能解释 `rustc`、Cargo 和 rustup 的区别。
- 不看资料也能使用 `cargo new` 创建并运行项目。
- 知道源代码、项目配置和构建产物分别在哪个目录。

完成后进入[阶段 1：变量、类型、函数与流程控制](01-basics.md)。
