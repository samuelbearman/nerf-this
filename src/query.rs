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

    pub struct QueryDetails {
        pub owner: String,
        pub repo: String,
    }

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

    pub fn parse_github_url(url: String) -> QueryDetails {
        let split_strings: Vec<&str> = url.split("/").collect();
        QueryDetails {
            owner: String::from(split_strings[3]),
            repo: String::from(split_strings[4]),
        }
    }
}

pub mod code {
    use crate::{
        output::code::print_code_search_content,
        query::util::{get_client, parse_github_url},
    };
    use octocrab::Octocrab;

    pub async fn query_code(
        repo_url: String,
        search_string: String,
        lang: String,
    ) -> octocrab::Result<(), Box<dyn std::error::Error>> {
        let query = parse_github_url(repo_url);

        let client: Octocrab = get_client();

        let query_string = format!(
            "\"{}\" repo:{}/{} language:{}",
            search_string, query.owner, query.repo, lang
        );

        let code_content = client.search().code(&query_string).send().await?;

        print_code_search_content(&code_content.items);

        Ok(())
    }
}

pub mod issues {

    const REPO_API_URL: &str = "https://api.github.com/repos";
    const _REPO_API_EVENTS: &str = "https://api.github.com/events";

    use crate::extensions::octocrab::UsersExt;
    use crate::output::issues::{print_contibutors, print_issues};
    use crate::query::util::{get_client, parse_github_url};
    use crate::query::DEFAULT_KEY_WORDS;
    use colored::Colorize;
    use octocrab::models::issues::Issue;
    use octocrab::{models, params, Octocrab};
    use url::Url;

    pub async fn query_contributors(
        repo_url: String,
    ) -> octocrab::Result<(), Box<dyn std::error::Error>> {
        let query = parse_github_url(repo_url);

        let client: Octocrab = get_client();

        let formated_url = format!(
            "{}/{}/{}/contributors",
            REPO_API_URL, query.owner, query.repo
        );

        let url = Url::parse(&formated_url)?;

        println!("{}", "Running Contributor Search\n".bright_red().bold());

        let contributors = client.list_contributors(url).await?;

        print_contibutors(&contributors.items);

        Ok(())
    }

    pub async fn query_issues(
        repo_url: String,
        fetch_count: u32,
        search_terms: String,
    ) -> octocrab::Result<(), Box<dyn std::error::Error>> {
        let query = parse_github_url(repo_url);

        let client: Octocrab = get_client();

        println!("{}", "Running Issues Search\n".bright_red().bold());

        let mut page = client
            .issues(query.owner, query.repo)
            .list()
            .state(params::State::All)
            .per_page(100)
            .send()
            .await?;
        let mut matched = Vec::new();
        let mut current_count = 1;

        let parsed_search_terms = search_terms.split(",").collect::<Vec<_>>();

        while current_count <= fetch_count {
            for issue in &page {
                if search_for_terms(issue, parsed_search_terms.as_slice()) {
                    matched.push(issue.clone());
                }
            }

            current_count = current_count + 1;

            page = match client.get_page::<models::issues::Issue>(&page.next).await? {
                Some(next_page) => next_page,
                None => break,
            };
        }

        print_issues(&matched);
        Ok(())
    }

    fn search_for_terms(issue: &Issue, search_terms: &[&str]) -> bool {
        if search_terms.len() > 0 {
            for word in search_terms {
                if issue.title.to_lowercase().contains(word) {
                    return true;
                }

                match &issue.body {
                    Some(text) => text.contains(word),
                    None => continue,
                };
            }
            false
        } else {
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
    }
}
