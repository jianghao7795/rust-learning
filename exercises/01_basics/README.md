# 阶段 1 练习：基础计算器

练习变量、基本类型、函数、`if`、`match`、循环和 `Option`。

运行：

```bash
cargo run -p stage01-basics -- 12.5 '*' 4
cargo test -p stage01-basics
```

## 动手任务

1. 给 `calculate` 增加取余 `%` 运算。
2. 给新增运算补充正常和除数为零测试。
3. 使用 `for` 输出 1～100 的 FizzBuzz。
4. 实现摄氏温度与华氏温度互转函数。
5. 修改命令行输入，支持连续计算多个表达式。

完成后运行 `cargo clippy -p stage01-basics --all-targets -- -D warnings`。
