use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub(crate) enum Error {
    #[error("[Newspaper] Invalid regex pattern.")]
    InvalidRegexPattern,

    #[error("[Newspaper] Signature does not match the required pattern.")]
    SignatureMismatch,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;