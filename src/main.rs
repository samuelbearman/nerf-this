mod search;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    search::issues::fetch("test".to_string()).await?;
    Ok(())
}
