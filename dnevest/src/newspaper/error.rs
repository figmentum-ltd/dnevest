use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("[Newspaper] Error parsing date {0}")]
    DateParsing(chrono::ParseError),

    #[error("[Newspaper] Invalid json {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("[Newspaper] Signature does not match the required pattern.")]
    SignatureMismatch,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
