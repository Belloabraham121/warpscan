//! WarpScan - Terminal-based Ethereum blockchain explorer
//!
//! A comprehensive terminal user interface for exploring Ethereum blockchain data,
//! similar to Etherscan but running in your terminal.


pub mod error;
pub mod config;
pub mod blockchain;
pub mod cache;
pub mod logging;
pub mod ui;
pub mod wallet;

// Re-export commonly used types
pub use crate::error::{Error, Result};
pub use crate::config::Config;