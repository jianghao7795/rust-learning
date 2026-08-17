# 阶段 7：线程、并发与异步

目标：掌握标准库线程和消息传递，再理解 Rust 异步模型。本阶段建议用时一到两周。

## 1. 创建线程

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        (1..=5).sum::<i32>()
    });

    let result = handle.join().expect("工作线程发生 panic");
    println!("结果：{result}");
}
```

`spawn` 返回 `JoinHandle`。调用 `join` 等待线程完成并取得结果。

线程需要取得外部数据所有权时使用 `move`：

```rust
let values = vec![1, 2, 3];

let handle = std::thread::spawn(move || {
    values.iter().sum::<i32>()
});

println!("{}", handle.join().unwrap());
```

## 2. 消息传递

Channel 让线程通过发送消息交换数据：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    for worker_id in 0..3 {
        let sender = sender.clone();
        thread::spawn(move || {
            sender.send((worker_id, worker_id * 2)).unwrap();
        });
    }
    drop(sender);

    for (worker_id, result) in receiver {
        println!("worker {worker_id}: {result}");
    }
}
```

需要 `drop(sender)`，否则接收循环可能一直等待原始发送端发出新消息。

## 3. 共享状态

多线程共享并修改数据时，常使用 `Arc<Mutex<T>>`：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut value = counter.lock().unwrap();
            *value += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}
```

- `Arc` 让多个线程共同拥有数据。
- `Mutex` 保证同一时刻只有一个线程修改数据。
- 锁的作用域应尽量短，不要持锁执行耗时 I/O。

优先考虑消息传递；确实需要共同访问同一状态时再使用锁。

## 4. `Send` 与 `Sync`

- 实现 `Send` 的类型可以安全地把所有权转移到另一个线程。
- 实现 `Sync` 的类型可以安全地被多个线程通过引用共享。

它们通常由编译器根据字段自动推导。多数时候不需要手动实现，但线程错误信息经常会提到它们。

## 5. 异步基础

异步适合大量等待 I/O 的任务。`async fn` 返回一个 `Future`，只有被运行时轮询时才推进：

```rust
async fn load_name() -> String {
    String::from("Rust")
}
```

标准库定义了 Future，但通常由 Tokio 等运行时负责调度。

创建异步项目并添加 Tokio：

```bash
cargo new stage07_async
cd stage07_async
cargo add tokio --features full
```

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let first = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "first"
    });

    let second = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "second"
    });

    println!("{} {}", first.await.unwrap(), second.await.unwrap());
}
```

`.await` 允许当前任务在等待时让出执行机会，它不会自动创建线程。

## 6. 线程与异步如何选择

| 情况 | 常见选择 |
| --- | --- |
| 少量 CPU 密集计算 | 线程或专用并行库 |
| 大量网络、定时器等 I/O 等待 | 异步 |
| 必须调用阻塞 API | 专用线程或 `spawn_blocking` |
| 简单后台任务 | 标准线程通常足够 |

并发是多个任务在一段时间内推进；并行是多个任务同一时刻执行；异步是一种协作式并发模型。三者不要混为一谈。

## 阶段项目：并发任务执行器

先使用标准库完成第一版：

1. 主线程准备一组整数任务。
2. 启动固定数量的工作线程。
3. 工作线程计算每个整数的平方。
4. 通过 channel 把结果发送回主线程。
5. 主线程收集并按输入顺序输出结果。
6. 工作线程 panic 时，主线程需要报告失败。

第二版异步练习：

1. 使用 Tokio 创建多个带不同延迟的模拟任务。
2. 给每个任务设置超时。
3. 汇总成功、超时和失败数量。
4. 限制同时运行的任务数，避免一次启动无限任务。

## 完成标准

- 会创建线程、移动数据并等待线程结束。
- 能根据数据关系选择 channel 或 `Arc<Mutex<T>>`。
- 能解释 `Send` 和 `Sync` 的基本含义。
- 能区分线程并发与异步任务。
- 知道阻塞操作不能直接长时间占用异步执行器线程。
- 两版任务执行器都有正常和失败路径测试。

完成后进入[阶段 8：综合项目](08-project.md)。
