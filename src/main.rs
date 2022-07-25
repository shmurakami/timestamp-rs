use chrono::{Local, TimeZone, Timelike};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Timestamp to show as system date time
    #[clap(short, value_parser)]
    timestamp: i64,
    /// Do not show date and timezone.
    #[clap(short, long, value_parser, default_value_t = false)]
    only_time: bool,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    let t: i64 = args.timestamp;
    let only = args.only_time;

    let l = Local.timestamp(t, 0);

    if only {
        println!("{:02?}:{:02?}:{:02?}", l.hour(), l.minute(), l.second());
    } else {
        println!("{}", l.to_string());
    }

    Ok(())
}
