use thiserror::Error;

use crate::{bindings::ByteArray, newspaper};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Problem while serialization: {0}")]
    SerializationFault(serde_json::Error),

    #[error("Cannot create the newspaper because this signature already exists")]
    DuplicateSignature,

    #[error("Newspaper domain error: {0}")]
    DomainError(#[from] newspaper::Error),

    #[error("Problem while deserialization: {0}")]
    DeserializationFault(serde_json::Error),

    #[error("{0}")]
    NotFound(&'static str),

    #[error("Cannot create the order because it already exists")]
    DuplicateOrder,
}

impl Error {
    pub(crate) fn serialize(&self) -> ByteArray {
        serde_json::to_vec(&self.to_string())
            .unwrap_or(b"Error occurs while serializing error".to_vec())
    }
}
