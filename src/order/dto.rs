use serde::Deserialize;

use std::{marker::PhantomData, result::Result as StdResult};

use crate::{Storage, Time};

use super::{
    details::Details, waybill::Waybill, Error, MaxCards, Order, OrderedNewspapers, Result,
};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub(crate) struct CreateOrder<S, T>
where
    S: Storage + Default,
    T: Time + Default,
{
    details: Details,
    newspapers: OrderedNewspapers,
    waybill: Waybill,
    #[serde(skip)]
    _storage: PhantomData<S>,
    #[serde(skip)]
    _time: PhantomData<T>,
}

impl<S, T> TryFrom<CreateOrder<S, T>> for Order
where
    S: Storage + Default,
    T: Time + Default,
{
    type Error = Error;

    fn try_from(dto: CreateOrder<S, T>) -> StdResult<Self, Self::Error> {
        fetch_max_cards(S::default())
            .and_then(|max_cards| dto.details.check(max_cards))
            .and_then(|()| check_newspapers(&dto.newspapers, S::default()))
            .and_then(|()| dto.waybill.check())
            .map(|()| {
                Order::new_unchecked(dto.details, dto.newspapers, dto.waybill, T::now().timestamp)
            })
    }
}

fn fetch_max_cards<S>(storage: S) -> Result<MaxCards>
where
    S: Storage,
{
    storage
        .retrieve("max_cards")
        .ok_or(Error::NotFound("Failed to fetch max cards."))
        .and_then(|data| serde_json::from_slice(&data).map_err(Error::DeserializationFault))
}

fn check_newspapers<S>(newspapers: &OrderedNewspapers, storage: S) -> Result<()>
where
    S: Storage,
{
    let mut at_least_one = false;
    // TODO use 'find/any'
    newspapers.iter().try_for_each(|opt_signature| {
        if let Some(signature) = opt_signature {
            at_least_one = true;
            let signature = signature.as_str();
            // TODO create a separate type for the newspapers
            if storage.retrieve(signature).is_none() {
                return Err(Error::InvalidOrder(format!(
                    "The signature {} is not found",
                    signature
                )));
            }
        }
        Ok(())
    })?;

    // TODO use 'into' instead
    if !at_least_one {
        Err(Error::InvalidOrder(
            "There must be at lease one selected newspaper.".to_string(),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::order::Signature;

    use super::Result;

    // #[test]
    // fn invalid_variant() {
    //     let mut host = MockHost::with_newspapers();

    //     let newspapers_1 = [None, Some(Signature::new("В3478")), None];
    //     assert_err(newspapers_1.invariant_held(&mut host), "The signature");

    //     let newspapers_2 = [None, None, None];
    //     assert_err(
    //         let newspapers_2.invariant_held(&mut host),
    //         "There must be at lease one selected newspaper",
    //     );
    // }

    fn assert_err(r: Result<()>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
