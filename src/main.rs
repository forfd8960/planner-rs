use clap::Parser;
use planner::{
    command::{Options, SubCommand},
    handler::{create, delete, list, update},
    model::TodoStore,
};

fn main() -> Result<(), String> {
    let opts = Options::parse();
    println!("{:?}", opts);

    let mut todo_store = TodoStore::load()?;
    handle_sub_commands(opts, &mut todo_store)
}

fn handle_sub_commands(opts: Options, todo_store: &mut TodoStore) -> Result<(), String> {
    match opts.cmd {
        SubCommand::Create(opts) => {
            let todo = create::create_todo(todo_store, opts)?;
            println!("created todo: {:?}", todo);
            Ok(())
        }
        SubCommand::List(opts) => {
            println!("list todo: {:?}", opts);
            let todos = list::list_todos(todo_store, opts)?;
            println!("todos: {:?}", todos);
            Ok(())
        }
        SubCommand::Delete(opts) => {
            println!("delete todo: {:?}", opts);
            delete::delete_todo(todo_store, opts)?;
            Ok(())
        }
        SubCommand::Move(opts) => {
            println!("move todo: {:?}", opts);
            let _ = update::update_todo(todo_store, opts)?;
            Ok(())
        }
    }
}
