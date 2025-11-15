//! WarpScan - Terminal Etherscan
//!
//! A comprehensive terminal-based Ethereum blockchain explorer

use std::io;
use std::sync::Arc;
use std::time::Duration;

use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind, MouseButton, MouseEvent,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::time::sleep;
use tracing::{error, info, warn};

use warpscan::{
    blockchain::BlockchainService,
    cache::CacheManager,
    config::Config,
    error::Result,
    logging::{
        init_logging, init_minimal_logging, log_config_info, log_shutdown_info, log_startup_info,
    },
    ui::{
        app::{App, AppState, InputMode},
        events::{Event as AppEvent, EventHandler},
        screens,
        theme::ThemeManager,
    },
    wallet::WalletManager,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration first
    let (config, config_loaded) = match Config::load() {
        Ok(config) => (config, true),
        Err(_) => (Config::default(), false),
    };

    // Initialize logging once with config
    if let Err(e) = init_logging(&config) {
        // Fallback to minimal logging if full logging fails
        init_minimal_logging();
        eprintln!("Failed to initialize full logging, using minimal: {}", e);
    }

    log_startup_info();

    if config_loaded {
        info!("Configuration loaded successfully");
    } else {
        warn!("Failed to load configuration, using defaults");
    }

    log_config_info(&config);

    // Initialize components
    let cache_manager = CacheManager::new(config.clone())?;
    let blockchain_client =
        BlockchainService::new(config.clone(), Arc::new(cache_manager.clone())).await?;
    let _wallet_manager = WalletManager::new();
    let theme_manager = ThemeManager::new();

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize application
    let mut app = App::new(config.clone(), blockchain_client, cache_manager);

    // Initialize event handler
    let mut event_handler = EventHandler::new(Duration::from_millis(100));

    // Main application loop
    let result = run_app(&mut terminal, &mut app, &mut event_handler, &theme_manager).await;

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    log_shutdown_info();

    if let Err(err) = result {
        error!("Application error: {}", err);
        return Err(err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &mut EventHandler,
    theme_manager: &ThemeManager,
) -> Result<()> {
    loop {
        // Check if we should quit before rendering
        if app.state == AppState::Quit {
            break;
        }

        // Render UI
        let theme = theme_manager.current();
        terminal.draw(|frame| {
            match app.state {
                AppState::Home => screens::render_home(frame, app, theme),
                AppState::BlockExplorer => screens::render_block_explorer(frame, app, theme),
                AppState::TransactionViewer => {
                    screens::render_transaction_viewer(frame, app, theme)
                }
                AppState::AddressLookup => screens::render_address_lookup(frame, app, theme),
                AppState::GasTracker => screens::render_gas_tracker(frame, app, theme),
                AppState::WalletManager => screens::render_wallet_manager(frame, app, theme),
                AppState::Settings => screens::render_settings(frame, app, theme),
                AppState::ContractSearch => screens::render_contract_search(frame, app, theme),
                AppState::TokenInfo => screens::render_token_info(frame, app, theme),
                AppState::ContractInteraction => {
                    screens::render_contract_interaction(frame, app, theme)
                }
                AppState::ContractVerification => {
                    screens::render_contract_verification(frame, app, theme)
                }
                AppState::MultisigWallet => screens::render_multisig_wallet(frame, app, theme),
                AppState::EventMonitor => screens::render_event_monitor(frame, app, theme),
                AppState::Help => screens::render_help(frame, app, theme),
                AppState::Quit => {
                    // Should not reach here due to check above
                }
            }
        })?;

        // Handle events
        if let Ok(event) = event_handler.next().await {
            match event {
                AppEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match handle_key_event(app, key_event.code).await {
                        Ok(should_quit) => {
                            if should_quit {
                                break;
                            }
                        }
                        Err(e) => {
                            app.set_error(format!("Error handling key event: {}", e));
                        }
                    }
                }
                AppEvent::Mouse(mouse_event) => match handle_mouse_event(app, mouse_event).await {
                    Ok(should_quit) => {
                        if should_quit {
                            break;
                        }
                    }
                    Err(e) => {
                        app.set_error(format!("Error handling mouse event: {}", e));
                    }
                },
                AppEvent::Tick => {
                    // Handle periodic updates
                    // Tick placeholder - no async operations needed here
                }
                _ => {}
            }
        }

        // Small delay to prevent excessive CPU usage
        sleep(Duration::from_millis(16)).await; // ~60 FPS
    }

    Ok(())
}

async fn handle_key_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode_keys(app, key_code).await,
        InputMode::Editing => handle_editing_mode_keys(app, key_code).await,
    }
}

async fn handle_normal_mode_keys(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Char('q') => return Ok(true), // Quit
        KeyCode::Esc => {
            // Escape key: go back to previous screen, or go to Home if already on Home
            if app.state == AppState::Home {
                // If on Home, Escape quits the app
                return Ok(true);
            } else {
                app.go_back();
            }
        }
        KeyCode::Char('h') => app.go_back(),
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
                    use warpscan::ui::models::AddressTab;
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
                    use warpscan::ui::models::AddressTab;
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
                _ => app.go_back(),
            }
        }
        KeyCode::Enter => {
            match app.state {
                AppState::Home => {
                    // For the new dashboard, Enter activates search or navigates to detailed view
                    if app.current_tab == 0 {
                        // In blocks section - navigate to block explorer with selected block
                        app.navigate_to(AppState::BlockExplorer);
                    } else if app.current_tab == 1 {
                        // In transactions section - navigate to transaction viewer with selected tx
                        app.navigate_to(AppState::TransactionViewer);
                    } else {
                        // In search bar - enter editing mode
                        app.input_mode = InputMode::Editing;
                    }
                }
                AppState::AddressLookup => {
                    // On address lookup, Enter on selected row navigates based on tab
                    use warpscan::ui::models::AddressTab;
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
                            app.navigate_to(AppState::TransactionViewer);
                            app.set_input(tx_hash);
                            // TODO: Implement transaction lookup
                            app.set_error("Transaction lookup not yet implemented".to_string());
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
            if app.state == AppState::Home {
                app.current_tab = 2; // Focus on search bar
                app.input_mode = InputMode::Editing;
            }
        }
        KeyCode::Char('b') => {
            // Quick access to blocks
            if app.state == AppState::Home {
                app.current_tab = 0;
            } else {
                app.navigate_to(AppState::BlockExplorer);
            }
        }
        KeyCode::Char('t') => {
            // Quick access to transactions
            if app.state == AppState::Home {
                app.current_tab = 1;
            } else {
                app.navigate_to(AppState::TransactionViewer);
            }
        }
        KeyCode::Char('a') => app.navigate_to(AppState::AddressLookup),
        KeyCode::Char('g') => app.navigate_to(AppState::GasTracker),
        KeyCode::Char('w') => app.navigate_to(AppState::WalletManager),
        KeyCode::Char('c') => app.navigate_to(AppState::Settings),
        KeyCode::Char('0') => app.navigate_to(AppState::Home),
        _ => {}
    }
    Ok(false)
}

/// Check if input looks like an Ethereum address
fn is_address(input: &str) -> bool {
    input.starts_with("0x")
        && input.len() == 42
        && input[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if input looks like a transaction hash
fn is_transaction_hash(input: &str) -> bool {
    input.starts_with("0x")
        && input.len() == 66
        && input[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if input looks like a block number
fn is_block_number(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_digit())
}

async fn handle_editing_mode_keys(app: &mut App, key_code: KeyCode) -> Result<bool> {
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
                        app.navigate_to(AppState::AddressLookup);
                        app.set_input(input.clone());
                        if let Err(e) = app.lookup_address(&input).await {
                            app.set_error(format!("Failed to lookup address: {}", e));
                        }
                    } else if is_transaction_hash(&input) {
                        // Navigate to transaction viewer
                        app.navigate_to(AppState::TransactionViewer);
                        app.set_input(input);
                        // TODO: Implement transaction lookup
                        app.set_error("Transaction lookup not yet implemented".to_string());
                    } else if is_block_number(&input) {
                        // Navigate to block explorer
                        app.navigate_to(AppState::BlockExplorer);
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
                        // TODO: Implement transaction lookup
                        app.set_error("Transaction lookup not yet implemented".to_string());
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

async fn handle_mouse_event(app: &mut App, mouse_event: MouseEvent) -> Result<bool> {
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

fn handle_home_click(app: &mut App, _x: u16, y: u16) {
    // Calculate which menu item was clicked based on position
    // This is a simplified implementation - in a real app you'd calculate based on actual layout
    if y >= 5 && y <= 18 {
        // Assuming menu items are in this range
        let item_index = (y - 5) as usize;
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

fn handle_input_screen_click(app: &mut App, _x: u16, y: u16) {
    // Handle clicks on input screens
    // If click is in input area, enter editing mode
    if y >= 3 && y <= 5 {
        // Assuming input field is in this range
        app.input_mode = InputMode::Editing;
    }
}

/// Handle mouse clicks on the address lookup screen
async fn handle_address_lookup_click(app: &mut App, x: u16, y: u16) -> Result<()> {
    use warpscan::ui::models::AddressTab;

    // Check if click is on tabs (approximately y=6-8, depending on layout)
    // Tabs are at content_chunks[1] which starts after title (3) + input (3) = 6
    if y >= 6 && y <= 8 {
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
                let row_index = (y - 10) as usize;
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
                app.navigate_to_transaction(&tx_hash);
            }
            // Check if click is on from address column
            else if click_x >= 25 && click_x < 50 {
                app.navigate_to_address(&from_addr).await;
            }
            // Check if click is on to address column
            else if click_x >= 50 && click_x < 75 {
                app.navigate_to_address(&to_addr).await;
            }
        }
    }

    Ok(())
}
