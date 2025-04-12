//! Points filtering logic

/// Filter parameters
#[derive(Debug, Clone, Default)]
pub struct FilterOpts {
    /// Only include log entries with these names
    pub only: Option<Vec<String>>,
    /// Filter out entries from the logs
    pub except: Option<Vec<String>>,
    /// Minimum epoch to display
    pub min: Option<f64>,
    /// Maximum epoch to display
    pub max: Option<f64>,
}

impl FilterOpts {
    /// Filter a plot based on its name.
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

    /// Trim the points of a plot based on the min and max values
    /// This function assumes that the points are sorted by epoch, and returns
    /// a subslice of the original points.
    pub fn trim<'a>(&self, points: &'a [(f64, f64)]) -> &'a [(f64, f64)] {
        let mut start = 0;
        let mut end = points.len();

        if let Some(min) = self.min {
            start = points
                .binary_search_by(|&(epoch, _)| epoch.partial_cmp(&min).unwrap())
                .unwrap_or_else(|x| x);
        }

        if let Some(max) = self.max {
            end = points
                .binary_search_by(|&(epoch, _)| epoch.partial_cmp(&max).unwrap())
                .unwrap_or_else(|x| x);
        }

        &points[start..end]
    }
}
