use chrono::{DateTime, FixedOffset};

/// Represents a git commit row for display.
#[derive(Debug)]
pub struct CommitRow {
    pub sha: String,
    #[allow(dead_code)]
    pub url: Option<String>,
    pub month_year: String,
    pub date: DateTime<FixedOffset>,
    pub author: String,
    pub message: String,
}
