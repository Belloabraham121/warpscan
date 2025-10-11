//! Status bar component for WarpScan
//!
//! This module contains the status bar component.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};
use crate::ui::theme::Theme;

/// Render status bar
pub fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    left_text: &str,
    right_text: &str,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_paragraph = Paragraph::new(left_text)
        .style(theme.status_bar())
        .alignment(Alignment::Left);

    let right_paragraph = Paragraph::new(right_text)
        .style(theme.status_bar())
        .alignment(Alignment::Right);

    frame.render_widget(left_paragraph, chunks[0]);
    frame.render_widget(right_paragraph, chunks[1]);
}