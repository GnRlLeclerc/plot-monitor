use clap::Parser;
use pm_lib::{FilterOpts, Logs};

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

    /// Filter out entries from the logs (comma separated)
    #[arg(short, long)]
    except: Option<String>,

    /// Only include log entries with these names (comma separated)
    #[arg(short, long)]
    only: Option<String>,

    /// Minimum epoch to display
    #[arg(long)]
    min_epoch: Option<usize>,

    /// Maximum epoch to display
    #[arg(long)]
    max_epoch: Option<usize>,

    /// Maximum value on the y axis
    /// By default, the maximum value will be calculated from the data
    #[arg(long)]
    max: Option<f64>,

    /// Minimum value on the y axis
    /// By default, the minimum value will be calculated from the data
    #[arg(long)]
    min: Option<f64>,

    /// Maximum span to display from the end of the logs
    #[arg(long)]
    span: Option<usize>,
}

fn names_from_arg(arg: Option<String>) -> Option<Vec<String>> {
    arg.map(|s| s.split(',').map(|s| s.to_string()).collect())
}

fn main() {
    let args = Args::parse();

    let filter = FilterOpts {
        only: names_from_arg(args.only),
        except: names_from_arg(args.except),
        min_x: args.min_epoch.map(|s| s as f64),
        max_x: args.max_epoch.map(|s| s as f64),
        max_y: args.max,
        min_y: args.min,
        span: args.span.map(|s| s as f64),
    };

    let logs = Logs::new(&args.path, filter);

    match args.mode {
        Mode::Tui => {
            pm_tui::run(logs).unwrap();
        }
        Mode::Gui => {
            todo!("GUI mode");
        }
    }
}
