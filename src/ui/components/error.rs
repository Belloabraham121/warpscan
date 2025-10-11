//! Error component for WarpScan
//!
//! This module contains the error message component.

use ratatui::{
    layout::{Alignment, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::ui::theme::Theme;

/// Render an error message
pub fn render_error(frame: &mut Frame, area: Rect, theme: &Theme, message: &str) {
    let block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .border_style(theme.error());

    let error_text = Text::from(vec![
        Line::from(vec![Span::styled("✗ ", theme.error())]),
        Line::from(message),
    ]);

    let paragraph = Paragraph::new(error_text)
        .block(block)
        .style(theme.error())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}