//! User Interface module for WarpScan
//!
//! This module provides the terminal user interface using ratatui.
//! It includes components for different screens and navigation.

pub mod app;
pub mod components;
pub mod events;
pub mod models;
pub mod screens;
pub mod theme;

pub use app::{App, AppState};
pub use events::{Event, EventHandler};
pub use theme::Theme;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use crate::error::{Error, Result};

/// Terminal type alias
pub type Tui = Terminal<CrosstermBackend<io::Stdout>>;

/// Initialize the terminal
pub fn init() -> Result<Tui> {
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| Error::ui(format!("Failed to enter alternate screen: {}", e)))?;
    
    enable_raw_mode()
        .map_err(|e| Error::ui(format!("Failed to enable raw mode: {}", e)))?;
    
    Terminal::new(CrosstermBackend::new(io::stdout()))
        .map_err(|e| Error::ui(format!("Failed to create terminal: {}", e)))
}

/// Restore the terminal
pub fn restore() -> Result<()> {
    disable_raw_mode()
        .map_err(|e| Error::ui(format!("Failed to disable raw mode: {}", e)))?;
    
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
        .map_err(|e| Error::ui(format!("Failed to leave alternate screen: {}", e)))?;
    
    Ok(())
}