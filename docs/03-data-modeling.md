# 阶段 3：结构体、枚举与模块

目标：使用 Rust 的类型系统描述业务数据，并把代码拆成清晰的模块。本阶段建议用时一周。

创建项目：

```bash
cargo new stage03_todo
cd stage03_todo
```

## 1. 结构体与方法

结构体把相关数据组合成一个类型：

```rust
#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }

    fn is_completed(&self) -> bool {
        self.completed
    }
}
```

- `Task::new` 是关联函数，常用于构造值。
- `&self` 只读借用当前值。
- `&mut self` 可以修改当前值。
- `self` 会取得当前值的所有权。
- `#[derive(Debug)]` 让类型可以使用 `{:?}` 调试输出。

## 2. 枚举与模式匹配

枚举适合表示多种互斥状态：

```rust
enum Command {
    Add(String),
    List,
    Complete(u32),
    Remove(u32),
}

fn describe(command: &Command) -> &str {
    match command {
        Command::Add(_) => "添加任务",
        Command::List => "列出任务",
        Command::Complete(_) => "完成任务",
        Command::Remove(_) => "删除任务",
    }
}
```

`match` 必须覆盖全部可能情况。分支中的 `_` 忽略不需要的数据。

## 3. `Option<T>`

Rust 没有普通的 `null`。可能没有结果时使用：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

查找任务：

```rust
fn find_task(tasks: &[Task], id: u32) -> Option<&Task> {
    tasks.iter().find(|task| task.id == id)
}
```

处理 `Option` 的几种方式：

```rust
match find_task(&tasks, 1) {
    Some(task) => println!("找到：{}", task.title),
    None => println!("任务不存在"),
}

if let Some(task) = find_task(&tasks, 1) {
    println!("找到：{}", task.title);
}
```

当两种分支都有重要行为时使用 `match`；只关注一种情况时可以使用 `if let`。

## 4. 模块

把项目拆成：

```text
src/
├── main.rs
├── command.rs
└── task.rs
```

`src/task.rs`：

```rust
#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    completed: bool,
}

impl Task {
    pub fn new(id: u32, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
```

`src/main.rs` 使用 `mod` 声明模块：

```rust
mod task;

use task::Task;

fn main() {
    let task = Task::new(1, "学习结构体");
    println!("{task:?}");
}
```

默认内容仅模块内部可见。只把其他模块确实需要的类型和方法标为 `pub`。

## 阶段项目：内存版待办事项

实现一个 `TodoList`：

```rust
struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}
```

需要实现：

```rust
impl TodoList {
    fn new() -> Self;
    fn add(&mut self, title: String) -> u32;
    fn list(&self) -> &[Task];
    fn complete(&mut self, id: u32) -> bool;
    fn remove(&mut self, id: u32) -> Option<Task>;
}
```

要求：

1. 新任务的 ID 不能重复。
2. 完成不存在的任务时返回 `false`。
3. 删除任务时返回被删除的任务，不存在则返回 `None`。
4. `Task` 和 `TodoList` 放在独立模块中。
5. 为添加、完成、删除分别编写测试。

## 完成标准

- 会使用结构体保存相关字段，使用 `impl` 实现行为。
- 会使用枚举表示有限状态或命令。
- 会使用 `Option` 表示“可能不存在”。
- 能通过模块和可见性组织代码。
- 待办事项项目通过格式化、Clippy 和全部测试。

完成后进入[阶段 4：集合、迭代器、泛型与 Trait](04-collections-traits.md)。

## 配套项目

[内存待办事项](../exercises/03_todo/)使用结构体、枚举、模式匹配和模块组织完整业务流程。
