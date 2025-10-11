//! Event handling modules for the terminal user interface

pub mod types;
pub mod handler;
pub mod utils;

// Re-export commonly used types and structs
pub use types::{Event, CustomEvent};
pub use handler::EventHandler;
pub use utils::KeyEventUtils;