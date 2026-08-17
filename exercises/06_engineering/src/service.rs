use std::error::Error;
use std::fmt;

use crate::{Task, TaskRepository};

#[derive(Debug, PartialEq, Eq)]
pub enum ServiceError {
    EmptyTitle,
    TaskNotFound(u32),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(formatter, "任务标题不能为空"),
            Self::TaskNotFound(id) => write!(formatter, "任务 {id} 不存在"),
        }
    }
}

impl Error for ServiceError {}

pub struct TodoService {
    repository: Box<dyn TaskRepository>,
    next_id: u32,
}

impl TodoService {
    pub fn new(repository: impl TaskRepository + 'static) -> Self {
        Self {
            repository: Box::new(repository),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: impl Into<String>) -> Result<u32, ServiceError> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(ServiceError::EmptyTitle);
        }

        let id = self.next_id;
        self.next_id += 1;
        self.repository.insert(Task::new(id, title));
        Ok(id)
    }

    pub fn complete(&mut self, id: u32) -> Result<(), ServiceError> {
        let task = self
            .repository
            .find_mut(id)
            .ok_or(ServiceError::TaskNotFound(id))?;
        task.complete();
        Ok(())
    }

    pub fn list(&self) -> Vec<Task> {
        self.repository.all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryTaskRepository;

    fn service() -> TodoService {
        TodoService::new(InMemoryTaskRepository::new())
    }

    #[test]
    fn rejects_empty_title() {
        let mut service = service();
        assert_eq!(service.add("   "), Err(ServiceError::EmptyTitle));
    }

    #[test]
    fn adds_and_completes_task() {
        let mut service = service();
        let id = service.add("拆分模块").expect("valid task");
        service.complete(id).expect("task should exist");

        assert!(service.list()[0].is_completed());
    }

    #[test]
    fn reports_missing_task() {
        let mut service = service();
        assert_eq!(service.complete(99), Err(ServiceError::TaskNotFound(99)));
    }
}
