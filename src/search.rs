pub mod issues {

    use colored::*;
    use octocrab::models::issues::Issue;
    use octocrab::{models, params};

    const KEY_WORDS: [&str; 3] = ["cve", "security", "vulnerability"];

    fn search_for_terms(issue: &Issue) -> bool {
        for word in KEY_WORDS {
            if issue.title.to_lowercase().contains(word) {
                return true;
            }
        }
        false
    }

    // fn parse_github_url(url: String) -> (String, String){
    // }

    pub async fn fetch(_repo_url: String) -> octocrab::Result<()> {
        let octocrab = octocrab::instance();
        let mut page = octocrab
            .issues("apache", "struts")
            .list()
            .state(params::State::All)
            .per_page(50)
            .send()
            .await?;
        loop {
            for issue in &page {
                if search_for_terms(issue) {
                    println!("{} - {}", issue.title.red(), issue.repository_url);
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
}
