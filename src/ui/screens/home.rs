//! Home screen for WarpScan
//!
//! This module contains the home screen implementation with dashboard functionality.

use crate::ui::{app::App, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

/// Render the home screen with TegroScan-style dashboard
pub fn render_home(frame: &mut Frame, app: &App, theme: &Theme) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Search bar
            Constraint::Length(6), // Network stats
            Constraint::Min(0),    // Content area
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("WARPSCAN")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, main_chunks[0]);

    // Create the search bar with enhanced styling
    let search_title = if app.input_mode == crate::ui::InputMode::Editing {
        "🔍 Search (Editing)"
    } else {
        "🔍 Search (Press / or s to search)"
    };

    let search_style = if app.current_tab == 2 {
        ratatui::style::Style::default()
            .fg(ratatui::style::Color::Black)
            .bg(ratatui::style::Color::Cyan)
    } else {
        ratatui::style::Style::default()
            .fg(ratatui::style::Color::White)
            .bg(ratatui::style::Color::Black)
    };

    let search_border_color = if app.current_tab == 2 {
        ratatui::style::Color::Cyan
    } else {
        ratatui::style::Color::Blue
    };

    let search_bar = Paragraph::new(app.get_input())
        .style(search_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(search_title)
                .border_style(ratatui::style::Style::default().fg(search_border_color))
                .title_style(
                    ratatui::style::Style::default()
                        .fg(ratatui::style::Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(search_bar, main_chunks[1]);

    // Show cursor when in editing mode
    if app.input_mode == crate::ui::InputMode::Editing && app.current_tab == 2 {
        frame.set_cursor_position((
            main_chunks[1].x + app.cursor_position as u16 + 1,
            main_chunks[1].y + 1,
        ));
    }

    // Network statistics
    render_network_stats(frame, main_chunks[2], app, theme);

    // Content area with blocks and transactions (no graph)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[3]);

    render_latest_blocks(frame, content_chunks[0], app, theme);
    render_latest_transactions(frame, content_chunks[1], app, theme);
}

/// Render network statistics section
fn render_network_stats(frame: &mut Frame, area: ratatui::layout::Rect, app: &App, _theme: &Theme) {
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let stats = &app.dashboard_data.network_stats;

    // Network Price with gradient-like styling
    let price_color = if stats.ethereum_price >= 0.0 {
        ratatui::style::Color::Green
    } else {
        ratatui::style::Color::Red
    };
    let price_block = Block::default()
        .title("💰 Network Price")
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(price_color))
        .title_style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );
    let price_text = Paragraph::new(format!("${:.2}\n{:+.2}%", stats.ethereum_price, 4.15))
        .style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::White)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .block(price_block)
        .alignment(Alignment::Center);
    frame.render_widget(price_text, stats_chunks[0]);

    // Market Cap with enhanced styling
    let market_cap_color = ratatui::style::Color::Green;
    let market_cap_block = Block::default()
        .title("📊 Market Cap")
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(market_cap_color))
        .title_style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );
    let market_cap_text = Paragraph::new(format!(
        "${:.2}B\n{:+.2}%",
        stats.market_cap / 1_000_000_000.0,
        1.23
    ))
    .style(
        ratatui::style::Style::default()
            .fg(ratatui::style::Color::White)
            .add_modifier(ratatui::style::Modifier::BOLD),
    )
    .block(market_cap_block)
    .alignment(Alignment::Center);
    frame.render_widget(market_cap_text, stats_chunks[1]);

    // Latest Block with network info
    let network_name = &app.config.network.name;
    let chain_id = app.config.network.chain_id;
    
    let latest_block_block = Block::default()
        .title("🔗 Latest Block")
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
        .title_style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );
    
    // Show block number and network info
    let block_info_text = if stats.latest_block > 0 {
        format!("#{}\n{}", stats.latest_block, stats.block_time)
    } else {
        "Loading...".to_string()
    };
    
    let latest_block_text = Paragraph::new(block_info_text)
        .style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::White)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .block(latest_block_block)
        .alignment(Alignment::Center);
    frame.render_widget(latest_block_text, stats_chunks[2]);
    
    // Add network info below the stats (if space allows)
    let network_info = format!("🌐 {} (Chain {})", network_name, chain_id);
    if stats_chunks[2].height > 4 {
        let network_para = Paragraph::new(network_info)
            .style(
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Cyan)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )
            .alignment(Alignment::Center);
        let network_area = ratatui::layout::Rect {
            x: stats_chunks[2].x,
            y: stats_chunks[2].y + stats_chunks[2].height - 1,
            width: stats_chunks[2].width,
            height: 1,
        };
        frame.render_widget(network_para, network_area);
    }

    // Transaction History with enhanced styling
    let tx_history_block = Block::default()
        .title("📈 Transaction History")
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta))
        .title_style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );
    let tx_history_text = Paragraph::new(format!(
        "{:.1}M\n14.5 TPS",
        stats.transactions_count as f64 / 1_000_000.0
    ))
    .style(
        ratatui::style::Style::default()
            .fg(ratatui::style::Color::White)
            .add_modifier(ratatui::style::Modifier::BOLD),
    )
    .block(tx_history_block)
    .alignment(Alignment::Center);
    frame.render_widget(tx_history_text, stats_chunks[3]);
}

/// Render latest blocks section
fn render_latest_blocks(frame: &mut Frame, area: ratatui::layout::Rect, app: &App, _theme: &Theme) {
    let blocks = &app.dashboard_data.latest_blocks;

    let block_items: Vec<ListItem> = blocks
        .iter()
        .enumerate()
        .map(|(i, block)| {
            let style = if app.current_tab == 0 && i == app.current_list_index {
                ratatui::style::Style::default()
                    .bg(ratatui::style::Color::Cyan)
                    .fg(ratatui::style::Color::Black)
                    .add_modifier(ratatui::style::Modifier::BOLD)
            } else {
                ratatui::style::Style::default().fg(ratatui::style::Color::White)
            };

            let content = format!(
                "#{:<8} │ {:<12} │ {:>3} txns │ {}",
                block.number,
                &block.hash[..12],
                block.transaction_count,
                block.timestamp
            );
            ListItem::new(content).style(style)
        })
        .collect();

    let border_color = if app.current_tab == 0 {
        ratatui::style::Color::Cyan
    } else {
        ratatui::style::Color::Blue
    };

    let blocks_list = List::new(block_items)
        .block(
            Block::default()
                .title("🔗 Latest Blocks")
                .borders(Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(border_color))
                .title_style(
                    ratatui::style::Style::default()
                        .fg(ratatui::style::Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
        )
        .highlight_style(
            ratatui::style::Style::default()
                .bg(ratatui::style::Color::Cyan)
                .fg(ratatui::style::Color::Black)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    if app.current_tab == 0 {
        list_state.select(Some(app.current_list_index));
    }

    frame.render_stateful_widget(blocks_list, area, &mut list_state);
}

/// Render latest transactions section
fn render_latest_transactions(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    app: &App,
    _theme: &Theme,
) {
    let transactions = &app.dashboard_data.latest_transactions;

    let tx_items: Vec<ListItem> = transactions
        .iter()
        .enumerate()
        .map(|(i, tx)| {
            let style = if app.current_tab == 1 && i == app.current_list_index {
                ratatui::style::Style::default()
                    .bg(ratatui::style::Color::Green)
                    .fg(ratatui::style::Color::Black)
                    .add_modifier(ratatui::style::Modifier::BOLD)
            } else {
                ratatui::style::Style::default().fg(ratatui::style::Color::White)
            };

            let content = format!(
                "{:<14} │ {:<10} │ {:>8.4} ETH │ {}",
                &tx.hash[..14],
                &tx.from[..10],
                tx.value,
                tx.timestamp
            );
            ListItem::new(content).style(style)
        })
        .collect();

    let border_color = if app.current_tab == 1 {
        ratatui::style::Color::Green
    } else {
        ratatui::style::Color::Blue
    };

    let transactions_list = List::new(tx_items)
        .block(
            Block::default()
                .title("💸 Latest Transactions")
                .borders(Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(border_color))
                .title_style(
                    ratatui::style::Style::default()
                        .fg(ratatui::style::Color::Green)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
        )
        .highlight_style(
            ratatui::style::Style::default()
                .bg(ratatui::style::Color::Green)
                .fg(ratatui::style::Color::Black)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    if app.current_tab == 1 {
        list_state.select(Some(app.current_list_index));
    }

    frame.render_stateful_widget(transactions_list, area, &mut list_state);
}
