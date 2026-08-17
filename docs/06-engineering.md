# 阶段 6：智能指针与项目工程化

目标：理解常见智能指针的使用边界，并把项目组织成可维护、可测试的结构。本阶段建议用时一周。

## 1. `Box<T>`

`Box<T>` 把值存放在堆上，栈上保存指针。常见用途是递归类型或需要固定大小的间接存储。

```rust
#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    End,
}

fn main() {
    let list = List::Node(
        1,
        Box::new(List::Node(2, Box::new(List::End))),
    );
    println!("{list:?}");
}
```

没有 `Box` 时，递归枚举的大小无法在编译期确定。

## 2. `Deref` 与 `Drop`

- `Deref` 决定解引用 `*value` 的行为，并支持解引用强制转换。
- `Drop` 在值离开作用域时执行清理。

`String` 可以传给需要 `&str` 的函数，就是解引用强制转换的常见例子。通常不需要自己实现这两个 trait，但要知道标准类型为什么能这样工作。

## 3. 共享所有权

### `Rc<T>`

`Rc<T>` 用于单线程中的共享只读所有权：

```rust
use std::rc::Rc;

let shared = Rc::new(String::from("shared"));
let first = Rc::clone(&shared);
let second = Rc::clone(&shared);

println!("引用数量：{}", Rc::strong_count(&shared));
println!("{first}, {second}");
```

### `Arc<T>`

`Arc<T>` 使用原子引用计数，适合跨线程共享所有权。原子操作有额外开销，单线程不需要为了“更强”而使用它。

## 4. 内部可变性

`RefCell<T>` 把借用规则从编译期移到运行时检查，适合单线程内部可变性：

```rust
use std::cell::RefCell;

let value = RefCell::new(vec![1, 2]);
value.borrow_mut().push(3);
assert_eq!(*value.borrow(), vec![1, 2, 3]);
```

违反借用规则时会在运行时 panic。不要把 `Rc<RefCell<T>>` 当作默认选择；先确认数据确实需要多个所有者并且共享时需要修改。

## 5. Package、Crate 与模块

- package：包含 `Cargo.toml` 的 Cargo 项目。
- crate：一个编译单元，可以是 library 或 binary。
- module：crate 内部组织和控制可见性的单元。

常见项目结构：

```text
todo_app/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── model.rs
│   ├── repository.rs
│   └── service.rs
└── tests/
    └── todo_flow.rs
```

推荐职责：

- `model`：数据类型。
- `repository`：数据读写。
- `service`：业务规则。
- `main`：参数解析、输入输出、组装依赖。

不要只因为文件变长就拆模块。模块边界应反映职责和依赖方向。

## 6. Workspace

多个相关 package 可以组成 workspace。根目录 `Cargo.toml`：

```toml
[workspace]
resolver = "3"
members = ["todo-core", "todo-cli"]
```

创建成员：

```bash
cargo new --lib todo-core
cargo new todo-cli
```

让 CLI 依赖核心库，在 `todo-cli/Cargo.toml` 添加：

```toml
[dependencies]
todo-core = { path = "../todo-core" }
```

在 workspace 根目录执行 `cargo test --workspace` 可测试全部成员。

## 7. 文档和质量工具

公共 API 使用文档注释：

```rust
/// 返回文本中的单词数量。
///
/// # Examples
///
/// ```
/// assert_eq!(todo_core::word_count("hello rust"), 2);
/// ```
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}
```

代码块会成为文档测试。生成文档：

```bash
cargo doc --no-deps --open
```

依赖管理原则：

1. 先确认标准库是否足够。
2. 查看 crate 的维护状态、文档、许可证和依赖数量。
3. 只启用需要的 feature。
4. 提交 `Cargo.lock`：应用程序通常应提交，库是否提交按项目策略决定。

## 阶段项目：重构待办事项程序

把之前的待办事项程序整理成 workspace：

```text
todo-workspace/
├── Cargo.toml
├── todo-core/
└── todo-cli/
```

要求：

1. `todo-core` 保存模型、业务逻辑和错误类型。
2. `todo-cli` 只负责命令行交互和文件存储。
3. 核心库不依赖终端输入输出。
4. 公共类型和函数都有文档注释。
5. 业务规则使用单元测试，完整流程使用集成测试。
6. 使用 `cargo test --doc` 验证文档示例。

## 完成标准

- 能解释 `Box`、`Rc`、`Arc` 和 `RefCell` 分别解决什么问题。
- 不会没有必要地使用共享可变状态。
- 能区分 package、crate、module 和 workspace。
- 项目模块职责清楚，公共 API 有文档。
- `cargo fmt --check`、`cargo clippy --workspace -- -D warnings` 和 `cargo test --workspace` 全部通过。

完成后进入[阶段 7：线程、并发与异步](07-concurrency-async.md)。
