use serde::{Deserialize, Serialize};

use crate::{bindings::ByteArray, services::ServiceError};

#[derive(Serialize, Debug, Deserialize)]
pub enum Event {
    NewspaperCreated(String),
    AddedEndYear(String),
}

impl Event {
    pub(crate) fn newspaper_created(signature: &str) -> Self {
        Event::NewspaperCreated(signature.to_string())
    }

    pub(crate) fn added_end_year(signature: &str) -> Self {
        Event::AddedEndYear(signature.to_string())
    }

    pub(crate) fn serialize(&self) -> Result<ByteArray, ServiceError> {
        serde_json::to_vec(&self).map_err(ServiceError::SerializationFault)
    }
}
