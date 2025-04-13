//! Points filtering logic

/// Filter parameters
#[derive(Debug, Clone, Default)]
pub struct FilterOpts {
    /// Only include log entries with these names
    pub only: Option<Vec<String>>,
    /// Filter out entries from the logs
    pub except: Option<Vec<String>>,
    /// Minimum epoch to display
    pub min_x: Option<f64>,
    /// Maximum epoch to display
    pub max_x: Option<f64>,
    /// Maximum value on the y axis (note: only used for display)
    pub max_y: Option<f64>,
    /// Minimum value on the y axis (note: only used for display)
    pub min_y: Option<f64>,
    /// Maximum span to display (from the end of the logs)
    pub span: Option<f64>,
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
    ///
    /// The `span` option will take precedence over the `min` and `max` options.
    pub fn trim<'a>(&self, points: &'a [(f64, f64)]) -> &'a [(f64, f64)] {
        if let Some(span) = self.span {
            let end = points[points.len() - 1].0;
            let start_value = end - span;

            let start = points
                .binary_search_by(|&(epoch, _)| epoch.partial_cmp(&start_value).unwrap())
                .unwrap_or_else(|x| x);

            return &points[start..];
        }

        let mut start = 0;
        let mut end = points.len();

        if let Some(min) = self.min_x {
            start = points
                .binary_search_by(|&(epoch, _)| epoch.partial_cmp(&min).unwrap())
                .unwrap_or_else(|x| x);
        }

        if let Some(max) = self.max_x {
            end = points
                .binary_search_by(|&(epoch, _)| epoch.partial_cmp(&max).unwrap())
                .unwrap_or_else(|x| x);
        }

        &points[start..end]
    }
}
