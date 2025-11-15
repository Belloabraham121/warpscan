//! Help popup component for WarpScan
//!
//! This module contains the help popup component.

use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

/// Render a help popup
pub fn render_help_popup(frame: &mut Frame, area: Rect, theme: &Theme) {
    let help_text = vec![
        "Navigation:",
        "  ↑/↓ or j/k    - Move up/down",
        "  ←/→ or h/l    - Move left/right",
        "  Tab/Shift+Tab - Switch tabs",
        "  Enter         - Select/Confirm",
        "  Esc           - Go back/Cancel",
        "  q             - Quit",
        "",
        "Shortcuts:",
        "  Ctrl+C        - Force quit",
        "  Ctrl+R        - Refresh",
        "  Ctrl+L        - Clear cache",
        "  Ctrl+S        - Save",
        "  ?             - Show this help",
        "",
        "Press any key to close",
    ];

    let help_items: Vec<ListItem> = help_text
        .iter()
        .map(|&text| {
            if text.is_empty() {
                ListItem::new(Line::from(""))
            } else if text.ends_with(':') {
                ListItem::new(Line::from(Span::styled(text, theme.header())))
            } else if text.starts_with("  ") {
                ListItem::new(Line::from(text))
            } else {
                ListItem::new(Line::from(Span::styled(text, theme.info())))
            }
        })
        .collect();

    let help_list = List::new(help_items)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        )
        .style(theme.normal());

    // Center the popup
    let popup_area = centered_rect(60, 70, area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(help_list, popup_area);
}

/// Create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
