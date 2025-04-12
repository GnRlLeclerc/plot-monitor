//! Parse a log file into a hashmap of points

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use serde_json::Value;

pub fn parse(file: File) -> HashMap<String, Vec<(f64, f64)>> {
    let mut points = HashMap::new();

    io::BufReader::new(file)
        // Read .jsonl line by line
        .lines()
        // Filter out lines that are not valid JSON objects
        .filter_map(|line| {
            if let Ok(line) = line {
                let value: Result<Value, serde_json::Error> = serde_json::from_str(line.as_str());

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
                    points.entry(key).or_insert(vec![]).push((i as f64, value));
                });
        });

    points
}
