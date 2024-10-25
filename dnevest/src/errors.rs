use thiserror::Error;

use crate::bindings::ByteArray;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Invalid json in command request: {0}")]
    InvalidCommandRequest(serde_json::Error),

    #[error("Invalid json in query request: {0}")]
    InvalidQueryRequest(serde_json::Error),
}

impl Error {
    pub(crate) fn to_byte_array(&self) -> ByteArray {
        let error = match self {
            Error::InvalidCommandRequest(err) => {
                format!("Invalid json in command request: {}", err)
            }
            Error::InvalidQueryRequest(err) => format!("Invalid json in query request: {}", err),
        };
        serde_json::to_vec(&error).unwrap_or(b"Error occurs while serializing errors".to_vec())
    }
}
