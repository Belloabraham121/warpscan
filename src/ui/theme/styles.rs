//! Style definitions and methods for themes

use super::colors::Theme;
use ratatui::style::{Modifier, Style};

impl Theme {
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
        Style::default().fg(self.foreground).bg(self.background)
    }

    /// Get style for active input fields
    pub fn input_active(&self) -> Style {
        Style::default().fg(self.background).bg(self.primary)
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
        Style::default().fg(self.foreground).bg(self.muted)
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
        Style::default().fg(self.background).bg(self.primary)
    }

    /// Get style for progress bar background
    pub fn progress_bg(&self) -> Style {
        Style::default().fg(self.muted).bg(self.background)
    }
}
