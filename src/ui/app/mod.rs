// Application state and core functionality
pub mod address;
pub mod core;
pub mod data;
pub mod events;
pub mod input;
pub mod mouse;
pub mod navigation;
pub mod state;
pub mod ui_state;
pub mod validation;

// Re-export all public types and the main App struct for convenience
pub use core::App;
pub use state::{AppState, DataMode, InputMode, ModeSelectionState};
