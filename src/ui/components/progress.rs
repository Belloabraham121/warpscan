//! Progress component for WarpScan
//!
//! This module contains the progress bar component.

use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Gauge},
    Frame,
};

/// Render a progress bar
pub fn render_progress(frame: &mut Frame, area: Rect, theme: &Theme, progress: f64, label: &str) {
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(label)
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .gauge_style(theme.progress())
        .ratio(progress.clamp(0.0, 1.0));

    frame.render_widget(gauge, area);
}
