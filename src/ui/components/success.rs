//! Success component for WarpScan
//!
//! This module contains the success message component.

use ratatui::{
    layout::{Alignment, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::ui::theme::Theme;

/// Render a success message
pub fn render_success(frame: &mut Frame, area: Rect, theme: &Theme, message: &str) {
    let block = Block::default()
        .title("Success")
        .borders(Borders::ALL)
        .border_style(theme.success());

    let success_text = Text::from(vec![
        Line::from(vec![Span::styled("✓ ", theme.success())]),
        Line::from(message),
    ]);

    let paragraph = Paragraph::new(success_text)
        .block(block)
        .style(theme.success())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}