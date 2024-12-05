use serde::{de::DeserializeOwned, Deserialize};
use std::result::Result;

use crate::{
    bindings::ByteArray,
    errors::Error,
    newspaper::{Date, Newspaper, Signature, Year},
    order::{CreateOrder, Order},
    Host,
};

// #[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub enum ExecuteMsg {
    CreateNewspaper {
        input: Newspaper,
    },
    AddFinalYear {
        signature: Signature,
        final_year: Year,
    },
    SpecifyMaxCards {
        max_number: u8,
    },
    // TODO use dto::CreateOrder instead of Order
    CreateOrder {
        order: Order,
    },
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub enum QueryMsg {
    NewspapersByDate { date: Date },
}

pub(crate) fn deserialize_msg<T: DeserializeOwned>(msg: ByteArray) -> Result<T, ByteArray> {
    serde_json::from_slice::<T>(&msg).map_err(|err| Error::InvalidRequest(err).serialize())
}

#[cfg(test)]
mod test_deserialization {
    use std::fmt::Debug;

    use crate::{bindings::ByteArray, newspaper::Date};

    use super::QueryMsg;

    #[test]
    fn invalid_json() {
        let missing_enum_variant = r#"{"date":"29-06-2024"}"#;
        assert_err(
            super::deserialize_msg::<QueryMsg>(missing_enum_variant.into()),
            "Invalid json in request",
        );
    }

    #[test]
    fn valid_query_msg() {
        let msg = r#"{"NewspapersByDate":{"date":"29-06-2024"}}"#;
        let expected = QueryMsg::NewspapersByDate {
            date: Date::try_from("29-06-2024".to_string()).unwrap(),
        };
        let res = super::deserialize_msg::<QueryMsg>(msg.into()).expect("deserialization failed");

        assert_eq!(res, expected);
    }

    fn assert_err<T: Debug>(r: Result<T, ByteArray>, msg: &str) {
        assert!(r.is_err());
        let err =
            serde_json::from_slice::<String>(&r.unwrap_err()).expect("deserialization failed");

        assert!(err.contains(msg))
    }
}
