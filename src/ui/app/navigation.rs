use super::core::App;
use super::state::{AppState, InputMode};

impl App {
    /// Navigate to a new state
    pub fn navigate_to(&mut self, new_state: AppState) {
        if self.state != new_state {
            self.navigation_history.push(self.state.clone());
            self.previous_state = Some(self.state.clone());
            let state_to_set = new_state.clone();
            self.state = new_state;
            self.reset_navigation_state();

            // Auto-enter editing mode for input screens if no data exists
            match state_to_set {
                AppState::AddressLookup => {
                    // If no address data exists, automatically enter editing mode
                    if self.address_data.is_none() {
                        self.input_mode = crate::ui::app::state::InputMode::Editing;
                    }
                }
                AppState::TransactionViewer => {
                    // If no transaction data exists, automatically enter editing mode
                    if self.transaction_data.is_none() {
                        self.input_mode = crate::ui::app::state::InputMode::Editing;
                    }
                }
                _ => {}
            }
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
        // Reset address data selection indices when switching screens
        if let Some(ref mut address_data) = self.address_data {
            address_data.selected_transaction_index = 0;
            address_data.selected_history_index = 0;
            address_data.selected_token_transfer_index = 0;
            address_data.selected_token_index = 0;
            address_data.selected_internal_txn_index = 0;
        }
    }

    /// Clear all messages
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }
}
