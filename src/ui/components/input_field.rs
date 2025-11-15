//! Input field component for WarpScan
//!
//! This module contains the input field component.

use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render input field
pub fn render_input_field(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    title: &str,
    input: &str,
    cursor_position: usize,
    is_active: bool,
) {
    let style = if is_active {
        theme.input_active()
    } else {
        theme.input()
    };

    let border_style = if is_active {
        theme.primary()
    } else {
        theme.border()
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);

    let input_text = if is_active {
        // Show cursor
        let mut text = input.to_string();
        if cursor_position <= text.len() {
            text.insert(cursor_position, '│');
        }
        text
    } else {
        input.to_string()
    };

    let paragraph = Paragraph::new(input_text)
        .block(block)
        .style(style)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}
