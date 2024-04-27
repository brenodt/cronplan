use core::panic;

use clap::Parser;
use cronplan;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Describe a cron schedule
    #[arg(short, long)]
    describe: String,
}

fn main() {
    let args = Args::parse();

    let mut fields = args.describe.split(' ');

    assert_eq!(fields.clone().count(), 5, "cron schedules must have 5 fields, space-delimited");

    let mut description = String::new();
    description += &cronplan::describe_minutes(fields.next());
    description += &cronplan::describe_hours(fields.next());
    description += &cronplan::describe_days(fields.next());
    description += &cronplan::describe_months(fields.next());
    description += &cronplan::describe_weekdays(fields.next());

    println!("{}", description);
}
