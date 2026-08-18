use std::time::Duration;

use stage07_async_tasks::{Task, run_tasks};

#[tokio::main]
async fn main() {
    let tasks = vec![
        Task {
            id: 1,
            delay: Duration::from_millis(40),
            should_fail: false,
        },
        Task {
            id: 2,
            delay: Duration::from_millis(150),
            should_fail: false,
        },
        Task {
            id: 3,
            delay: Duration::from_millis(20),
            should_fail: true,
        },
    ];
    let summary = run_tasks(tasks, 2, Duration::from_millis(100)).await;

    for (id, outcome) in summary.outcomes {
        println!("任务 {id}: {outcome:?}");
    }
    println!(
        "成功 {}，超时 {}，失败 {}",
        summary.succeeded, summary.timed_out, summary.failed
    );
}
