//! Theme and styling modules for the terminal user interface

pub mod colors;
pub mod manager;
pub mod styles;

// Re-export commonly used types and structs
pub use colors::Theme;
pub use manager::ThemeManager;
