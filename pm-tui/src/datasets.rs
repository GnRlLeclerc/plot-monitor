//! Convert logs to ratatui datasets for plotting

use pm_lib::Logs;
use ratatui::prelude::*;
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

/// Draw datasets on the given area
pub fn draw_datasets(logs: &Logs, rect: Rect, buf: &mut Buffer) {
    let mut x_min = 0.0_f64;
    let mut x_max = 0.0_f64;
    let mut y_min = 0.0_f64;
    let mut y_max = 0.0_f64;

    let mut cmap = Colormap::new();

    // Create datasets
    let datasets = logs
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

    // TODO: compute labels and space them properly

    Chart::new(datasets)
        .x_axis(
            Axis::default()
                .title("Epochs")
                .style(Style::default().gray())
                .bounds([x_min, x_max])
                .labels([format!("{}", x_min as i64), format!("{}", x_max as i64)]),
        )
        .y_axis(
            Axis::default()
                .title("Value")
                .style(Style::default().gray())
                .bounds([y_min, y_max])
                .labels([format!("{:5.3}", y_min), format!("{:5.3}", y_max)]),
        )
        .render(rect, buf);
}
