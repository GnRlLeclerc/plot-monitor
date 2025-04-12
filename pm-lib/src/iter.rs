//! Implement iterators for the logs, based on the internal filters

use crate::Logs;

/// Make logs iterable
impl Logs {
    /// Iterate over the filtered logs. Returns None if the file does not exist.
    pub fn iter(&self) -> Option<impl Iterator<Item = (&str, &[(f64, f64)])>> {
        if let Some(points) = &self.points {
            return Some(
                points
                    .iter()
                    .filter(|&(name, _)| self.filter.apply(name))
                    .map(|(name, points)| (name.as_str(), self.filter.trim(points))),
            );
        }
        None
    }
}
