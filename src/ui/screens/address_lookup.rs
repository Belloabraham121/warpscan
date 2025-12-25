use crate::ui::{app::App, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState, Tabs, Wrap,
    },
    Frame,
};

/// Render the address lookup screen
pub fn render_address_lookup(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Input
            Constraint::Min(0),    // Content
        ])
        .split(frame.area());

    // Title
    let title = ratatui::widgets::Paragraph::new("Address Lookup")
        .style(theme.title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(theme.border()),
        );
    frame.render_widget(title, chunks[0]);

    // Input field
    crate::ui::components::render_input_field(
        frame,
        chunks[1],
        theme,
        "Enter address:",
        &app.input,
        app.cursor_position,
        app.input_mode == crate::ui::InputMode::Editing,
    );

    // Content area
    if let Some(ref address_data) = app.address_data {
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Address type indicator
                Constraint::Length(3), // Tabs
                Constraint::Min(0),    // Tab content
            ])
            .split(chunks[2]);

        // Address type indicator
        render_address_type_indicator(frame, content_chunks[0], &address_data.details, theme);

        // Tabs
        render_address_tabs(frame, content_chunks[1], &address_data.current_tab, theme);

        // Tab content
        match address_data.current_tab {
            crate::ui::models::AddressTab::Details => {
                render_address_details_tab(frame, content_chunks[2], &address_data.details, theme);
            }
            crate::ui::models::AddressTab::Transactions => {
                render_address_transactions_tab(
                    frame,
                    content_chunks[2],
                    &address_data.transactions,
                    address_data.selected_transaction_index,
                    theme,
                );
            }
            crate::ui::models::AddressTab::AccountHistory => {
                render_address_history_tab(
                    frame,
                    content_chunks[2],
                    &address_data.account_history,
                    address_data.selected_history_index,
                    theme,
                );
            }
            crate::ui::models::AddressTab::TokenTransfers => {
                render_token_transfers_tab(
                    frame,
                    content_chunks[2],
                    &address_data.token_transfers,
                    address_data.selected_token_transfer_index,
                    theme,
                );
            }
            crate::ui::models::AddressTab::Tokens => {
                render_tokens_tab(
                    frame,
                    content_chunks[2],
                    &address_data.tokens,
                    address_data.selected_token_index,
                    theme,
                );
            }
            crate::ui::models::AddressTab::InternalTxns => {
                render_internal_txns_tab(
                    frame,
                    content_chunks[2],
                    &address_data.internal_transactions,
                    address_data.selected_internal_txn_index,
                    theme,
                );
            }
        }
    } else if !app.input.is_empty() {
        let message = if let Some(ref error) = app.error_message {
            error.clone()
        } else {
            "Loading address data...".to_string()
        };

        crate::ui::components::render_loading(frame, chunks[2], theme, &message);
    } else {
        let prompt = ratatui::widgets::Paragraph::new("Enter an Ethereum address to view details")
            .style(theme.muted())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_style(theme.border()),
            );
        frame.render_widget(prompt, chunks[2]);
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
        AddressType::EOA => ("Wallet", theme.success()),
        AddressType::Contract => ("Contract", theme.warning()),
        AddressType::Token => ("Token Contract", theme.primary()),
        AddressType::MultiSig => ("Multi-Sig Wallet", theme.info()),
        AddressType::Exchange => ("Exchange", theme.error()),
        AddressType::Unknown => ("Unknown", theme.muted()),
    };

    // Build the indicator line with type, address, and optionally ENS name
    let mut spans = vec![
        Span::styled("Type: ", theme.label()),
        Span::styled(type_text, type_style),
        Span::raw(" | "),
        Span::styled("Address: ", theme.label()),
        Span::styled(&details.address, theme.normal()),
    ];

    // Add ENS name for EOA addresses if it exists
    if let AddressType::EOA = details.address_type {
        if let Some(ens_name) = &details.ens_name {
            spans.push(Span::raw(" | "));
            spans.push(Span::styled("ENS: ", theme.label()));
            spans.push(Span::styled(ens_name, theme.primary()));
        }
    }

    let indicator = Paragraph::new(Line::from(spans)).block(
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

    let tab_titles = vec![
        "Details",
        "Transactions",
        "Account History",
        "Token Transfers",
        "Tokens",
        "Internal Txns",
    ];

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
            Span::styled(
                {
                    // Format balance with appropriate precision
                    let balance_value = details.balance;
                    if balance_value >= 1_000_000.0 {
                        format!("{:.2} ETH", balance_value)
                    } else if balance_value >= 1_000.0 {
                        format!("{:.2} ETH", balance_value)
                    } else if balance_value >= 1.0 {
                        format!("{:.4} ETH", balance_value)
                    } else {
                        format!("{:.6} ETH", balance_value)
                    }
                },
                theme.success(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Address Type: ", theme.label()),
            Span::styled(
                match details.address_type {
                    crate::ui::models::AddressType::EOA => "EOA (Wallet)",
                    crate::ui::models::AddressType::Contract => "Contract",
                    crate::ui::models::AddressType::Token => "Token Contract",
                    crate::ui::models::AddressType::MultiSig => "Multi-Sig Wallet",
                    crate::ui::models::AddressType::Exchange => "Exchange",
                    crate::ui::models::AddressType::Unknown => "Unknown",
                },
                theme.primary(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Token Count: ", theme.label()),
            Span::styled(details.token_count.to_string(), theme.normal()),
        ]),
        Line::from(vec![
            Span::styled("Estimated Net Worth: ", theme.label()),
            Span::styled(
                format!("${:.2}", details.estimated_net_worth),
                theme.warning(),
            ),
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
                theme.normal(),
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
    selected_index: usize,
    theme: &Theme,
) {
    // Header
    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Transaction Hash",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Method",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Block",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "From",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "To",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Amount",
            theme.label().add_modifier(Modifier::BOLD),
        )),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD));

    // Show empty state if no transactions
    if transactions.is_empty() {
        let empty_message = Paragraph::new("No transactions found")
            .style(theme.muted())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Transactions")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(theme.border()),
            );
        frame.render_widget(empty_message, area);
        return;
    }

    // Rows
    let rows: Vec<Row> = transactions
        .iter()
        .enumerate()
        .map(|(idx, tx)| {
            let is_selected = idx == selected_index;
            let row_style = if is_selected {
                theme.selected()
            } else {
                match tx.status {
                    crate::ui::models::TransactionStatus::Success => theme.success(),
                    crate::ui::models::TransactionStatus::Failed => theme.error(),
                    crate::ui::models::TransactionStatus::Pending => theme.warning(),
                }
            };

            let age = chrono::DateTime::from_timestamp(tx.timestamp as i64, 0)
                .map(|dt| {
                    let now = chrono::Utc::now();
                    let dur = now.signed_duration_since(dt);
                    if dur.num_days() > 0 {
                        format!("{}d ago", dur.num_days())
                    } else if dur.num_hours() > 0 {
                        format!("{}h ago", dur.num_hours())
                    } else if dur.num_minutes() > 0 {
                        format!("{}m ago", dur.num_minutes())
                    } else {
                        format!("{}s ago", dur.num_seconds())
                    }
                })
                .unwrap_or_else(|| "—".to_string());

            // Simplify method to only the function name
            let method_display = if tx.method.is_empty() {
                tx.tx_type.clone()
            } else {
                let m = tx.method.trim();
                let name = m.split('(').next().unwrap_or(m);
                name.to_string()
            };

            // Make transaction hash and addresses clickable (use primary color)
            let hash_style = if is_selected {
                theme.selected()
            } else {
                theme.primary().add_modifier(Modifier::UNDERLINED)
            };

            let address_style = if is_selected {
                theme.selected()
            } else {
                theme.info().add_modifier(Modifier::UNDERLINED)
            };

            Row::new(vec![
                Cell::from(Span::styled(format!("{:.10}...", tx.tx_hash), hash_style)),
                Cell::from(Span::styled(method_display, row_style)),
                Cell::from(Span::styled(tx.block.to_string(), row_style)),
                Cell::from(Span::styled(age, theme.muted())),
                Cell::from(Span::styled(format!("{:.10}...", tx.from), address_style)),
                Cell::from(Span::styled(format!("{:.10}...", tx.to), address_style)),
                Cell::from(Span::styled(
                    format!("{:.4} ETH", tx.value),
                    theme.warning(),
                )),
            ])
            .style(row_style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(18),     // hash
            Constraint::Length(14),     // method
            Constraint::Length(10),     // block
            Constraint::Length(10),     // age
            Constraint::Percentage(20), // from
            Constraint::Percentage(20), // to
            Constraint::Length(14),     // amount
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Transactions (Press Enter on row to view details, click addresses to navigate)")
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(theme.border()),
    )
    .column_spacing(1)
    .highlight_style(theme.selected())
    .highlight_symbol(" █ ");

    let mut state = TableState::default();
    state.select(Some(selected_index));
    frame.render_stateful_widget(table, area, &mut state);

    // Render scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None);
    let mut scrollbar_state =
        ScrollbarState::new(transactions.len().saturating_sub(1)).position(selected_index);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
}

/// Render the Account History tab
fn render_address_history_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    history: &[crate::ui::models::AccountHistoryEntry],
    selected_index: usize,
    theme: &Theme,
) {
    // Show empty state if no history
    if history.is_empty() {
        let empty_message = Paragraph::new("No account history found")
            .style(theme.muted())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Account History")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(theme.border()),
            );
        frame.render_widget(empty_message, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Age",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Action",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "From",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "To",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Transaction",
            theme.label().add_modifier(Modifier::BOLD),
        )),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = history
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            let is_selected = idx == selected_index;
            let row_style = if is_selected {
                theme.selected()
            } else {
                // Remove background color - just use foreground color
                Style::default().fg(theme.foreground)
            };

            let address_style = if is_selected {
                theme.selected()
            } else {
                theme.info().add_modifier(Modifier::UNDERLINED)
            };

            let hash_style = if is_selected {
                theme.selected()
            } else {
                theme.primary().add_modifier(Modifier::UNDERLINED)
            };

            Row::new(vec![
                Cell::from(Span::styled(&entry.age, theme.muted())),
                Cell::from(Span::styled(&entry.action, row_style)),
                Cell::from(Span::styled(
                    format!("{:.10}...", entry.from),
                    address_style,
                )),
                Cell::from(Span::styled(format!("{:.10}...", entry.to), address_style)),
                Cell::from(Span::styled(
                    format!("{:.10}...", entry.tx_hash),
                    hash_style,
                )),
            ])
            .style(row_style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(12), // age
            Constraint::Length(15), // action
            Constraint::Percentage(25), // from
            Constraint::Percentage(25), // to
            Constraint::Percentage(23), // transaction
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Account History (Press Enter on row to view transaction, click addresses to navigate)")
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(theme.border()),
    )
    .column_spacing(1)
    .highlight_style(theme.selected())
    .highlight_symbol(" █ ");

    let mut state = TableState::default();
    state.select(Some(selected_index));
    frame.render_stateful_widget(table, area, &mut state);

    // Render scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None);
    let mut scrollbar_state =
        ScrollbarState::new(history.len().saturating_sub(1)).position(selected_index);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
}

/// Render the Token Transfers tab
fn render_token_transfers_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    transfers: &[crate::ui::models::TokenTransfer],
    selected_index: usize,
    theme: &Theme,
) {
    // Show empty state if no transfers
    if transfers.is_empty() {
        let empty_message = Paragraph::new("No token transfers found")
            .style(theme.muted())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Token Transfers")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(theme.border()),
            );
        frame.render_widget(empty_message, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Token ID",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Transaction",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "From",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "To",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Token",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Amount",
            theme.label().add_modifier(Modifier::BOLD),
        )),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = transfers
        .iter()
        .enumerate()
        .map(|(idx, transfer)| {
            let is_selected = idx == selected_index;
            let row_style = if is_selected {
                theme.selected()
            } else {
                // Remove background color - just use foreground color
                Style::default().fg(theme.foreground)
            };

            let token_id_text = transfer
                .token_id
                .as_ref()
                .map(|id| format!("#{}", id))
                .unwrap_or_else(|| "N/A".to_string());

            let address_style = if is_selected {
                theme.selected()
            } else {
                theme.info().add_modifier(Modifier::UNDERLINED)
            };

            let hash_style = if is_selected {
                theme.selected()
            } else {
                theme.primary().add_modifier(Modifier::UNDERLINED)
            };

            Row::new(vec![
                Cell::from(Span::styled(token_id_text, theme.info())),
                Cell::from(Span::styled(
                    format!("{:.10}...", transfer.txn_hash),
                    hash_style,
                )),
                Cell::from(Span::styled(
                    format!("{:.10}...", transfer.from),
                    address_style,
                )),
                Cell::from(Span::styled(
                    format!("{:.10}...", transfer.to),
                    address_style,
                )),
                Cell::from(Span::styled(&transfer.token_symbol, row_style)),
                Cell::from(Span::styled(
                    format!("{:.4}", transfer.amount),
                    theme.warning(),
                )),
            ])
            .style(row_style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(12), // token id
            Constraint::Length(18), // transaction
            Constraint::Percentage(20), // from
            Constraint::Percentage(20), // to
            Constraint::Length(10), // token symbol
            Constraint::Length(15), // amount
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Token Transfers (Press Enter on row to view transaction, click addresses to navigate)")
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(theme.border()),
    )
    .column_spacing(1)
    .highlight_style(theme.selected())
    .highlight_symbol(" █ ");

    let mut state = TableState::default();
    state.select(Some(selected_index));
    frame.render_stateful_widget(table, area, &mut state);

    // Render scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None);
    let mut scrollbar_state =
        ScrollbarState::new(transfers.len().saturating_sub(1)).position(selected_index);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
}

/// Render the Tokens tab
fn render_tokens_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    tokens: &[crate::ui::models::TokenInfo],
    selected_index: usize,
    theme: &Theme,
) {
    // Show empty state if no tokens
    if tokens.is_empty() {
        let empty_message = Paragraph::new("No tokens found")
            .style(theme.muted())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Tokens")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(theme.border()),
            );
        frame.render_widget(empty_message, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Symbol",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Type",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Contract",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Balance",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Value USD",
            theme.label().add_modifier(Modifier::BOLD),
        )),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = tokens
        .iter()
        .enumerate()
        .map(|(idx, token)| {
            let is_selected = idx == selected_index;
            let row_style = if is_selected {
                theme.selected()
            } else {
                // Remove background color - just use foreground color
                Style::default().fg(theme.foreground)
            };

            let token_type_text = match &token.token_type {
                crate::ui::models::TokenType::ERC20 => "ERC-20",
                crate::ui::models::TokenType::ERC721 => "ERC-721 (NFT)",
                crate::ui::models::TokenType::ERC1155 => "ERC-1155",
                crate::ui::models::TokenType::Other(name) => name,
            };

            let address_style = if is_selected {
                theme.selected()
            } else {
                theme.info().add_modifier(Modifier::UNDERLINED)
            };

            // Use different colors for different columns to make it less bland
            let symbol_style = if is_selected {
                theme.selected()
            } else {
                theme.primary()
            };
            let name_style = if is_selected {
                theme.selected()
            } else {
                Style::default().fg(theme.foreground)
            };
            let type_style = if is_selected {
                theme.selected()
            } else {
                theme.label()
            };
            let balance_style = if is_selected {
                theme.selected()
            } else {
                theme.warning()
            };
            let value_style = if is_selected {
                theme.selected()
            } else {
                theme.success()
            };

            Row::new(vec![
                Cell::from(Span::styled(&token.symbol, symbol_style)),
                Cell::from(Span::styled(&token.name, name_style)),
                Cell::from(Span::styled(token_type_text, type_style)),
                Cell::from(Span::styled(
                    format!("{:.10}...", token.contract_address),
                    address_style,
                )),
                Cell::from(Span::styled(format!("{:.4}", token.balance), balance_style)),
                Cell::from(Span::styled(
                    format!("${:.2}", token.value_usd),
                    value_style,
                )),
            ])
            .style(row_style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(10), // symbol
            Constraint::Length(20), // name
            Constraint::Length(12), // type
            Constraint::Length(18), // contract
            Constraint::Length(15), // balance
            Constraint::Length(12), // value
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Tokens (Click contract address to view details)")
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(theme.border()),
    )
    .column_spacing(1)
    .highlight_style(theme.selected())
    .highlight_symbol(" █ ");

    let mut state = TableState::default();
    state.select(Some(selected_index));
    frame.render_stateful_widget(table, area, &mut state);

    // Render scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None);
    let mut scrollbar_state =
        ScrollbarState::new(tokens.len().saturating_sub(1)).position(selected_index);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
}

/// Render the Internal Transactions tab
fn render_internal_txns_tab(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    internal_txns: &[crate::ui::models::InternalTransaction],
    selected_index: usize,
    theme: &Theme,
) {
    // Show empty state if no internal transactions
    if internal_txns.is_empty() {
        let empty_message = Paragraph::new("No internal transactions found")
            .style(theme.muted())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Internal Transactions")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(theme.border()),
            );
        frame.render_widget(empty_message, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Parent Tx",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Type",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Block",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "From",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "To",
            theme.label().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Value",
            theme.label().add_modifier(Modifier::BOLD),
        )),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = internal_txns
        .iter()
        .enumerate()
        .map(|(idx, tx)| {
            let is_selected = idx == selected_index;
            let row_style = if is_selected {
                theme.selected()
            } else {
                // Remove background color - just use foreground color
                Style::default().fg(theme.foreground)
            };

            let address_style = if is_selected {
                theme.selected()
            } else {
                theme.info().add_modifier(Modifier::UNDERLINED)
            };

            let hash_style = if is_selected {
                theme.selected()
            } else {
                theme.primary().add_modifier(Modifier::UNDERLINED)
            };

            Row::new(vec![
                Cell::from(Span::styled(
                    format!("{:.10}...", tx.parent_tx_hash),
                    hash_style,
                )),
                Cell::from(Span::styled(&tx.tx_type, row_style)),
                Cell::from(Span::styled(tx.block.to_string(), row_style)),
                Cell::from(Span::styled(format!("{:.10}...", tx.from), address_style)),
                Cell::from(Span::styled(format!("{:.10}...", tx.to), address_style)),
                Cell::from(Span::styled(format!("{:.4} ETH", tx.value), row_style)),
            ])
            .style(row_style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(18),     // parent tx
            Constraint::Length(12),     // type
            Constraint::Length(10),     // block
            Constraint::Percentage(25), // from
            Constraint::Percentage(25), // to
            Constraint::Length(14),     // value
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Internal Transactions (Click addresses or transactions to navigate)")
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(theme.border()),
    )
    .column_spacing(1)
    .highlight_style(theme.selected())
    .highlight_symbol(" █ ");

    let mut state = TableState::default();
    state.select(Some(selected_index));
    frame.render_stateful_widget(table, area, &mut state);

    // Render scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None);
    let mut scrollbar_state =
        ScrollbarState::new(internal_txns.len().saturating_sub(1)).position(selected_index);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
}
