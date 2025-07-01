mod app;
mod cli;
mod data;
mod gitlog;
mod tui;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use chrono_english::{parse_date_string, parse_duration, Dialect, Interval};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration as StdDuration;

/// Parses a natural language time range into a (since, until) tuple.
/// Parses a natural language time range using `chrono-english`.
pub(crate) fn parse_range(input: &str) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
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
    let mut app = app::App::new(commits);
    let mut terminal = tui::Tui::new()?;

    loop {
        terminal.draw(&app)?;

        if event::poll(StdDuration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => break,
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Down, _) => app.next(),
                    (KeyCode::Up, _) => app.previous(),
                    _ => {}
                }
            }
        }
    }

    terminal.shutdown()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn parse_range_defaults_to_last_year() {
        let (since, until) = parse_range("nonsense").unwrap();
        assert!(until > since);
    }

    #[test]
    fn parse_range_since_keyword() {
        let (since, until) = parse_range("since yesterday").unwrap();
        assert!(until > since);
    }

    #[test]
    fn parse_range_with_explicit_range() {
        let (since, until) = parse_range("2020-01-01 to 2020-01-10").unwrap();
        assert!(until > since);
        assert_eq!(since.year(), 2020);
        assert_eq!(until.year(), 2020);
    }
}
