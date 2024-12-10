use serde::Deserialize;

use std::{marker::PhantomData, result::Result as StdResult};

use crate::{Storage, Time};

use super::{delivery::UncheckedDelivery, wish_card::UncheckedWishCard, Error, Order};

#[derive(Deserialize)]
#[serde(bound = "")]
pub(crate) struct OrderRequest<S, T>
where
    S: Storage + Default,
    T: Time + Default,
{
    wish_card: UncheckedWishCard<S>,
    delivery: UncheckedDelivery,
    #[serde(skip)]
    _storage: PhantomData<S>,
    #[serde(skip)]
    _time: PhantomData<T>,
}

impl<S, T> TryFrom<OrderRequest<S, T>> for Order
where
    S: Storage + Default,
    T: Time + Default,
{
    type Error = Error;

    fn try_from(dto: OrderRequest<S, T>) -> StdResult<Self, Self::Error> {
        dto.wish_card.try_into().and_then(|wish_card| {
            dto.delivery
                .try_into()
                .map(|delivery| Order::new_unchecked(wish_card, delivery, T::now().timestamp))
        })
    }
}
