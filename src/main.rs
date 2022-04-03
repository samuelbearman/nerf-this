use clap::Parser;

mod search;
mod cli;
mod output;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args = cli::Args::parse();

    search::issues::query_issues(args.url, args.fetch_count).await?;
    
    Ok(())
    
}
