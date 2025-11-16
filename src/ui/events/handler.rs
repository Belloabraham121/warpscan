//! Event handler for managing terminal and application events

use super::types::{CustomEvent, Event};
use crate::error::{Error, Result};
use crossterm::event::{self, Event as CrosstermEvent};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

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
                                if sender.send(Event::Key(key)).is_err() {
                                    break;
                                }
                            }
                            Ok(CrosstermEvent::Mouse(mouse)) => {
                                if sender.send(Event::Mouse(mouse)).is_err() {
                                    break;
                                }
                            }
                            Ok(CrosstermEvent::Resize(w, h)) => {
                                if sender.send(Event::Resize(w, h)).is_err() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if sender.send(Event::Tick).is_err() {
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

    /// Get the next event
    pub async fn next(&mut self) -> Result<Event> {
        self.receiver.recv().await.ok_or(Error::EventChannelClosed)
    }

    /// Send a custom event
    pub fn send_custom_event(&self, event: CustomEvent) -> Result<()> {
        self.sender
            .send(Event::Custom(event))
            .map_err(|_| Error::EventChannelClosed)
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.handler.abort();
    }
}
