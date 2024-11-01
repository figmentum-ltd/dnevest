mod date;
mod dto;
mod error;
mod frequency;
mod newspaper;
mod signature;

pub(crate) use date::Date;
pub(crate) use dto::QueryNewspaperDTO;
pub(super) use error::Error;
pub(super) use frequency::WeeklyFrequency;
pub(super) use newspaper::{newspapers_by_date, Newspaper};
pub(super) use signature::Signature;

type Year = u32;
