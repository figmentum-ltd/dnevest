use std::result::Result as StdResult;

use crate::{
    bindings::{self, ByteArray},
    newspaper::{self, Date, Newspaper},
    response::Event,
    Storage,
};

mod error;

#[cfg(test)]
pub(crate) mod mock_host;

pub(super) use error::Error as ServiceError;
#[cfg(test)]
pub(crate) use mock_host::MockHost;

pub fn create_newspaper<A: Storage>(
    adapter: &mut A,
    input: Newspaper,
) -> StdResult<bindings::Event, ByteArray> {
    self::new_newspaper(adapter, input).map_err(|error| error.serialize())
}

pub fn newspapers_by_date<A: Storage>(
    adapter: &mut A,
    date: Date,
) -> StdResult<ByteArray, ByteArray> {
    newspaper::newspapers_by_date(adapter, date)
        .map_err(|error| ServiceError::DomainError(error).serialize())
}

// TODO! - do we need 'newspaper' to pe present in every name
fn new_newspaper<A: Storage>(
    adapter: &mut A,
    newspaper: Newspaper,
) -> StdResult<bindings::Event, ServiceError> {
    let signature = newspaper.identificator();
    adapter
        .retrieve(signature)
        .map(|_| Err(ServiceError::DuplicateSignature))
        .unwrap_or({
            let serialized_newspaper =
                serde_json::to_vec(&newspaper).map_err(ServiceError::SerializationFault)?;

            adapter.persist(signature, &serialized_newspaper);

            let serialized_event = Event::NewspaperCreated(signature).serialize()?;
            Ok(bindings::Event {
                id: "dnevest_n_n".to_string(),
                content: serialized_event,
            })
        })
}

#[cfg(test)]
mod tests {
    use crate::{
        bindings,
        newspaper::Newspaper,
        services::{MockHost, ServiceError},
    };

    #[test]
    fn create() {
        let mut adapter = MockHost::new();
        let newspaper = newspaper();

        let res = super::create_newspaper(&mut adapter, newspaper);
        assert_eq!((res.unwrap()).id, "dnevest_n_n".to_string());
    }

    #[test]
    fn dublicate_signature() {
        let mut adapter = MockHost::new();

        let res = super::new_newspaper(&mut adapter, newspaper());
        assert_eq!((res.unwrap()).id, "dnevest_n_n".to_string());

        let err = super::new_newspaper(&mut adapter, newspaper());
        assert_err(
            err,
            "Cannot create the newspaper because this signature already exists",
        );
    }

    fn newspaper() -> Newspaper {
        Newspaper::new_unchecked(
            "В1905",
            "Поглед",
            1966,
            Some(1996),
            [true, false, false, false, false, false, false],
        )
    }

    fn assert_err(r: Result<bindings::Event, ServiceError>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
