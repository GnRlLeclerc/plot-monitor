use std::sync::{Arc, RwLock};

use crossbeam::channel::Receiver;
use notify::INotifyWatcher;
use parse::parse;

mod filter;
mod iter;
mod parse;
mod watch;

pub use filter::*;
use watch::{Watched, start_watcher};

/// Logs holder struct.
/// It is thread-safe in order to be accessible by background processing threads
/// and frontend rendering threads at the same time.
#[derive(Debug)]
pub struct Logs {
    /// Watched file info
    pub file: Arc<Watched>,

    /// Filter parameters
    pub filter: RwLock<FilterOpts>,

    /// Channel that notifies when the internal data was updated
    pub updates: Receiver<bool>,

    #[allow(dead_code)]
    watcher: INotifyWatcher,
}

impl Logs {
    /// Instantiate a new `Logs` object.
    /// Leak the file path to make it static.
    pub fn new(filepath: &str, filter: FilterOpts) -> Logs {
        let watched = Arc::new(Watched::new(filepath));

        // Start a background file watcher
        let (tx, rx) = crossbeam::channel::unbounded::<bool>();
        let watcher = start_watcher(watched.clone(), tx);

        let logs = Logs {
            watcher,
            updates: rx,
            filter: filter.into(),
            file: watched,
        };

        logs.file.refresh(); // Load the file into the logs

        logs
    }
}
