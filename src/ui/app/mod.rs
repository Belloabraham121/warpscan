// Application state and core functionality
pub mod state;
pub mod core;
pub mod navigation;
pub mod input;
pub mod ui_state;
pub mod data;
pub mod address;

// Re-export all public types and the main App struct for convenience
pub use state::{AppState, InputMode};
pub use core::App;