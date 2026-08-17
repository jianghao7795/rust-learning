# 阶段 2 练习：所有权与借用

练习 `String`、`&str`、可变借用、切片和生命周期关系。

运行：

```bash
cargo run -p stage02-ownership
cargo test -p stage02-ownership
```

## 动手任务

1. 不使用 `clone()` 实现返回最后一个单词的 `last_word`。
2. 实现 `shorter`，返回两个字符串切片中较短的一个。
3. 实现 `largest_mut`，返回可修改的最大元素引用。
4. 分别制造 move 后使用、重叠可变借用和悬垂引用错误，阅读 `cargo check` 的提示后修复。
5. 解释每个函数为什么使用 `&str`、`&mut String` 或切片。
