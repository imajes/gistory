use crate::data::CommitRow;

/// Application state for the TUI.
pub struct App {
    commits: Vec<CommitRow>,
    pub selected: usize,
}

impl App {
    pub fn new(commits: Vec<CommitRow>) -> Self {
        Self { commits, selected: 0 }
    }

    pub fn next(&mut self) {
        if self.commits.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.commits.len();
    }

    pub fn previous(&mut self) {
        if self.commits.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.commits.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn selected_commit(&self) -> Option<&CommitRow> {
        self.commits.get(self.selected)
    }

    /// Returns the zero-based index of the current page.
    pub fn current_page(&self) -> usize {
        if self.commits.is_empty() {
            0
        } else {
            self.selected / Self::per_page()
        }
    }

    /// Returns the total number of pages.
    pub fn page_count(&self) -> usize {
        let per_page = Self::per_page();
        if self.commits.is_empty() {
            1
        } else {
            self.commits.len().div_ceil(per_page)
        }
    }

    /// Items shown on the current page.
    pub fn page_commits(&self) -> &[CommitRow] {
        let per_page = Self::per_page();
        let start = self.current_page() * per_page;
        let end = usize::min(start + per_page, self.commits.len());
        &self.commits[start..end]
    }

    /// Number of commits to display per page.
    pub const fn per_page() -> usize {
        30
    }

    /// Total number of commits loaded.
    pub fn commit_count(&self) -> usize {
        self.commits.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    fn make_commit(id: usize) -> CommitRow {
        let date = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap();
        CommitRow {
            sha: format!("{:07x}", id),
            url: None,
            month_year: "January 2020".into(),
            date,
            author: "Test".into(),
            message: format!("commit {}", id),
        }
    }

    fn sample_app(count: usize) -> App {
        let commits = (0..count).map(make_commit).collect();
        App::new(commits)
    }

    #[test]
    fn next_wraps_to_start() {
        let mut app = sample_app(2);
        app.next();
        app.next();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn previous_wraps_to_end() {
        let mut app = sample_app(3);
        app.previous();
        assert_eq!(app.selected, 2);
    }

    #[test]
    fn page_count_computes_ceil() {
        let app = sample_app(App::per_page() * 2 + 1);
        assert_eq!(app.page_count(), 3);
    }

    #[test]
    fn page_commits_returns_correct_slice() {
        let mut app = sample_app(App::per_page() + 5);
        assert_eq!(app.page_commits().len(), App::per_page());
        app.selected = App::per_page();
        assert_eq!(app.page_commits().len(), 5);
    }

    #[test]
    fn current_page_reflects_selection() {
        let mut app = sample_app(App::per_page() + 1);
        assert_eq!(app.current_page(), 0);
        app.selected = App::per_page();
        assert_eq!(app.current_page(), 1);
    }
}
