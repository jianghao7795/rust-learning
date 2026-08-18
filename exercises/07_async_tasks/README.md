# 阶段 7 项目二：异步任务调度器

这是 `docs/07-concurrency-async.md` 的异步配套项目。它用 Tokio 并发执行模拟 I/O 任务，限制并发数，并分别统计成功、超时和失败。

```bash
cargo run -p stage07-async-tasks
cargo test -p stage07-async-tasks
```

重点观察 `async fn`、`.await`、`tokio::spawn` 和 `timeout` 如何配合，以及限制并发为什么不同于限制线程数量。
