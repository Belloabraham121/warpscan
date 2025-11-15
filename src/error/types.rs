//! Error types and definitions
//!
//! This module defines the main error types used throughout the application.

use thiserror::Error;

/// Main error type for WarpScan
#[derive(Error, Debug)]
pub enum Error {
    /// Network connection errors
    #[error("Network error: {0}")]
    Network(String),

    /// Cache-related errors
    #[error("Cache error: {0}")]
    Cache(String),

    /// Wallet-related errors
    #[error("Wallet error: {0}")]
    Wallet(String),

    /// Contract interaction errors
    #[error("Contract error: {0}")]
    Contract(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// UI-related errors
    #[error("UI error: {0}")]
    Ui(String),

    /// Blockchain-related errors
    #[error("Blockchain error: {0}")]
    Blockchain(String),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Parsing errors
    #[error("Parse error: {0}")]
    Parse(String),

    /// Generic application errors
    #[error("Application error: {0}")]
    App(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Event channel closed error
    #[error("Event channel closed")]
    EventChannelClosed,
}

/// Convenient Result type alias
pub type Result<T> = std::result::Result<T, Error>;
