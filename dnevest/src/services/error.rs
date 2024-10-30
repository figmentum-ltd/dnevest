use std::result::Result as StdResult;
use thiserror::Error;

use crate::bindings::ByteArray;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Problem while serialization")]
    SerializationFault,
}

pub(super) fn serialize_errors<T>(errors: Vec<Error>) -> StdResult<T, ByteArray> {
    let serialized_errors: Vec<String> = errors
        .into_iter()
        .map(|error| match error {
            Error::SerializationFault => "Problem while serialization".to_string(),
        })
        .collect();

    let serialized_result = serde_json::to_vec(&serialized_errors)
        .unwrap_or(b"Error occurs while serializing errors".to_vec());

    Err(serialized_result)
}
