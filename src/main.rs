mod cli;
mod gitlog;
mod data;

use clap::Parser;
use chrono::{Utc, Duration, DateTime};
use anyhow::Result;
use prettytable::{Table, row};
use chrono_english::{parse_date_string, parse_duration, Dialect, Interval};

/// Parses a natural language time range into a (since, until) tuple.
/// Parses a natural language time range using `chrono-english`.
fn parse_range(input: &str) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    let now = Utc::now();
    let input = input.trim();
    let lower = input.to_lowercase();
    let dialect = Dialect::Uk;

    if let Some(rest) = lower.strip_prefix("since ") {
        let since = parse_date_string(rest, now, dialect)?;
        return Ok((since, now));
    }

    if let Some(idx) = lower.find(" to ") {
        let since_str = input[..idx].trim();
        let until_str = input[idx + 4..].trim();
        let since = parse_date_string(since_str, now, dialect)?;
        let until = parse_date_string(until_str, now, dialect)?;
        return Ok((since, until));
    }

    if let Ok(interval) = parse_duration(input) {
        use Interval::*;
        let since = match interval {
            Seconds(s) => now + Duration::seconds(s as i64),
            Days(d) => now + Duration::days(d as i64),
            Months(m) => now + Duration::days(30 * m as i64),
        };
        return Ok((since, now));
    }

    if let Ok(date) = parse_date_string(input, now, dialect) {
        return Ok((date, now));
    }

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
