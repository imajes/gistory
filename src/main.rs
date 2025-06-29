mod cli;
mod gitlog;
mod data;

use clap::Parser;
use chrono::{Utc, Duration, DateTime};
use anyhow::Result;
use prettytable::{Table, row, cell};
use event_parser::parse;

/// Parses a natural language time range into a (since, until) tuple.
fn parse_range(input: &str) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    // Parse events from the input
    let events = parse(input)?;
    if let Some(ev) = events.first() {
        // Extract start and end times (use start if end is missing)
        let start = ev.start;
        let end = ev.end.unwrap_or(start);
        // Normalize to UTC
        let since = start.with_timezone(&Utc);
        let until = end.with_timezone(&Utc);
        return Ok((since, until));
    }
    // Fallback: past year
    let now = Utc::now();
    let since = now - Duration::days(365);
    Ok((since, now))
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let repo_path = cli.repo.as_deref().unwrap_or(".");
    let (since, until) = parse_range(&cli.range)?;
    let commits = gitlog::load_commits(repo_path, since, until)?;

    let mut table = Table::new();
    table.add_row(row!["SHA", "Month", "Date", "Author", "Message"]);
    for commit in commits {
        table.add_row(row![
            commit.sha,
            commit.month_year,
            commit.date.format("%m/%d/%y").to_string(),
            commit.author,
            commit.message
        ]);
    }
    table.printstd();
    Ok(())
}
