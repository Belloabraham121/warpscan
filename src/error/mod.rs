//! Error handling for WarpScan
//!
//! This module defines the main error types used throughout the application
//! and provides convenient Result type aliases.

pub mod types;
pub mod helpers;

// Re-export commonly used types
pub use types::{Error, Result};