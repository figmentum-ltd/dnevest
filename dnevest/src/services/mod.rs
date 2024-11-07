use std::result::Result as StdResult;

use crate::{
    bindings::{self, ByteArray},
    newspaper::{self, Date, Newspaper},
    response::Event,
    HostImports,
};

mod error;
mod mock_host;

pub(super) use error::Error as ServiceError;
#[cfg(test)]
pub(crate) use mock_host::MockHost;

pub fn create_newspaper<H: HostImports>(
    host: &mut H,
    input: Newspaper,
) -> StdResult<bindings::Event, ByteArray> {
    self::new_newspaper(host, input).map_err(|error| error.serialize())
}

pub fn newspapers_by_date<H: HostImports>(
    host: &mut H,
    date: Date,
) -> StdResult<ByteArray, ByteArray> {
    newspaper::newspapers_by_date(host, date).map_err(|error| error.serialize())
}

// TODO! - do we need 'newspaper' to pe present in every name
fn new_newspaper<H: HostImports>(
    host: &mut H,
    newspaper: Newspaper,
) -> StdResult<bindings::Event, ServiceError> {
    let signature = newspaper.identificator();
    host.retrieve(signature)
        .map(|_| Err(ServiceError::DuplicateSignature))
        .unwrap_or({
            let serialized_newspaper =
                serde_json::to_vec(&newspaper).map_err(ServiceError::SerializationFault)?;

            host.persist(signature, &serialized_newspaper);

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
        let mut host = MockHost::new();
        let newspaper = newspaper();

        let res = super::create_newspaper(&mut host, newspaper);
        assert_eq!((res.unwrap()).id, "dnevest_n_n".to_string());
    }

    #[test]
    fn dublicate_signature() {
        let mut host = MockHost::new();

        let res = super::new_newspaper(&mut host, newspaper());
        assert_eq!((res.unwrap()).id, "dnevest_n_n".to_string());

        let err = super::new_newspaper(&mut host, newspaper());
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
