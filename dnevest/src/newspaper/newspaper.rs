use serde::{Deserialize, Serialize};

use crate::{bindings::ByteArray, services::ServiceError};

use super::{
    dto::{NewspaperDTO, QueryNewspaperDTO},
    frequency::WeeklyFrequency,
    signature::Signature,
    // Date,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(try_from = "NewspaperDTO", into = "NewspaperDTO")]
pub(crate) struct Newspaper {
    signature: Signature,
    name: String,
    start_year: u16,
    end_year: Option<u16>,
    weekly_shedule: WeeklyFrequency,
}

impl Newspaper {
    pub(super) fn new(
        signature: Signature,
        name: String,
        start_year: u16,
        end_year: Option<u16>,
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

    // pub(crate) fn signature(&self) -> &Signature {
    //     &self.signature
    // }

    pub(crate) fn signature_str(&self) -> &str {
        self.signature.signature()
    }

    fn published_on(&self, day_index: usize) -> bool {
        self.weekly_shedule.published_on(day_index)
    }
}
//TODO! create new method 'identificator' ~ signature()

impl From<Newspaper> for NewspaperDTO {
    fn from(value: Newspaper) -> Self {
        Self::new(
            value.signature.into(),
            value.name,
            value.start_year,
            value.end_year,
            value.weekly_shedule,
        )
    }
}

// impl From<Newspaper> for QueryNewspaperDTO {
//     fn from(value: Newspaper) -> Self {
//         Self::new(value.signature_str().to_string(), value.name)
//     }
// }

// pub(crate) fn newspapers_by_date(date: Date) -> Result<ByteArray, ServiceError> {
//     let newspapers = self::load_newspapers();
//     self::published_on(date, newspapers)
// }

// fn published_on(date: Date, newspapers: Vec<Newspaper>) -> Result<ByteArray, ServiceError> {
//     let day = (date.day_of_week().number_from_monday() - 1) as usize;

//     let published_newspapers: Vec<QueryNewspaperDTO> = newspapers
//         .into_iter()
//         .filter(|newspaper| newspaper.published_on(day))
//         .map(|published_newspaper| QueryNewspaperDTO::from(published_newspaper))
//         .collect();

//     serde_json::to_vec(&published_newspapers).map_err(|_| ServiceError::SerializationFault)
// }

// fn load_newspapers() -> Vec<Newspaper> {
//     vec![
//         Newspaper::new(
//             Signature::new("В4667"),
//             "Орбита".to_string(),
//             1969,
//             Some(1991),
//             WeeklyFrequency::new([false, false, false, false, false, true, false]),
//         ),
//         Newspaper::new(
//             Signature::new("В1616"),
//             "Народен спор".to_string(),
//             1944,
//             Some(1989),
//             WeeklyFrequency::new([true, false, false, true, false, true, false]),
//         ),
//         Newspaper::new(
//             Signature::new("В1612"),
//             "Труд".to_string(),
//             1946,
//             None,
//             WeeklyFrequency::new([true, true, true, true, true, true, true]),
//         ),
//     ]
// }
