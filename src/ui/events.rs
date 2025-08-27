//! Event handling for the terminal user interface

use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use crate::error::{Error, Result};

/// Application events
#[derive(Debug, Clone)]
pub enum Event {
    /// Terminal event
    Key(KeyEvent),
    /// Mouse event
    Mouse(crossterm::event::MouseEvent),
    /// Resize event
    Resize(u16, u16),
    /// Tick event for periodic updates
    Tick,
    /// Custom application events
    Custom(CustomEvent),
}

/// Custom application events
#[derive(Debug, Clone)]
pub enum CustomEvent {
    /// Data loaded successfully
    DataLoaded {
        operation: String,
        data: serde_json::Value,
    },
    /// Error occurred
    Error {
        operation: String,
        message: String,
    },
    /// Network status changed
    NetworkStatusChanged {
        connected: bool,
    },
    /// Cache updated
    CacheUpdated {
        key: String,
    },
    /// Wallet operation completed
    WalletOperationCompleted {
        operation: String,
        success: bool,
        message: String,
    },
    /// Contract interaction result
    ContractInteractionResult {
        success: bool,
        transaction_hash: Option<String>,
        error: Option<String>,
    },
    /// Real-time data update
    RealTimeUpdate {
        data_type: String,
        data: serde_json::Value,
    },
}

/// Event handler for managing terminal and application events
pub struct EventHandler {
    /// Event receiver
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Event sender
    sender: mpsc::UnboundedSender<Event>,
    /// Handler for terminal events
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Create a new event handler
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let handler = {
            let sender = sender.clone();
            tokio::spawn(async move {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| Duration::from_secs(0));

                    if event::poll(timeout).unwrap_or(false) {
                        match event::read() {
                            Ok(CrosstermEvent::Key(key)) => {
                                if let Err(_) = sender.send(Event::Key(key)) {
                                    break;
                                }
                            }
                            Ok(CrosstermEvent::Mouse(mouse)) => {
                                if let Err(_) = sender.send(Event::Mouse(mouse)) {
                                    break;
                                }
                            }
                            Ok(CrosstermEvent::Resize(w, h)) => {
                                if let Err(_) = sender.send(Event::Resize(w, h)) {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if let Err(_) = sender.send(Event::Tick) {
                            break;
                        }
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            receiver,
            sender,
            handler,
        }
    }

    /// Get the event sender
    pub fn sender(&self) -> mpsc::UnboundedSender<Event> {
        self.sender.clone()
    }

    /// Receive the next event
    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| Error::ui("Event channel closed"))
    }

    /// Send a custom event
    pub fn send_custom_event(&self, event: CustomEvent) -> Result<()> {
        self.sender
            .send(Event::Custom(event))
            .map_err(|_| Error::ui("Failed to send custom event"))
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.handler.abort();
    }
}

/// Key event utilities
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
        key_event.code == KeyCode::Tab
    }

    /// Check if the key event is Shift+Tab
    pub fn is_shift_tab(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::BackTab
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
        matches!(
            key_event.code,
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right
        )
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

    /// Get the character from a key event if it's a printable character
    pub fn get_char(key_event: &KeyEvent) -> Option<char> {
        match key_event.code {
            KeyCode::Char(c) => Some(c),
            _ => None,
        }
    }

    /// Check if the key event represents a number
    pub fn is_number(key_event: &KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char(c) => c.is_ascii_digit(),
            _ => false,
        }
    }

    /// Get the number from a key event if it's a digit
    pub fn get_number(key_event: &KeyEvent) -> Option<u8> {
        match key_event.code {
            KeyCode::Char(c) if c.is_ascii_digit() => c.to_digit(10).map(|d| d as u8),
            _ => None,
        }
    }
}