use clap::Parser;

mod search;
mod cli;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args = cli::Args::parse();

    search::issues::query_issues(args.url).await?;
    
    Ok(())
    
}
