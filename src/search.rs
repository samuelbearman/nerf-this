pub mod issues {

    use crate::models::issues::Issue;

    const KEY_WORDS: [&str; 3] = ["cve", "security", "vulnerability"];

    pub fn search_for_terms(issue: &Issue) -> bool {
        for word in KEY_WORDS {
            if issue.title.to_lowercase().contains(word) {
                return true;
            }
        }
        false
    }
}
