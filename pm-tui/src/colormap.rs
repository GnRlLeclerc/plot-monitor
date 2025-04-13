//! Colormaps for the different plots

use ratatui::prelude::Color;

/// Valid colors for the colormap
static COLORS: [Color; 6] = [
    Color::Blue,
    Color::Green,
    Color::Red,
    Color::Yellow,
    Color::Magenta,
    Color::Cyan,
];

pub struct Colormap {
    index: usize,
}

impl Colormap {
    /// Create a new colormap
    pub fn new() -> Self {
        Colormap { index: 0 }
    }

    pub fn next(&mut self) -> Color {
        let color = COLORS[self.index];
        self.index += 1;
        if self.index >= COLORS.len() {
            self.index = 0;
        }
        color
    }
}
