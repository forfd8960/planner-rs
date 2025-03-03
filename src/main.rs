use clap::Parser;
use planner::command::{Options, SubCommand};

fn main() {
    let opts = Options::parse();
    println!("{:?}", opts);
    handle_sub_commands(opts)
}

fn handle_sub_commands(opts: Options) {
    match opts.cmd {
        SubCommand::Create(opts) => {
            planner::handler::create::create_todo(opts);
        }
        SubCommand::List(opts) => {
            planner::handler::list::list_todos(opts);
        }
        SubCommand::Delete(opts) => {
            planner::handler::delete::delete_todo(opts);
        }
        SubCommand::Move(opts) => {
            planner::handler::update::update_todo(opts);
        }
    }
}
