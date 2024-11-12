use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::result;

use crate::{bindings::ByteArray, HostImports};

use super::{
    dto::QueryNewspaperDTO,
    error::{Error, Result},
    frequency::WeeklyFrequency,
    signature::{self, Signature},
    Date, Year,
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
        Self {
            signature: Signature::new(signature),
            name: name.to_string(),
            start_year,
            end_year,
            weekly_schedule: WeeklyFrequency::new(publicated_on),
        }
    }

    pub(crate) fn identificator(&self) -> &str {
        self.signature.signature()
    }

    fn invariant_held(&self) -> Result<()> {
        // TODO!- find a way to get current year without using std
        // old: let current_year: Year = Utc::now().year(); -> where ::now requires std
        let current_year: Year = 2024;

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

impl TryFrom<UncheckedNewspaper> for Newspaper {
    type Error = Error;

    fn try_from(unchecked: UncheckedNewspaper) -> result::Result<Self, Self::Error> {
        let obj = Self {
            signature: unchecked.signature,
            name: unchecked.name,
            start_year: unchecked.start_year,
            end_year: unchecked.end_year,
            weekly_schedule: unchecked.weekly_schedule,
        };
        obj.invariant_held().map(|()| obj)
    }
}

impl From<Newspaper> for QueryNewspaperDTO {
    fn from(value: Newspaper) -> Self {
        Self::new(value.identificator().to_string(), value.name)
    }
}

pub(crate) fn newspapers_by_date<H: HostImports>(host: &mut H, date: Date) -> Result<ByteArray> {
    let year = date.year();
    let day = (date.day_of_week().number_from_monday() - 1) as usize;

    let published_newspapers: Vec<QueryNewspaperDTO> = host
        .retrieve_range(
            &signature::SIGN.to_string(),
            &signature::next_letter(signature::SIGN).to_string(),
        )
        .into_iter()
        .filter_map(|ser_newspaper| {
            serde_json::from_slice::<Newspaper>(&ser_newspaper)
                .map_err(Error::DeserializationFault)
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
        HostImports,
    };

    use super::Newspaper;

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

        let deserialized: Newspaper =
            serde_json::from_str(json).expect("Failed to deserialize JSON");
        let expected_dto = Newspaper::new_unchecked(
            "В1612",
            "Труд",
            1946,
            None,
            [true, true, true, true, true, true, true],
        );

        assert_eq!(expected_dto, deserialized);
    }

    #[test]
    fn newspapers_by_date() {
        let mut host = MockHost::with_newspapers();

        //05.07.1987 was a sunday
        let publicated_1 = publicized_on(5, 7, 1987, &mut host);
        let expected_1 = vec![QueryNewspaperDTO::new_test("В1612", "Труд")];
        assert_publication_eq(publicated_1, expected_1);

        //14.07.1990 was a saturday
        let publicated_2 = publicized_on(14, 7, 1990, &mut host);
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

    fn publicized_on<H: HostImports>(
        day: u32,
        month: u32,
        year: Year,
        host: &mut H,
    ) -> Vec<QueryNewspaperDTO> {
        let res = super::newspapers_by_date(host, Date::new(day, month, year))
            .expect("Failed to retrieve newspapers published on the specified date");
        serde_json::from_slice(&res)
            .expect("Failed to deserialize the published newspapers from the result")
    }

    fn assert_publication_eq(publicated: Vec<QueryNewspaperDTO>, expected: Vec<QueryNewspaperDTO>) {
        let publicated_set: HashSet<QueryNewspaperDTO> = publicated.into_iter().collect();
        let expected_set: HashSet<QueryNewspaperDTO> = expected.into_iter().collect();
        assert_eq!(publicated_set, expected_set);
    }
}

#[cfg(test)]
mod test_invariant {
    use crate::newspaper::Year;

    use super::{Newspaper, Result};

    const CURRENT_YEAR: Year = 2024;
    const PUBLICATED_ON: [bool; 7] = [true, false, false, true, false, true, false];

    #[test]
    fn start_year_in_future() {
        let newspaper =
            Newspaper::new_unchecked("В1111", "Добро утро", CURRENT_YEAR + 1, None, PUBLICATED_ON)
                .invariant_held();
        assert_err(newspaper, "start_year cannot be in the future");
    }

    #[test]
    fn start_year_before_end() {
        let newspaper = Newspaper::new_unchecked(
            "В9999",
            "Лека нощ",
            CURRENT_YEAR,
            Some(CURRENT_YEAR - 5),
            PUBLICATED_ON,
        )
        .invariant_held();
        assert_err(newspaper, "start_year cannot be after end_year");
    }

    fn assert_err(r: Result<()>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
