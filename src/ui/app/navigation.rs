use super::core::App;
use super::state::{AppState, InputMode};

impl App {
    /// Check if a screen is implemented and can be navigated to
    fn is_screen_implemented(state: &AppState) -> bool {
        matches!(
            state,
            AppState::Home
                | AppState::BlockExplorer
                | AppState::TransactionViewer
                | AppState::AddressLookup
                | AppState::GasTracker
                | AppState::WalletManager
                | AppState::Settings
                | AppState::Quit
        )
    }

    /// Navigate to a new state
    /// Only allows navigation to implemented screens
    pub fn navigate_to(&mut self, new_state: AppState) {
        // Prevent navigation to unimplemented screens
        if !Self::is_screen_implemented(&new_state) {
            // Set an error message instead of navigating
            self.set_error(format!(
                "Screen '{}' is not yet implemented",
                new_state.title()
            ));
            return;
        }

        if self.state != new_state {
            self.navigation_history.push(self.state.clone());
            self.previous_state = Some(self.state.clone());
            self.state = new_state;
            self.reset_navigation_state();
        }
    }

    /// Go back to the previous state
    pub fn go_back(&mut self) {
        if let Some(previous) = self.navigation_history.pop() {
            self.previous_state = Some(self.state.clone());
            self.state = previous;
            self.reset_navigation_state();
        }
    }

    /// Reset navigation-related state
    fn reset_navigation_state(&mut self) {
        self.current_tab = 0;
        self.current_list_index = 0;
        self.scroll_offset = 0;
        self.input.clear();
        self.cursor_position = 0;
        self.input_mode = InputMode::Normal;
        self.clear_messages();
    }

    /// Clear all messages
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }
}