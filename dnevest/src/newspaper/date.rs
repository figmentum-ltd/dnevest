use chrono::{Datelike, NaiveDate, Weekday};
use serde::Deserialize;

use crate::bindings::ByteArray;

use super::error::{Error, Result};

const FORMAT: &str = "%d-%m-%Y";

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(super) struct Date {
    date: NaiveDate,
}

impl Date {
    pub(crate) fn parse_from_json(json_data: ByteArray) -> Result<Self> {
        #[derive(Deserialize)]
        struct DateInput {
            date: String,
        }

        serde_json::from_slice::<DateInput>(&json_data)
            .map_err(Error::JsonError)
            .and_then(|date_input| {
                NaiveDate::parse_from_str(&date_input.date, FORMAT)
                    .map_err(|err| Error::DateParsing(err))
                    .map(|date| Date::new(date))
            })
    }

    pub(super) fn day_of_week(&self) -> Weekday {
        self.date.weekday()
    }

    fn new(date: NaiveDate) -> Self {
        Self { date }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Weekday};

    use super::{Date, Result};

    #[test]
    fn invalid_json() {
        let res = Date::parse_from_json(r#"{"29-02-2024"}"#.into());
        assert_err(res, "Invalid json");
    }

    #[test]
    fn valid_date_parcing() {
        let res = Date::parse_from_json(r#"{"date":"29-02-2024"}"#.into()).unwrap();
        let expected = Date::new(NaiveDate::from_ymd_opt(2024, 02, 29).unwrap());

        assert_eq!(res, expected);
    }

    #[test]
    fn invalid_date() {
        let msg = "Error parsing date";

        let invalid_date = Date::parse_from_json(r#"{"date":"29-02-2026"}"#.into());
        assert_err(invalid_date, msg);

        let not_existing_month = Date::parse_from_json(r#"{"date":"1-13-2025"}"#.into());
        assert_err(not_existing_month, msg);

        let dd_mm_yyyy = Date::parse_from_json(r#"{"date":"13/06/2022"}"#.into());
        assert_err(dd_mm_yyyy, msg);

        let yyyy_mm_dd = Date::parse_from_json(r#"{"date":"2023_12_24"}"#.into());
        assert_err(yyyy_mm_dd, msg);
    }

    #[test]
    fn test_various_days_of_week() {
        let date1 = Date::parse_from_json(r#"{"date":"01-01-2023"}"#.into()).unwrap();
        assert_eq!(date1.day_of_week(), Weekday::Sun);

        let date2 = Date::parse_from_json(r#"{"date":"04-07-2023"}"#.into()).unwrap();
        assert_eq!(date2.day_of_week(), Weekday::Tue);

        let date3 = Date::parse_from_json(r#"{"date":"25-12-2023"}"#.into()).unwrap();
        assert_eq!(date3.day_of_week(), Weekday::Mon);
    }

    fn assert_err(r: Result<Date>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
