use crate::{
    command::options::CreateOpts,
    model::{Status, Todo, TodoStore},
};

pub fn create_todo(todo_store: &mut TodoStore, opts: CreateOpts) -> Result<Todo, String> {
    println!("create todo: {:?}", opts);

    let new_todo = todo_store.create(&Todo {
        id: 0,
        name: opts.name,
        desc: opts.desc,
        status: Status::PLAN,
        finish_date: opts.finish_date,
    })?;

    todo_store.save()?;
    Ok(new_todo)
}
