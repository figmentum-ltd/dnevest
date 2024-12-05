use serde::{Deserialize, Serialize};

use error::{Error, Result};
use wish_card::WishCard;

mod delivery;
mod dto;
mod error;
mod wish_card;

use crate::newspaper::Signature;
use delivery::Delivery;
pub(crate) use dto::CreateOrder;
pub(crate) use wish_card::MaxCards;

// TODO rename type to be Cover
pub(crate) type OrderedNewspapers = [Option<Signature>; 3];

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
pub(crate) struct Order {
    details: WishCard,
    newspapers: OrderedNewspapers,
    waybill: Delivery,
    // TODO rename to "created_on_ms"
    timestamp: u64,
}

impl Order {
    fn new_unchecked(
        details: WishCard,
        newspapers: OrderedNewspapers,
        waybill: Delivery,
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
        format!("{}_{}", self.timestamp, self.waybill.phone())
    }
}

#[cfg(test)]
mod test {
    // #[test]
    // fn key_generate() {
    //     assert_eq!(
    //         "1732880395_0873528495",
    //         super::generate_key(1732880395, "0873528495")
    //     );
    //     assert_eq!(
    //         "1732834395_+359873528495",
    //         super::generate_key(1732834395, "+359873528495")
    //     );
    // }
}
