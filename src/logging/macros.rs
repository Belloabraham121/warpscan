//! Logging macros
//!
//! This module provides convenient macros for logging and performance timing.

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