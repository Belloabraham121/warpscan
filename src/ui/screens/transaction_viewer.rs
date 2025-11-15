//! Transaction viewer screen for WarpScan
//!
//! This module contains the transaction viewer screen implementation.

use crate::ui::{app::App, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the transaction viewer screen
pub fn render_transaction_viewer(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Input
            Constraint::Min(0),    // Content
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
        "Enter Transaction Hash:",
        app.get_input(),
        app.cursor_position,
        app.input_mode == crate::ui::InputMode::Editing,
    );

    // Content area
    if app.is_loading("transaction_search") {
        let message = "Loading transaction information...";
        crate::ui::components::render_loading(frame, chunks[2], theme, message);
    } else if let Some(ref error) = app.error_message {
        let error_text = Text::from(vec![
            Line::from(Span::styled("Error: ", theme.error())),
            Line::from(error.clone()),
        ]);
        let error_paragraph = Paragraph::new(error_text)
            .block(
                Block::default()
                    .title("Error")
                    .borders(Borders::ALL)
                    .border_style(theme.error()),
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(error_paragraph, chunks[2]);
    } else if let Some(ref tx_data) = app.transaction_data {
        render_transaction_details(frame, chunks[2], tx_data, app, theme);
    } else if !app.input.is_empty() {
        let prompt = Paragraph::new("Press Enter to search for transaction")
            .style(theme.muted())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Transaction Information")
                    .borders(Borders::ALL)
                    .border_style(theme.border()),
            );
        frame.render_widget(prompt, chunks[2]);
    } else {
        let prompt = Paragraph::new("Enter a transaction hash to view details")
            .style(theme.muted())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Transaction Information")
                    .borders(Borders::ALL)
                    .border_style(theme.border()),
            );
        frame.render_widget(prompt, chunks[2]);
    }
}

/// Render detailed transaction information
fn render_transaction_details(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    tx: &crate::ui::models::TransactionDetails,
    app: &App,
    theme: &Theme,
) {
    use crate::ui::models::TransactionStatus;

    // Split area into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Status and hash
            Constraint::Length(4), // Overview
            Constraint::Length(4), // Gas info
            Constraint::Length(6), // Additional details
            Constraint::Min(8),    // Transfers section (needs more space for detailed view)
            Constraint::Min(3),    // Input data (expandable)
        ])
        .split(area);

    // Status and Hash section
    let status_text = match tx.status {
        TransactionStatus::Success => ("✓ Success", theme.success()),
        TransactionStatus::Failed => ("✗ Failed", theme.error()),
        TransactionStatus::Pending => ("⏳ Pending", theme.warning()),
    };

    let status_line = Line::from(vec![
        Span::styled("Status: ", theme.label()),
        Span::styled(status_text.0, status_text.1),
        Span::raw(" | "),
        Span::styled("Hash: ", theme.label()),
        Span::styled(&tx.hash, theme.primary()),
    ]);

    let status_block = Paragraph::new(status_line).block(
        Block::default()
            .title("Transaction Status")
            .borders(Borders::ALL)
            .border_style(theme.border()),
    );
    frame.render_widget(status_block, chunks[0]);

    // Overview section (From, To, Value, Block)
    let to_address = tx
        .to
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Contract Creation");
    let to_style = if tx.to.is_some() {
        theme.info()
    } else {
        theme.warning()
    };

    let overview_lines = vec![
        Line::from(vec![
            Span::styled("From: ", theme.label()),
            Span::styled(&tx.from, theme.info()),
        ]),
        Line::from(vec![
            Span::styled("To: ", theme.label()),
            Span::styled(to_address, to_style),
        ]),
        Line::from(vec![
            Span::styled("Value: ", theme.label()),
            Span::styled(format!("{:.6} ETH", tx.value), theme.warning()),
            Span::raw(" | "),
            Span::styled("Block: ", theme.label()),
            Span::styled(tx.block_number.to_string(), theme.normal()),
        ]),
    ];

    let overview_block = Paragraph::new(Text::from(overview_lines)).block(
        Block::default()
            .title("Transaction Overview")
            .borders(Borders::ALL)
            .border_style(theme.border()),
    );
    frame.render_widget(overview_block, chunks[1]);

    // Gas Information section
    let gas_efficiency = if tx.gas_limit > 0 {
        (tx.gas_used as f64 / tx.gas_limit as f64) * 100.0
    } else {
        0.0
    };

    let gas_lines = vec![
        Line::from(vec![
            Span::styled("Gas Limit: ", theme.label()),
            Span::styled(tx.gas_limit.to_string(), theme.normal()),
            Span::raw(" | "),
            Span::styled("Gas Used: ", theme.label()),
            Span::styled(tx.gas_used.to_string(), theme.normal()),
            Span::raw(" | "),
            Span::styled("Efficiency: ", theme.label()),
            Span::styled(
                format!("{:.1}%", gas_efficiency),
                if gas_efficiency > 90.0 {
                    theme.warning()
                } else {
                    theme.success()
                },
            ),
        ]),
        Line::from(vec![
            Span::styled("Gas Price: ", theme.label()),
            Span::styled(format!("{} gwei", tx.gas_price), theme.normal()),
            Span::raw(" | "),
            Span::styled("Transaction Fee: ", theme.label()),
            Span::styled(format!("{:.6} ETH", tx.transaction_fee), theme.warning()),
        ]),
    ];

    let gas_block = Paragraph::new(Text::from(gas_lines)).block(
        Block::default()
            .title("Gas Information")
            .borders(Borders::ALL)
            .border_style(theme.border()),
    );
    frame.render_widget(gas_block, chunks[2]);

    // Additional Details section
    let timestamp_str = chrono::DateTime::from_timestamp(tx.timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let method_display = tx.method.as_ref().map(|m| m.as_str()).unwrap_or("N/A");

    let details_lines = vec![
        Line::from(vec![
            Span::styled("Timestamp: ", theme.label()),
            Span::styled(timestamp_str, theme.normal()),
            Span::raw(" | "),
            Span::styled("Confirmations: ", theme.label()),
            Span::styled(tx.confirmations.to_string(), theme.success()),
        ]),
        Line::from(vec![
            Span::styled("Nonce: ", theme.label()),
            Span::styled(tx.nonce.to_string(), theme.normal()),
            Span::raw(" | "),
            Span::styled("Transaction Index: ", theme.label()),
            Span::styled(
                tx.transaction_index
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
                theme.normal(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Method: ", theme.label()),
            Span::styled(method_display, theme.primary()),
        ]),
    ];

    // Add contract address if this is a contract creation
    let mut final_lines = details_lines;
    if let Some(ref contract_addr) = tx.contract_address {
        final_lines.push(Line::from(vec![
            Span::styled("Contract Created: ", theme.label()),
            Span::styled(contract_addr, theme.warning()),
        ]));
    }

    let details_block = Paragraph::new(Text::from(final_lines))
        .block(
            Block::default()
                .title("Additional Details")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(details_block, chunks[3]);

    // Transfers section
    render_transfers_section(frame, chunks[4], tx, theme);

    // Input Data section (expandable)
    render_input_data_section(frame, chunks[5], tx, app, theme);
}

/// Render expandable input data section
fn render_input_data_section(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    tx: &crate::ui::models::TransactionDetails,
    app: &App,
    theme: &Theme,
) {
    // Calculate available space for input data section
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let is_expanded = app.input_data_expanded;
    let preview_length = 66;
    let input_preview = if tx.input_data.len() > preview_length && !is_expanded {
        format!("{}...", &tx.input_data[..preview_length])
    } else {
        tx.input_data.clone()
    };

    // Format input data with word wrapping if expanded
    let input_lines: Vec<Line> = if is_expanded {
        // Split long hex string into chunks for better display
        let chunk_size = 64; // Display 64 chars per line
        input_preview
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|chunk| {
                Line::from(Span::styled(
                    chunk.iter().collect::<String>(),
                    theme.muted(),
                ))
            })
            .collect()
    } else {
        vec![Line::from(Span::styled(input_preview, theme.muted()))]
    };

    let expand_hint = if is_expanded {
        " (Press 'i' to collapse)"
    } else {
        " (Press 'i' to expand)"
    };

    let input_block = Paragraph::new(Text::from(input_lines))
        .block(
            Block::default()
                .title(format!("Input Data{}", expand_hint))
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(input_block, input_area[0]);
}

/// Render transfers section with all transfers and net transfers
fn render_transfers_section(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    tx: &crate::ui::models::TransactionDetails,
    theme: &Theme,
) {
    use crate::ui::models::transaction::TransferType;

    // Build transfer lines
    let mut transfer_lines = Vec::new();

    // Add header
    transfer_lines.push(Line::from(vec![Span::styled(
        "All Transfers:",
        theme.label().add_modifier(ratatui::style::Modifier::BOLD),
    )]));

    if tx.transfers.is_empty() {
        transfer_lines.push(Line::from(vec![Span::styled(
            "  No transfers found",
            theme.muted(),
        )]));
    } else {
        // Add each transfer with full details
        for (idx, transfer) in tx.transfers.iter().enumerate() {
            let type_text = match transfer.transfer_type {
                TransferType::ETH => "ETH Transfer".to_string(),
                TransferType::Token => {
                    if let Some(ref name) = transfer.token_name {
                        format!(
                            "{} ({})",
                            name,
                            transfer.token_symbol.as_deref().unwrap_or("Token")
                        )
                    } else {
                        transfer
                            .token_symbol
                            .as_deref()
                            .unwrap_or("Token")
                            .to_string()
                    }
                }
                TransferType::Internal => "Internal ETH Transfer".to_string(),
            };

            let value_text = if let Some(ref symbol) = transfer.token_symbol {
                format!("{:.18} {}", transfer.value, symbol)
            } else {
                format!("{:.18} ETH", transfer.value)
            };

            // Full address display with all details
            transfer_lines.push(Line::from(vec![
                Span::styled(format!("  {}. ", idx + 1), theme.muted()),
                Span::styled(
                    type_text,
                    theme.primary().add_modifier(ratatui::style::Modifier::BOLD),
                ),
            ]));
            transfer_lines.push(Line::from(vec![
                Span::raw("     From: "),
                Span::styled(&transfer.from, theme.info()),
            ]));
            transfer_lines.push(Line::from(vec![
                Span::raw("     To:   "),
                Span::styled(&transfer.to, theme.info()),
            ]));
            transfer_lines.push(Line::from(vec![
                Span::raw("     Value: "),
                Span::styled(
                    value_text,
                    theme.warning().add_modifier(ratatui::style::Modifier::BOLD),
                ),
            ]));
            if let Some(ref token_addr) = transfer.token_address {
                transfer_lines.push(Line::from(vec![
                    Span::raw("     Token Address: "),
                    Span::styled(token_addr, theme.primary()),
                ]));
            }
            transfer_lines.push(Line::from(vec![Span::raw("")])); // Empty line between transfers
        }
    }

    // Add net transfers section with transaction fee
    transfer_lines.push(Line::from(vec![Span::raw("")]));
    transfer_lines.push(Line::from(vec![Span::styled(
        "Net Transfers (including transaction fee):",
        theme.label().add_modifier(ratatui::style::Modifier::BOLD),
    )]));

    // Calculate net transfers per address including transaction fee
    use std::collections::HashMap;
    let mut net_transfers: HashMap<String, f64> = HashMap::new();

    // Add all transfers
    for transfer in &tx.transfers {
        // Subtract from sender
        *net_transfers.entry(transfer.from.clone()).or_insert(0.0) -= transfer.value;
        // Add to receiver
        *net_transfers.entry(transfer.to.clone()).or_insert(0.0) += transfer.value;
    }

    // Subtract transaction fee from the transaction sender (from address)
    if tx.transaction_fee > 0.0 {
        *net_transfers.entry(tx.from.clone()).or_insert(0.0) -= tx.transaction_fee;
    }

    // Sort addresses for consistent display
    let mut net_entries: Vec<_> = net_transfers.iter().collect();
    net_entries.sort_by(|a, b| a.0.cmp(b.0));

    if net_entries.is_empty() {
        transfer_lines.push(Line::from(vec![Span::styled(
            "  No net transfers",
            theme.muted(),
        )]));
    } else {
        for (address, net_value) in net_entries {
            if net_value.abs() > 0.000000000000000001 {
                // Show all transfers, even very small ones
                let net_style = if *net_value > 0.0 {
                    theme.success()
                } else {
                    theme.error()
                };
                let sign = if *net_value > 0.0 { "+" } else { "" };

                transfer_lines.push(Line::from(vec![
                    Span::styled("  • ", theme.muted()),
                    Span::styled("Address: ", theme.label()),
                    Span::styled(address, theme.info()),
                ]));
                transfer_lines.push(Line::from(vec![
                    Span::raw("    Net Change: "),
                    Span::styled(
                        format!("{}{:.18} ETH", sign, net_value),
                        net_style.add_modifier(ratatui::style::Modifier::BOLD),
                    ),
                ]));
                transfer_lines.push(Line::from(vec![Span::raw("")])); // Empty line
            }
        }
    }

    // Add transaction fee details
    if tx.transaction_fee > 0.0 {
        transfer_lines.push(Line::from(vec![Span::raw("")]));
        transfer_lines.push(Line::from(vec![Span::styled(
            "Transaction Fee Details:",
            theme.label().add_modifier(ratatui::style::Modifier::BOLD),
        )]));
        transfer_lines.push(Line::from(vec![
            Span::raw("  Paid by: "),
            Span::styled(&tx.from, theme.info()),
        ]));
        transfer_lines.push(Line::from(vec![
            Span::raw("  Amount: "),
            Span::styled(
                format!("{:.18} ETH", tx.transaction_fee),
                theme.warning().add_modifier(ratatui::style::Modifier::BOLD),
            ),
        ]));
        transfer_lines.push(Line::from(vec![
            Span::raw("  Gas Used: "),
            Span::styled(format!("{}", tx.gas_used), theme.normal()),
        ]));
        transfer_lines.push(Line::from(vec![
            Span::raw("  Gas Price: "),
            Span::styled(format!("{} gwei", tx.gas_price), theme.normal()),
        ]));
    }

    let transfers_block = Paragraph::new(Text::from(transfer_lines))
        .block(
            Block::default()
                .title("Transfers & Net Transfers")
                .borders(Borders::ALL)
                .border_style(theme.border()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(transfers_block, area);
}
