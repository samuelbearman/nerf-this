use colored::*;
use octocrab::{models, params};

mod search;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    fetch().await?;
    Ok(())
}

async fn fetch() -> octocrab::Result<()> {
    let octocrab = octocrab::instance();
    let mut page = octocrab
        .issues("qos-ch", "reload4j")
        .list()
        .state(params::State::All)
        .per_page(50)
        .send()
        .await?;

    loop {
        for issue in &page {
            if search::issues::search_for_terms(issue) {
                println!("{} - {}", issue.title.red(), issue.url);
            }
        }
        page = match octocrab
            .get_page::<models::issues::Issue>(&page.next)
            .await?
        {
            Some(next_page) => next_page,
            None => break,
        }
    }

    Ok(())
}
