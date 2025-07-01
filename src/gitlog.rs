use anyhow::Result;
use chrono::{DateTime, FixedOffset, Utc};
use git2::{Repository, Sort};

use crate::data::CommitRow;

/// Loads commits from the given repository path between since and until.
pub fn load_commits(
    repo_path: &str,
    since: DateTime<Utc>,
    until: DateTime<Utc>,
) -> Result<Vec<CommitRow>> {
    let repo = Repository::open(repo_path)?;
    let github_url = get_github_remote_url(&repo);

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut commits = Vec::new();
    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

        // Convert git2::Time to chrono DateTime<FixedOffset>
        let time = commit.time();
        let secs = time.seconds();
        let offset_minutes = time.offset_minutes();
        let offset = FixedOffset::east_opt(offset_minutes * 60).expect("offset out of range");
        let datetime: DateTime<FixedOffset> = DateTime::<Utc>::from_timestamp(secs, 0)
            .expect("timestamp out of range")
            .with_timezone(&offset);
        let datetime_utc = datetime.with_timezone(&Utc);

        if datetime_utc < since || datetime_utc > until {
            continue;
        }

        let sha = oid.to_string()[..7].to_string();
        let url = github_url.as_ref().map(|base| format!("{}/commit/{}", base, oid));
        let month_year = datetime.format("%B %Y").to_string();

        commits.push(CommitRow {
            sha,
            url,
            month_year,
            date: datetime,
            author: commit.author().name().unwrap_or("").to_string(),
            message: commit.message().unwrap_or("").to_string(),
        });
    }
    Ok(commits)
}

/// Attempts to construct a GitHub URL base from the origin remote.
fn get_github_remote_url(repo: &Repository) -> Option<String> {
    if let Ok(remote) = repo.find_remote("origin") {
        if let Some(url) = remote.url() {
            if url.starts_with("git@github.com:") {
                let path = url.trim_start_matches("git@github.com:").trim_end_matches(".git");
                return Some(format!("https://github.com/{}", path));
            } else if url.starts_with("https://github.com/") {
                let path = url.trim_start_matches("https://github.com/").trim_end_matches(".git");
                return Some(format!("https://github.com/{}", path));
            }
        }
    }
    None
}
