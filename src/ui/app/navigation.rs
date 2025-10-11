use super::core::App;
use super::state::{AppState, InputMode};

impl App {
    /// Navigate to a new state
    pub fn navigate_to(&mut self, new_state: AppState) {
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