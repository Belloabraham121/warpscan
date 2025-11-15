use super::core::App;

impl App {
    /// Move to next tab
    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.saturating_add(1);
    }

    /// Move to previous tab
    pub fn previous_tab(&mut self) {
        self.current_tab = self.current_tab.saturating_sub(1);
    }

    /// Move to next item in list
    pub fn next_item(&mut self) {
        self.current_list_index = self.current_list_index.saturating_add(1);
    }

    /// Move to previous item in list
    pub fn previous_item(&mut self) {
        self.current_list_index = self.current_list_index.saturating_sub(1);
    }

    /// Scroll down
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Scroll up
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}
