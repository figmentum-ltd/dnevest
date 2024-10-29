use std::{collections::HashMap, result::Result as StdResult};

use crate::{
    bindings::{self, ByteArray},
    newspaper::{self, Date, Newspaper, NewspaperDTO},
    response::Event,
    Component, HostImports,
};

mod error;

pub(super) use error::Error as ServiceError;

pub fn create_newspaper<H: HostImports>(
    host: &mut H,
    input: NewspaperDTO,
) -> StdResult<bindings::Event, ByteArray> {
    input
        .try_into()
        .map_err(|err| ServiceError::InvalidNewspaper(err))
        .and_then(|newspaper| self::new_newspaper(host, newspaper))
        .or_else(|error| error::serialize_errors(vec![error]))
}

pub fn newspapers_by_date(date: Date) -> Result<ByteArray, ByteArray> {
    newspaper::newspapers_by_date(date).or_else(|error| error::serialize_errors(vec![error]))
}

// TODO! - do we need 'newspaper' to pe present in every name
fn new_newspaper<H: HostImports>(
    host: &mut H,
    newspaper: Newspaper,
) -> StdResult<bindings::Event, ServiceError> {
    // TODO! remove the cloning
    let obj = newspaper.clone();
    let signature = obj.signature_str();
    let dto: NewspaperDTO = newspaper.into();

    let serialized_newspaper =
        serde_json::to_vec(&dto).map_err(|_| ServiceError::SerializationFault)?;

    host.persist(signature, &serialized_newspaper);

    let serialized_event = Event::NewspaperCreated(signature).serialize_event()?;
    Ok(bindings::Event {
        id: "dnevest_n_n".to_string(),
        content: serialized_event,
    })
}

struct MockHost {
    store: HashMap<String, ByteArray>,
}

impl MockHost {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&ByteArray> {
        self.store.get(key)
    }
}

impl HostImports for MockHost {
    fn persist(&mut self, key: &str, req: &ByteArray) {
        self.store.insert(key.to_string(), req.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bindings::{self, ByteArray},
        newspaper::{NewspaperDTO, Signature, WeeklyFrequency},
    };

    use super::{MockHost, ServiceError};

    #[test]
    fn create() {
        let mut host = MockHost::new();
        let dto = NewspaperDTO::new(
            Signature::new("В4667"),
            "Орбита".to_string(),
            1969,
            Some(1991),
            WeeklyFrequency::new([false, false, false, false, false, true, false]),
        );
        let serialized_newspaper = serde_json::to_vec(&dto).unwrap();

        let res = super::create_newspaper(&mut host, dto);

        assert_eq!(&serialized_newspaper, host.get("В4667").unwrap())
    }

    #[test]
    fn err_in_creation() {
        let mut host = MockHost::new();
        let dto = NewspaperDTO::new(
            Signature::new("B4667"),
            "Орбита".to_string(),
            1969,
            Some(1991),
            WeeklyFrequency::new([false, false, false, false, false, true, false]),
        );

        let res = super::create_newspaper(&mut host, dto);

        assert!(res.is_err())
    }
}
