pub const DEFAULT_KEY_WORDS: [&str; 14] = [
    "cve",
    "security",
    "vulnerability",
    "log4j",
    "nist",
    "poc",
    "rce",
    "attack",
    "dos",
    "hack",
    "owasp",
    "sniff",
    "risk",
    "threat",
];

mod util {
    use octocrab::Octocrab;
    use std::env;

    pub fn get_client() -> Octocrab {
        let github_token = match env::var_os("GITHUB") {
            Some(v) => v.into_string().unwrap(),
            None => panic!("$GITHUB is not set"),
        };

        octocrab::OctocrabBuilder::new()
            .personal_token(github_token)
            .build()
            .unwrap()
    }
}

pub mod global {
    use crate::output::global::print_repository;
    use crate::query::util::get_client;
    use octocrab::models::Repository;
    use octocrab::{Octocrab, Page};

    pub async fn search() -> octocrab::Result<()> {
        let client: Octocrab = get_client();

        let pages: Page<Repository> = client
            .search()
            .repositories("poc")
            .sort("updated")
            .send()
            .await?;

        print_repository(pages.into_iter());
        Ok(())
    }
}

pub mod issues {

    use crate::extensions::octocrab::{OrganisationExt, UsersExt};
    use crate::output::issues::print_issues;
    use crate::query::util::get_client;
    use crate::query::DEFAULT_KEY_WORDS;
    use colored::Colorize;
    use indicatif::ProgressBar;
    use octocrab::models::issues::Issue;
    use octocrab::{models, params, Octocrab};
    use url::Url;

    struct QueryDetails {
        owner: String,
        repo: String,
    }

    pub async fn query_issues(repo_url: String, fetch_count: u32) -> octocrab::Result<(), Box<dyn std::error::Error>> {
        let query = parse_github_url(repo_url);

        let client: Octocrab = get_client();

        let test_url = Url::parse("https://api.github.com/repos/apache/struts/contributors")?;

        let test = client.list_contributors(test_url).await?;

        println!("{}", "Running Issues search\n".bright_red().bold());

        let bar = ProgressBar::new(fetch_count.into());
        let mut page = client
            .issues(query.owner, query.repo)
            .list()
            .state(params::State::All)
            .per_page(100)
            .send()
            .await?;
        let mut matched = Vec::new();
        let mut current_count = 1;
        while current_count <= fetch_count {
            for issue in &page {
                if search_for_terms(issue) {
                    matched.push(issue.clone());
                }
            }

            current_count = current_count + 1;
            bar.inc(1);
            page = match client.get_page::<models::issues::Issue>(&page.next).await? {
                Some(next_page) => next_page,
                None => break,
            };
        }
        bar.finish();
        print_issues(matched.into_iter());
        Ok(())
    }

    fn search_for_terms(issue: &Issue) -> bool {
        for word in DEFAULT_KEY_WORDS {
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