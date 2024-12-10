use serde::{Deserialize, Serialize};

mod cover;
mod delivery;
mod dto;
mod error;
mod wish_card;

use delivery::Delivery;
pub(crate) use dto::OrderRequest;
pub(crate) use error::Error;
use error::Result;
pub(crate) use wish_card::MaxCards;
use wish_card::WishCard;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
pub(crate) struct Order {
    wish_card: WishCard,
    delivery: Delivery,
    created_on_ms: u64,
}

impl Order {
    fn new_unchecked(wish_card: WishCard, delivery: Delivery, created_on_ms: u64) -> Self {
        Self {
            wish_card,
            delivery,
            created_on_ms,
        }
    }

    pub(crate) fn identifier(&self) -> String {
        format!("{}_{}", self.created_on_ms, self.delivery.phone())
    }
}
