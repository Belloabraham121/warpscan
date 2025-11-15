//! Logging configuration for WarpScan
//!
//! This module sets up the tracing framework for structured logging throughout the application.

pub mod macros;
pub mod perf;
pub mod setup;
pub mod utils;

// Re-export commonly used functions and types
pub use perf::PerfTimer;
pub use setup::{init_logging, init_minimal_logging};
pub use utils::{log_config_info, log_error_with_context, log_shutdown_info, log_startup_info};
