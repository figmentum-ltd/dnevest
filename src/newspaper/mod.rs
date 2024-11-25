mod date;
mod dto;
mod error;
mod frequency;
mod newspaper;
mod signature;

pub(crate) use date::Date;
pub(crate) use error::Error;
pub(super) use newspaper::{newspapers_by_date, Newspaper};

#[cfg(test)]
pub(crate) use dto::QueryNewspaperDTO;
pub(super) use frequency::WeeklyFrequency;
pub(super) use signature::Signature;

pub(crate) type Year = u16;
