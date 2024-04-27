use crate::cronmentator::{BlockType, describe_type_by_pattern};

pub(crate) fn assert_valid_hour(m: i32) {
    if m < 0 || m > 23 {
        panic!("invalid hour")
    }
}

pub fn describe_hours(hours: Option<&str>) -> String {
    match hours {
        Some("*") => "".to_string(),
        Some(_) => {
            let h = &hours.expect("must be a &str");
            let t = BlockType::Hour;
            if let Some(value) = describe_type_by_pattern(h, t) {
                return value;
            }
            panic!("unsupported or invalid format for hour")
        }
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cronmentator::hours::describe_hours;

    #[test]
    fn literal() {
        let result = "2";
        assert_eq!(describe_hours(Option::from(result)), " at hour 2");
    }

    #[test]
    fn list() {
        let result = "12,16,0";
        assert_eq!(describe_hours(Option::from(result)), " at hours 12, 16, 0");
    }

    #[test]
    fn range() {
        let result = "5-10";
        assert_eq!(describe_hours(Option::from(result)), "At every hour from 5 through 10");
    }

    #[test]
    fn steps() {
        let result = "12/5";
        assert_eq!(describe_hours(Option::from(result)), "At every 5th hour from 12 through 23");
    }

    #[test]
    #[should_panic(expected = "invalid hour")]
    fn invalid_negative_minute() {
        let result = "-1";
        describe_hours(Option::from(result));
    }

    #[test]
    #[should_panic(expected = "invalid hour")]
    fn invalid_positive_minute() {
        let result = "60";
        describe_hours(Option::from(result));
    }
}
