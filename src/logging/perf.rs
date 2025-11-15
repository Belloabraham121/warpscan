//! Performance timing utilities
//!
//! This module provides utilities for measuring and logging performance metrics.

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
