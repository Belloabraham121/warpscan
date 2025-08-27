//! WarpScan - Terminal-based Ethereum blockchain explorer
//!
//! A comprehensive terminal user interface for exploring Ethereum blockchain data,
//! similar to Etherscan but running in your terminal.

pub mod blockchain;
pub mod cache;
pub mod config;
pub mod error;
pub mod logging;
pub mod ui;
pub mod wallet;

// Re-export commonly used types
pub use error::{Error, Result};
pub use config::Config;