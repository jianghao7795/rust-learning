# 阶段 5 练习：文件统计器

练习 `Result`、`?`、自定义错误、单元测试和集成测试。

运行示例：

```bash
cargo run -p stage05-file-stats -- 05_file_stats/tests/fixtures/sample.txt
cargo test -p stage05-file-stats
```

命令需要在 `exercises` 目录中执行。

## 动手任务

1. 给空文件增加 `EmptyFile` 错误。
2. 支持一次分析多个文件，并为每个文件独立报告错误。
3. 把输出格式抽象为 `ReportFormat` 枚举。
4. 添加文件不存在的集成测试。
5. 检查业务代码中的 `unwrap()`，把可恢复失败改为错误传播。
