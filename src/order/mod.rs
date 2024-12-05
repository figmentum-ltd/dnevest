use serde::{Deserialize, Serialize};

use details::Details;
use error::{Error, Result};

mod details;
mod dto;
mod error;
mod waybill;

use crate::newspaper::Signature;
pub(crate) use details::MaxCards;
pub(crate) use dto::CreateOrder;
use waybill::Waybill;

// TODO rename type to be Cover
pub(crate) type OrderedNewspapers = [Option<Signature>; 3];

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
pub(crate) struct Order {
    details: Details,
    newspapers: OrderedNewspapers,
    waybill: Waybill,
    // TODO rename to "created_on_ms"
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
