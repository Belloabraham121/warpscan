//! Logging configuration for WarpScan
//!
//! This module sets up the tracing framework for structured logging throughout the application.

use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};
use std::fs::OpenOptions;
use crate::config::Config;

/// Initialize the logging system
pub fn init_logging(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let log_level = match config.ui.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    // Create log directory if it doesn't exist
    let log_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(".warpscan")
        .join("logs");
    
    std::fs::create_dir_all(&log_dir)?;

    // File appender for logs
    let log_file = log_dir.join("warpscan.log");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    // Console layer for terminal output
    let console_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_span_events(FmtSpan::CLOSE)
        .with_filter(EnvFilter::from_default_env().add_directive(log_level.into()));

    // File layer for persistent logging
    let file_layer = fmt::layer()
        .with_writer(file)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::FULL)
        .with_ansi(false)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()));

    // Initialize the subscriber
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .try_init()
        .map_err(|e| format!("Failed to initialize tracing subscriber: {}", e))?;

    tracing::info!("Logging initialized with level: {}", log_level);
    tracing::debug!("Log file location: {:?}", log_dir.join("warpscan.log"));

    Ok(())
}

/// Initialize minimal logging for early startup
pub fn init_minimal_logging() {
    let console_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));

    let _ = tracing_subscriber::registry()
        .with(console_layer)
        .try_init();
}

/// Log application startup information
pub fn log_startup_info() {
    tracing::info!("WarpScan Terminal Etherscan starting up");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));
    tracing::debug!("Rust version: {}", option_env!("CARGO_PKG_RUST_VERSION").unwrap_or("unknown"));
    tracing::debug!("Target: {}", std::env::consts::ARCH);
}

/// Log application shutdown information
pub fn log_shutdown_info() {
    tracing::info!("WarpScan shutting down gracefully");
}

/// Log configuration information
pub fn log_config_info(config: &Config) {
    tracing::info!("Configuration loaded successfully");
    tracing::debug!("Network: {:?}", config.network.name);
    tracing::debug!("RPC URL: {}", config.network.rpc_url);
    tracing::debug!("Cache enabled: {}", config.cache.enabled);
    tracing::debug!("Cache TTL: {}s", config.cache.ttl_seconds);
    tracing::debug!("Theme: {}", config.ui.theme);
}

/// Log error with context
pub fn log_error_with_context(error: &dyn std::error::Error, context: &str) {
    tracing::error!("Error in {}: {}", context, error);
    
    let mut source = error.source();
    let mut level = 1;
    while let Some(err) = source {
        tracing::error!("  Caused by (level {}): {}", level, err);
        source = err.source();
        level += 1;
    }
}

/// Performance timing helper
pub struct PerfTimer {
    name: String,
    start: std::time::Instant,
}

impl PerfTimer {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        tracing::debug!("Starting timer: {}", name);
        Self {
            name,
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    pub fn log_elapsed(&self) {
        let elapsed = self.elapsed();
        tracing::debug!("Timer '{}' elapsed: {:?}", self.name, elapsed);
    }
}

impl Drop for PerfTimer {
    fn drop(&mut self) {
        self.log_elapsed();
    }
}

/// Macro for creating performance timers
#[macro_export]
macro_rules! perf_timer {
    ($name:expr) => {
        $crate::logging::PerfTimer::new($name)
    };
}

/// Macro for logging function entry/exit
#[macro_export]
macro_rules! trace_fn {
    () => {
        let _span = tracing::trace_span!(target: "warpscan::trace", "{}", function_name!()).entered();
    };
    ($($arg:tt)*) => {
        let _span = tracing::trace_span!(target: "warpscan::trace", "{}", function_name!(), $($arg)*).entered();
    };
}