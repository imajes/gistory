use std::io::{stdout, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};

use crate::app::App;

/// Wrapper around ratatui Terminal with crossterm backend.
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> std::io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn shutdown(&mut self) -> std::io::Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &App) -> std::io::Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(f.size());

            let rows = app.commits().iter().enumerate().map(|(i, c)| {
                let style = if i == app.selected {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                Row::new(vec![
                    Cell::from(c.sha.clone()),
                    Cell::from(c.month_year.clone()),
                    Cell::from(c.date.format("%m/%d/%y").to_string()),
                    Cell::from(c.author.clone()),
                    Cell::from(c.message.clone()),
                ])
                .style(style)
            });

            let table = Table::new(
                rows,
                [
                    Constraint::Length(8),
                    Constraint::Length(12),
                    Constraint::Length(10),
                    Constraint::Length(20),
                    Constraint::Length(30),
                ],
            )
            .header(
                Row::new(vec!["SHA", "Month", "Date", "Author", "Message"])
                    .style(Style::default().add_modifier(Modifier::BOLD)),
            )
            .block(Block::default().borders(Borders::ALL).title("Commits"));

            f.render_widget(table, chunks[0]);

            let msg = app.selected_commit().map(|c| c.message.clone()).unwrap_or_default();
            let paragraph =
                Paragraph::new(msg).block(Block::default().borders(Borders::ALL).title("Message"));
            f.render_widget(paragraph, chunks[1]);
        })?;
        Ok(())
    }
}

