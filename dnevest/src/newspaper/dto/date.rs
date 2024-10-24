use serde::Deserialize;

use crate::newspaper::{Date, Error};

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct DateDTO(pub(super) String);

impl TryFrom<DateDTO> for Date {
    type Error = Error;

    fn try_from(value: DateDTO) -> Result<Self, Self::Error> {
        Self::try_new(value.0.as_str())
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::newspaper::Date;

    #[test]
    fn deserialize() {
        let date = Date::new(NaiveDate::from_ymd_opt(2024, 06, 29).unwrap());
        let deserialized = serde_json::from_str(r#""29-06-2024""#).unwrap();

        assert_eq!(date, deserialized)
    }
}
