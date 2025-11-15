//! Help screen for WarpScan
//!
//! This module contains the help screen implementation.

use crate::ui::{app::App, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the help screen
pub fn render_help(frame: &mut Frame, _app: &App, theme: &Theme) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Content area
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Help")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, main_chunks[0]);

    // Content area
    let content = Paragraph::new("Help screen - Coming soon")
        .style(theme.normal())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.secondary()),
        );
    frame.render_widget(content, main_chunks[1]);
}
