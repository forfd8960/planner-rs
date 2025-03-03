pub struct Todo {
    pub id: u64,
    pub name: String,
    pub desc: String,
    pub status: Status,
    pub finish_date: String,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    PLAN,
    TODO,
    WIP,
    DONE,
}

pub fn create_todo(todo: &Todo) -> Result<Todo, String> {
    println!("create todo");
    Ok(Todo {
        id: 1,
        name: todo.name.clone(),
        desc: todo.desc.clone(),
        status: Status::PLAN,
        finish_date: todo.finish_date.clone(),
    })
}

pub fn delete_todo(todo_id: u64) -> Result<(), String> {
    println!("delete todo: {}", todo_id);
    Ok(())
}

pub fn move_todo(todo_id: u64, target_state: Status) -> Result<(), String> {
    println!("move todo: {} to: {:?}", todo_id, target_state);
    Ok(())
}

pub fn list_todos() -> Result<Vec<Todo>, String> {
    println!("list todos...");
    Ok(vec![])
}
