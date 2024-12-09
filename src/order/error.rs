use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("[order] The card number does not exist.")]
    InvalidCard,

    #[error("[order] Problem while deserialization: {0}")]
    DeserializationFault(serde_json::Error),

    #[error("[order] {0}.")]
    NotFound(String),

    #[error("[order] Invalid waybill invariant: {0}")]
    InvalidDelivery(&'static str),

    #[error("[order] {0}")]
    InvalidOrder(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
