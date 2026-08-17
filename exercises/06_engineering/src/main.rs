use stage06_engineering::{InMemoryTaskRepository, TodoService};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repository = InMemoryTaskRepository::new();
    let mut service = TodoService::new(repository);

    let id = service.add("学习项目分层")?;
    service.complete(id)?;

    for task in service.list() {
        println!("{}: {} [{}]", task.id(), task.title(), task.is_completed());
    }

    Ok(())
}
