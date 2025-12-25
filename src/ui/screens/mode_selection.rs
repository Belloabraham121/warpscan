//! Mode selection screen for choosing data source
//!
//! This module provides the startup mode selection screen where users choose
//! between Local Node (RPC) or Etherscan API for data queries.

use crate::ui::{app::App, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the mode selection screen
pub fn render_mode_selection(frame: &mut Frame, app: &App, _theme: &Theme) {
    let area = frame.area();

    // Create a centered popup-like layout
    let popup_area = centered_rect(60, 40, area);

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(5), // Description
            Constraint::Length(8), // Options
            Constraint::Length(3), // Instructions
            Constraint::Min(0),
        ])
        .split(popup_area);

    // Title
    let title = Paragraph::new("WARPSCAN - Select Data Source")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("🔧 Configuration")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        );
    frame.render_widget(title, chunks[0]);

    // Description
    let description = Paragraph::new(
        "Choose how WarpScan will fetch blockchain data:\n\n\
         • Local Node: Use Anvil/Hardhat RPC directly (no API key needed)\n\
         • Etherscan: Use Etherscan API (requires API key)",
    )
    .style(Style::default().fg(Color::White))
    .block(Block::default().borders(Borders::ALL))
    .wrap(Wrap { trim: true })
    .alignment(Alignment::Left);
    frame.render_widget(description, chunks[1]);

    // Options
    let option_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    // Local Node option
    let is_local_selected = app.current_tab == 0;
    let local_style = if is_local_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let local_border_style = if is_local_selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let local_option = Paragraph::new(
        "🖥️  Local Node\n\n\
         Use RPC endpoint directly\n\
         (Anvil/Hardhat)\n\n\
         • No API key needed\n\
         • Fast local queries\n\
         • Perfect for development",
    )
    .style(local_style)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(local_border_style)
            .title("Press [1] or ←")
            .title_style(Style::default().fg(Color::Cyan)),
    )
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    frame.render_widget(local_option, option_chunks[0]);

    // Etherscan option
    let is_etherscan_selected = app.current_tab == 1;
    let etherscan_style = if is_etherscan_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let etherscan_border_style = if is_etherscan_selected {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Gray)
    };

    let etherscan_option = Paragraph::new(
        "🌐 Etherscan API\n\n\
         Use Etherscan API for data\n\n\
         • Requires API key\n\
         • Rich indexed data\n\
         • Works with mainnet/testnets",
    )
    .style(etherscan_style)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(etherscan_border_style)
            .title("Press [2] or →")
            .title_style(Style::default().fg(Color::Green)),
    )
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    frame.render_widget(etherscan_option, option_chunks[1]);

    // Instructions
    let instructions = Paragraph::new(
        "Use arrow keys (← →) or number keys (1/2) to select, then press ENTER to confirm",
    )
    .style(Style::default().fg(Color::Yellow))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(instructions, chunks[3]);
}

/// Helper function to center a rect with given width and height percentages
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

