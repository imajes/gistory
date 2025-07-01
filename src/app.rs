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
            (self.commits.len() + per_page - 1) / per_page
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
