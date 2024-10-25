use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    // #[error("[Newspaper] Error parsing date {0}")]
    // DateParsing(chrono::ParseError),
    #[error("[Newspaper] Invalid json {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("[Newspaper] Signature does not match the required pattern.")]
    SignatureMismatch,
}

impl Error {
    pub(crate) fn to_string(&self) -> String {
        match self {
            // Error::DateParsing(err) => format!("[Newspaper] Error parsing date: {}", err),
            Error::JsonError(err) => format!("[Newspaper] Invalid JSON: {}", err),
            Error::SignatureMismatch => {
                "[Newspaper] Signature does not match the required pattern.".to_string()
            }
        }
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
