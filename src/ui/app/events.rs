//! Key event handling for the application

use super::super::models::AddressTab;
use super::core::App;
use super::state::{AppState, DataMode, InputMode, ModeSelectionState};
use crate::error::Result;
use crossterm::event::KeyCode;

/// Handle key events based on input mode
pub async fn handle_key_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode_keys(app, key_code).await,
        InputMode::Editing => handle_editing_mode_keys(app, key_code).await,
    }
}

/// Handle key events in normal (non-editing) mode
async fn handle_normal_mode_keys(app: &mut App, key_code: KeyCode) -> Result<bool> {
    // Handle mode selection first
    if app.mode_selection_state == ModeSelectionState::Selecting {
        match key_code {
            KeyCode::Left | KeyCode::Char('1') => {
                app.current_tab = 0; // Select Local Node
            }
            KeyCode::Right | KeyCode::Char('2') => {
                app.current_tab = 1; // Select Etherscan
            }
            KeyCode::Enter => {
                // Confirm selection
                let selected_mode = if app.current_tab == 0 {
                    tracing::info!(target: "warpscan", "User selected Local Node mode");
                    DataMode::LocalNode
                } else {
                    tracing::info!(target: "warpscan", "User selected Etherscan mode");
                    DataMode::Etherscan
                };

                app.data_mode = Some(selected_mode.clone());

                // If Local Node selected, switch provider to local RPC directly
                if matches!(selected_mode, DataMode::LocalNode) {
                    if let Err(e) = app.blockchain_client.switch_to_local_node().await {
                        tracing::error!(
                            target: "warpscan",
                            "Failed to switch to local node: {}. Continuing anyway...",
                            e
                        );
                        // Continue anyway - user can still try
                    } else {
                        // Update App's config to match (for UI display)
                        app.config.network.rpc_url = "http://127.0.0.1:8545".to_string();
                        app.config.network.node_type = Some("anvil".to_string());
                        app.config.network.chain_id = 31337;
                        app.config.network.name = "Anvil Local".to_string();
                    }
                }

                app.mode_selection_state = ModeSelectionState::Selected;
                app.state = AppState::Home;
                app.current_tab = 0; // Reset tab for home screen

                // Set loading state immediately
                app.set_loading("dashboard_refresh", true);

                // Spawn dashboard refresh in background using a channel pattern
                // We'll use the event system to trigger it asynchronously
                let event_sender = app.event_sender.clone();
                if let Some(sender) = event_sender {
                    // Send event to trigger refresh in next tick
                    use crate::ui::events::{CustomEvent, Event};
                    let _ = sender.send(Event::Custom(CustomEvent::DataLoaded {
                        operation: "dashboard_refresh_requested".to_string(),
                        data: serde_json::json!({}),
                    }));
                } else {
                    // Fallback: trigger refresh directly but it will block
                    // The yields inside will help somewhat
                    app.pending_dashboard_refresh = true;
                }

                // Start subscriptions for homepage (non-blocking, quick operation)
                if let Err(e) = app.start_subscriptions().await {
                    tracing::warn!(target: "warpscan", "Failed to start subscriptions: {}", e);
                }
            }
            _ => {}
        }
        return Ok(false);
    }

    match key_code {
        KeyCode::Char('q') => return Ok(true), // Quit
        KeyCode::Esc => {
            // Escape key: go back to previous screen, or go to Home if already on Home
            if app.state == AppState::Home {
                // If on Home, Escape quits the app
                return Ok(true);
            } else {
                app.go_back().await;
            }
        }
        KeyCode::Char('h') => app.go_back().await,
        KeyCode::Up => {
            match app.state {
                AppState::Home => {
                    if app.current_tab == 0 {
                        // Navigate within blocks list
                        if app.current_list_index > 0 {
                            app.current_list_index -= 1;
                        }
                    } else if app.current_tab == 1 {
                        // Navigate within transactions list
                        if app.current_list_index > 0 {
                            app.current_list_index -= 1;
                        }
                    }
                }
                AppState::AddressLookup => {
                    // Navigate within address data tables
                    app.address_select_previous_item();
                }
                _ => app.previous_item(),
            }
        }
        KeyCode::Down => {
            match app.state {
                AppState::Home => {
                    if app.current_tab == 0 {
                        // Navigate within blocks list
                        let max_index = app.dashboard_data.latest_blocks.len().saturating_sub(1);
                        if app.current_list_index < max_index {
                            app.current_list_index += 1;
                        }
                    } else if app.current_tab == 1 {
                        // Navigate within transactions list
                        let max_index = app
                            .dashboard_data
                            .latest_transactions
                            .len()
                            .saturating_sub(1);
                        if app.current_list_index < max_index {
                            app.current_list_index += 1;
                        }
                    }
                }
                AppState::AddressLookup => {
                    // Navigate within address data tables
                    app.address_select_next_item();
                }
                _ => app.next_item(),
            }
        }
        KeyCode::Right | KeyCode::Tab => {
            match app.state {
                AppState::Home => {
                    // Switch between search bar, blocks list, and transactions list
                    app.current_tab = (app.current_tab + 1) % 3;
                    app.current_list_index = 0; // Reset selection when switching tabs
                }
                AppState::AddressLookup => {
                    // Switch to next address tab
                    if let Some(current) = app.get_current_address_tab() {
                        let next = match current {
                            AddressTab::Details => AddressTab::Transactions,
                            AddressTab::Transactions => AddressTab::AccountHistory,
                            AddressTab::AccountHistory => AddressTab::TokenTransfers,
                            AddressTab::TokenTransfers => AddressTab::Tokens,
                            AddressTab::Tokens => AddressTab::InternalTxns,
                            AddressTab::InternalTxns => AddressTab::Details,
                        };
                        app.switch_address_tab(next);
                    }
                }
                _ => app.next_tab(),
            }
        }
        KeyCode::Left => {
            match app.state {
                AppState::Home => {
                    // Switch between search bar, blocks list, and transactions list
                    app.current_tab = if app.current_tab == 0 {
                        2
                    } else {
                        app.current_tab - 1
                    };
                    app.current_list_index = 0; // Reset selection when switching tabs
                }
                AppState::AddressLookup => {
                    // Switch to previous address tab
                    if let Some(current) = app.get_current_address_tab() {
                        let prev = match current {
                            AddressTab::Details => AddressTab::InternalTxns,
                            AddressTab::Transactions => AddressTab::Details,
                            AddressTab::AccountHistory => AddressTab::Transactions,
                            AddressTab::TokenTransfers => AddressTab::AccountHistory,
                            AddressTab::Tokens => AddressTab::TokenTransfers,
                            AddressTab::InternalTxns => AddressTab::Tokens,
                        };
                        app.switch_address_tab(prev);
                    }
                }
                _ => app.go_back().await,
            }
        }
        KeyCode::Enter => {
            match app.state {
                AppState::Home => {
                    // For the new dashboard, Enter activates search or navigates to detailed view
                    if app.current_tab == 0 {
                        // In blocks section - navigate to block explorer with selected block
                        app.navigate_to(AppState::BlockExplorer).await;
                    } else if app.current_tab == 1 {
                        // In transactions section - navigate to transaction viewer with selected tx
                        app.navigate_to(AppState::TransactionViewer).await;
                    } else {
                        // In search bar - enter editing mode
                        app.input_mode = InputMode::Editing;
                    }
                }
                AppState::AddressLookup => {
                    // If already in editing mode, don't do anything (let editing mode handler take over)
                    if app.input_mode == InputMode::Editing {
                        return Ok(false);
                    }

                    // On address lookup, Enter on selected row navigates based on tab
                    let navigation_data = app.address_data.as_ref().and_then(|address_data| {
                        match address_data.current_tab {
                            AddressTab::Transactions => address_data
                                .transactions
                                .get(address_data.selected_transaction_index)
                                .map(|tx| ("tx", tx.tx_hash.clone())),
                            AddressTab::AccountHistory => address_data
                                .account_history
                                .get(address_data.selected_history_index)
                                .map(|entry| ("tx", entry.tx_hash.clone())),
                            AddressTab::TokenTransfers => address_data
                                .token_transfers
                                .get(address_data.selected_token_transfer_index)
                                .map(|transfer| ("tx", transfer.txn_hash.clone())),
                            _ => None,
                        }
                    });

                    if let Some((item_type, tx_hash)) = navigation_data {
                        if item_type == "tx" {
                            // Navigate to transaction viewer with this transaction hash
                            app.navigate_to_transaction(&tx_hash).await;
                            return Ok(false);
                        }
                    }
                    // Otherwise, enter editing mode for address input
                    app.input_mode = InputMode::Editing;
                }
                AppState::BlockExplorer | AppState::TransactionViewer => {
                    // Enter editing mode for input
                    app.input_mode = InputMode::Editing;
                }
                _ => {}
            }
        }
        KeyCode::Char('r') => {
            // Refresh current screen - placeholder for future implementation
        }
        KeyCode::Char('/') | KeyCode::Char('s') => {
            // Quick access to search - enter editing mode
            match app.state {
                AppState::Home => {
                    app.current_tab = 2; // Focus on search bar
                    app.input_mode = InputMode::Editing;
                }
                AppState::AddressLookup | AppState::TransactionViewer | AppState::BlockExplorer => {
                    // Enter editing mode for input fields on these screens
                    app.input_mode = InputMode::Editing;
                }
                _ => {}
            }
        }
        KeyCode::Char('b') => {
            // Quick access to blocks
            if app.state == AppState::Home {
                app.current_tab = 0;
            } else {
                app.navigate_to(AppState::BlockExplorer).await;
            }
        }
        KeyCode::Char('t') => {
            // Quick access to transactions
            if app.state == AppState::Home {
                app.current_tab = 1;
            } else {
                app.navigate_to(AppState::TransactionViewer).await;
            }
        }
        KeyCode::Char('a') => app.navigate_to(AppState::AddressLookup).await,
        KeyCode::Char('g') => app.navigate_to(AppState::GasTracker).await,
        KeyCode::Char('w') => app.navigate_to(AppState::WalletManager).await,
        KeyCode::Char('c') => app.navigate_to(AppState::Settings).await,
        KeyCode::Char('0') => app.navigate_to(AppState::Home).await,
        KeyCode::Char('i') => {
            // Toggle input data expansion in transaction viewer
            if app.state == AppState::TransactionViewer {
                app.input_data_expanded = !app.input_data_expanded;
            }
        }
        _ => {}
    }
    Ok(false)
}

/// Handle key events in editing mode
async fn handle_editing_mode_keys(app: &mut App, key_code: KeyCode) -> Result<bool> {
    use super::validation::{is_address, is_block_number, is_transaction_hash};

    // Handle Tab key to exit editing mode and switch tabs on AddressLookup screen
    if app.state == AppState::AddressLookup && key_code == KeyCode::Tab {
        // Exit editing mode and switch to next tab
        app.input_mode = InputMode::Normal;
        // Switch to next address tab
        if let Some(current) = app.get_current_address_tab() {
            let next = match current {
                AddressTab::Details => AddressTab::Transactions,
                AddressTab::Transactions => AddressTab::AccountHistory,
                AddressTab::AccountHistory => AddressTab::TokenTransfers,
                AddressTab::TokenTransfers => AddressTab::Tokens,
                AddressTab::Tokens => AddressTab::InternalTxns,
                AddressTab::InternalTxns => AddressTab::Details,
            };
            app.switch_address_tab(next);
        }
        return Ok(false);
    }

    match key_code {
        KeyCode::Enter => {
            // Process input based on current screen
            let input = app.get_input().trim().to_string();
            app.input_mode = InputMode::Normal;

            if input.is_empty() {
                return Ok(false);
            }

            match app.state {
                AppState::AddressLookup => {
                    // On address lookup screen, search for the address
                    if is_address(&input) {
                        // Call lookup_address - it now yields periodically to keep UI responsive
                        if let Err(e) = app.lookup_address(&input).await {
                            app.set_error(format!("Failed to lookup address: {}", e));
                        }
                    } else {
                        app.set_error("Invalid address format. Address must start with 0x and be 42 characters long.".to_string());
                    }
                }
                AppState::Home => {
                    // On home screen search bar, detect what type of input it is
                    if is_address(&input) {
                        // Navigate to address lookup and search
                        app.navigate_to(AppState::AddressLookup).await;
                        app.set_input(input.clone());
                        if let Err(e) = app.lookup_address(&input).await {
                            app.set_error(format!("Failed to lookup address: {}", e));
                        }
                    } else if is_transaction_hash(&input) {
                        // Navigate to transaction viewer
                        app.navigate_to_transaction(&input).await;
                    } else if is_block_number(&input) {
                        // Navigate to block explorer
                        app.navigate_to(AppState::BlockExplorer).await;
                        app.set_input(input);
                        // TODO: Implement block lookup
                        app.set_error("Block lookup not yet implemented".to_string());
                    } else {
                        app.set_error("Invalid input. Please enter an address (0x...), transaction hash, or block number.".to_string());
                    }
                }
                AppState::BlockExplorer => {
                    // On block explorer, search for block
                    if is_block_number(&input) {
                        // TODO: Implement block lookup
                        app.set_error("Block lookup not yet implemented".to_string());
                    } else {
                        app.set_error("Invalid block number format".to_string());
                    }
                }
                AppState::TransactionViewer => {
                    // On transaction viewer, search for transaction
                    if is_transaction_hash(&input) {
                        app.navigate_to_transaction(&input).await;
                    } else {
                        app.set_error("Invalid transaction hash format. Hash must start with 0x and be 66 characters long.".to_string());
                    }
                }
                _ => {
                    // For other screens, just clear the input
                    app.clear_input();
                }
            }
        }
        KeyCode::Esc => {
            // Cancel editing and exit editing mode
            app.clear_input();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            app.add_char(c);
        }
        KeyCode::Backspace => {
            app.remove_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        _ => {}
    }
    Ok(false)
}
