use serde::{Deserialize, Serialize};

use std::{marker::PhantomData, result::Result as StdResult};

use details::Details;
use error::{Error, Result};

mod details;
mod error;
mod waybill;

use crate::{newspaper::Signature, Host, Storage, Time};
pub(crate) use details::MaxCards;
use waybill::Waybill;

type OrderedNewspapers = [Option<Signature>; 3];

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(try_from = "UncheckedOrder<Host, Host>")]
pub(crate) struct Order {
    details: Details,
    newspapers: OrderedNewspapers,
    waybill: Waybill,
    timestamp: u64,
}

impl Order {
    fn new_unchecked(
        details: Details,
        newspapers: OrderedNewspapers,
        waybill: Waybill,
        timestamp: u64,
    ) -> Self {
        Self {
            details,
            newspapers,
            waybill,
            timestamp,
        }
    }

    pub(crate) fn identifier(&self) -> String {
        generate_key(self.timestamp, self.waybill.phone())
    }

    fn invariant_held<S: Storage>(&self, storage: &mut S) -> Result<()> {
        let mut at_least_one = false;
        self.newspapers.iter().try_for_each(|opt_signature| {
            if let Some(signature) = opt_signature {
                at_least_one = true;
                let signature = signature.as_str();
                if storage.retrieve(signature).is_none() {
                    return Err(Error::InvalidOrder(format!(
                        "The signature {} is not found",
                        signature
                    )));
                }
            }
            Ok(())
        })?;

        if !at_least_one {
            Err(Error::InvalidOrder(
                "There must be at lease one selected newspaper.".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Deserialize)]
struct UncheckedOrder<S, T>
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

impl<S, T> TryFrom<UncheckedOrder<S, T>> for Order
where
    S: Storage + Default,
    T: Time + Default,
{
    type Error = Error;

    fn try_from(unchecked: UncheckedOrder<S, T>) -> StdResult<Self, Self::Error> {
        let mut storage_instance = S::default();
        let obj = Order::new_unchecked(
            unchecked.details,
            unchecked.newspapers,
            unchecked.waybill,
            T::now().timestamp,
        );
        obj.invariant_held(&mut storage_instance).map(|()| obj)
    }
}

fn generate_key(timestamp: u64, order_detail: &str) -> String {
    format!("{}_{}", timestamp, order_detail)
}

#[cfg(test)]
mod test {
    use crate::services::MockHost;

    use super::{
        details::{Details, Frame, Rgb},
        waybill::{OrderType, Waybill},
        Order, OrderedNewspapers, Result, Signature,
    };

    #[test]
    fn invalid_variant() {
        let mut host = MockHost::with_newspapers();

        let order_1 = order([None, Some(Signature::new("В3478")), None]);
        assert_err(order_1.invariant_held(&mut host), "The signature");

        let order_1 = order([None, None, None]);
        assert_err(
            order_1.invariant_held(&mut host),
            "There must be at lease one selected newspaper",
        );
    }

    #[test]
    fn key_generate() {
        assert_eq!(
            "1732880395_0873528495",
            super::generate_key(1732880395, "0873528495")
        );
        assert_eq!(
            "1732834395_+359873528495",
            super::generate_key(1732834395, "+359873528495")
        );
    }

    fn order(newspapers: OrderedNewspapers) -> Order {
        let details = Details::new_unchecked(
            Rgb::new(255, 0, 0),
            Frame::White,
            "Честит рожден ден!".to_string(),
            "Times New Roman".to_string(),
            12,
            10,
        );
        let waybill = Waybill::new_unchecked(
            "Тодор Георгиев".to_string(),
            "0873528495".to_string(),
            "Пловдив, ул.Тракия 12".to_string(),
            OrderType::Standart,
        );
        Order::new_unchecked(details, newspapers, waybill, 1732752206)
    }

    fn assert_err(r: Result<()>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
