mod cli;
mod gitlog;
mod data;

use clap::Parser;
use chrono::{Utc, Duration, DateTime};
use anyhow::Result;
use prettytable::{Table, row};
use event_parser::ParserBuilder;

/// Parses a natural language time range into a (since, until) tuple.
/// Parses a natural language time range using `event-parser`.
fn parse_range(input: &str) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    let parser = ParserBuilder::default().build()?;
    let events = parser.parse(input)?;
    let now = Utc::now();
    if let Some(ev) = events.first() {
        let since = ev.start.with_timezone(&Utc);
        let until = ev.end.unwrap_or(ev.start).with_timezone(&Utc);
        return Ok((since, until));
    }
    // Fallback: past year
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
