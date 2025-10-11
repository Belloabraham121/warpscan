use super::core::App;

impl App {
    /// Set loading state for an operation
    pub fn set_loading(&mut self, operation: &str, loading: bool) {
        self.loading_states.insert(operation.to_string(), loading);
    }

    /// Check if an operation is loading
    pub fn is_loading(&self, operation: &str) -> bool {
        self.loading_states.get(operation).copied().unwrap_or(false)
    }

    /// Set error message
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }

    /// Set success message
    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }

    /// Cache data for a key
    pub fn cache_data(&mut self, key: String, data: serde_json::Value) {
        self.data_cache.insert(key, data);
    }

    /// Get cached data for a key
    pub fn get_cached_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data_cache.get(key)
    }

    /// Clear cached data
    pub fn clear_cache(&mut self) {
        self.data_cache.clear();
    }
}