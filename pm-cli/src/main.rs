use clap::Parser;
use pm_lib::Logs;

/// Simple TUI to monitor logs from JSONL files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the JSONL file
    path: String,
}

fn main() {
    let args = Args::parse();

    let logs = Logs::from_file(args.path.as_str());

    println!("{:?}", logs);
}
