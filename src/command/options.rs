use clap::{Args, Parser};

#[derive(Debug, Args)]
pub struct CreateOpts {
    #[arg(short = 'n', long)]
    pub name: String,
    #[arg(short = 'd', long)]
    pub desc: String,
    #[arg(short = 'f', long)]
    pub finish_date: String,
}

#[derive(Debug, Args)]
pub struct DeleteOpts {
    #[arg(short, long)]
    pub id: u64,
}

#[derive(Debug, Args)]
pub struct MoveOpts {
    #[arg(short, long)]
    pub id: u64,
    #[arg(short, long)]
    pub status: String,
}

#[derive(Debug, Args)]
pub struct ListOpts {
    #[arg(short, long)]
    pub sort: String,
    #[arg(short, long)]
    pub status: String,
    #[arg(short, long)]
    pub finsh_date: String,
}
