use clap::Parser;

#[derive(Debug, Parser)]
pub struct CreateOpts {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub desc: String,
    #[arg(short, long)]
    pub finsh_date: String,
}

#[derive(Debug, Parser)]
pub struct DeleteOpts {
    #[arg(short, long)]
    pub id: u64,
}

#[derive(Debug, Parser)]
pub struct MoveOpts {
    #[arg(short, long)]
    pub id: u64,
    #[arg(short, long)]
    pub status: String,
}

#[derive(Debug, Parser)]
pub struct ListOpts {
    #[arg(short, long)]
    pub sort: String,
    #[arg(short, long)]
    pub status: String,
    #[arg(short, long)]
    pub finsh_date: String,
}
