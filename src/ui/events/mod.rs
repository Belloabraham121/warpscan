//! Event handling modules for the terminal user interface

pub mod handler;
pub mod types;
pub mod utils;

// Re-export commonly used types and structs
pub use handler::EventHandler;
pub use types::{CustomEvent, Event};
pub use utils::KeyEventUtils;
