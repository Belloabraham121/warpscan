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
            Constraint::Length(3), // Title
            Constraint::Length(3), // Search input
            Constraint::Length(3), // Address type indicator
            Constraint::Length(3), // Tab navigation
            Constraint::Min(0),    // Content area
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

    // Check if we have address data to display
    if let Some(address_data) = &app.address_data {
        // Address type indicator
        render_address_type_indicator(frame, chunks[2], &address_data.details, theme);
        
        // Tab navigation
        render_address_tabs(frame, chunks[3], &address_data.current_tab, theme);
        
        // Content area based on current tab
        match address_data.current_tab {
            crate::ui::models::AddressTab::Details => {
                render_address_details_tab(frame, chunks[4], &address_data.details, theme);
            }
            crate::ui::models::AddressTab::Transactions => {
                render_address_transactions_tab(frame, chunks[4], &address_data.transactions, theme);
            }
            crate::ui::models::AddressTab::AccountHistory => {
                render_address_history_tab(frame, chunks[4], &address_data.account_history, theme);
            }
            crate::ui::models::AddressTab::TokenTransfers => {
                render_token_transfers_tab(frame, chunks[4], &address_data.token_transfers, theme);
            }
            crate::ui::models::AddressTab::Tokens => {
                render_tokens_tab(frame, chunks[4], &address_data.tokens, theme);
            }
            crate::ui::models::AddressTab::InternalTxns => {
                render_internal_txns_tab(frame, chunks[4], &address_data.internal_transactions, theme);
            }
        }
    } else {
        // Show loading or prompt
        let content = if app.is_loading("address_search") {
            Text::from("Loading address information...")
        } else if let Some(error) = &app.error_message {
            Text::from(vec![
                Line::from(Span::styled("Error: ", theme.error())),
                Line::from(error.clone()),
            ])
        } else {
            Text::from(vec![
                Line::from("Enter an Ethereum address to lookup"),
                Line::from(""),
                Line::from("Supported formats:"),
                Line::from("• 0x1234567890abcdef1234567890abcdef12345678"),
                Line::from("• ENS names (e.g., vitalik.eth)"),
                Line::from(""),
                Line::from("Press Enter to search"),
            ])
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
}

/// Render address type indicator
fn render_address_type_indicator(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    details: &crate::ui::models::AddressDetails,
    theme: &Theme,
) {
    use crate::ui::models::AddressType;
    
    let (type_text, type_style) = match details.address_type {
        AddressType::EOA => ("EOA (Wallet)", theme.success()),
        AddressType::Contract => ("Contract", theme.warning()),
        AddressType::Token => ("Token Contract", theme.primary()),
        AddressType::MultiSig => ("Multi-Sig Wallet", theme.info()),
        AddressType::Exchange => ("Exchange", theme.error()),
        AddressType::Unknown => ("Unknown", theme.muted()),
    };

    let indicator_text = if let Some(name) = &details.contract_name {
        format!("{} - {}", type_text, name)
    } else {
        type_text.to_string()
    };

    let indicator = Paragraph::new(Line::from(vec![
        Span::styled("Type: ", theme.label()),
        Span::styled(indicator_text, type_style),
        Span::raw(" | "),
        Span::styled("Address: ", theme.label()),
        Span::styled(&details.address, theme.normal()),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border()),
    );

    frame.render_widget(indicator, area);
}

/// Render address tabs navigation
fn render_address_tabs(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    current_tab: &crate::ui::models::AddressTab,
    theme: &Theme,
) {
    use crate::ui::models::AddressTab;
    
    let tab_titles = vec!["Details", "Transactions", "Account History", "Token Transfers", "Tokens", "Internal Txns"];
    
    let selected_index = match current_tab {
        AddressTab::Details => 0,
        AddressTab::Transactions => 1,
        AddressTab::AccountHistory => 2,
        AddressTab::TokenTransfers => 3,
        AddressTab::Tokens => 4,
        AddressTab::InternalTxns => 5,
    };

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .style(theme.normal())
        .highlight_style(theme.selected())
        .select(selected_index);

    frame.render_widget(tabs, area);
}

/// Render the Details tab
fn render_address_details_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    details: &crate::ui::models::AddressDetails,
    theme: &Theme,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left column - Basic info
    let left_content = Text::from(vec![
        Line::from(vec![
            Span::styled("Balance: ", theme.label()),
            Span::styled(format!("{:.4} ETH", details.balance), theme.success()),
        ]),
        Line::from(vec![
            Span::styled("Token Count: ", theme.label()),
            Span::styled(details.token_count.to_string(), theme.normal()),
        ]),
        Line::from(vec![
            Span::styled("Estimated Net Worth: ", theme.label()),
            Span::styled(format!("${:.2}", details.estimated_net_worth), theme.warning()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Total Transactions: ", theme.label()),
            Span::styled(details.total_transactions.to_string(), theme.normal()),
        ]),
        Line::from(vec![
            Span::styled("Outgoing Transfers: ", theme.label()),
            Span::styled(details.outgoing_transfers.to_string(), theme.normal()),
        ]),
        Line::from(vec![
            Span::styled("Total Gas Used: ", theme.label()),
            Span::styled(format!("{}", details.total_gas_used), theme.normal()),
        ]),
    ]);

    let left_paragraph = Paragraph::new(left_content)
        .block(
            Block::default()
                .title("Account Summary")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(left_paragraph, chunks[0]);

    // Right column - Additional info
    let mut right_lines = vec![
        Line::from(vec![
            Span::styled("Last Activity: ", theme.label()),
            Span::styled(
                chrono::DateTime::from_timestamp(details.last_activity as i64, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
                theme.normal()
            ),
        ]),
        Line::from(""),
    ];

    if let Some(creator) = &details.contract_creator {
        right_lines.push(Line::from(vec![
            Span::styled("Contract Creator: ", theme.label()),
            Span::styled(creator.clone(), theme.normal()),
        ]));
    }

    if let Some(creation_tx) = &details.creation_tx_hash {
        right_lines.push(Line::from(vec![
            Span::styled("Creation Tx: ", theme.label()),
            Span::styled(creation_tx.clone(), theme.normal()),
        ]));
    }

    let right_content = Text::from(right_lines);

    let right_paragraph = Paragraph::new(right_content)
        .block(
            Block::default()
                .title("Additional Information")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(right_paragraph, chunks[1]);
}

/// Render the Transactions tab
fn render_address_transactions_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    transactions: &[crate::ui::models::AddressTransaction],
    theme: &Theme,
) {
    let items: Vec<ListItem> = transactions
        .iter()
        .map(|tx| {
            let status_style = match tx.status {
                crate::ui::models::TransactionStatus::Success => theme.success(),
                crate::ui::models::TransactionStatus::Failed => theme.error(),
                crate::ui::models::TransactionStatus::Pending => theme.warning(),
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{:.10}...", tx.tx_hash), theme.normal()),
                Span::raw(" | "),
                Span::styled(&tx.tx_type, theme.label()),
                Span::raw(" | "),
                Span::styled(tx.block.to_string(), theme.normal()),
                Span::raw(" | "),
                Span::styled(format!("{:.8}...", tx.from), theme.normal()),
                Span::raw(" → "),
                Span::styled(format!("{:.8}...", tx.to), theme.normal()),
                Span::raw(" | "),
                Span::styled(format!("{:.4} ETH", tx.value), theme.warning()),
                Span::raw(" | "),
                Span::styled(format!("{:.6} ETH", tx.fee), status_style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Transactions (Tx Hash | Type | Method | Block | Value | Fee)")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .style(theme.normal());

    frame.render_widget(list, area);
}

/// Render the Account History tab
fn render_address_history_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    history: &[crate::ui::models::AccountHistoryEntry],
    theme: &Theme,
) {
    let items: Vec<ListItem> = history
        .iter()
        .map(|entry| {
            ListItem::new(Line::from(vec![
                Span::styled(&entry.age, theme.muted()),
                Span::raw(" | "),
                Span::styled(&entry.action, theme.label()),
                Span::raw(" | "),
                Span::styled(format!("{:.10}...", entry.from), theme.normal()),
                Span::raw(" → "),
                Span::styled(format!("{:.10}...", entry.to), theme.normal()),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Account History (Age | Action | From → To)")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .style(theme.normal());

    frame.render_widget(list, area);
}

/// Render the Token Transfers tab
fn render_token_transfers_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    transfers: &[crate::ui::models::TokenTransfer],
    theme: &Theme,
) {
    let items: Vec<ListItem> = transfers
        .iter()
        .map(|transfer| {
            let token_id_text = transfer.token_id
                .as_ref()
                .map(|id| format!("#{}", id))
                .unwrap_or_else(|| "N/A".to_string());

            ListItem::new(Line::from(vec![
                Span::styled(token_id_text, theme.info()),
                Span::raw(" | "),
                Span::styled(format!("{:.10}...", transfer.txn_hash), theme.normal()),
                Span::raw(" | "),
                Span::styled(format!("{:.8}...", transfer.from), theme.normal()),
                Span::raw(" → "),
                Span::styled(format!("{:.8}...", transfer.to), theme.normal()),
                Span::raw(" | "),
                Span::styled(format!("{} {}", transfer.amount, transfer.token_symbol), theme.warning()),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Token Transfers (Token ID | Txn Hash | From → To | Amount)")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .style(theme.normal());

    frame.render_widget(list, area);
}

/// Render the Tokens tab
fn render_tokens_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    tokens: &[crate::ui::models::TokenInfo],
    theme: &Theme,
) {
    let items: Vec<ListItem> = tokens
        .iter()
        .map(|token| {
            let token_type_text = match &token.token_type {
                crate::ui::models::TokenType::ERC20 => "ERC-20",
                crate::ui::models::TokenType::ERC721 => "ERC-721 (NFT)",
                crate::ui::models::TokenType::ERC1155 => "ERC-1155",
                crate::ui::models::TokenType::Other(name) => name,
            };

            ListItem::new(Line::from(vec![
                Span::styled(&token.symbol, theme.primary()),
                Span::raw(" | "),
                Span::styled(&token.name, theme.normal()),
                Span::raw(" | "),
                Span::styled(token_type_text, theme.label()),
                Span::raw(" | "),
                Span::styled(format!("{:.4}", token.balance), theme.warning()),
                Span::raw(" | "),
                Span::styled(format!("${:.2}", token.value_usd), theme.success()),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Tokens (Symbol | Name | Type | Balance | Value USD)")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .style(theme.normal());

    frame.render_widget(list, area);
}

/// Render the Internal Transactions tab
fn render_internal_txns_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    internal_txns: &[crate::ui::models::InternalTransaction],
    theme: &Theme,
) {
    let items: Vec<ListItem> = internal_txns
        .iter()
        .map(|tx| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:.10}...", tx.parent_tx), theme.muted()),
                Span::raw(" | "),
                Span::styled(format!("{:.10}...", tx.hash), theme.normal()),
                Span::raw(" | "),
                Span::styled(&tx.tx_type, theme.label()),
                Span::raw(" | "),
                Span::styled(tx.block.to_string(), theme.normal()),
                Span::raw(" | "),
                Span::styled(format!("{:.8}...", tx.from), theme.normal()),
                Span::raw(" → "),
                Span::styled(format!("{:.8}...", tx.to), theme.normal()),
                Span::raw(" | "),
                Span::styled(format!("{:.4}/{:.4}", tx.value_in, tx.value_out), theme.warning()),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Internal Txns (Parent Tx | Hash | Type | Block | From → To | Value In/Out)")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .style(theme.normal());

    frame.render_widget(list, area);
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