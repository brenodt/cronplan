mod minutes;
mod hours;
mod days;
mod months;
mod weekdays;

pub use minutes::*;
pub use hours::*;
pub use days::*;
pub use months::*;
pub use weekdays::*;

#[derive(Clone, Copy)]
enum BlockType {
    Minute,
    Hour,
    Day,
    Month,
    Weekday,
}

fn describe_type_by_pattern(w: &&str, t: BlockType) -> Option<String> {
    if w.contains(",") {
        return Some(describe_value_list(w, t));
    } else if w.contains("/") {
        return Some(describe_step(w, t));
    } else if !w.parse::<i32>().is_err() {
        return Some(describe_literal(w, t));
    } else if w.contains("-") {
        return Some(describe_range(w, t));
    }
    None
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
        assert_valid(t, v);
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
    assert_eq!(values.len(), 2, "step requires exactly two bounds");

    let start = values[0];
    let end = values[1];
    assert!(end > start, "end must be greater than start");

    assert_valid_range(t, start, end);

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
    assert_eq!(values.len(), 2, "range requires exactly two bounds");

    let start = values[0];
    let end = values[1];
    assert!(end > start, "end must be greater than start");

    assert_valid_range(t, start, end);

    let mut desc = String::new();
    desc += prefix;
    desc += &start.to_string();
    desc += " through ";
    desc += &end.to_string();
    desc
}

fn describe_literal(m: &str, t: BlockType) -> String {
    assert_valid(t, m.parse::<i32>().unwrap());
    let prefix = match t {
        BlockType::Minute => "At minute ",
        BlockType::Hour => " past hour ",
        BlockType::Day => " on day-of-month ",
        BlockType::Month => " in ",   // TODO map month number to label
        BlockType::Weekday => " on ", // TODO map weekday number to label
    };

    format!("{}{}", prefix, m)
}

fn assert_valid(t: BlockType, v: i32) {
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
}

fn assert_valid_range(t: BlockType, start: i32, end: i32) {
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
}
