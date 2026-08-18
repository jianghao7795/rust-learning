# 阶段 8 项目三：本地键值存储

这是 `docs/08-project.md` 的方向 C。项目实现 `set`、`get`、`remove`，并用追加式二进制日志保存数据；重新启动后会重放日志恢复索引。

```bash
cargo run -p stage08-kv-store -- /tmp/learning.kv set language Rust
cargo run -p stage08-kv-store -- /tmp/learning.kv get language
cargo run -p stage08-kv-store -- /tmp/learning.kv remove language
cargo test -p stage08-kv-store
```

记录格式为操作码、键长度、值长度、键和值。库代码负责持久化和恢复，命令行入口只负责参数与输出。
