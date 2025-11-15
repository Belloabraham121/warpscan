//! Error helper functions
//!
//! This module provides convenient functions for creating specific error types.

use super::types::Error;

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

    /// Create a serialization/deserialization error
    pub fn serialization(err: serde_json::Error) -> Self {
        Error::Serialization(err)
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
