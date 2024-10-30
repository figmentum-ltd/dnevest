use std::result::Result as StdResult;

use crate::{
    bindings::{self, ByteArray},
    newspaper::{self, Date, Newspaper},
    response::Event,
    HostImports,
};

mod error;

pub(super) use error::Error as ServiceError;

pub fn create_newspaper<H: HostImports>(
    host: &mut H,
    input: Newspaper,
) -> StdResult<bindings::Event, ByteArray> {
    self::new_newspaper(host, input).or_else(|error| error::serialize_errors(vec![error]))
}

pub fn newspapers_by_date(date: Date) -> Result<ByteArray, ByteArray> {
    newspaper::newspapers_by_date(date).or_else(|error| error::serialize_errors(vec![error]))
}

// TODO! - do we need 'newspaper' to pe present in every name
fn new_newspaper<H: HostImports>(
    host: &mut H,
    newspaper: Newspaper,
) -> StdResult<bindings::Event, ServiceError> {
    let signature = newspaper.identificator();
    let serialized_newspaper =
        serde_json::to_vec(&newspaper).map_err(|_| ServiceError::SerializationFault)?;

    host.persist(signature, &serialized_newspaper);

    let serialized_event = Event::NewspaperCreated(signature).serialize_event()?;
    Ok(bindings::Event {
        id: "dnevest_n_n".to_string(),
        content: serialized_event,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        bindings::ByteArray,
        newspaper::{Newspaper, Signature, WeeklyFrequency},
        HostImports,
    };

    struct MockHost {
        store: HashMap<String, ByteArray>,
    }

    impl MockHost {
        fn new() -> Self {
            Self {
                store: HashMap::new(),
            }
        }
    }

    impl HostImports for MockHost {
        fn persist(&mut self, key: &str, req: &ByteArray) {
            self.store.insert(key.to_string(), req.clone());
        }
    }

    #[test]
    fn create() {
        let mut host = MockHost::new();
        let newspaper = Newspaper::new(
            Signature::new("В4667"),
            "Орбита".to_string(),
            1969,
            Some(1991),
            WeeklyFrequency::new([false, false, false, false, false, true, false]),
        );

        let res = super::create_newspaper(&mut host, newspaper);
        assert_eq!((&res.unwrap()).id, "dnevest_n_n".to_string());
    }
}
