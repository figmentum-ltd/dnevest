use serde::Serialize;

use std::result::Result as StdResult;

use crate::{
    bindings::{self, ByteArray},
    newspaper::{self, Date, Newspaper, Signature, Year},
    order::{MaxCards, Order, OrderRequest},
    response::Event,
    Storage, Time,
};

mod error;

#[cfg(test)]
pub(crate) mod mock_host;

pub(super) use error::Error as ServiceError;
#[cfg(test)]
pub(crate) use mock_host::MockHost;

pub(crate) fn create_newspaper<S>(input: Newspaper) -> StdResult<Vec<bindings::Event>, ByteArray>
where
    S: Storage + Default,
{
    self::new_newspaper::<S>(input).map_err(|error| error.serialize())
}

pub(crate) fn add_final_year<S, T>(
    signature: Signature,
    final_year: Year,
) -> StdResult<Vec<bindings::Event>, ByteArray>
where
    S: Storage + Default,
    T: Time + Default,
{
    self::define_end_year::<S, T>(signature.as_str(), final_year).map_err(|error| error.serialize())
}

pub(crate) fn specify_max_cards<S>(max_number: u8) -> StdResult<Vec<bindings::Event>, ByteArray>
where
    S: Storage + Default,
{
    self::configure_max_cards::<S>(max_number).map_err(|error| error.serialize())
}

pub(crate) fn create_order<S, T>(
    order: OrderRequest<S, T>,
) -> StdResult<Vec<bindings::Event>, ByteArray>
where
    S: Storage + Default,
    T: Time + Default,
{
    self::place_order(order).map_err(|error| error.serialize())
}

pub(crate) fn newspapers_by_date<S, T>(date: Date) -> StdResult<ByteArray, ByteArray>
where
    S: Storage + Default,
    T: Time + Default,
{
    newspaper::newspapers_by_date::<S, T>(date)
        .map_err(|error| ServiceError::DomainError(error).serialize())
}

// TODO! - do we need 'newspaper' to pe present in every name
fn new_newspaper<S>(newspaper: Newspaper) -> StdResult<Vec<bindings::Event>, ServiceError>
where
    S: Storage + Default,
{
    let signature = newspaper.identificator();
    let mut storage = S::default();
    storage
        .retrieve(signature)
        .map(|_| Err(ServiceError::DuplicateSignature))
        .unwrap_or({
            persist_and_emit_event(
                &mut storage,
                signature,
                &newspaper,
                "dnevest_n_n",
                Event::newspaper_created(signature),
            )
        })
}

fn define_end_year<S, T>(
    signature: &str,
    final_year: Year,
) -> StdResult<Vec<bindings::Event>, ServiceError>
where
    S: Storage + Default,
    T: Time + Default,
{
    let mut storage = S::default();
    storage
        .retrieve(signature)
        .ok_or(ServiceError::NotFound("Newspaper not found"))
        .and_then(|ser_newspaper| {
            serde_json::from_slice(&ser_newspaper)
                .map_err(ServiceError::DeserializationFault)
                .and_then(|newspaper: Newspaper| {
                    newspaper
                        .add_end_year(final_year, T::now())
                        .map_err(ServiceError::DomainError)
                        .and_then(|newspaper| {
                            persist_and_emit_event(
                                &mut storage,
                                signature,
                                &newspaper,
                                "dnevest_end_y",
                                Event::added_end_year(signature),
                            )
                        })
                })
        })
}

fn configure_max_cards<S>(max_number: u8) -> StdResult<Vec<bindings::Event>, ServiceError>
where
    S: Storage + Default,
{
    let mut storage = S::default();
    let key = "max_cards";
    let needs_update = storage
        .retrieve(key)
        .and_then(|ser| {
            serde_json::from_slice::<MaxCards>(&ser)
                .map_err(ServiceError::DeserializationFault)
                .ok()
        })
        .map_or(true, |max_cards| max_cards.number() != max_number);

    if needs_update {
        persist_and_emit_event(
            &mut storage,
            key,
            &MaxCards::new(max_number),
            "dnevest_max_card",
            Event::specified_max_cards(key),
        )
    } else {
        Ok(Vec::new())
    }
}

fn place_order<S, T>(order: OrderRequest<S, T>) -> StdResult<Vec<bindings::Event>, ServiceError>
where
    S: Storage + Default,
    T: Time + Default,
{
    order
        .try_into()
        .map_err(ServiceError::InvalidOrder)
        .and_then(|order: Order| {
            let key = order.identifier();
            let key = key.as_str();
            let mut storage = S::default();
            storage
                .retrieve(key)
                .map(|_| Err(ServiceError::DuplicateOrder))
                .unwrap_or({
                    persist_and_emit_event(
                        &mut storage,
                        key,
                        &order,
                        "dnevest_n_o",
                        Event::saved_order(key),
                    )
                })
        })
}

fn persist_and_emit_event<S, I>(
    storage: &mut S,
    key: &str,
    item: &I,
    event_id: &str,
    event: Event,
) -> StdResult<Vec<bindings::Event>, ServiceError>
where
    S: Storage,
    I: Serialize,
{
    serde_json::to_vec(item)
        .map_err(ServiceError::SerializationFault)
        .and_then(|serialized| {
            storage.persist(key, &serialized);
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

    use super::OrderRequest;

    #[test]
    fn create_newspaper() {
        let newspaper = newspaper();

        let res = super::create_newspaper::<MockHost>(newspaper);
        assert_eq!((res.unwrap())[0].id, "dnevest_n_n".to_string());
    }

    #[test]
    fn newspaper_not_found() {
        let res = super::define_end_year::<MockHost, MockHost>("В1223", 2021);
        assert_err(res, "Newspaper not found");
    }

    #[test]
    fn add_max_cards() {
        let res = super::configure_max_cards::<MockHost>(30);
        assert_eq!(res.unwrap()[0].id, "dnevest_max_card");
    }

    #[test]
    fn create_order() {
        let res = super::create_order(order());
        assert_eq!(res.unwrap()[0].id, "dnevest_n_o");
    }

    #[test]
    fn persist_and_emit_event() {
        let mut storage = MockHost::default();
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
            &mut storage,
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

    fn order() -> OrderRequest<MockHost, MockHost> {
        let json = r#"{"wish_card":{"covers":{"preference":"В1616","options":["В4667",null]},"background":[255,0,0],"frame":"White","message":"Честит рожден ден!","font_type":"Times New Roman","font_size":12,"template_id":10},"delivery":{"customer_names":"Тодор Георгиев","phone_number":"0873528495","address":"Пловдив, ул.Тракия 12","priority":"Standart"}}"#;
        let unchecked: OrderRequest<MockHost, MockHost> =
            serde_json::from_str(json).expect("failed to deserialize JSON");
        unchecked
    }

    fn assert_err(r: Result<Vec<bindings::Event>, ServiceError>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
