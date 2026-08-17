use crate::Task;

pub trait TaskRepository {
    fn insert(&mut self, task: Task);
    fn find(&self, id: u32) -> Option<&Task>;
    fn find_mut(&mut self, id: u32) -> Option<&mut Task>;
    fn all(&self) -> Vec<Task>;
}

#[derive(Debug, Default)]
pub struct InMemoryTaskRepository {
    tasks: Vec<Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn insert(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn find(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id() == id)
    }

    fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id() == id)
    }

    fn all(&self) -> Vec<Task> {
        self.tasks.clone()
    }
}
