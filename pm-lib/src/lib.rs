use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Logs {
    /// Map of data points, stored as (epoch, value) tuples
    pub points: HashMap<String, Vec<(usize, f64)>>,
}

impl Logs {
    pub fn from_file(path: &str) -> Logs {
        let mut logs = Logs {
            points: HashMap::new(),
        };

        let file = File::open(path).expect("Unable to open file");

        io::BufReader::new(file)
            // Read .jsonl line by line
            .lines()
            // Filter out lines that are not valid JSON objects
            .filter_map(|line| {
                if let Ok(line) = line {
                    let value: Result<Value, serde_json::Error> =
                        serde_json::from_str(line.as_str());

                    if let Ok(value) = value {
                        return match value {
                            Value::Object(map) => Some(map),
                            _ => None,
                        };
                    }
                    None
                } else {
                    None
                }
            })
            // Number the valid lines
            .enumerate()
            // Load the values of each line into the logs
            .for_each(|(i, map)| {
                map.into_iter()
                    .filter_map(|(key, value)| match value {
                        Value::Number(num) => match num.as_f64() {
                            Some(num) => Some((key, num)),
                            None => None,
                        },
                        _ => None,
                    })
                    .for_each(|(key, value)| {
                        logs.points.entry(key).or_insert(vec![]).push((i, value));
                    });
            });

        logs
    }
}
