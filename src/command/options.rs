use clap::Args;

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
    #[arg(long)]
    pub sort: Option<String>,
    #[arg(long)]
    pub status: Option<String>,
    #[arg(short, long)]
    pub finsh_date: Option<String>,
}
