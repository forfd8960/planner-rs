use crate::{
    command::options::ListOpts,
    model::{Todo, TodoStore},
};

pub fn list_todos(store: &mut TodoStore, opts: ListOpts) -> Result<Vec<Todo>, String> {
    println!("list todos: {:?}", opts);
    store.list()
}
