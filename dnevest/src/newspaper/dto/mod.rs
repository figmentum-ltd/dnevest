use serde::{Deserialize, Serialize};

mod query_newspaper;

use super::{error::Error, frequency::WeeklyFrequency, Newspaper, Signature};

pub(crate) use query_newspaper::QueryNewspaperDTO;

//TODO! - remove DTO-s
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct NewspaperDTO {
    signature: Signature,
    name: String,
    start_year: u16,
    end_year: Option<u16>,
    weekly_shedule: WeeklyFrequency,
}

impl NewspaperDTO {
    pub(crate) fn new(
        signature: Signature,
        name: String,
        start_year: u16,
        end_year: Option<u16>,
        weekly_shedule: WeeklyFrequency,
    ) -> Self {
        Self {
            signature,
            name: name,
            start_year,
            end_year,
            weekly_shedule,
        }
    }
}

impl TryFrom<NewspaperDTO> for Newspaper {
    type Error = Error;

    fn try_from(dto: NewspaperDTO) -> Result<Self, Self::Error> {
        Ok(Newspaper::new(
            dto.signature,
            dto.name,
            dto.start_year,
            dto.end_year,
            dto.weekly_shedule,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::newspaper::{Newspaper, Signature, WeeklyFrequency};

    use super::NewspaperDTO;

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
        let dto: NewspaperDTO = newspaper.try_into().unwrap();
        let serialized = serde_json::to_string(&dto).unwrap();

        assert_eq!(
            serialized,
            r#"{"signature":"В4667","name":"Орбита","start_year":1969,"end_year":1991,"weekly_shedule":[false,false,false,false,false,true,false]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"signature":"В1612","name":"Труд","start_year":1946,"end_year":null,"weekly_shedule":[true,true,true,true,true,true,true]}"#;

        let deserialized: NewspaperDTO =
            serde_json::from_str(json).expect("Failed to deserialize JSON");
        let expected_dto = NewspaperDTO::new(
            Signature::new("В1612"),
            "Труд".to_string(),
            1946,
            None,
            WeeklyFrequency::new([true, true, true, true, true, true, true]),
        );

        assert_eq!(expected_dto, deserialized);
    }
}
