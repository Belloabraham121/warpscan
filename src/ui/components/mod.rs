//! UI Components for WarpScan
//!
//! This module contains reusable UI components for the terminal interface.

pub mod error;
pub mod help_popup;
pub mod input_field;
pub mod loading;
pub mod progress;
pub mod status_bar;
pub mod success;

// Re-export all component functions for convenience
pub use error::render_error;
pub use help_popup::render_help_popup;
pub use input_field::render_input_field;
pub use loading::render_loading;
pub use progress::render_progress;
pub use status_bar::render_status_bar;
pub use success::render_success;
