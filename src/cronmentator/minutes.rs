use crate::cronmentator::{BlockType, describe_type_by_pattern};

pub(crate) fn assert_valid_minute(m: i32) {
    if m < 0 || m > 59 {
        panic!("invalid minute")
    }
}

pub fn describe_minutes(minutes: Option<&str>) -> String {
    match minutes {
        Some("*") => "At every minute".to_string(),
        Some(_) => {
            let m = &minutes.expect("must be a &str");
            let t = BlockType::Minute;
            if let Some(value) = describe_type_by_pattern(m, t) {
                return value;
            }
            panic!("unsupported or invalid format for minute")
        }
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cronmentator::minutes::describe_minutes;

    #[test]
    fn literal() {
        let result = "1";
        assert_eq!(describe_minutes(Option::from(result)), "At minute 1");
    }
    #[test]
    fn list() {
        let result = "12,30,48";
        assert_eq!(describe_minutes(Option::from(result)), "At minute 12, 30, 48");
    }
    #[test]
    fn range() {
        let result = "5-10";
        assert_eq!(describe_minutes(Option::from(result)), "At every hour from 5 through 10");
    }
    #[test]
    fn steps() {
        let result = "6/24";
        assert_eq!(describe_minutes(Option::from(result)), "At every 24th minute from 6 through 59");
    }
    #[test]
    #[should_panic(expected = "invalid minute")]
    fn invalid_negative_minute() {
        let result = "-1";
        describe_minutes(Option::from(result));
    }
    #[test]
    #[should_panic(expected = "invalid minute")]
    fn invalid_positive_minute() {
        let result = "60";
        describe_minutes(Option::from(result));
    }
}
