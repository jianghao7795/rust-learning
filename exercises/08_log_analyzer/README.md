# 阶段 8 项目一：日志分析器

读取格式为 `<时间> <级别> <消息>` 的日志，统计日志级别并列出错误消息。

运行：

```bash
cargo run -p stage08-log-analyzer -- 08_log_analyzer/tests/fixtures/app.log
cargo test -p stage08-log-analyzer
```

命令需要在 `exercises` 目录中执行。

## 动手任务

1. 支持 `--level ERROR` 参数，只显示指定级别。
2. 统计相同错误消息的出现次数。
3. 支持同时读取多个日志文件。
4. 增加 JSON 输出格式。
5. 对超大文件改成逐行读取，避免一次载入全部内容。
6. 为命令行成功和失败退出码增加端到端测试。

同一教学章节还提供[待办事项 Web API](../08_todo_api/)和[本地键值存储](../08_kv_store/)。
