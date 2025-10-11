//! Color definitions and theme variants

use ratatui::style::Color;

/// Application theme
#[derive(Debug, Clone)]
pub struct Theme {
    /// Primary color
    pub primary: Color,
    /// Secondary color
    pub secondary: Color,
    /// Accent color
    pub accent: Color,
    /// Background color
    pub background: Color,
    /// Foreground/text color
    pub foreground: Color,
    /// Success color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error color
    pub error: Color,
    /// Info color
    pub info: Color,
    /// Muted/disabled color
    pub muted: Color,
    /// Border color
    pub border: Color,
    /// Selected/highlighted color
    pub selected: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            accent: Color::Magenta,
            background: Color::Black,
            foreground: Color::White,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            muted: Color::DarkGray,
            border: Color::Gray,
            selected: Color::LightCyan,
        }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::Cyan,
            accent: Color::Magenta,
            background: Color::White,
            foreground: Color::Black,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            muted: Color::Gray,
            border: Color::DarkGray,
            selected: Color::LightBlue,
        }
    }

    /// Create a custom theme with Ethereum-inspired colors
    pub fn ethereum() -> Self {
        Self {
            primary: Color::Rgb(98, 126, 234), // Ethereum blue
            secondary: Color::Rgb(255, 255, 255), // White
            accent: Color::Rgb(255, 193, 7), // Gold/yellow
            background: Color::Rgb(32, 33, 36), // Dark gray
            foreground: Color::Rgb(255, 255, 255), // White
            success: Color::Rgb(76, 175, 80), // Green
            warning: Color::Rgb(255, 152, 0), // Orange
            error: Color::Rgb(244, 67, 54), // Red
            info: Color::Rgb(33, 150, 243), // Light blue
            muted: Color::Rgb(158, 158, 158), // Gray
            border: Color::Rgb(66, 66, 66), // Dark gray
            selected: Color::Rgb(144, 202, 249), // Light blue
        }
    }
}