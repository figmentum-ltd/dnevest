use serde::Serialize;

use std::result::Result as StdResult;

use crate::{
    bindings::{self, ByteArray},
    newspaper::{self, Date, Newspaper, Signature, Year},
    order::{MaxCards, Order},
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

pub(crate) fn specify_max_cards<A: Storage>(
    adapter: &mut A,
    max_number: u8,
) -> StdResult<Vec<bindings::Event>, ByteArray> {
    self::configure_max_cards(adapter, max_number).map_err(|error| error.serialize())
}

pub(crate) fn create_order<A: Storage>(
    adapter: &mut A,
    order: Order,
) -> StdResult<Vec<bindings::Event>, ByteArray> {
    self::place_order(adapter, order).map_err(|error| error.serialize())
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

fn configure_max_cards<A: Storage>(
    adapter: &mut A,
    max_number: u8,
) -> StdResult<Vec<bindings::Event>, ServiceError> {
    let key = "max_cards";
    let needs_update = adapter
        .retrieve(key)
        .and_then(|ser| {
            serde_json::from_slice::<MaxCards>(&ser)
                .map_err(ServiceError::DeserializationFault)
                .ok()
        })
        .map_or(true, |max_cards| max_cards.number() != max_number);

    if needs_update {
        persist_and_emit_event(
            adapter,
            key,
            &MaxCards::new(max_number),
            "dnevest_max_card",
            Event::specified_max_cards(key),
        )
    } else {
        Ok(Vec::new())
    }
}

fn place_order<A: Storage>(
    adapter: &mut A,
    order: Order,
) -> StdResult<Vec<bindings::Event>, ServiceError> {
    let key = order.identifier();
    let key = key.as_str();
    adapter
        .retrieve(key)
        .map(|_| Err(ServiceError::DuplicateOrder))
        .unwrap_or({
            persist_and_emit_event(adapter, key, &order, "dnevest_n_o", Event::saved_order(key))
        })
}

fn persist_and_emit_event<A: Storage, T: Serialize>(
    adapter: &mut A,
    key: &str,
    item: &T,
    event_id: &str,
    event: Event,
) -> StdResult<Vec<bindings::Event>, ServiceError> {
    serde_json::to_vec(item)
        .map_err(ServiceError::SerializationFault)
        .and_then(|serialized| {
            adapter.persist(key, &serialized);
            event.serialize().map(|serialized_event| {
                vec![bindings::Event {
                    id: event_id.to_string(),
                    content: serialized_event,
                }]
            })
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
    fn create_newspaper() {
        let mut adapter = MockHost::default();
        let newspaper = newspaper();

        let res = super::create_newspaper(&mut adapter, newspaper);
        assert_eq!((res.unwrap())[0].id, "dnevest_n_n".to_string());
    }

    #[test]
    fn dublicate_signature() {
        let mut adapter = MockHost::default();

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
        let mut adapter = MockHost::default();
        let res = super::define_end_year(&mut adapter, "В1223", 2021);
        assert_err(res, "Newspaper not found");
    }

    #[test]
    fn add_max_cards() {
        let mut adapter = MockHost::default();
        let res = super::configure_max_cards(&mut adapter, 30);
        assert_eq!(res.unwrap()[0].id, "dnevest_max_card");

        let res_dub = super::configure_max_cards(&mut adapter, 30);

        assert!(res_dub.unwrap().is_empty());
    }

    #[test]
    fn persist_and_emit_event() {
        let mut adapter = MockHost::default();
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
