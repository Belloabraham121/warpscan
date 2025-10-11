use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::ui::{app::App, theme::Theme};

/// Render the gas tracker screen
pub fn render_gas_tracker(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Gas Tracker")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, chunks[0]);

    // Gas information
    let gas_info = if app.is_loading("gas_tracker") {
        Text::from("Loading gas information...")
    } else {
        Text::from(vec![
            Line::from("Current Gas Prices:"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Safe: ", theme.label()),
                Span::styled("20 gwei", theme.success()),
            ]),
            Line::from(vec![
                Span::styled("Standard: ", theme.label()),
                Span::styled("25 gwei", theme.warning()),
            ]),
            Line::from(vec![
                Span::styled("Fast: ", theme.label()),
                Span::styled("30 gwei", theme.error()),
            ]),
            Line::from(""),
            Line::from("Press 'r' to refresh"),
        ])
    };

    let gas_paragraph = Paragraph::new(gas_info)
        .block(
            Block::default()
                .title("Gas Information")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(gas_paragraph, chunks[1]);
}