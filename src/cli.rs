use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
    /// Max number of github api fetches
    #[clap(short, long, default_value_t = 1)]
    pub fetch_count: u32,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    Issues { url: String },
    Contributors { url: String },
    GlobalSearch,
}
