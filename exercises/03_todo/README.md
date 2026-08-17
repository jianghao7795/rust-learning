# 阶段 3 练习：内存待办事项

练习结构体、枚举、方法、`Option`、模式匹配和模块。

运行：

```bash
cargo run -p stage03-todo
cargo test -p stage03-todo
```

## 动手任务

1. 给任务增加 `priority` 枚举，支持低、中、高三级。
2. 实现按完成状态筛选任务的方法。
3. 实现 `rename(id, title)`，任务不存在时返回 `false`。
4. 给 `Command` 增加 `Rename` 变体并在 `execute` 中处理。
5. 保证空标题不能被添加，并为这个规则编写测试。
