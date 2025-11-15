use crate::ui::{app::App, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

/// Render the wallet manager screen
pub fn render_wallet_manager(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Wallet Manager")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, chunks[0]);

    // Tabs
    let tab_titles = vec!["Wallets", "Generate", "Import"];
    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .select(app.current_tab)
        .style(theme.tab())
        .highlight_style(theme.tab_active());
    frame.render_widget(tabs, chunks[1]);

    // Content based on selected tab
    let content = match app.current_tab {
        0 => Text::from("Wallet list will be displayed here"),
        1 => Text::from("Wallet generation interface will be displayed here"),
        2 => Text::from("Wallet import interface will be displayed here"),
        _ => Text::from("Unknown tab"),
    };

    let content_paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title("Wallet Operations")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(content_paragraph, chunks[2]);
}
