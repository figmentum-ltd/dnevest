mod date;
mod dto;
mod error;
mod frequency;
mod newspaper;
mod signature;

// pub(crate) use date::Date;
// pub(crate) use dto::{DateDTO, NewspaperDTO, QueryNewspaperDTO};
pub(crate) use dto::{NewspaperDTO, SignatureDTO};
pub(super) use error::Error;
pub(super) use frequency::WeeklyFrequency;
pub(super) use newspaper::{
    Newspaper,
    // newspapers_by_date
};
pub(super) use signature::Signature;
