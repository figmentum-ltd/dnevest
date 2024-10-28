use thiserror::Error;

use crate::bindings::ByteArray;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Invalid json in request: {0}")]
    InvalidRequest(serde_json::Error),
}

impl Error {
    pub(crate) fn to_byte_array(&self) -> ByteArray {
        let error = match self {
            Error::InvalidRequest(err) => format!("Invalid json in request: {}", err),
        };
        serde_json::to_vec(&error).unwrap_or(b"Error occurs while serializing errors".to_vec())
    }
}
