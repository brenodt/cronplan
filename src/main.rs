use core::panic;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Describe a cron schedule
    #[arg(short, long)]
    describe: String,
}

enum BlockType {
    Minute,
    Hour,
    Day,
    Month,
    Weekday,
}

fn main() {
    let args = Args::parse();

    let mut fields = args.describe.split(' ');

    assert!(
        fields.clone().count() == 5,
        "cron schedules must have 5 fields, space-delimited"
    );

    let mut description = String::new();
    description += &describe_minutes(fields.next());
    description += &describe_hours(fields.next());
    description += &describe_days(fields.next());
    description += &describe_months(fields.next());
    description += &describe_weekdays(fields.next());

    println!("{}", description);
}

fn assert_valid_minute(m: i32) {
    if m < 0 || m > 59 {
        panic!("invalid minute")
    }
}

fn assert_valid_hour(m: i32) {
    if m < 0 || m > 23 {
        panic!("invalid hour")
    }
}

fn assert_valid_day(m: i32) {
    if m < 0 || m > 31 {
        panic!("invalid day of month")
    }
}

// TODO should also allow string values : JAN FEB MAR APR MAY JUN JUL AUG SEP OCT NOV DEC
fn assert_valid_month(m: i32) {
    if m < 1 || m > 12 {
        panic!("invalid month")
    }
}

// TODO should also allow string values : SUN MON TUE WED THU FRI SAT
fn assert_valid_weekday(m: i32) {
    if m < 0 || m > 6 {
        panic!("invalid weekday")
    }
}

fn describe_minutes(minutes: Option<&str>) -> String {
    match minutes {
        Some("*") => "At every minute".to_string(),
        Some(_) => {
            let m = &minutes.expect("must be a &str");
            if m.contains(",") {
                return describe_value_list(m, BlockType::Minute);
            } else if m.contains("-") {
                return describe_range(m, BlockType::Minute);
            } else if m.contains("/") {
                return describe_step(m, BlockType::Minute);
            }
            panic!("unsupported or invalid format for minute")
        }
        None => "".to_string(),
    }
}

fn describe_hours(hours: Option<&str>) -> String {
    match hours {
        Some("*") => "".to_string(),
        Some(_) => {
            let h = &hours.expect("must be a &str");
            if h.contains(",") {
                return describe_value_list(h, BlockType::Hour);
            } else if h.contains("-") {
                return describe_range(h, BlockType::Hour);
            } else if h.contains("/") {
                return describe_step(h, BlockType::Hour);
            }
            panic!("unsupported or invalid format for hour")
        }
        None => "".to_string(),
    }
}

fn describe_days(days: Option<&str>) -> String {
    match days {
        Some("*") => "".to_string(),
        Some(_) => {
            let d = &days.expect("must be a &str");
            if d.contains(",") {
                return describe_value_list(d, BlockType::Day);
            } else if d.contains("-") {
                return describe_range(d, BlockType::Day);
            } else if d.contains("/") {
                return describe_step(d, BlockType::Day);
            }
            panic!("unsupported or invalid format for days")
        }
        None => "".to_string(),
    }
}

fn describe_months(months: Option<&str>) -> String {
    match months {
        Some("*") => "".to_string(),
        Some(_) => {
            let m = &months.expect("must be a &str");
            if m.contains(",") {
                return describe_value_list(m, BlockType::Month);
            } else if m.contains("-") {
                return describe_range(m, BlockType::Month);
            } else if m.contains("/") {
                return describe_step(m, BlockType::Month);
            }
            panic!("unsupported or invalid format for months")
        }
        None => "".to_string(),
    }
}

fn describe_weekdays(weekdays: Option<&str>) -> String {
    match weekdays {
        Some("*") => "".to_string(),
        Some(_) => {
            let w = &weekdays.expect("must be a &str");
            if w.contains(",") {
                return describe_value_list(w, BlockType::Weekday);
            } else if w.contains("-") {
                return describe_range(w, BlockType::Weekday);
            } else if w.contains("/") {
                return describe_step(w, BlockType::Weekday);
            }
            panic!("unsupported or invalid format for weekday")
        }
        None => "".to_string(),
    }
}

fn describe_value_list(m: &str, t: BlockType) -> String {
    let values: Vec<i32> = m.split(",").map(|n| n.parse::<i32>().unwrap()).collect();

    let prefix = match t {
        BlockType::Minute => "At minute ",
        BlockType::Hour => " past hour ",
        BlockType::Day => " on day-of-month ",
        BlockType::Month => " in ",   // TODO map month number to label
        BlockType::Weekday => " on ", // TODO map weekday number to label
    };

    let mut desc = String::new();
    values.iter().for_each(|&v| {
        match t {
            BlockType::Minute => {
                assert_valid_minute(v);
            }
            BlockType::Hour => {
                assert_valid_hour(v);
            }
            BlockType::Day => {
                assert_valid_day(v);
            }
            BlockType::Month => {
                assert_valid_month(v);
            }
            BlockType::Weekday => {
                assert_valid_weekday(v);
            }
        }
        if desc.is_empty() {
            desc += prefix;
            desc += &v.to_string();
        } else {
            desc += ", ";
            desc += &v.to_string();
        }
    });
    desc
}

// TODO should allow "-" on first element; e.g. "1-10/2" should read "At every 2nd minute from 1 through 10"
fn describe_step(s: &str, t: BlockType) -> String {
    if 1 != s.chars().filter(|c| *c == '/').count() {
        panic!("only one step per slot")
    }

    let prefix = match t {
        BlockType::Minute => "At every ",
        BlockType::Hour => " past every ",
        BlockType::Day => " on every ",
        BlockType::Month => " in every ",
        BlockType::Weekday => "on every ",
    };
    let label = match t {
        BlockType::Minute => "minute",
        BlockType::Hour => "hour",
        BlockType::Day => "day-of-month",
        BlockType::Month => "month",
        BlockType::Weekday => "day-of-week",
    };
    let upper_bound = match t {
        BlockType::Minute => "59",
        BlockType::Hour => "23",
        BlockType::Day => "31",
        BlockType::Month => "December",
        BlockType::Weekday => "Sunday",
    };

    let values: Vec<i32> = s.split("/").map(|n| n.parse::<i32>().unwrap()).collect();
    assert!(values.len() == 2, "step requires exactly two bounds");

    let start = values[0];
    let end = values[1];
    assert!(end > start, "end must be greater than start");
    match t {
        BlockType::Minute => {
            assert_valid_minute(start);
            assert_valid_minute(end);
        }
        BlockType::Hour => {
            assert_valid_hour(start);
            assert_valid_hour(end);
        }
        BlockType::Day => {
            assert_valid_day(start);
            assert_valid_day(end);
        }
        BlockType::Month => {
            assert_valid_month(start);
            assert_valid_month(end);
        }
        BlockType::Weekday => {
            assert_valid_weekday(start);
            assert_valid_weekday(end);
        }
    }

    let mut desc = String::new();
    desc += prefix;
    match end % 10 {
        1 => desc += &format!("{}st", end),
        2 => desc += &format!("{}nd", end),
        3 => desc += &format!("{}rd", end),
        _ => desc += &format!("{}th", end),
    }
    desc += &format!(" {} from ", label);
    desc += &start.to_string();
    desc += &format!(" through {}", upper_bound);
    desc
}

fn describe_range(s: &str, t: BlockType) -> String {
    if 1 != s.chars().filter(|c| *c == '-').count() {
        panic!("only one range per slot")
    }

    let prefix = match t {
        BlockType::Minute => "At every hour from ",
        BlockType::Hour => " past every hour from ",
        BlockType::Day => " on every day-of-month from ",
        BlockType::Month => " in every month from ",
        BlockType::Weekday => " on every day-of-week from ",
    };
    let values: Vec<i32> = s.split("-").map(|n| n.parse::<i32>().unwrap()).collect();
    assert!(values.len() == 2, "range requires exactly two bounds");

    let start = values[0];
    let end = values[1];
    assert!(end > start, "end must be greater than start");

    match t {
        BlockType::Minute => {
            assert_valid_minute(start);
            assert_valid_minute(end);
        }
        BlockType::Hour => {
            assert_valid_hour(start);
            assert_valid_hour(end);
        }
        BlockType::Day => {
            assert_valid_day(start);
            assert_valid_day(end);
        }
        BlockType::Month => {
            assert_valid_month(start);
            assert_valid_month(end);
        }
        BlockType::Weekday => {
            assert_valid_weekday(start);
            assert_valid_weekday(end);
        }
    }

    let mut desc = String::new();
    desc += prefix;
    desc += &start.to_string();
    desc += " through ";
    desc += &end.to_string();
    desc
}
