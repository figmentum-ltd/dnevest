use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("[Newspaper] Signature does not match the required pattern.")]
    SignatureMismatch,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
