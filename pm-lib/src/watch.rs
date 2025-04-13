//! Utilities for watching the target file

use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    path::Path,
    sync::{Arc, Mutex},
    thread::spawn,
};

use crossbeam::channel::Sender;
use notify::{Event, EventKind, INotifyWatcher, Watcher, recommended_watcher};

use crate::parse;

/// A struct that holds the data for a file being watched.
/// It should be referenced by a watcher thread to be updated on file change.
///
/// This object has interior mutability, it is safe to hold multiple references to it.
#[derive(Debug)]
pub struct Watched {
    /// File name
    pub name: Arc<OsStr>,

    /// File path
    pub path: Arc<Path>,

    /// Map of data points, stored as (epoch, value) tuples
    pub points: Mutex<Option<HashMap<String, Vec<(f64, f64)>>>>,
}

impl Watched {
    pub fn new(filepath: &str) -> Self {
        let filepath = Path::new(filepath);
        Watched {
            name: filepath.file_name().unwrap().into(),
            path: filepath.into(),
            points: None.into(),
        }
    }

    /// Refresh the logs from the file.
    /// If the file is not found, the points will be set to None.
    pub fn refresh(&self) {
        if let Ok(file) = File::open(&self.path) {
            let result = parse(file); // Lock only after file processing
            *self.points.lock().unwrap() = Some(result);
        } else {
            *self.points.lock().unwrap() = None;
        }
    }
}

/// Check if the event is about the file we are interested in
fn is_event_about_file(filename: &OsStr, event: &Event) -> bool {
    // Filter by kind (rules out unwanted events + most of directory-related events)
    match event.kind {
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
            // Filter by file name
            event
                .paths
                .iter()
                .any(|path| path.file_name() == Some(filename))
        }
        _ => false,
    }
}

/// Start a file watcher in a background thread.
pub fn start_watcher(watched: Arc<Watched>, signal: Sender<bool>) -> INotifyWatcher {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = recommended_watcher(tx).expect("Failed to create watcher");

    // Start a watcher on the directory (and not the file itself)
    // so that it can handle file creation and deletion as well.
    let directory = watched.path.parent().unwrap();
    watcher
        .watch(directory, notify::RecursiveMode::NonRecursive)
        .expect("Failed to watch file");

    // Watch the file for changes and update the logs in a background thread
    spawn(move || {
        for res in rx {
            match res {
                Ok(event) => {
                    if is_event_about_file(&watched.name, &event) {
                        watched.refresh();
                        signal.send(true).unwrap(); // Signal the data update
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    watcher
}
