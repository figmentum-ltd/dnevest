use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub(crate) enum Error {
    #[error("[Newspaper] Error parsing date {0}")]
    DateParsing(chrono::ParseError),

    #[error("[Newspaper] Signature does not match the required pattern.")]
    SignatureMismatch,

    #[error("Invalid year: {0}")]
    InvalidYear(&'static str),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
