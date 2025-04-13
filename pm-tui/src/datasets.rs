//! Convert logs to ratatui datasets for plotting

use std::ffi::OsStr;

use pm_lib::Logs;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::Marker,
    widgets::{Axis, Chart, Dataset, GraphType},
};

use crate::colormap::Colormap;

/// Convert a series of points to a dataset
pub fn to_dataset<'a>(name: &'a str, points: &'a [(f64, f64)]) -> Dataset<'a> {
    Dataset::default()
        .name(name)
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .data(points)
}

/// Draw an error message on the given area
pub fn draw_error(file: &OsStr, msg: &str, rect: Rect, buf: &mut Buffer) {
    Paragraph::new(msg.bold().red())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: rect.height / 2,
                    bottom: 0,
                })
                .title(file.to_str().unwrap())
                .title_alignment(Alignment::Center),
        )
        .render(rect, buf);
}

/// Draw datasets on the given area
pub fn draw_datasets(logs: &Logs, rect: Rect, buf: &mut Buffer) {
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;

    let mut cmap = Colormap::new();

    let datasets = logs.lock_iter();

    // If the file is not found, just display an error screen
    if datasets.is_none() {
        draw_error(&logs.file.name, "FILE NOT FOUND", rect, buf);
        return;
    }
    let datasets = datasets.unwrap();

    // Create datasets
    let datasets = datasets
        .iter()
        .map(|(name, points)| {
            // Compute bounds
            x_min = x_min.min(points[0].0);
            x_max = x_max.max(points[points.len() - 1].0);
            y_min = y_min.min(points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min));
            y_max = y_max.max(points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max));

            return to_dataset(name, points).style(Style::default().fg(cmap.next()));
        })
        .collect::<Vec<_>>();

    if datasets.is_empty() {
        draw_error(&logs.file.name, "NO DATA", rect, buf);
        return;
    }

    Chart::new(datasets)
        .x_axis(
            Axis::default()
                .title("Epochs")
                .style(Style::default().gray())
                .bounds([x_min, x_max])
                .labels([
                    format!("{}", x_min as i64),
                    format!("{}", ((x_max + x_min) / 2.0) as i64),
                    format!("{}", x_max as i64),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Value")
                .style(Style::default().gray())
                .bounds([y_min, y_max])
                .labels([
                    format!("{:.2e}", y_min),
                    format!("{:.2e}", (y_max + y_min) / 2.0),
                    format!("{:.2e}", y_max),
                ]),
        )
        .block(
            Block::default()
                .padding(Padding::horizontal(1))
                .title(logs.file.name.to_str().unwrap())
                .title_alignment(Alignment::Center),
        )
        .render(rect, buf);
}
