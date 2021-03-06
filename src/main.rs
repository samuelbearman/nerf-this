use clap::Parser;

mod query;
mod cli;
mod output;
mod extensions;

#[tokio::main]
async fn main() -> octocrab::Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    match args.action {
        cli::Action::Issues { url, search_terms, fetch_count } => query::issues::query_issues(url, fetch_count, search_terms).await?,
        cli::Action::Contributors { url } => query::issues::query_contributors(url).await?,
        cli::Action::Code { url, search_string, lang } => query::code::query_code(url, search_string, lang).await?,
        cli::Action::User { url: _ } => println!("Not implemented"),
    }
        
    Ok(())
    
}
