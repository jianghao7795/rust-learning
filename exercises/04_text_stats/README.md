# 阶段 4 练习：文本统计器

练习 `String`、`HashMap`、闭包、迭代器和 trait。

运行：

```bash
cargo run -p stage04-text-stats
cargo test -p stage04-text-stats
```

## 动手任务

1. 增加字符数统计，并与字节数进行比较。
2. 忽略单词首尾的常见标点符号。
3. 实现返回出现频率最高的前 N 个单词。
4. 定义 `Report` trait，为 `TextStats` 实现 Markdown 报告。
5. 使用 `filter`、`map` 和 `collect` 重写一段显式循环，并比较可读性。
