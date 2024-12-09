use serde::{Deserialize, Serialize};

use error::{Error, Result};
use wish_card::WishCard;

mod cover;
mod delivery;
mod dto;
mod error;
mod wish_card;

use delivery::Delivery;
pub(crate) use dto::OrderRequest;
pub(crate) use wish_card::MaxCards;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
pub(crate) struct Order {
    details: WishCard,
    waybill: Delivery,
    created_on_ms: u64,
}

impl Order {
    fn new_unchecked(details: WishCard, waybill: Delivery, created_on_ms: u64) -> Self {
        Self {
            details,
            waybill,
            created_on_ms,
        }
    }

    pub(crate) fn identifier(&self) -> String {
        format!("{}_{}", self.created_on_ms, self.waybill.phone())
    }
}
