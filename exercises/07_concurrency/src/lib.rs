use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskError {
    NoWorkers,
    Overflow(u64),
    WorkerPanicked,
}

impl fmt::Display for TaskError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoWorkers => write!(formatter, "工作线程数量必须大于零"),
            Self::Overflow(value) => write!(formatter, "{value} 的平方超出 u64 范围"),
            Self::WorkerPanicked => write!(formatter, "工作线程发生 panic"),
        }
    }
}

impl Error for TaskError {}

pub fn square_all(values: &[u64], worker_count: usize) -> Result<Vec<u64>, TaskError> {
    if worker_count == 0 {
        return Err(TaskError::NoWorkers);
    }
    if values.is_empty() {
        return Ok(Vec::new());
    }

    let values = Arc::new(values.to_vec());
    let next_index = Arc::new(AtomicUsize::new(0));
    let (sender, receiver) = mpsc::channel();
    let mut handles = Vec::new();

    for _ in 0..worker_count.min(values.len()) {
        let values = Arc::clone(&values);
        let next_index = Arc::clone(&next_index);
        let sender = sender.clone();

        handles.push(thread::spawn(move || {
            loop {
                let index = next_index.fetch_add(1, Ordering::Relaxed);
                let Some(value) = values.get(index).copied() else {
                    break;
                };
                let result = value.checked_mul(value).ok_or(TaskError::Overflow(value));
                if sender.send((index, result)).is_err() {
                    break;
                }
            }
        }));
    }
    drop(sender);

    let mut results = vec![None; values.len()];
    let mut first_error = None;
    for (index, result) in receiver {
        match result {
            Ok(value) => results[index] = Some(value),
            Err(error) if first_error.is_none() => first_error = Some(error),
            Err(_) => {}
        }
    }

    for handle in handles {
        if handle.join().is_err() {
            return Err(TaskError::WorkerPanicked);
        }
    }
    if let Some(error) = first_error {
        return Err(error);
    }

    Ok(results
        .into_iter()
        .map(|result| result.expect("every task should produce a result"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_input_order() {
        assert_eq!(square_all(&[5, 2, 8, 1], 3), Ok(vec![25, 4, 64, 1]));
    }

    #[test]
    fn accepts_empty_input() {
        assert_eq!(square_all(&[], 2), Ok(Vec::new()));
    }

    #[test]
    fn rejects_zero_workers() {
        assert_eq!(square_all(&[1], 0), Err(TaskError::NoWorkers));
    }

    #[test]
    fn reports_overflow() {
        assert_eq!(
            square_all(&[u64::MAX], 1),
            Err(TaskError::Overflow(u64::MAX))
        );
    }
}
