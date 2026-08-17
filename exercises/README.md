# Rust 分阶段练习

这里包含阶段 1～8 的独立练习项目。阶段 0 是环境准备，没有单独的代码项目。

| 目录 | 练习 |
| --- | --- |
| `01_basics` | 基础语法与计算器 |
| `02_ownership` | 所有权、借用与切片 |
| `03_todo` | 结构体、枚举与待办事项 |
| `04_text_stats` | 集合、迭代器与文本统计 |
| `05_file_stats` | 错误处理与文件统计 |
| `06_engineering` | 模块化与工程组织 |
| `07_concurrency` | 线程、Channel 与并发任务 |
| `08_log_analyzer` | 综合项目：日志分析器 |

在本目录运行全部检查：

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

运行单个练习：

```bash
cargo run -p stage01-basics
cargo test -p stage01-basics
```
