use crate::cronmentator::{BlockType, describe_type_by_pattern};

// TODO should also allow string values : SUN MON TUE WED THU FRI SAT
pub(crate) fn assert_valid_weekday(m: i32) {
    if m < 0 || m > 6 {
        panic!("invalid weekday")
    }
}

pub fn describe_weekdays(weekdays: Option<&str>) -> String {
    match weekdays {
        Some("*") => "".to_string(),
        Some(_) => {
            let w = &weekdays.expect("must be a &str");
            let t = BlockType::Weekday;
            if let Some(value) = describe_type_by_pattern(w, t) {
                return value;
            }
            panic!("unsupported or invalid format for weekday")
        }
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cronmentator::weekdays::describe_weekdays;

    #[test]
    fn literal() {
        let result = "6";
        assert_eq!(describe_weekdays(Option::from(result)), " on 6");
    }
    #[test]
    fn list() {
        let result = "1,3,5";
        assert_eq!(describe_weekdays(Option::from(result)), " on 1, 3, 5");
    }
    #[test]
    fn range() {
        let result = "2-4";
        assert_eq!(describe_weekdays(Option::from(result)), " on every day-of-week from 2 through 4");
    }
    #[test]
    fn steps() {
        let result = "5/6";
        assert_eq!(describe_weekdays(Option::from(result)), "on every 6th day-of-week from 5 through Sunday");
    }
    #[test]
    #[should_panic(expected = "invalid weekday")]
    fn invalid_negative_minute() {
        let result = "-1";
        describe_weekdays(Option::from(result));
    }
    #[test]
    #[should_panic(expected = "invalid weekday")]
    fn invalid_positive_minute() {
        let result = "7";
        describe_weekdays(Option::from(result));
    }
}
