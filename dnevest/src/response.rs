use serde::{Deserialize, Serialize};

use crate::{bindings::ByteArray, services::ServiceError};

#[derive(Serialize, Debug, Deserialize)]
pub enum Event<'a> {
    NewspaperCreated(&'a str),
}

impl<'a> Event<'a> {
    pub(crate) fn serialize_event(&self) -> Result<ByteArray, ServiceError> {
        serde_json::to_vec(&self).map_err(|_| ServiceError::SerializationFault)
    }
}
