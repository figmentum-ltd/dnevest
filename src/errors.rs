use thiserror::Error;

use crate::bindings::ByteArray;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Invalid json in request: {0}")]
    InvalidRequest(serde_json::Error),
}

impl Error {
    pub(crate) fn serialize(&self) -> ByteArray {
        serde_json::to_vec(&self.to_string())
            .unwrap_or(b"Error occurs while serializing error".to_vec())
    }
}
