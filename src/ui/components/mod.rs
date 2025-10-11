//! UI Components for WarpScan
//!
//! This module contains reusable UI components for the terminal interface.

pub mod loading;
pub mod error;
pub mod success;
pub mod progress;
pub mod help_popup;
pub mod input_field;
pub mod status_bar;

// Re-export all component functions for convenience
pub use loading::render_loading;
pub use error::render_error;
pub use success::render_success;
pub use progress::render_progress;
pub use help_popup::render_help_popup;
pub use input_field::render_input_field;
pub use status_bar::render_status_bar;