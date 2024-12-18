use chrono::{DateTime, Datelike};
use serde::{Deserialize, Serialize};

use std::result::Result as StdResult;

#[cfg(test)]
use crate::services::MockHost;
use crate::{
    bindings::{component::dnevest::time::Clock, ByteArray},
    Host, Storage, Time,
};

use super::{
    dto::QueryNewspaperDTO,
    error::{Error, Result},
    signature::{self, Signature},
    Date, WeeklyFrequency, Year,
};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(try_from = "UncheckedNewspaper")]
pub(crate) struct Newspaper {
    signature: Signature,
    name: String,
    start_year: Year,
    end_year: Option<Year>,
    weekly_schedule: WeeklyFrequency,
}

impl Newspaper {
    #[cfg(test)]
    pub(crate) fn new_unchecked(
        signature: &str,
        name: &str,
        start_year: Year,
        end_year: Option<Year>,
        publicated_on: [bool; 7],
    ) -> Self {
        Self::new(
            Signature::new(signature),
            name.to_string(),
            start_year,
            end_year,
            WeeklyFrequency::new(publicated_on),
        )
    }

    pub(crate) fn identificator(&self) -> &str {
        self.signature.as_str()
    }

    pub(crate) fn add_end_year(self, end_year: Year, now: Clock) -> Result<Self> {
        let current_year = extract_year(now.timestamp);
        self.end_year.map_or(
            {
                let obj = Self::new(
                    self.signature,
                    self.name,
                    self.start_year,
                    Some(end_year),
                    self.weekly_schedule,
                );
                obj.invariant_held(current_year).map(|()| obj)
            },
            |_| Err(Error::EndYearExists),
        )
    }

    fn new(
        signature: Signature,
        name: String,
        start_year: Year,
        end_year: Option<Year>,
        weekly_schedule: WeeklyFrequency,
    ) -> Self {
        Self {
            signature,
            name,
            start_year,
            end_year,
            weekly_schedule,
        }
    }

    fn invariant_held(&self, current_year: Year) -> Result<()> {
        (self.start_year > current_year)
            .then_some(Err(Error::InvalidYear(
                "start_year cannot be in the future",
            )))
            .unwrap_or({
                self.end_year
                    .filter(|end| self.start_year > *end)
                    .map(|_| Err(Error::InvalidYear("start_year cannot be after end_year")))
                    .unwrap_or(Ok(()))
            })
    }

    fn published_on(&self, day_index: usize, year: Year) -> bool {
        self.start_year <= year
            && self.end_year.map_or(true, |end| end >= year)
            && self.weekly_schedule.published_on(day_index)
    }
}

#[derive(Deserialize)]
struct UncheckedNewspaper {
    signature: Signature,
    name: String,
    start_year: Year,
    end_year: Option<Year>,
    weekly_schedule: WeeklyFrequency,
}

impl UncheckedNewspaper {
    fn into_checked(self, now: Clock) -> Result<Newspaper> {
        let current_year = extract_year(now.timestamp);
        let obj = Newspaper::new(
            self.signature,
            self.name,
            self.start_year,
            self.end_year,
            self.weekly_schedule,
        );
        obj.invariant_held(current_year).map(|()| obj)
    }
}

impl TryFrom<UncheckedNewspaper> for Newspaper {
    type Error = Error;

    fn try_from(unchecked: UncheckedNewspaper) -> StdResult<Self, Self::Error> {
        unchecked.into_checked(Host::now())
    }
}

fn extract_year(millis: u64) -> Year {
    let datetime = DateTime::from_timestamp_millis(
        i64::try_from(millis).expect("u64 value is too large for i64"),
    )
    .expect("invalid timestamp");
    u16::try_from(datetime.year()).expect("year cannot be negative")
}

#[cfg(test)]
fn try_from_unchecked(unchecked: UncheckedNewspaper) -> Result<Newspaper> {
    unchecked.into_checked(MockHost::now())
}

impl From<Newspaper> for QueryNewspaperDTO {
    fn from(value: Newspaper) -> Self {
        Self::new(value.identificator().to_string(), value.name)
    }
}

pub(crate) fn newspapers_by_date<S, T>(date: Date) -> Result<ByteArray>
where
    S: Storage + Default,
    T: Time + Default,
{
    let year = date.year();
    let day = (date.day_of_week().number_from_monday() - 1) as usize;

    let published_newspapers: Vec<QueryNewspaperDTO> = S::default()
        .retrieve_range(
            &signature::SIGN.to_string(),
            &signature::next_letter(signature::SIGN).to_string(),
        )
        .into_iter()
        .filter_map(|ser_newspaper| {
            serde_json::from_slice::<UncheckedNewspaper>(&ser_newspaper)
                .map_err(Error::DeserializationFault)
                .and_then(|unchecked| unchecked.into_checked(T::now()))
                .ok()
                .and_then(|newspaper| newspaper.published_on(day, year).then(|| newspaper.into()))
        })
        .collect::<Vec<QueryNewspaperDTO>>();

    serde_json::to_vec(&published_newspapers).map_err(Error::SerializationFault)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        newspaper::{Date, QueryNewspaperDTO, Year},
        services::MockHost,
        Time,
    };

    use super::{Error, Newspaper, Result, UncheckedNewspaper};

    #[test]
    fn serialize() {
        let newspaper = Newspaper::new_unchecked(
            "В4667",
            "Орбита",
            1969,
            Some(1991),
            [false, false, false, false, false, true, false],
        );
        let serialized = serde_json::to_string(&newspaper).unwrap();

        assert_eq!(
            serialized,
            r#"{"signature":"В4667","name":"Орбита","start_year":1969,"end_year":1991,"weekly_schedule":[false,false,false,false,false,true,false]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"signature":"В1612","name":"Труд","start_year":1946,"end_year":null,"weekly_schedule":[true,true,true,true,true,true,true]}"#;

        let unchecked: UncheckedNewspaper =
            serde_json::from_str(json).expect("Failed to deserialize JSON");
        let checked: Newspaper = super::try_from_unchecked(unchecked)
            .expect("Failed to convert UncheckedNewspaper to Newspaper");
        let expected_dto = Newspaper::new_unchecked(
            "В1612",
            "Труд",
            1946,
            None,
            [true, true, true, true, true, true, true],
        );

        assert_eq!(expected_dto, checked);
    }

    #[test]
    fn newspapers_by_date() {
        let _storage = MockHost::default();

        //05.07.1987 was a sunday
        let publicated_1 = publicized_on(5, 7, 1987);
        let expected_1 = vec![QueryNewspaperDTO::new_test("В1612", "Труд")];
        assert_publication_eq(publicated_1, expected_1);

        //14.07.1990 was a saturday
        let publicated_2 = publicized_on(14, 7, 1990);
        let expected_2 = vec![
            QueryNewspaperDTO::new_test("В1612", "Труд"),
            QueryNewspaperDTO::new_test("В4667", "Орбита"),
        ];
        assert_publication_eq(publicated_2, expected_2);
        assert_publication_eq(
            vec![
                QueryNewspaperDTO::new_test("В4667", "Орбита"),
                QueryNewspaperDTO::new_test("В1612", "Труд"),
            ],
            vec![
                QueryNewspaperDTO::new_test("В1612", "Труд"),
                QueryNewspaperDTO::new_test("В4667", "Орбита"),
            ],
        );
    }

    #[test]
    fn existing_end_year() {
        let res = Newspaper::add_end_year(newspaper(), 1998, MockHost::now());
        let err = Error::EndYearExists;

        assert_err(res, err.to_string());
    }

    #[test]
    fn add_end_year() {
        let obj = Newspaper::new_unchecked(
            "В1616",
            "Народен спорт",
            1944,
            None,
            [true, false, false, true, false, true, false],
        );
        let res = Newspaper::add_end_year(obj, 1989, MockHost::now());
        assert_eq!(res.unwrap(), newspaper())
    }

    fn publicized_on(day: u16, month: u16, year: Year) -> Vec<QueryNewspaperDTO> {
        let res = super::newspapers_by_date::<MockHost, MockHost>(Date::new(day, month, year))
            .expect("Failed to retrieve newspapers published on the specified date");
        serde_json::from_slice(&res)
            .expect("Failed to deserialize the published newspapers from the result")
    }

    fn newspaper() -> Newspaper {
        Newspaper::new_unchecked(
            "В1616",
            "Народен спорт",
            1944,
            Some(1989),
            [true, false, false, true, false, true, false],
        )
    }

    fn assert_publication_eq(publicated: Vec<QueryNewspaperDTO>, expected: Vec<QueryNewspaperDTO>) {
        let publicated_set: HashSet<QueryNewspaperDTO> = publicated.into_iter().collect();
        let expected_set: HashSet<QueryNewspaperDTO> = expected.into_iter().collect();
        assert_eq!(publicated_set, expected_set);
    }

    fn assert_err(r: Result<Newspaper>, msg: String) {
        assert!(r.is_err());
        assert!(r.expect_err("expected an error").to_string().contains(&msg))
    }
}

#[cfg(test)]
mod test_invariant {
    use crate::services::mock_host;

    use super::{Newspaper, Result, Year};

    const PUBLICATED_ON: [bool; 7] = [true, false, false, true, false, true, false];
    const CURRENT_YEAR: Year = mock_host::CURRENT_YEAR;

    #[test]
    fn start_year_in_future() {
        let res =
            Newspaper::new_unchecked("В1111", "Добро утро", CURRENT_YEAR + 1, None, PUBLICATED_ON)
                .invariant_held(CURRENT_YEAR);
        assert_err(res, "start_year cannot be in the future");
    }

    #[test]
    fn start_year_before_end() {
        let res = Newspaper::new_unchecked(
            "В9999",
            "Лека нощ",
            CURRENT_YEAR,
            Some(CURRENT_YEAR - 5),
            PUBLICATED_ON,
        )
        .invariant_held(CURRENT_YEAR);
        assert_err(res, "start_year cannot be after end_year");
    }

    #[test]
    fn the_same_start_and_end_year() {
        let res = Newspaper::new_unchecked("В5555", "Добра среща", 1978, Some(1978), PUBLICATED_ON)
            .invariant_held(CURRENT_YEAR);
        assert!(res.is_ok())
    }

    fn assert_err(r: Result<()>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
