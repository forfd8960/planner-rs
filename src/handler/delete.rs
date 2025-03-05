use crate::{command::options::DeleteOpts, model::TodoStore};

pub fn delete_todo(todo_store: &mut TodoStore, opts: DeleteOpts) -> Result<(), String> {
    println!("delete todo: {:?}", opts);
    todo_store.delete(opts.id)?;

    todo_store.save()
}
