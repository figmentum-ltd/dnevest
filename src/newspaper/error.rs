use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("[Newspaper] Error parsing date {0}")]
    DateParsing(chrono::ParseError),

    #[error("[Newspaper] Signature does not match the required pattern.")]
    SignatureMismatch,

    #[error("Invalid year: {0}")]
    InvalidYear(&'static str),

    #[error("[Newspaper] Problem while deserialization: {0}")]
    DeserializationFault(serde_json::Error),

    #[error("[Newspaper] Problem while serialization: {0}")]
    SerializationFault(serde_json::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
