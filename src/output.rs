pub mod issues {
    use comfy_table::modifiers::UTF8_ROUND_CORNERS;
    use comfy_table::presets::UTF8_FULL;
    use comfy_table::*;
    use octocrab::models::issues::Issue;

    pub fn print_issues(results: Vec<Issue>) {
        let mut table = Table::new();

        for matched_issue in results {
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["Title", "Url", "State", "Created", "Closed"])
                .add_row(vec![
                    Cell::new(matched_issue.title).fg(Color::Red),
                    Cell::new(matched_issue.html_url.as_str()),
                    match matched_issue.state.as_ref() {
                        "open" => Cell::new(matched_issue.state).fg(Color::Green),
                        "closed" => Cell::new(matched_issue.state).fg(Color::Red),
                        _ => Cell::new("N/A").fg(Color::Yellow),
                    },
                    Cell::new(matched_issue.created_at.naive_local()).fg(Color::Green),
                    match matched_issue.closed_at {
                        Some(date) => Cell::new(date.naive_local()).fg(Color::Red),
                        None => Cell::new("N/A").fg(Color::Yellow),
                    },
                ]);
        }

        println!("{table}");
    }
}
