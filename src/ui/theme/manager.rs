//! Theme manager for handling theme switching

use super::colors::Theme;

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