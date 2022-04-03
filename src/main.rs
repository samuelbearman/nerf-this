use clap::Parser;

mod query;
mod cli;
mod output;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args = cli::Args::parse();

    match args.action {
        cli::Action::Issues { url } => query::issues::query_issues(url, args.fetch_count).await?,
        cli::Action::GlobalSearch => query::global::search().await?,
    }
        
    Ok(())
    
}
