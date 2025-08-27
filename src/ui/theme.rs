//! Theme and styling for the terminal user interface

use ratatui::style::{Color, Modifier, Style};

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

    /// Get style for normal text
    pub fn normal(&self) -> Style {
        Style::default().fg(self.foreground).bg(self.background)
    }

    /// Get style for primary text
    pub fn primary(&self) -> Style {
        Style::default().fg(self.primary)
    }

    /// Get style for secondary text
    pub fn secondary(&self) -> Style {
        Style::default().fg(self.secondary)
    }

    /// Get style for accent text
    pub fn accent(&self) -> Style {
        Style::default().fg(self.accent)
    }

    /// Get style for success text
    pub fn success(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Get style for warning text
    pub fn warning(&self) -> Style {
        Style::default().fg(self.warning)
    }

    /// Get style for error text
    pub fn error(&self) -> Style {
        Style::default().fg(self.error)
    }

    /// Get style for info text
    pub fn info(&self) -> Style {
        Style::default().fg(self.info)
    }

    /// Get style for muted text
    pub fn muted(&self) -> Style {
        Style::default().fg(self.muted)
    }

    /// Get style for borders
    pub fn border(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Get style for selected items
    pub fn selected(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.selected)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for highlighted items
    pub fn highlighted(&self) -> Style {
        Style::default()
            .fg(self.selected)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for titles
    pub fn title(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for headers
    pub fn header(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }

    /// Get style for labels
    pub fn label(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for values
    pub fn value(&self) -> Style {
        Style::default().fg(self.foreground)
    }

    /// Get style for input fields
    pub fn input(&self) -> Style {
        Style::default()
            .fg(self.foreground)
            .bg(self.background)
    }

    /// Get style for active input fields
    pub fn input_active(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.primary)
    }

    /// Get style for buttons
    pub fn button(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for active buttons
    pub fn button_active(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for tabs
    pub fn tab(&self) -> Style {
        Style::default().fg(self.muted)
    }

    /// Get style for active tabs
    pub fn tab_active(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }

    /// Get style for status bar
    pub fn status_bar(&self) -> Style {
        Style::default()
            .fg(self.foreground)
            .bg(self.muted)
    }

    /// Get style for help text
    pub fn help(&self) -> Style {
        Style::default().fg(self.muted)
    }

    /// Get style for code/monospace text
    pub fn code(&self) -> Style {
        Style::default()
            .fg(self.info)
            .add_modifier(Modifier::ITALIC)
    }

    /// Get style for addresses
    pub fn address(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for transaction hashes
    pub fn transaction_hash(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for block numbers
    pub fn block_number(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for amounts/values
    pub fn amount(&self) -> Style {
        Style::default()
            .fg(self.success)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for gas values
    pub fn gas(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for timestamps
    pub fn timestamp(&self) -> Style {
        Style::default().fg(self.muted)
    }

    /// Get style for loading indicators
    pub fn loading(&self) -> Style {
        Style::default()
            .fg(self.info)
            .add_modifier(Modifier::SLOW_BLINK)
    }

    /// Get style for progress bars
    pub fn progress(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.primary)
    }

    /// Get style for progress bar background
    pub fn progress_bg(&self) -> Style {
        Style::default()
            .fg(self.muted)
            .bg(self.background)
    }
}

/// Theme manager for handling theme switching
pub struct ThemeManager {
    current_theme: Theme,
    available_themes: Vec<(&'static str, Theme)>,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new() -> Self {
        let available_themes = vec![
            ("Dark", Theme::dark()),
            ("Light", Theme::light()),
            ("Ethereum", Theme::ethereum()),
        ];

        Self {
            current_theme: Theme::default(),
            available_themes,
        }
    }

    /// Get the current theme
    pub fn current(&self) -> &Theme {
        &self.current_theme
    }

    /// Set the current theme by name
    pub fn set_theme(&mut self, name: &str) -> bool {
        if let Some((_, theme)) = self.available_themes.iter().find(|(n, _)| *n == name) {
            self.current_theme = theme.clone();
            true
        } else {
            false
        }
    }

    /// Get available theme names
    pub fn available_themes(&self) -> Vec<&'static str> {
        self.available_themes.iter().map(|(name, _)| *name).collect()
    }

    /// Cycle to the next theme
    pub fn next_theme(&mut self) {
        let current_index = self
            .available_themes
            .iter()
            .position(|(_, theme)| {
                // Simple comparison by checking primary color
                theme.primary == self.current_theme.primary
            })
            .unwrap_or(0);

        let next_index = (current_index + 1) % self.available_themes.len();
        self.current_theme = self.available_themes[next_index].1.clone();
    }
}