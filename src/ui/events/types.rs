//! Event types for the terminal user interface

use crossterm::event::{KeyEvent, MouseEvent};

/// Application events
#[derive(Debug, Clone)]
pub enum Event {
    /// Terminal event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Resize event
    Resize(u16, u16),
    /// Tick event for periodic updates
    Tick,
    /// Custom application events
    Custom(CustomEvent),
}

/// Custom application events
#[derive(Debug, Clone)]
pub enum CustomEvent {
    /// Data loaded successfully
    DataLoaded {
        operation: String,
        data: serde_json::Value,
    },
    /// Error occurred
    Error {
        operation: String,
        message: String,
    },
    /// Network status changed
    NetworkStatusChanged {
        connected: bool,
    },
    /// Cache updated
    CacheUpdated {
        key: String,
    },
    /// Wallet operation completed
    WalletOperationCompleted {
        operation: String,
        success: bool,
        message: String,
    },
    /// Contract interaction result
    ContractInteractionResult {
        success: bool,
        transaction_hash: Option<String>,
        error: Option<String>,
    },
    /// Real-time data update
    RealTimeUpdate {
        data_type: String,
        data: serde_json::Value,
    },
}