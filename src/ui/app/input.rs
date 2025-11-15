use super::core::App;
use super::state::InputMode;

impl App {
    /// Enter input mode
    pub fn enter_input_mode(&mut self) {
        self.input_mode = InputMode::Editing;
    }

    /// Exit input mode
    pub fn exit_input_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    /// Add character to input
    pub fn add_char(&mut self, c: char) {
        if self.input_mode == InputMode::Editing {
            self.input.insert(self.cursor_position, c);
            self.cursor_position += 1;
        }
    }

    /// Remove character from input
    pub fn remove_char(&mut self) {
        if self.input_mode == InputMode::Editing && self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    /// Move cursor left
    pub fn move_cursor_left(&mut self) {
        if self.input_mode == InputMode::Editing && self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Move cursor right
    pub fn move_cursor_right(&mut self) {
        if self.input_mode == InputMode::Editing && self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    /// Clear input
    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    /// Get current input
    pub fn get_input(&self) -> &str {
        &self.input
    }

    /// Process current input and clear it
    pub fn process_input(&mut self) -> String {
        let input = self.input.clone();
        self.clear_input();
        input
    }

    /// Set input text
    pub fn set_input(&mut self, text: String) {
        self.input = text;
        self.cursor_position = self.input.len();
    }
}
