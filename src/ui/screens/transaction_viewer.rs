//! Transaction viewer screen for WarpScan
//!
//! This module contains the transaction viewer screen implementation.

use crate::ui::{app::App, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the transaction viewer screen
pub fn render_transaction_viewer(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Transaction Viewer")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, chunks[0]);

    // Input field for transaction hash
    crate::ui::components::render_input_field(
        frame,
        chunks[1],
        theme,
        "Enter Transaction Hash",
        app.get_input(),
        app.cursor_position,
        app.input_mode == crate::ui::InputMode::Editing,
    );

    // Content area
    let content = if app.is_loading("transaction_search") {
        Text::from("Loading transaction information...")
    } else if let Some(error) = &app.error_message {
        Text::from(vec![
            Line::from(Span::styled("Error: ", theme.error())),
            Line::from(error.clone()),
        ])
    } else {
        Text::from("Enter a transaction hash to search")
    };

    let content_paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title("Transaction Information")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(content_paragraph, chunks[2]);
}
