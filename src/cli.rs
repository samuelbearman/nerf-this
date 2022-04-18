use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
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
    Issues {
        /// Github URL (Ex. https://github.com/apache/struts)
        url: String,
        /// Custom search terms for searching issues (Ex. "security,vuln,bad")
        #[clap(short, long, default_value_t = String::from(""))]
        search_terms: String,
    },
    /// Get list of all contributors in repo
    Contributors {
        /// Github URL (Ex. https://github.com/apache/struts)
        url: String,
    },
    /// Search all repositories on github
    GlobalSearch,
}
