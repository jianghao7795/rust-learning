# 阶段 6 练习：工程化待办事项

练习模型、存储和服务分层，以及 trait、`Box<T>`、库与可执行程序的组织方式。

运行：

```bash
cargo run -p stage06-engineering
cargo test -p stage06-engineering
cargo doc -p stage06-engineering --no-deps
```

## 目录职责

- `model.rs`：任务数据。
- `repository.rs`：存储接口和内存实现。
- `service.rs`：业务规则。
- `main.rs`：创建依赖并展示结果。

## 动手任务

1. 实现一个文件版 `TaskRepository`。
2. 增加 `rename` 业务操作和测试。
3. 给空标题增加独立错误类型。
4. 把 repository 和 service 拆成两个 workspace package。
5. 给所有公共 API 添加带示例的文档注释并运行文档测试。
