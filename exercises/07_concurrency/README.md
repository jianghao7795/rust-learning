# 阶段 7 练习：并发任务执行器

练习线程、`move` 闭包、Channel、`Arc` 和原子类型。程序使用固定数量的工作线程计算整数平方。

运行：

```bash
cargo run -p stage07-concurrency
cargo test -p stage07-concurrency
```

## 动手任务

1. 在线程中打印 worker ID，观察任务分配。
2. 增加立方计算模式。
3. 使用 `Arc<Mutex<Vec<_>>>` 重写结果收集，再比较两种方案。
4. 增加任务取消标志。
5. 单独创建异步练习，使用 Tokio 为模拟任务增加超时和并发限制。
