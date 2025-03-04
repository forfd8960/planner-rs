use crate::{
    command::options::CreateOpts,
    model::{Status, Todo, TodoStore},
};

pub fn create_todo(opts: CreateOpts, todo_store: &mut TodoStore) -> Result<Todo, String> {
    println!("create todo: {:?}", opts);

    todo_store.create(&Todo {
        id: 0,
        name: opts.name,
        desc: opts.desc,
        status: Status::PLAN,
        finish_date: opts.finish_date,
    })
}
