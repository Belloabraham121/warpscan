//! Loading component for WarpScan
//!
//! This module contains the loading spinner component.

use ratatui::{
    layout::{Alignment, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::ui::theme::Theme;

/// Render a loading spinner
pub fn render_loading(frame: &mut Frame, area: Rect, theme: &Theme, message: &str) {
    let block = Block::default()
        .title("Loading")
        .borders(Borders::ALL)
        .border_style(theme.border());

    let loading_text = Text::from(vec![
        Line::from(vec![Span::styled("⠋ ", theme.loading())]),
        Line::from(message),
    ]);

    let paragraph = Paragraph::new(loading_text)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}