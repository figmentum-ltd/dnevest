use std::result::Result as StdResult;

use crate::{
    bindings::{self, ByteArray},
    newspaper::{self, Date, Newspaper, Signature, Year},
    response::Event,
    Storage, Time,
};

mod error;

#[cfg(test)]
pub(crate) mod mock_host;

pub(super) use error::Error as ServiceError;
#[cfg(test)]
pub(crate) use mock_host::MockHost;

pub(crate) fn create_newspaper<A: Storage>(
    adapter: &mut A,
    input: Newspaper,
) -> StdResult<Vec<bindings::Event>, ByteArray> {
    self::new_newspaper(adapter, input).map_err(|error| error.serialize())
}

pub(crate) fn add_final_year<A: Storage + Time>(
    adapter: &mut A,
    signature: Signature,
    final_year: Year,
) -> StdResult<Vec<bindings::Event>, ByteArray> {
    self::define_end_year(adapter, signature.as_str(), final_year)
        .map_err(|error| error.serialize())
}

pub(crate) fn newspapers_by_date<A: Storage + Time>(
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
) -> StdResult<Vec<bindings::Event>, ServiceError> {
    let signature = newspaper.identificator();
    adapter
        .retrieve(signature)
        .map(|_| Err(ServiceError::DuplicateSignature))
        .unwrap_or({
            persist_and_emit_event(
                adapter,
                signature,
                &newspaper,
                "dnevest_n_n",
                Event::newspaper_created(signature),
            )
        })
}

fn define_end_year<A: Storage + Time>(
    adapter: &mut A,
    signature: &str,
    final_year: Year,
) -> StdResult<Vec<bindings::Event>, ServiceError> {
    adapter
        .retrieve(signature)
        .ok_or(ServiceError::NotFound("Newspaper not found"))
        .and_then(|ser_newspaper| {
            serde_json::from_slice(&ser_newspaper)
                .map_err(ServiceError::DeserializationFault)
                .and_then(|newspaper: Newspaper| {
                    newspaper
                        .add_end_year(final_year, A::now())
                        .map_err(ServiceError::DomainError)
                        .and_then(|newspaper| {
                            persist_and_emit_event(
                                adapter,
                                signature,
                                &newspaper,
                                "dnevest_end_y",
                                Event::added_end_year(signature),
                            )
                        })
                })
        })
}

fn persist_and_emit_event<A: Storage>(
    adapter: &mut A,
    signature: &str,
    newspaper: &Newspaper,
    event_id: &str,
    event: Event,
) -> StdResult<Vec<bindings::Event>, ServiceError> {
    serde_json::to_vec(newspaper)
        .map_err(ServiceError::SerializationFault)
        .and_then(|serialized| {
            adapter.persist(signature, &serialized);
            event.serialize().map(|serialized_event| vec![bindings::Event {
                id: event_id.to_string(),
                content: serialized_event,
            }])
        })
}

#[cfg(test)]
mod tests {
    use crate::{
        bindings,
        newspaper::Newspaper,
        response::Event,
        services::{MockHost, ServiceError},
    };

    #[test]
    fn create() {
        let mut adapter = MockHost::new();
        let newspaper = newspaper();

        let res = super::create_newspaper(&mut adapter, newspaper);
        assert_eq!((res.unwrap())[0].id, "dnevest_n_n".to_string());
    }

    #[test]
    fn dublicate_signature() {
        let mut adapter = MockHost::new();

        let res = super::new_newspaper(&mut adapter, newspaper());
        assert_eq!((res.unwrap())[0].id, "dnevest_n_n".to_string());

        let err = super::new_newspaper(&mut adapter, newspaper());
        assert_err(
            err,
            "Cannot create the newspaper because this signature already exists",
        );
    }

    #[test]
    fn newspaper_not_found() {
        let mut adapter = MockHost::with_newspapers();
        let res = super::define_end_year(&mut adapter, "В1223", 2021);
        assert_err(res, "Newspaper not found");
    }

    #[test]
    fn persist_and_emit_event() {
        let mut adapter = MockHost::with_newspapers();
        let newspaper = Newspaper::new_unchecked(
            "В1612",
            "Труд",
            1946,
            Some(2024),
            [true, true, true, true, true, true, true],
        );
        let signature = newspaper.identificator();
        let event_id = "dnevest_end_y";
        let event = super::persist_and_emit_event(
            &mut adapter,
            signature,
            &newspaper,
            event_id,
            Event::added_end_year(signature),
        )
        .unwrap();

        assert_eq!(event[0].id, event_id.to_string());
        assert_eq!(
            event[0].content,
            Event::added_end_year(signature)
                .serialize()
                .expect("serialization failed")
        )
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

    fn assert_err(r: Result<Vec<bindings::Event>, ServiceError>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
