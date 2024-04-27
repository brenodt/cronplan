use crate::cronmentator::{BlockType, describe_type_by_pattern};

// TODO should also allow string values : JAN FEB MAR APR MAY JUN JUL AUG SEP OCT NOV DEC
pub(crate) fn assert_valid_month(m: i32) {
    if m < 1 || m > 12 {
        panic!("invalid month")
    }
}

pub fn describe_months(months: Option<&str>) -> String {
    match months {
        Some("*") => "".to_string(),
        Some(_) => {
            let m = &months.expect("must be a &str");
            let t = BlockType::Month;
            if let Some(value) = describe_type_by_pattern(m, t) {
                return value;
            }
            panic!("unsupported or invalid format for months")
        }
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cronmentator::months::describe_months;

    #[test]
    fn literal() {
        let result = "2";
        assert_eq!(describe_months(Option::from(result)), " in 2");
    }

    #[test]
    fn list() {
        let result = "4,8,12";
        assert_eq!(describe_months(Option::from(result)), " in 4, 8, 12");
    }

    #[test]
    fn range() {
        let result = "5-10";
        assert_eq!(describe_months(Option::from(result)), " in every month from 5 through 10");
    }

    #[test]
    fn steps() {
        let result = "1/5";
        assert_eq!(describe_months(Option::from(result)), " in every 5th month from 1 through December");
    }

    #[test]
    #[should_panic(expected = "invalid month")]
    fn invalid_negative_minute() {
        let result = "0";
        describe_months(Option::from(result));
    }

    #[test]
    #[should_panic(expected = "invalid month")]
    fn invalid_positive_minute() {
        let result = "13";
        describe_months(Option::from(result));
    }
}
