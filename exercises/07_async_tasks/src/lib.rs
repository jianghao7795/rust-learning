use std::collections::VecDeque;
use std::time::Duration;

use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub delay: Duration,
    pub should_fail: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskOutcome {
    Success,
    TimedOut,
    Failed,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TaskSummary {
    pub outcomes: Vec<(u64, TaskOutcome)>,
    pub succeeded: usize,
    pub timed_out: usize,
    pub failed: usize,
}

pub async fn run_tasks(
    tasks: Vec<Task>,
    concurrency: usize,
    task_timeout: Duration,
) -> TaskSummary {
    if concurrency == 0 {
        return TaskSummary {
            failed: tasks.len(),
            outcomes: tasks
                .into_iter()
                .map(|task| (task.id, TaskOutcome::Failed))
                .collect(),
            ..TaskSummary::default()
        };
    }

    let task_count = tasks.len();
    let mut pending: VecDeque<_> = tasks.into_iter().enumerate().collect();
    let mut running = JoinSet::new();
    let mut ordered = vec![None; task_count];

    while !pending.is_empty() || !running.is_empty() {
        while running.len() < concurrency {
            let Some((index, task)) = pending.pop_front() else {
                break;
            };
            running.spawn(async move {
                let id = task.id;
                let result = timeout(task_timeout, async move {
                    sleep(task.delay).await;
                    if task.should_fail {
                        TaskOutcome::Failed
                    } else {
                        TaskOutcome::Success
                    }
                })
                .await
                .unwrap_or(TaskOutcome::TimedOut);
                (index, id, result)
            });
        }

        if let Some(Ok((index, id, outcome))) = running.join_next().await {
            ordered[index] = Some((id, outcome));
        }
    }

    let outcomes: Vec<_> = ordered.into_iter().flatten().collect();
    let mut summary = TaskSummary {
        outcomes,
        ..TaskSummary::default()
    };
    for (_, outcome) in &summary.outcomes {
        match outcome {
            TaskOutcome::Success => summary.succeeded += 1,
            TaskOutcome::TimedOut => summary.timed_out += 1,
            TaskOutcome::Failed => summary.failed += 1,
        }
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn classifies_results_and_keeps_input_order() {
        let tasks = vec![
            Task {
                id: 1,
                delay: Duration::from_millis(1),
                should_fail: false,
            },
            Task {
                id: 2,
                delay: Duration::from_millis(30),
                should_fail: false,
            },
            Task {
                id: 3,
                delay: Duration::from_millis(1),
                should_fail: true,
            },
        ];

        let summary = run_tasks(tasks, 2, Duration::from_millis(10)).await;

        assert_eq!(summary.succeeded, 1);
        assert_eq!(summary.timed_out, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(
            summary.outcomes,
            vec![
                (1, TaskOutcome::Success),
                (2, TaskOutcome::TimedOut),
                (3, TaskOutcome::Failed),
            ]
        );
    }

    #[tokio::test]
    async fn rejects_zero_concurrency_without_panicking() {
        let tasks = vec![Task {
            id: 7,
            delay: Duration::ZERO,
            should_fail: false,
        }];
        let summary = run_tasks(tasks, 0, Duration::from_secs(1)).await;
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.outcomes, vec![(7, TaskOutcome::Failed)]);
    }
}
