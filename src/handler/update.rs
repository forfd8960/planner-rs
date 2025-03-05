use crate::{command::options::MoveOpts, model::TodoStore};

pub fn update_todo(todo_store: &mut TodoStore, opts: MoveOpts) -> Result<(), String> {
    println!("update todo options: {:?}", opts);

    match opts.status.as_str().parse() {
        Ok(status) => {
            todo_store.update_status(opts.id, status)?;
            todo_store.save()
        }
        Err(e) => Err(e),
    }
}
