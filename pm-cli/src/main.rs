use pm_lib::Logs;

fn main() {
    let log_path = "examples/logs.jsonl";
    let logs = Logs::from_file(log_path);
    println!("Logs: {:#?}", logs);
}
