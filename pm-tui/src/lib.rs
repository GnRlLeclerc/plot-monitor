//! Plot Monitor TUI

use std::io;

use app::App;
use pm_lib::Logs;

mod app;
mod colormap;
mod datasets;

/// Run the plot-monitor TUI
pub fn run(logs: Logs) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new(logs).run(&mut terminal);
    ratatui::restore();
    app_result
}
