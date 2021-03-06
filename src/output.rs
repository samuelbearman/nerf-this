pub mod issues {
    use comfy_table::modifiers::UTF8_ROUND_CORNERS;
    use comfy_table::presets::UTF8_FULL;
    use comfy_table::*;
    use octocrab::models::issues::Issue;
    use octocrab::models::User;

    pub fn print_issues(results: &Vec<Issue>) {
        if results.len() > 0 {
            let mut table = Table::new();

            for matched_issue in results {
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_header(vec!["Title", "Url", "State", "User", "Created", "Closed"])
                    .add_row(vec![
                        Cell::new(&matched_issue.title).fg(Color::Red),
                        Cell::new(&matched_issue.html_url.as_str()),
                        match matched_issue.state.as_ref() {
                            "open" => Cell::new(&matched_issue.state).fg(Color::Green),
                            "closed" => Cell::new(&matched_issue.state).fg(Color::Red),
                            _ => Cell::new("N/A").fg(Color::Yellow),
                        },
                        Cell::new(format!("https://github.com/{}", matched_issue.user.login)),
                        Cell::new(matched_issue.created_at.naive_local()).fg(Color::Green),
                        match matched_issue.closed_at {
                            Some(date) => Cell::new(date.naive_local()).fg(Color::Red),
                            None => Cell::new("N/A").fg(Color::Yellow),
                        },
                    ]);
            }

            println!("{table}");
        } else {
            println!("No Results");
        }
    }

    pub fn print_contibutors(results: &Vec<User>) {
        if results.len() > 0 {
            let mut table = Table::new();

            for matched_issue in results {
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_header(vec!["Username", "Profile Link"])
                    .add_row(vec![
                        Cell::new(&matched_issue.login).fg(Color::Red),
                        Cell::new(&matched_issue.html_url.as_str()).fg(Color::Red),
                    ]);
            }

            println!("{table}");
        } else {
            println!("No Results");
        }
    }
}

pub mod code {
    use comfy_table::modifiers::UTF8_ROUND_CORNERS;
    use comfy_table::presets::UTF8_FULL;
    use comfy_table::*;
    use octocrab::models::Code;


    pub fn print_code_search_content(results: &Vec<Code>) {
        if results.len() > 0 {
            let mut table = Table::new();

            for matched_issue in results {
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_header(vec!["File", "Path", "URL"])
                    .add_row(vec![
                        Cell::new(&matched_issue.name).fg(Color::Red),
                        Cell::new(&matched_issue.path).fg(Color::Red),
                        Cell::new(&matched_issue.git_url.as_str()).fg(Color::Red),
                    ]);
            }

            println!("{table}");
        } else {
            println!("No Results");
        }
    }
}