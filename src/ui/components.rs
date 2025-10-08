//! UI Components for WarpScan
//!
//! This module contains reusable UI components for the terminal interface.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap, Sparkline,
    },
    Frame,
};
use crate::ui::theme::Theme;
use crate::ui::models::DailyTransactionData;

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

/// Render daily transactions graph
pub fn render_daily_transactions_graph(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    data: &[DailyTransactionData],
) {
    // Use all 31 days of data for full coverage
    let values: Vec<u64> = data
        .iter()
        .map(|d| d.transaction_count)
        .collect();

    if values.is_empty() {
        return;
    }

    // Create layout with title and chart area that fully covers the box
    let chart_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title and stats area
            Constraint::Min(0),    // Chart area (fills remaining space)
        ])
        .margin(0) // No margin to use full width
        .split(area);

    // Title and stats in the top area
    let top_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(1), // Stats
        ])
        .split(chart_layout[0]);

    // Render title
    let title = Paragraph::new("Daily Transactions (Last 31 Days)")
        .style(theme.header())
        .alignment(Alignment::Center);
    frame.render_widget(title, top_layout[0]);

    // Render stats in one line
    let avg = values.iter().sum::<u64>() / values.len() as u64;
    let max_val = values.iter().max().copied().unwrap_or(0);
    let min_val = values.iter().min().copied().unwrap_or(0);

    let stats_text = format!(
        "Avg: {:.0}K  Max: {:.0}K  Min: {:.0}K",
        avg as f64 / 1_000.0,
        max_val as f64 / 1_000.0,
        min_val as f64 / 1_000.0
    );

    let stats_paragraph = Paragraph::new(stats_text)
        .style(theme.info())
        .alignment(Alignment::Center);
    frame.render_widget(stats_paragraph, top_layout[1]);

    // Using left and right borders only (like the ratatui example)
    // This maximizes chart width while maintaining visual separation
    // Width = area.width - 2 (only for left and right borders, no top/bottom)
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(theme.border())
                .title("Transaction Volume"),
        )
        .data(&values)
        .style(theme.primary())
        .max(max_val)
        .direction(ratatui::widgets::RenderDirection::LeftToRight);
    
    frame.render_widget(sparkline, chart_layout[1]);
}