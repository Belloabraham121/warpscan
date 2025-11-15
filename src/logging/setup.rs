//! Logging setup and initialization
//!
//! This module handles the setup and initialization of the tracing framework.

use crate::config::Config;
use std::fs::OpenOptions;
use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

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
