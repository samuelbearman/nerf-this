use clap::Parser;

mod query;
mod cli;
mod output;
mod extensions;

#[tokio::main]
async fn main() -> octocrab::Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    match args.action {
        cli::Action::Issues { url, search_terms } => query::issues::query_issues(url, args.fetch_count, search_terms).await?,
        cli::Action::Contributors { url } => query::issues::query_contributors(url).await?,
        cli::Action::GlobalSearch => query::global::search().await?,
    }
        
    Ok(())
    
}
