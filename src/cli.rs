use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    /// Search through code of a given repository
    Code {
        /// Github repository URL (Ex. https://github.com/apache/struts)
        url: String,
        /// 
        #[clap(short, long, default_value_t = String::from(""))]
        search_string: String,
        ///
        #[clap(short, long, default_value_t = String::from(""))]
        lang: String,
    },
    User {
        /// Github user URL (Ex. https://github.com/samuelbearman)
        url: String,
    },
    /// Search issues of repo, match to security related terms
    Issues {
        /// Github repository URL (Ex. https://github.com/apache/struts)
        url: String,
        /// Custom search terms for searching issues (Ex. "security,vuln,bad")
        #[clap(short, long, default_value_t = String::from(""))]
        search_terms: String,
        /// Max number of github api fetches
        #[clap(short, long, default_value_t = 10)]
        fetch_count: u32,
    },
    /// Get list of all contributors in repo
    Contributors {
        /// Github repository URL (Ex. https://github.com/apache/struts)
        url: String,
    },
}
