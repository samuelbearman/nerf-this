pub mod issues {

    use comfy_table::modifiers::UTF8_ROUND_CORNERS;
    use comfy_table::presets::UTF8_FULL;
    use comfy_table::*;
    use octocrab::models::issues::Issue;
    use octocrab::{models, params};
    use std::env;

    struct QueryDetails {
        owner: String,
        repo: String,
    }

    const KEY_WORDS: [&str; 6] = ["cve", "security", "vulnerability", "log4j", "nist", "poc"];

    pub async fn query_issues(repo_url: String) -> octocrab::Result<()> {
        let github_token = match env::var_os("GITHUB") {
            Some(v) => v.into_string().unwrap(),
            None => panic!("$GITHUB is not set")
        };

        let query = parse_github_url(repo_url);

        let octocrab = octocrab::OctocrabBuilder::new()
            .personal_token(github_token)
            .build()?;
        let mut page = octocrab
            .issues(query.owner, query.repo)
            .list()
            .state(params::State::All)
            .per_page(100)
            .send()
            .await?;
        let mut matched = Vec::new();
        loop {
            for issue in &page {
                if search_for_terms(issue) {
                    matched.push(issue.clone());
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
        print_search_results(matched);
        Ok(())
    }

    fn print_search_results(results: Vec<Issue>) {
        let mut table = Table::new();

        for matched_issue in results {
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["Title", "Url", "State"])
                .add_row(vec![
                    Cell::new(matched_issue.title).fg(Color::Red),
                    Cell::new(matched_issue.html_url.as_str()),
                    match matched_issue.state.as_ref() {
                        "open" => Cell::new(matched_issue.state).fg(Color::Green),
                        "closed" => Cell::new(matched_issue.state).fg(Color::Red),
                        _ => Cell::new("N/A").fg(Color::Yellow),
                    },
                ]);
        }

        println!("{table}");
    }

    fn search_for_terms(issue: &Issue) -> bool {
        for word in KEY_WORDS {
            if issue.title.to_lowercase().contains(word) {
                return true;
            }

            match &issue.body {
                Some(text) => text.contains(word),
                None => continue,
            };
        }
        false
    }

    fn parse_github_url(url: String) -> QueryDetails {
        let split_strings: Vec<&str> = url.split("/").collect();
        QueryDetails {
            owner: String::from(split_strings[3]),
            repo: String::from(split_strings[4]),
        }
    }
}
