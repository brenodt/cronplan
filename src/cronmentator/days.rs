use crate::cronmentator::{BlockType, describe_type_by_pattern};

pub(crate) fn assert_valid_day(m: i32) {
    if m < 1 || m > 31 {
        panic!("invalid day of month")
    }
}

pub fn describe_days(days: Option<&str>) -> String {
    match days {
        Some("*") => "".to_string(),
        Some(_) => {
            let d = &days.expect("must be a &str");
            let t = BlockType::Day;
            if let Some(value) = describe_type_by_pattern(d, t) {
                return value;
            }
            panic!("unsupported or invalid format for days")
        }
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cronmentator::days::describe_days;

    #[test]
    fn literal() {
        let result = "2";
        assert_eq!(describe_days(Option::from(result)), " on day-of-month 2");
    }

    #[test]
    fn list() {
        let result = "4,8,12";
        assert_eq!(describe_days(Option::from(result)), " on day-of-month 4, 8, 12");
    }

    #[test]
    fn range() {
        let result = "5-10";
        assert_eq!(describe_days(Option::from(result)), " on every day-of-month from 5 through 10");
    }

    #[test]
    fn steps() {
        let result = "1/5";
        assert_eq!(describe_days(Option::from(result)), " on every 5th day-of-month from 1 through 31");
    }

    #[test]
    #[should_panic(expected = "invalid day")]
    fn invalid_negative_minute() {
        let result = "0";
        describe_days(Option::from(result));
    }

    #[test]
    #[should_panic(expected = "invalid day")]
    fn invalid_positive_minute() {
        let result = "32";
        describe_days(Option::from(result));
    }
}
