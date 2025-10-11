//! Block explorer screen for WarpScan
//!
//! This module contains the block explorer screen implementation.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::ui::{app::App, theme::Theme};

/// Render the block explorer screen
pub fn render_block_explorer(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Block Explorer")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, chunks[0]);

    // Input field for block number
    crate::ui::components::render_input_field(
        frame,
        chunks[1],
        theme,
        "Enter Block Number or Hash",
        app.get_input(),
        app.cursor_position,
        app.input_mode == crate::ui::InputMode::Editing,
    );

    // Content area
    let content = if app.is_loading("block_search") {
        Text::from("Loading block information...")
    } else if let Some(error) = &app.error_message {
        Text::from(vec![
            Line::from(Span::styled("Error: ", theme.error())),
            Line::from(error.clone()),
        ])
    } else {
        Text::from("Enter a block number or hash to search")
    };

    let content_paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title("Block Information")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(content_paragraph, chunks[2]);
}