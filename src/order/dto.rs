use serde::Deserialize;

use std::{marker::PhantomData, result::Result as StdResult};

use crate::{Storage, Time};

use super::{delivery::Delivery, wish_card::WishCard, Error, MaxCards, Order, Result};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub(crate) struct OrderRequest<S, T>
where
    S: Storage + Default,
    T: Time + Default,
{
    details: WishCard,
    waybill: Delivery,
    #[serde(skip)]
    _storage: PhantomData<S>,
    #[serde(skip)]
    _time: PhantomData<T>,
}

// impl<S, T> TryFrom<OrderRequest<S, T>> for Order
// where
//     S: Storage + Default,
//     T: Time + Default,
// {
//     type Error = Error;

//     fn try_from(dto: OrderRequest<S, T>) -> StdResult<Self, Self::Error> {
//         fetch_max_cards(S::default())
//             .and_then(|max_cards| dto.details.check(max_cards))
//             .and_then(|()| dto.newspapers.try
//             .and_then(|()| dto.waybill.check())
//             .map(|()| {
//                 Order::new_unchecked(dto.details, dto.newspapers, dto.waybill, T::now().timestamp)
//             })
//     }
// }

fn fetch_max_cards<S>(storage: S) -> Result<MaxCards>
where
    S: Storage,
{
    storage
        .retrieve("max_cards")
        .ok_or(Error::NotFound("Failed to fetch max cards.".into()))
        .and_then(|data| serde_json::from_slice(&data).map_err(Error::DeserializationFault))
}

#[cfg(test)]
mod test {
    // use crate::order::Signature;

    // use super::Result;

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

    // fn assert_err(r: Result<()>, msg: &str) {
    //     assert!(r.expect_err("expected an error").to_string().contains(msg))
    // }
}
