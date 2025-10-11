//! Screen implementations for WarpScan
//!
//! This module contains the implementation of different screens/views in the application.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame,
};
use crate::ui::{app::App, theme::Theme};

/// Available screens in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    BlockExplorer,
    TransactionViewer,
    AddressLookup,
    ContractSearch,
    TokenInformation,
    GasTracker,
    ContractInteraction,
    ContractVerification,
    WalletManager,
    MultiSigWallet,
    EventMonitor,
    Help,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Home
    }
}

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
    let search_title = if app.input_mode == crate::ui::app::InputMode::Editing {
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
                .title_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(search_bar, main_chunks[1]);
    
    // Show cursor when in editing mode
     if app.input_mode == crate::ui::app::InputMode::Editing && app.current_tab == 2 {
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
        .title_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD));
    let price_text = Paragraph::new(format!(
        "${:.2}\n{:+.2}%",
        stats.ethereum_price, 4.15
    ))
    .style(ratatui::style::Style::default().fg(ratatui::style::Color::White).add_modifier(ratatui::style::Modifier::BOLD))
    .block(price_block)
    .alignment(Alignment::Center);
    frame.render_widget(price_text, stats_chunks[0]);

    // Market Cap with enhanced styling
    let market_cap_color = ratatui::style::Color::Green;
    let market_cap_block = Block::default()
        .title("📊 Market Cap")
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(market_cap_color))
        .title_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD));
    let market_cap_text = Paragraph::new(format!(
        "${:.2}B\n{:+.2}%",
        stats.market_cap / 1_000_000_000.0,
        1.23
    ))
    .style(ratatui::style::Style::default().fg(ratatui::style::Color::White).add_modifier(ratatui::style::Modifier::BOLD))
    .block(market_cap_block)
    .alignment(Alignment::Center);
    frame.render_widget(market_cap_text, stats_chunks[1]);

    // Latest Block with modern styling
    let latest_block_block = Block::default()
        .title("🔗 Latest Block")
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
        .title_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD));
    let latest_block_text = Paragraph::new(format!(
        "#{}\n12 secs ago",
        stats.latest_block
    ))
    .style(ratatui::style::Style::default().fg(ratatui::style::Color::White).add_modifier(ratatui::style::Modifier::BOLD))
    .block(latest_block_block)
    .alignment(Alignment::Center);
    frame.render_widget(latest_block_text, stats_chunks[2]);

    // Transaction History with enhanced styling
    let tx_history_block = Block::default()
        .title("📈 Transaction History")
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta))
        .title_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD));
    let tx_history_text = Paragraph::new(format!(
        "{:.1}M\n14.5 TPS",
        stats.transactions_count as f64 / 1_000_000.0
    ))
    .style(ratatui::style::Style::default().fg(ratatui::style::Color::White).add_modifier(ratatui::style::Modifier::BOLD))
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
                .title_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)),
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
fn render_latest_transactions(frame: &mut Frame, area: ratatui::layout::Rect, app: &App, _theme: &Theme) {
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
                .title_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green).add_modifier(ratatui::style::Modifier::BOLD)),
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

/// Render the block explorer screen
pub fn render_block_explorer(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Block Explorer")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, chunks[0]);

    // Input field for block number
    crate::ui::components::render_input_field(
        frame,
        chunks[1],
        theme,
        "Enter Block Number or Hash",
        app.get_input(),
        app.cursor_position,
        app.input_mode == crate::ui::app::InputMode::Editing,
    );

    // Content area
    let content = if app.is_loading("block_search") {
        Text::from("Loading block information...")
    } else if let Some(error) = &app.error_message {
        Text::from(vec![
            Line::from(Span::styled("Error: ", theme.error())),
            Line::from(error.clone()),
        ])
    } else {
        Text::from("Enter a block number or hash to search")
    };

    let content_paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title("Block Information")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(content_paragraph, chunks[2]);
}

/// Render the transaction viewer screen
pub fn render_transaction_viewer(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Transaction Viewer")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, chunks[0]);

    // Input field for transaction hash
    crate::ui::components::render_input_field(
        frame,
        chunks[1],
        theme,
        "Enter Transaction Hash",
        app.get_input(),
        app.cursor_position,
        app.input_mode == crate::ui::app::InputMode::Editing,
    );

    // Content area
    let content = if app.is_loading("transaction_search") {
        Text::from("Loading transaction information...")
    } else if let Some(error) = &app.error_message {
        Text::from(vec![
            Line::from(Span::styled("Error: ", theme.error())),
            Line::from(error.clone()),
        ])
    } else {
        Text::from("Enter a transaction hash to search")
    };

    let content_paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title("Transaction Information")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(content_paragraph, chunks[2]);
}

/// Render the address lookup screen
pub fn render_address_lookup(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Address Lookup")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.primary()),
        );
    frame.render_widget(title, chunks[0]);

    // Input field for address
    crate::ui::components::render_input_field(
        frame,
        chunks[1],
        theme,
        "Enter Ethereum Address",
        app.get_input(),
        app.cursor_position,
        app.input_mode == crate::ui::app::InputMode::Editing,
    );

    // Content area
    let content = if app.is_loading("address_search") {
        Text::from("Loading address information...")
    } else if let Some(error) = &app.error_message {
        Text::from(vec![
            Line::from(Span::styled("Error: ", theme.error())),
            Line::from(error.clone()),
        ])
    } else {
        Text::from("Enter an Ethereum address to lookup")
    };

    let content_paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title("Address Information")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(content_paragraph, chunks[2]);
}

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