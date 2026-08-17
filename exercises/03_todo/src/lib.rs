mod task;

pub use task::Task;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Add(String),
    List,
    Complete(u32),
    Remove(u32),
}

#[derive(Debug)]
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: impl Into<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task::new(id, title));
        id
    }

    pub fn list(&self) -> &[Task] {
        &self.tasks
    }

    pub fn complete(&mut self, id: u32) -> bool {
        let Some(task) = self.tasks.iter_mut().find(|task| task.id() == id) else {
            return false;
        };
        task.complete();
        true
    }

    pub fn remove(&mut self, id: u32) -> Option<Task> {
        let index = self.tasks.iter().position(|task| task.id() == id)?;
        Some(self.tasks.remove(index))
    }

    pub fn execute(&mut self, command: Command) -> Option<String> {
        match command {
            Command::Add(title) => Some(format!("已添加任务 {}", self.add(title))),
            Command::List => Some(format!("共 {} 个任务", self.tasks.len())),
            Command::Complete(id) => self.complete(id).then(|| format!("已完成任务 {id}")),
            Command::Remove(id) => self
                .remove(id)
                .map(|task| format!("已删除任务：{}", task.title())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_tasks_with_unique_ids() {
        let mut list = TodoList::new();
        assert_eq!(list.add("学习 struct"), 1);
        assert_eq!(list.add("学习 enum"), 2);
        assert_eq!(list.list().len(), 2);
    }

    #[test]
    fn completes_existing_task() {
        let mut list = TodoList::new();
        let id = list.add("写测试");

        assert!(list.complete(id));
        assert!(list.list()[0].is_completed());
        assert!(!list.complete(999));
    }

    #[test]
    fn removes_and_returns_task() {
        let mut list = TodoList::new();
        let id = list.add("可以删除");

        let removed = list.remove(id).expect("task should exist");
        assert_eq!(removed.title(), "可以删除");
        assert!(list.list().is_empty());
    }
}
