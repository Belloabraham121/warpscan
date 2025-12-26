//! WarpScan - Terminal Etherscan
//!
//! A comprehensive terminal-based Ethereum blockchain explorer

use std::io;
use std::sync::Arc;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
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
        app::{
            events::handle_key_event, mouse::handle_mouse_event, App, AppState, ModeSelectionState,
        },
        events::{Event as AppEvent, EventHandler},
        screens,
        theme::ThemeManager,
    },
    wallet::WalletManager,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration first with auto-detection
    let (config, config_loaded) = match Config::load_with_auto_detect().await {
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

    // Don't refresh dashboard until mode is selected
    // refresh_dashboard() will be called after mode selection

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
            // Show mode selection first if not yet selected
            if app.mode_selection_state == ModeSelectionState::Selecting {
                screens::render_mode_selection(frame, app, theme);
                return;
            }

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
                    // Ignore Ctrl+C (don't navigate to Settings)
                    if key_event.code == KeyCode::Char('c')
                        && key_event
                            .modifiers
                            .contains(crossterm::event::KeyModifiers::CONTROL)
                    {
                        // Ctrl+C - do nothing or quit (user's choice)
                        continue;
                    }
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
