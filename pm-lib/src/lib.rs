use std::{collections::HashMap, fs::File};

use parse::parse;

mod filter;
mod iter;
mod parse;

pub use filter::*;

#[derive(Debug, Clone)]
pub struct Logs {
    /// Map of data points, stored as (epoch, value) tuples
    pub points: Option<HashMap<String, Vec<(f64, f64)>>>,

    /// Path to the JSONL file
    pub file: String,

    /// Filter parameters
    pub filter: FilterOpts,
}

impl Logs {
    pub fn new(path: &str, filter: FilterOpts) -> Logs {
        let mut logs = Logs {
            file: path.to_string(),
            points: None,
            filter,
        };
        logs.refresh();

        logs
    }

    /// Refresh the logs from the file.
    /// If the file is not found, the points will be set to None.
    pub fn refresh(&mut self) {
        if let Ok(file) = File::open(self.file.as_str()) {
            self.points = Some(parse(file));
        } else {
            self.points = None;
        }
    }
}
