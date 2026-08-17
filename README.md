# Rust 从零开始学习

这是一套可以按顺序完成的 Rust 学习教程。建议每天学习 1～2 小时，每完成一个阶段再进入下一个阶段。

## 学习阶段

| 阶段 | 主题 | 建议时间 | 阶段成果 |
| --- | --- | --- | --- |
| 0 | [环境与 Cargo](docs/00-environment.md) | 1 天 | 独立创建并运行 Rust 项目 |
| 1 | [变量、类型、函数与流程控制](docs/01-basics.md) | 1 周 | 完成命令行计算器 |
| 2 | [所有权、借用与生命周期](docs/02-ownership.md) | 1～2 周 | 理解 Rust 的内存管理方式 |
| 3 | [结构体、枚举与模块](docs/03-data-modeling.md) | 1 周 | 完成内存版待办事项程序 |
| 4 | [集合、迭代器、泛型与 Trait](docs/04-collections-traits.md) | 1 周 | 完成文本统计器 |
| 5 | [错误处理与测试](docs/05-errors-tests.md) | 1 周 | 完成可靠的文件读取程序 |
| 6 | [智能指针与项目工程化](docs/06-engineering.md) | 1 周 | 组织多模块、可维护的项目 |
| 7 | [线程、并发与异步](docs/07-concurrency-async.md) | 1～2 周 | 完成并发任务执行器 |
| 8 | [综合项目](docs/08-project.md) | 2～3 周 | 独立交付一个完整 Rust 项目 |

## 配套练习目录

阶段 1～8 都有独立、可运行的 Cargo 项目，统一入口见 [exercises/README.md](exercises/README.md)。

| 阶段 | 练习目录 |
| --- | --- |
| 1 | [基础计算器](exercises/01_basics/) |
| 2 | [所有权与借用](exercises/02_ownership/) |
| 3 | [内存待办事项](exercises/03_todo/) |
| 4 | [文本统计器](exercises/04_text_stats/) |
| 5 | [文件统计器](exercises/05_file_stats/) |
| 6 | [工程化待办事项](exercises/06_engineering/) |
| 7 | [并发任务执行器](exercises/07_concurrency/) |
| 8 | [综合日志分析器](exercises/08_log_analyzer/) |

在 `exercises` 目录中可以一次检查全部练习：

```bash
cargo test --workspace
```

## 学习方法

每个阶段都按下面的顺序完成：

1. 阅读“核心知识”。
2. 手动输入并运行示例，不要只复制粘贴。
3. 完成练习，先自己思考再查看资料。
4. 执行阶段末尾的检查命令。
5. 能回答“完成标准”中的问题后再进入下一阶段。

每天 90 分钟可以这样安排：

- 20 分钟阅读知识点。
- 45 分钟写代码和练习。
- 15 分钟理解编译错误并修改。
- 10 分钟整理笔记和复习。

## 通用检查命令

养成在每个项目中执行这些命令的习惯：

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

遇到编译错误时，先处理第一条错误。认真阅读编译器给出的代码位置、原因和 `help`，修改后使用 `cargo check` 快速验证。

## 推荐资料

- Rust 官方教程：<https://doc.rust-lang.org/book/>
- Rust By Example：<https://doc.rust-lang.org/rust-by-example/>
- Rustlings 练习：<https://github.com/rust-lang/rustlings>
- Rust 标准库文档：<https://doc.rust-lang.org/std/>

现在从[阶段 0：环境与 Cargo](docs/00-environment.md)开始。
