# 阶段 4：集合、迭代器、泛型与 Trait

目标：处理多条数据，使用迭代器表达数据转换，并通过泛型和 trait 复用代码。本阶段建议用时一周。

创建项目：

```bash
cargo new stage04_text_stats
cd stage04_text_stats
```

## 1. 常用集合

### `Vec<T>`

```rust
let mut scores = vec![80, 90, 75];
scores.push(100);

for score in &scores {
    println!("{score}");
}
```

### `String`

```rust
let mut text = String::from("Rust");
text.push(' ');
text.push_str("语言");
```

Rust 字符串是 UTF-8，不能直接使用整数下标。根据需求使用 `chars()`、`bytes()` 或切片，并注意切片边界必须位于有效的 UTF-8 边界。

### `HashMap<K, V>`

```rust
use std::collections::HashMap;

let mut counts = HashMap::new();

for word in ["rust", "safe", "rust"] {
    *counts.entry(word).or_insert(0) += 1;
}

assert_eq!(counts.get("rust"), Some(&2));
```

## 2. 闭包

闭包是可以捕获周围环境的匿名函数：

```rust
let minimum = 60;
let passed = |score: &i32| *score >= minimum;

assert!(passed(&80));
```

闭包常作为迭代器方法的参数。

## 3. 迭代器

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let squares: Vec<i32> = numbers
    .iter()
    .filter(|number| **number % 2 == 0)
    .map(|number| number * number)
    .collect();

assert_eq!(squares, vec![4, 16, 36]);
```

三种遍历方式对所有权的影响：

| 方法 | 典型元素类型 | 结果 |
| --- | --- | --- |
| `iter()` | `&T` | 只读借用集合 |
| `iter_mut()` | `&mut T` | 可修改集合元素 |
| `into_iter()` | `T` | 消耗集合并取得元素所有权 |

常用方法：

- `map`：把每个元素转换成另一个值。
- `filter`：只保留满足条件的元素。
- `find`：查找第一个满足条件的元素。
- `any`、`all`：判断是否存在或是否全部满足条件。
- `collect`：收集成集合。
- `fold`、`sum`：聚合数据。

迭代器是惰性的，只有调用 `collect`、`sum` 等消费方法后才真正执行。

## 4. 泛型

泛型让同一结构支持不同类型：

```rust
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

fn first<T>(values: &[T]) -> Option<&T> {
    values.first()
}
```

泛型不是“任何操作都能做”。只有加入 trait bound 后，才能使用相应能力。

## 5. Trait

Trait 描述共享行为：

```rust
trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("{} - {}", self.title, self.author)
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}
```

等价的泛型约束写法：

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summary());
}
```

先熟悉 `Debug`、`Display`、`Clone`、`Default`、`From` 和 `Iterator` 等标准 trait。

## 阶段项目：文本统计器

实现：

```rust
#[derive(Debug, PartialEq)]
struct TextStats {
    lines: usize,
    words: usize,
    bytes: usize,
    frequencies: HashMap<String, usize>,
}

fn analyze(text: &str) -> TextStats {
    todo!()
}
```

要求：

1. 统计行数、单词数和字节数。
2. 单词统一转换为小写后统计频率。
3. 提供返回出现次数最多单词的函数。
4. 空文本不能发生 panic。
5. 为英文、多行文本、重复单词和空文本写测试。

## 额外练习

1. 计算整数列表的平均数、中位数和众数。
2. 给阶段 3 的待办事项增加筛选和排序。
3. 定义 `Report` trait，让 `TextStats` 和待办事项统计都能生成文本报告。

## 完成标准

- 能根据需求选择 `Vec`、`String` 或 `HashMap`。
- 能区分 `iter`、`iter_mut` 和 `into_iter`。
- 会组合 `filter`、`map` 和 `collect`。
- 能解释泛型负责复用类型结构，trait 负责描述行为约束。
- 文本统计器通过格式化、Clippy 和全部测试。

完成后进入[阶段 5：错误处理与测试](05-errors-tests.md)。

## 配套项目

[文本统计器](../exercises/04_text_stats/)使用集合、迭代器、闭包和 trait 生成统计报告。
