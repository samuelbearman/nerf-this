use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
    /// Max number of github api fetches
    #[clap(short, long, default_value_t = 10)]
    pub fetch_count: u32,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    /// Search issues of repo, match to security related terms
    Issues { url: String },
    /// Get list of all contributors in repo
    Contributors { url: String },
    /// Search all repositories on github
    GlobalSearch,
}
