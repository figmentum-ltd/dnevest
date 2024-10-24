use serde::{Deserialize, Serialize};

mod date;
mod signature;

use super::{error::Error, frequency::WeeklyFrequency, Newspaper};

pub(crate) use date::DateDTO;
pub(crate) use signature::SignatureDTO;

//TODO! - remove DTO-s
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct NewspaperDTO {
    signature: SignatureDTO,
    name: String,
    start_year: u16,
    end_year: Option<u16>,
    published_on: WeeklyFrequency,
}

impl NewspaperDTO {
    pub(super) fn new(
        signature: SignatureDTO,
        name: String,
        start_year: u16,
        end_year: Option<u16>,
        published_on: WeeklyFrequency,
    ) -> Self {
        Self {
            signature,
            name: name,
            start_year,
            end_year,
            published_on,
        }
    }
}

impl TryFrom<NewspaperDTO> for Newspaper {
    type Error = Error;

    fn try_from(dto: NewspaperDTO) -> Result<Self, Self::Error> {
        dto.signature.try_into().and_then(|signature| {
            Ok(Newspaper::new(
                signature,
                dto.name,
                dto.start_year,
                dto.end_year,
                dto.published_on,
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::newspaper::{Newspaper, Signature, WeeklyFrequency};

    use super::{NewspaperDTO, SignatureDTO};

    #[test]
    fn serialize() {
        let published_on = WeeklyFrequency::new([false, false, false, false, false, true, false]);
        let newspaper = Newspaper::new(
            Signature::new("В4667"),
            "Орбита".to_string(),
            1969,
            Some(1991),
            published_on,
        );
        let dto: NewspaperDTO = newspaper.try_into().unwrap();
        let serialized = serde_json::to_string(&dto).unwrap();

        assert_eq!(
            serialized,
            r#"{"signature":"В4667","name":"Орбита","start_year":1969,"end_year":1991,"published_on":[false,false,false,false,false,true,false]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"signature":"В1612","name":"Труд","start_year":1946,"end_year":null,"published_on":[true,true,true,true,true,true,true]}"#;

        let deserialized: NewspaperDTO =
            serde_json::from_str(json).expect("Failed to deserialize JSON");
        let expected_dto = NewspaperDTO::new(
            SignatureDTO("В1612".to_string()),
            "Труд".to_string(),
            1946,
            None,
            WeeklyFrequency::new([true, true, true, true, true, true, true]),
        );

        assert_eq!(expected_dto, deserialized);
    }
}
