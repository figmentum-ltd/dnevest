mod details;
mod error;

// use crate::newspaper::{Signature};
pub(crate) use details::MaxCards;

use details::Details;
use error::{Error, Result};

struct Order {
    details: Details,
    // newspapers: [Option<Signature>; 3],
    // waybill: Waybill
}
