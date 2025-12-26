//! Mouse event handling for the application

use super::super::models::AddressTab;
use super::core::App;
use super::state::AppState;
use crate::error::Result;
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

/// Handle mouse events
pub async fn handle_mouse_event(app: &mut App, mouse_event: MouseEvent) -> Result<bool> {
    match mouse_event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let x = mouse_event.column;
            let y = mouse_event.row;

            // Handle mouse clicks based on current screen and position
            match app.state {
                AppState::Home => {
                    // Handle clicks on home screen menu items
                    handle_home_click(app, x, y);
                }
                AppState::AddressLookup => {
                    // Handle clicks on address lookup screen (tabs, transactions, addresses)
                    if let Err(e) = handle_address_lookup_click(app, x, y).await {
                        app.set_error(format!("Error handling click: {}", e));
                    }
                }
                AppState::BlockExplorer | AppState::TransactionViewer => {
                    // Handle clicks on input fields or buttons
                    handle_input_screen_click(app, x, y);
                }
                _ => {
                    // Handle clicks on other screens
                }
            }
        }
        MouseEventKind::ScrollUp => {
            // Handle scroll up - move selection up
            app.previous_item();
        }
        MouseEventKind::ScrollDown => {
            // Handle scroll down - move selection down
            app.next_item();
        }
        _ => {
            // Handle other mouse events if needed
        }
    }
    Ok(false)
}

/// Handle clicks on the home screen
fn handle_home_click(app: &mut App, _x: u16, y: u16) {
    // Calculate which menu item was clicked based on position
    // This is a simplified implementation - in a real app you'd calculate based on actual layout
    if (5..=18).contains(&y) {
        // Assuming menu items are in this range
        let item_index = y.saturating_sub(5) as usize;
        match item_index {
            0 => app.navigate_to(AppState::BlockExplorer),
            1 => app.navigate_to(AppState::TransactionViewer),
            2 => app.navigate_to(AppState::AddressLookup),
            3 => app.navigate_to(AppState::ContractSearch),
            4 => app.navigate_to(AppState::TokenInfo),
            5 => app.navigate_to(AppState::GasTracker),
            6 => app.navigate_to(AppState::ContractInteraction),
            7 => app.navigate_to(AppState::ContractVerification),
            8 => app.navigate_to(AppState::WalletManager),
            9 => app.navigate_to(AppState::MultisigWallet),
            10 => app.navigate_to(AppState::EventMonitor),
            11 => app.navigate_to(AppState::Settings),
            12 => app.navigate_to(AppState::Help),
            _ => {}
        }
    }
}

/// Handle clicks on input screens
fn handle_input_screen_click(app: &mut App, _x: u16, y: u16) {
    // Handle clicks on input screens
    // If click is in input area, enter editing mode
    if (3..=5).contains(&y) {
        // Assuming input field is in this range
        app.input_mode = super::state::InputMode::Editing;
    }
}

/// Handle mouse clicks on the address lookup screen
async fn handle_address_lookup_click(app: &mut App, x: u16, y: u16) -> Result<()> {
    // Check if click is on tabs (approximately y=6-8, depending on layout)
    // Tabs are at content_chunks[1] which starts after title (3) + input (3) = 6
    if (6..=8).contains(&y) {
        // Calculate which tab was clicked based on x position
        // Each tab is approximately 15-20 characters wide
        let tab_width = 18;
        let tab_index = (x as usize / tab_width).min(5); // Max 6 tabs (0-5)

        let tab = match tab_index {
            0 => AddressTab::Details,
            1 => AddressTab::Transactions,
            2 => AddressTab::AccountHistory,
            3 => AddressTab::TokenTransfers,
            4 => AddressTab::Tokens,
            5 => AddressTab::InternalTxns,
            _ => return Ok(()),
        };

        app.switch_address_tab(tab);
        return Ok(());
    }

    // Check if click is in the table area (y > 8)
    if y > 8 {
        // Extract data we need before making mutable calls
        let click_data = app.address_data.as_ref().and_then(|address_data| {
            // Calculate which row was clicked (approximate)
            // Table starts around y=9, header is 1 row, so data starts at y=10
            if y >= 10 {
                let row_index = y.saturating_sub(10) as usize;
                match address_data.current_tab {
                    AddressTab::Transactions => {
                        if row_index < address_data.transactions.len() {
                            address_data.transactions.get(row_index).map(|tx| {
                                (
                                    "transactions",
                                    tx.tx_hash.clone(),
                                    tx.from.clone(),
                                    tx.to.clone(),
                                    x,
                                )
                            })
                        } else {
                            None
                        }
                    }
                    AddressTab::AccountHistory => {
                        if row_index < address_data.account_history.len() {
                            address_data.account_history.get(row_index).map(|entry| {
                                (
                                    "history",
                                    entry.tx_hash.clone(),
                                    entry.from.clone(),
                                    entry.to.clone(),
                                    x,
                                )
                            })
                        } else {
                            None
                        }
                    }
                    AddressTab::TokenTransfers => {
                        if row_index < address_data.token_transfers.len() {
                            address_data.token_transfers.get(row_index).map(|transfer| {
                                (
                                    "transfers",
                                    transfer.txn_hash.clone(),
                                    transfer.from.clone(),
                                    transfer.to.clone(),
                                    x,
                                )
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        });

        if let Some((_tab_type, tx_hash, from_addr, to_addr, click_x)) = click_data {
            // Column positions vary by tab, but approximate ranges:
            // Transaction hash: x < 20-25
            // From address: x 25-50
            // To address: x 50-75

            // Check if click is on transaction hash column
            if click_x < 25 {
                app.navigate_to_transaction(&tx_hash).await;
            }
            // Check if click is on from address column
            else if (25..50).contains(&click_x) {
                app.navigate_to_address(&from_addr).await;
            }
            // Check if click is on to address column
            else if (50..75).contains(&click_x) {
                app.navigate_to_address(&to_addr).await;
            }
        }
    }

    Ok(())
}
