//! Points filtering logic

/// Filter parameters
#[derive(Debug, Clone, Default)]
pub struct FilterOpts {
    /// Only include log entries with these names
    pub only: Option<Vec<String>>,
    /// Filter out entries from the logs
    pub except: Option<Vec<String>>,
}

impl FilterOpts {
    /// Apply filtering logic to a point iterator depending on the filter options
    /// The `except` option will take precedence over the `only` option.
    pub fn apply(&self, name: &str) -> bool {
        if let Some(except) = &self.except {
            if except.contains(&name.to_string()) {
                return false;
            }
        } else if let Some(only) = &self.only {
            if !only.contains(&name.to_string()) {
                return false;
            }
        }

        true
    }
}
