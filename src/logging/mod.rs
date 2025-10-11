//! Logging configuration for WarpScan
//!
//! This module sets up the tracing framework for structured logging throughout the application.

pub mod setup;
pub mod utils;
pub mod perf;
pub mod macros;

// Re-export commonly used functions and types
pub use setup::{init_logging, init_minimal_logging};
pub use utils::{log_startup_info, log_shutdown_info, log_config_info, log_error_with_context};
pub use perf::PerfTimer;