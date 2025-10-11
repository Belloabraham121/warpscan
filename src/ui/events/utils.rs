//! Utility functions for key event handling

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Utility functions for key events
pub struct KeyEventUtils;

impl KeyEventUtils {
    /// Check if the key event is Ctrl+C
    pub fn is_ctrl_c(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if the key event is Ctrl+D
    pub fn is_ctrl_d(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('d') && key_event.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if the key event is Ctrl+L
    pub fn is_ctrl_l(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('l') && key_event.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if the key event is Ctrl+R
    pub fn is_ctrl_r(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('r') && key_event.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if the key event is Ctrl+S
    pub fn is_ctrl_s(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('s') && key_event.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if the key event is Enter
    pub fn is_enter(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Enter
    }

    /// Check if the key event is Escape
    pub fn is_escape(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Esc
    }

    /// Check if the key event is Tab
    pub fn is_tab(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Tab && !key_event.modifiers.contains(KeyModifiers::SHIFT)
    }

    /// Check if the key event is Shift+Tab
    pub fn is_shift_tab(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::BackTab || 
        (key_event.code == KeyCode::Tab && key_event.modifiers.contains(KeyModifiers::SHIFT))
    }

    /// Check if the key event is Backspace
    pub fn is_backspace(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Backspace
    }

    /// Check if the key event is Delete
    pub fn is_delete(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Delete
    }

    /// Check if the key event is an arrow key
    pub fn is_arrow_key(key_event: &KeyEvent) -> bool {
        matches!(key_event.code, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right)
    }

    /// Check if the key event is Up arrow
    pub fn is_up(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Up
    }

    /// Check if the key event is Down arrow
    pub fn is_down(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Down
    }

    /// Check if the key event is Left arrow
    pub fn is_left(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Left
    }

    /// Check if the key event is Right arrow
    pub fn is_right(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Right
    }

    /// Check if the key event is Page Up
    pub fn is_page_up(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::PageUp
    }

    /// Check if the key event is Page Down
    pub fn is_page_down(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::PageDown
    }

    /// Check if the key event is Home
    pub fn is_home(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Home
    }

    /// Check if the key event is End
    pub fn is_end(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::End
    }

    /// Get the character from a key event
    pub fn get_char(key_event: &KeyEvent) -> Option<char> {
        match key_event.code {
            KeyCode::Char(c) => Some(c),
            _ => None,
        }
    }

    /// Check if the key event is a number
    pub fn is_number(key_event: &KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char(c) => c.is_ascii_digit(),
            _ => false,
        }
    }

    /// Get the number from a key event
    pub fn get_number(key_event: &KeyEvent) -> Option<u8> {
        match key_event.code {
            KeyCode::Char(c) if c.is_ascii_digit() => c.to_digit(10).map(|d| d as u8),
            _ => None,
        }
    }
}