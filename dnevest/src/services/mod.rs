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
) -> Result<ByteArray, ByteArray> {
    newspaper::newspapers_by_date(host, date).map_err(|error| error.serialize())
}

// TODO! - do we need 'newspaper' to pe present in every name
fn new_newspaper<H: HostImports>(
    host: &mut H,
    newspaper: Newspaper,
) -> StdResult<bindings::Event, ServiceError> {
    let signature = newspaper.identificator();
    let serialized_newspaper =
        serde_json::to_vec(&newspaper).map_err(|err| ServiceError::SerializationFault(err))?;

    host.persist(signature, &serialized_newspaper);

    let serialized_event = Event::NewspaperCreated(signature).serialize()?;
    Ok(bindings::Event {
        id: "dnevest_n_n".to_string(),
        content: serialized_event,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        newspaper::{Newspaper, Signature, WeeklyFrequency},
        services::MockHost,
    };

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
