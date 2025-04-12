//! Implement iterators for the logs, based on the internal filters

use crate::Logs;

/// Make logs iterable
impl Logs {
    pub fn iter(&self) -> impl Iterator<Item = (&str, &[(f64, f64)])> {
        self.points
            .iter()
            .filter(|&(name, _)| self.filter.apply(name))
            .map(|(name, points)| (name.as_str(), self.filter.trim(points)))
    }
}
