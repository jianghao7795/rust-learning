use stage03_todo::{Command, TodoList};

fn main() {
    let mut list = TodoList::new();
    println!("{:?}", list.execute(Command::Add("学习结构体".into())));
    println!("{:?}", list.execute(Command::Add("学习枚举".into())));
    println!("{:?}", list.execute(Command::Complete(1)));

    for task in list.list() {
        let mark = if task.is_completed() { "x" } else { " " };
        println!("[{mark}] {}: {}", task.id(), task.title());
    }
}
