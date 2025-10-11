//! WarpScan - Terminal Etherscan
//!
//! A comprehensive terminal-based Ethereum blockchain explorer

use std::io;
use std::sync::Arc;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind, MouseEvent, MouseEventKind, MouseButton},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use tokio::time::sleep;
use tracing::{error, info, warn};

use warpscan::{
    blockchain::BlockchainService,
    cache::CacheManager,
    config::Config,
    error::Result,
    logging::{init_logging, init_minimal_logging, log_config_info, log_shutdown_info, log_startup_info},
    ui::{
        app::{App, AppState, InputMode},
        events::{EventHandler, Event as AppEvent},
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
        Err(_) => (Config::default(), false)
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
    let blockchain_client = BlockchainService::new(config.clone(), Arc::new(cache_manager.clone())).await?;
    let _wallet_manager = WalletManager::new();
    let theme_manager = ThemeManager::new();

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize application
    let mut app = App::new(
        config.clone(),
        blockchain_client,
        cache_manager,
    );

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
        // Render UI
        let theme = theme_manager.current();
        terminal.draw(|frame| {
            match app.state {
                AppState::Home => screens::render_home(frame, app, theme),
                AppState::BlockExplorer => screens::render_block_explorer(frame, app, theme),
                AppState::TransactionViewer => screens::render_transaction_viewer(frame, app, theme),
                AppState::AddressLookup => screens::render_address_lookup(frame, app, theme),
                AppState::GasTracker => screens::render_gas_tracker(frame, app, theme),
                AppState::WalletManager => screens::render_wallet_manager(frame, app, theme),
                _ => {
                    // For unimplemented screens, show a placeholder
                    let placeholder = ratatui::widgets::Paragraph::new("Screen not yet implemented")
                        .block(
                            ratatui::widgets::Block::default()
                                .title("WarpScan")
                                .borders(ratatui::widgets::Borders::ALL),
                        );
                    frame.render_widget(placeholder, frame.area());
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
                AppEvent::Mouse(mouse_event) => {
                    match handle_mouse_event(app, mouse_event).await {
                        Ok(should_quit) => {
                            if should_quit {
                                break;
                            }
                        }
                        Err(e) => {
                            app.set_error(format!("Error handling mouse event: {}", e));
                        }
                    }
                }
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
            // Escape key - go back to previous screen or home
            if app.state == AppState::Home {
                // If already at home, quit the application
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
                        let max_index = app.dashboard_data.latest_transactions.len().saturating_sub(1);
                        if app.current_list_index < max_index {
                            app.current_list_index += 1;
                        }
                    }
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
                 _ => app.next_tab(),
             }
         }
         KeyCode::Left => {
             match app.state {
                 AppState::Home => {
                     // Switch between search bar, blocks list, and transactions list
                     app.current_tab = if app.current_tab == 0 { 2 } else { app.current_tab - 1 };
                     app.current_list_index = 0; // Reset selection when switching tabs
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
                AppState::BlockExplorer | AppState::TransactionViewer | AppState::AddressLookup => {
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
        KeyCode::Char('0') => app.navigate_to(AppState::Home),
        _ => {}
    }
    Ok(false)
}

async fn handle_editing_mode_keys(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Enter => {
            // Process input and exit editing mode
            let input_text = app.process_input();
            if !input_text.is_empty() {
                // Process the search input
                process_search_input(app, &input_text).await?;
            }
            app.input_mode = InputMode::Normal;
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

async fn process_search_input(app: &mut App, input: &str) -> Result<()> {
    let trimmed_input = input.trim();
    
    // Determine what type of search this is based on the input format
    if trimmed_input.len() == 66 && trimmed_input.starts_with("0x") {
        // Transaction hash (64 hex chars + 0x prefix)
        app.set_input(trimmed_input.to_string());
        app.navigate_to(AppState::TransactionViewer);
    } else if trimmed_input.len() == 42 && trimmed_input.starts_with("0x") {
        // Ethereum address (40 hex chars + 0x prefix)
        app.set_input(trimmed_input.to_string());
        app.navigate_to(AppState::AddressLookup);
    } else if trimmed_input.parse::<u64>().is_ok() {
        // Block number
        app.set_input(trimmed_input.to_string());
        app.navigate_to(AppState::BlockExplorer);
    } else if trimmed_input.starts_with("0x") && trimmed_input.len() <= 66 {
        // Could be a block hash or partial hash
        app.set_input(trimmed_input.to_string());
        app.navigate_to(AppState::BlockExplorer);
    } else {
        // Unknown format, show error
        app.set_error(format!("Unknown search format: {}", trimmed_input));
    }
    
    Ok(())
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
                AppState::BlockExplorer | AppState::TransactionViewer | AppState::AddressLookup => {
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
    if y >= 5 && y <= 18 { // Assuming menu items are in this range
        let item_index = (y - 5) as usize;
        if item_index < 12 { // We have 12 menu items
            app.current_list_index = item_index;
            // Simulate Enter key press to navigate
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
                11 => app.navigate_to(AppState::Help),
                _ => {}
            }
        }
    }
}

fn handle_input_screen_click(app: &mut App, _x: u16, y: u16) {
    // Handle clicks on input screens
    // If click is in input area, enter editing mode
    if y >= 3 && y <= 5 { // Assuming input field is in this range
        app.input_mode = InputMode::Editing;
    }
}
