use serde::{Deserialize, Serialize};

use super::{dto::NewspaperDTO, frequency::WeeklyFrequency, signature::Signature};

#[derive(Clone, Serialize, Deserialize)]
#[serde(try_from = "NewspaperDTO", into = "NewspaperDTO")]
pub(crate) struct Newspaper {
    signature: Signature,
    name: String,
    start_year: u16,
    end_year: Option<u16>,
    published_on: WeeklyFrequency,
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
            published_on,
        }
    }

    // pub(crate) fn signature(&self) -> &Signature {
    //     &self.signature
    // }

    pub(crate) fn signature_str(&self) -> &str {
        self.signature.signature()
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
            value.published_on,
        )
    }
}

pub(crate) fn load_newspapers() -> Vec<Newspaper> {
    vec![
        Newspaper::new(
            Signature::new("В4667"),
            "Орбита".to_string(),
            1969,
            Some(1991),
            WeeklyFrequency::new([false, false, false, false, false, true, false]),
        ),
        Newspaper::new(
            Signature::new("В1616"),
            "Народен спор".to_string(),
            1944,
            Some(1989),
            WeeklyFrequency::new([true, false, false, true, false, true, false]),
        ),
        Newspaper::new(
            Signature::new("В1612"),
            "Труд".to_string(),
            1946,
            None,
            WeeklyFrequency::new([true, true, true, true, true, true, true]),
        ),
    ]
}
