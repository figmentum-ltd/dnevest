use serde::{Deserialize, Serialize};

use crate::{bindings::ByteArray, services::ServiceError, HostImports};

use super::{dto::QueryNewspaperDTO, frequency::WeeklyFrequency, signature::Signature, Date, Year};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct Newspaper {
    signature: Signature,
    name: String,
    start_year: Year,
    end_year: Option<Year>,
    weekly_shedule: WeeklyFrequency,
}

impl Newspaper {
    pub(crate) fn new(
        signature: Signature,
        name: String,
        start_year: Year,
        end_year: Option<Year>,
        published_on: WeeklyFrequency,
    ) -> Self {
        Self {
            signature,
            name,
            start_year,
            end_year,
            weekly_shedule: published_on,
        }
    }

    pub(crate) fn identificator(&self) -> &str {
        &self.signature.signature()
    }

    fn published_on(&self, day_index: usize, year: Year) -> bool {
        self.start_year <= year
            && self.end_year.map_or(true, |end| end >= year)
            && self.weekly_shedule.published_on(day_index)
    }
}

impl From<&Newspaper> for QueryNewspaperDTO {
    fn from(value: &Newspaper) -> Self {
        Self::new(value.identificator(), &value.name)
    }
}

pub(crate) fn newspapers_by_date<H: HostImports>(
    host: &mut H,
    date: Date,
) -> Result<ByteArray, ServiceError> {
    let ser_newspapers = host.retrieve_range("В", "Г");
    self::deserialize_newspapers(ser_newspapers)
        .and_then(|newspapers| self::published_on(date, &newspapers))
}

fn published_on(date: Date, newspapers: &Vec<Newspaper>) -> Result<ByteArray, ServiceError> {
    let year = date.year();
    let day = (date.day_of_week().number_from_monday() - 1) as usize;

    let published_newspapers: Vec<QueryNewspaperDTO> = newspapers
        .into_iter()
        .filter(|newspaper| newspaper.published_on(day, year))
        .map(|published_newspaper| QueryNewspaperDTO::from(published_newspaper))
        .collect();

    serde_json::to_vec(&published_newspapers).map_err(|_| ServiceError::SerializationFault)
}

fn deserialize_newspapers(serialized: Vec<ByteArray>) -> Result<Vec<Newspaper>, ServiceError> {
    serialized
        .into_iter()
        .map(|ser_newspaper| {
            serde_json::from_slice::<Newspaper>(&ser_newspaper)
                .map_err(|_| ServiceError::DeserializationFault)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        newspaper::{Date, QueryNewspaperDTO, Signature, WeeklyFrequency, Year},
        services::MockHost,
        HostImports,
    };

    use super::Newspaper;

    #[test]
    fn serialize() {
        let weekly_shedule = WeeklyFrequency::new([false, false, false, false, false, true, false]);
        let newspaper = Newspaper::new(
            Signature::new("В4667"),
            "Орбита".to_string(),
            1969,
            Some(1991),
            weekly_shedule,
        );
        let serialized = serde_json::to_string(&newspaper).unwrap();

        assert_eq!(
            serialized,
            r#"{"signature":"В4667","name":"Орбита","start_year":1969,"end_year":1991,"weekly_shedule":[false,false,false,false,false,true,false]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"signature":"В1612","name":"Труд","start_year":1946,"end_year":null,"weekly_shedule":[true,true,true,true,true,true,true]}"#;

        let deserialized: Newspaper =
            serde_json::from_str(json).expect("Failed to deserialize JSON");
        let expected_dto = Newspaper::new(
            Signature::new("В1612"),
            "Труд".to_string(),
            1946,
            None,
            WeeklyFrequency::new([true, true, true, true, true, true, true]),
        );

        assert_eq!(expected_dto, deserialized);
    }

    #[test]
    fn newspapers_by_date() {
        let mut host = MockHost::new();

        //05.07.1987 was a sunday
        let publicated_1 = publicized_on(5, 7, 1987, &mut host);
        let expected_1 = vec![QueryNewspaperDTO::new("В1612", "Труд")];
        assert_eq!(publicated_1, expected_1);

        //14.07.1990 was a saturday
        let publicated_2 = publicized_on(14, 7, 1990, &mut host);
        let expected_2 = vec![
            QueryNewspaperDTO::new("В4667", "Орбита"),
            QueryNewspaperDTO::new("В1612", "Труд"),
        ];
        assert_eq!(publicated_2, expected_2);
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
}
