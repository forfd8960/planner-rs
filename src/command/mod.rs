use clap::Parser;
use options::{CreateOpts, DeleteOpts, ListOpts, MoveOpts};

pub mod options;

#[derive(Debug, Parser)]
#[command(name="task-planner", version, author, about="What Todo Next?", long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "create", about = "create a todo list")]
    Create(CreateOpts),
    #[command(name = "delete", about = "delete a todo list")]
    Delete(DeleteOpts),
    #[command(name = "list", about = "list todos by filters")]
    List(ListOpts),
    #[command(name = "move", about = "move a todo to another status")]
    Move(MoveOpts),
}
