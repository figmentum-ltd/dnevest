use super::{dto::NewspaperDTO, frequency::WeeklyFrequency, signature::Signature};

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
}

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
