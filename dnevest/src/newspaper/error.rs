use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("[Newspaper] Error parsing date {0}")]
    DateParsing(chrono::ParseError),

    #[error("[Newspaper] Signature does not match the required pattern.")]
    SignatureMismatch,
}

impl Error {
    #[cfg(test)]
    pub(crate) fn to_string(&self) -> String {
        match self {
            Error::DateParsing(err) => format!("[Newspaper] Error parsing date: {}", err),
            Error::SignatureMismatch => {
                "[Newspaper] Signature does not match the required pattern.".to_string()
            }
        }
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
