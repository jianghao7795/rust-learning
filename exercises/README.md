# Rust 分阶段练习

这里包含 `docs/` 下每个教学 Markdown 对应的独立项目。所有项目都有可运行入口和自动化测试。

| 教学 Markdown | 项目目录 | 项目 |
| --- | --- | --- |
| `00-environment.md` | [`00_hello_cargo`](00_hello_cargo/) | Cargo 问候程序 |
| `01-basics.md` | [`01_basics`](01_basics/) | 命令行计算器 |
| `02-ownership.md` | [`02_ownership`](02_ownership/) | 所有权与借用工具集 |
| `03-data-modeling.md` | [`03_todo`](03_todo/) | 内存待办事项 |
| `04-collections-traits.md` | [`04_text_stats`](04_text_stats/) | 文本统计器 |
| `05-errors-tests.md` | [`05_file_stats`](05_file_stats/) | 文件统计器 |
| `06-engineering.md` | [`06_engineering`](06_engineering/) | 分层待办事项应用 |
| `07-concurrency-async.md` | [`07_concurrency`](07_concurrency/) | 线程任务执行器 |
| `07-concurrency-async.md` | [`07_async_tasks`](07_async_tasks/) | Tokio 异步任务调度器 |
| `08-project.md` | [`08_log_analyzer`](08_log_analyzer/) | 命令行日志分析器 |
| `08-project.md` | [`08_todo_api`](08_todo_api/) | 待办事项 Web API |
| `08-project.md` | [`08_kv_store`](08_kv_store/) | 本地键值存储 |

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
