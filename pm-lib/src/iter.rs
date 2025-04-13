//! Implement iterators for the logs, based on the internal filters

use std::{collections::HashMap, sync::MutexGuard};

use crate::{FilterOpts, Logs};

/// A thread-safe iterable struct over the log points.
/// It must hold mutex guards for both the points and the filter during iteration.
pub struct LogsIterable<'a> {
    points: MutexGuard<'a, Option<HashMap<String, Vec<(f64, f64)>>>>,
    filter: MutexGuard<'a, FilterOpts>,
}

impl Logs {
    /// Create a new iterable struct over the logs.
    /// It will hold the locks for both the points and the filter.
    pub fn lock_iter(&self) -> Option<LogsIterable> {
        let points = self.file.points.lock().unwrap();

        if points.is_none() {
            return None;
        }

        let filter = self.filter.lock().unwrap();

        Some(LogsIterable { points, filter })
    }
}

impl<'a> LogsIterable<'a> {
    pub fn iter(&'a self) -> impl Iterator<Item = (&'a str, &'a [(f64, f64)])> {
        self.points
            .as_ref()
            .unwrap()
            .iter()
            .filter(|&(name, _)| self.filter.apply(name))
            .map(|(name, points)| (name.as_str(), self.filter.trim(points)))
    }
}
