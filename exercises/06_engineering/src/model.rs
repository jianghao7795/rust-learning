#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    id: u32,
    title: String,
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

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
