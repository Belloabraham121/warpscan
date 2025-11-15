//! Contract Verification screen for WarpScan
//!
//! This module contains the contract verification screen implementation.

use crate::ui::{app::App, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the contract verification screen
pub fn render_contract_verification(frame: &mut Frame, _app: &App, theme: &Theme) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Content area
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Contract Verification")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, main_chunks[0]);

    // Content area
    let content = Paragraph::new("Contract Verification screen - Coming soon")
        .style(theme.normal())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.secondary()),
        );
    frame.render_widget(content, main_chunks[1]);
}
