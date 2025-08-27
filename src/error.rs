//! Error handling for WarpScan
//!
//! This module defines the main error types used throughout the application
//! and provides convenient Result type aliases.

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
}

/// Convenient Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Helper functions for creating specific error types
impl Error {
    /// Create a network error
    pub fn network<S: Into<String>>(msg: S) -> Self {
        Error::Network(msg.into())
    }

    /// Create a cache error
    pub fn cache<S: Into<String>>(msg: S) -> Self {
        Error::Cache(msg.into())
    }

    /// Create a wallet error
    pub fn wallet<S: Into<String>>(msg: S) -> Self {
        Error::Wallet(msg.into())
    }

    /// Create a contract error
    pub fn contract<S: Into<String>>(msg: S) -> Self {
        Error::Contract(msg.into())
    }

    /// Create a UI error
    pub fn ui<S: Into<String>>(msg: S) -> Self {
        Error::Ui(msg.into())
    }

    /// Create a blockchain error
    pub fn blockchain<S: Into<String>>(msg: S) -> Self {
        Error::Blockchain(msg.into())
    }

    /// Create a parse error
    pub fn parse<S: Into<String>>(msg: S) -> Self {
        Error::Parse(msg.into())
    }

    /// Create an application error
    pub fn app<S: Into<String>>(msg: S) -> Self {
        Error::App(msg.into())
    }

    /// Create a validation error
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        Error::Validation(msg.into())
    }
}