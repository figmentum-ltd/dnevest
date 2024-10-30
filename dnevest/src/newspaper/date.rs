use std::result;

use chrono::{Datelike, NaiveDate, Weekday};
use serde::Deserialize;

use super::{
    error::{Error, Result},
    Year,
};

const FORMAT: &str = "%d-%m-%Y";

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize, Clone)]
#[serde(try_from = "String")]
pub(crate) struct Date(NaiveDate);

impl Date {
    pub(super) fn try_new(date: &str) -> Result<Self> {
        NaiveDate::parse_from_str(&date, FORMAT)
            .map_err(|err| Error::DateParsing(err))
            .map(|date| Date(date))
    }

    pub(super) fn day_of_week(&self) -> Weekday {
        self.0.weekday()
    }

    pub(super) fn year(&self) -> Year {
        self.0
            .year()
            .try_into()
            .expect("Year must be a positive number")
    }

    #[cfg(test)]
    pub(crate) fn new(day: u32, month: u32, year: Year) -> Self {
        Self(
            NaiveDate::from_ymd_opt(
                year.try_into().expect("Failed conver u32 to i32"),
                month,
                day,
            )
            .expect("Failed create a valid NaiveDate object"),
        )
    }
}

impl TryFrom<String> for Date {
    type Error = Error;

    fn try_from(value: String) -> result::Result<Self, Self::Error> {
        Self::try_new(value.as_str())
    }
}

#[cfg(test)]
mod test_parse {
    use super::{Date, Result};

    #[test]
    fn valid_date() {
        let res = Date::try_new("29-02-2024").unwrap();
        let expected = Date::new(29, 02, 2024);

        assert_eq!(res, expected);
    }

    #[test]
    fn invalid_date() {
        let msg = "Error parsing date";

        let invalid_date = Date::try_new("29-02-2026");
        assert_err(invalid_date, msg);

        let not_existing_month = Date::try_new("1-13-2025");
        assert_err(not_existing_month, msg);

        let dd_mm_yyyy = Date::try_new("13/06/2022");
        assert_err(dd_mm_yyyy, msg);

        let yyyy_mm_dd = Date::try_new("2023_12_24");
        assert_err(yyyy_mm_dd, msg);
    }

    fn assert_err(r: Result<Date>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}

#[cfg(test)]
mod test {
    use chrono::Weekday;

    use crate::newspaper::Date;

    #[test]
    fn test_various_days_of_week() {
        let date1 = Date::try_new("01-01-2023").unwrap();
        assert_eq!(date1.day_of_week(), Weekday::Sun);

        let date2 = Date::try_new("04-07-2023").unwrap();
        assert_eq!(date2.day_of_week(), Weekday::Tue);

        let date3 = Date::try_new("25-12-2023").unwrap();
        assert_eq!(date3.day_of_week(), Weekday::Mon);
    }
}
