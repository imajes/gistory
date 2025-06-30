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

    pub fn commits(&self) -> &[CommitRow] {
        &self.commits
    }
}
