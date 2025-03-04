use clap::Parser;
use planner::{
    command::{Options, SubCommand},
    handler::create,
    model::TodoStore,
};

fn main() -> Result<(), String> {
    let opts = Options::parse();
    println!("{:?}", opts);

    let mut todo_store = planner::model::TodoStore::load()?;
    handle_sub_commands(opts, &mut todo_store)
}

fn handle_sub_commands(opts: Options, todo_store: &mut TodoStore) -> Result<(), String> {
    match opts.cmd {
        SubCommand::Create(opts) => {
            let todo = create::create_todo(opts, todo_store)?;
            println!("created todo: {:?}", todo);
            Ok(())
        }
        SubCommand::List(opts) => Ok(()),
        SubCommand::Delete(opts) => Ok(()),
        SubCommand::Move(opts) => Ok(()),
    }
}
