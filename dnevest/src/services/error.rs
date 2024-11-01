use thiserror::Error;

use crate::bindings::ByteArray;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Problem while serialization: {0}")]
    SerializationFault(serde_json::Error),

    #[error("Problem while deserialization: {0}")]
    DeserializationFault(serde_json::Error),
}

impl Error {
    pub(crate) fn serialize(&self) -> ByteArray {
        serde_json::to_vec(&self.to_string())
            .unwrap_or(b"Error occurs while serializing error".to_vec())
    }
}
