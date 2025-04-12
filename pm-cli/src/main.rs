use clap::Parser;
use pm_lib::Logs;

/// Display mode
#[derive(Debug, Default, Clone, clap::ValueEnum)]
#[clap(rename_all = "lowercase")]
enum Mode {
    /// TUI mode (ratatui)
    #[default]
    Tui,
    /// GUI mode (gtk plotters)
    Gui,
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Tui => "tui".to_string(),
            Mode::Gui => "gui".to_string(),
        }
    }
}

/// Simple TUI to monitor logs from JSONL files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the JSONL file
    path: String,

    /// Display mode
    #[arg(short, long, default_value_t)]
    mode: Mode,
}

fn main() {
    let args = Args::parse();

    let logs = Logs::new(args.path.as_str());

    match args.mode {
        Mode::Tui => {
            pm_tui::run(logs).unwrap();
        }
        Mode::Gui => {
            todo!("GUI mode");
        }
    }
}
