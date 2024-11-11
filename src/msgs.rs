use serde::{de::DeserializeOwned, Deserialize};
use std::result::Result;

use crate::{
    bindings::ByteArray,
    errors::Error,
    newspaper::{Date, Newspaper},
};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub enum ExecuteMsg {
    CreateNewspaper { input: Newspaper },
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

    use crate::{
        bindings::ByteArray,
        newspaper::{Date, Newspaper},
    };

    use super::{ExecuteMsg, QueryMsg};

    #[test]
    fn valid_execute_msg() {
        let msg = r#"{"CreateNewspaper":{"input":{"signature":"В4667","name":"Орбита","start_year":1969,"end_year":1991,"weekly_schedule":[false,false,false,false,false,true,false]}}}"#;
        let newspaper = Newspaper::new_unchecked(
            "В4667",
            "Орбита",
            1969,
            Some(1991),
            [false, false, false, false, false, true, false],
        );
        let expected = ExecuteMsg::CreateNewspaper { input: newspaper };
        let res = super::deserialize_msg::<ExecuteMsg>(msg.into()).expect("deserialization failed");

        assert_eq!(res, expected)
    }

    #[test]
    fn invalid_json() {
        let error_msgs = "Invalid json in request";

        let missing_field = r#"{"CreateNewspaper":{"signature":"В4667","name":"Орбита","start_year":1969,"end_year":1991,"weekly_schedule":[false,false,false,false,false,true,false]}}"#;
        assert_err(
            super::deserialize_msg::<ExecuteMsg>(missing_field.into()),
            error_msgs,
        );

        let missing_start_year = r#"{"CreateNewspaper":{"input":{"signature":"В4667","name":"Орбита","start_year":,"end_year":null,"weekly_schedule":[false,false,false,false,false,true,false]}}}"#;
        assert_err(
            super::deserialize_msg::<ExecuteMsg>(missing_start_year.into()),
            error_msgs,
        );

        let missing_enum_variant = r#"{"date":"29-06-2024"}"#;
        assert_err(
            super::deserialize_msg::<QueryMsg>(missing_enum_variant.into()),
            error_msgs,
        );
    }

    #[test]
    fn invalid_newspaper() {
        let start_in_future = r#"{"CreateNewspaper":{"input":{"signature":"В4667","name":"Орбита","start_year":2100,"end_year":null,"weekly_schedule":[false,false,false,false,false,true,false]}}}"#;
        assert_err(
            super::deserialize_msg::<ExecuteMsg>(start_in_future.into()),
            "start_year cannot be in the future",
        );

        let end_before_start = r#"{"CreateNewspaper":{"input":{"signature":"В4667","name":"Орбита","start_year":1991,"end_year":1969,"weekly_schedule":[false,false,false,false,false,true,false]}}}"#;
        assert_err(
            super::deserialize_msg::<ExecuteMsg>(end_before_start.into()),
            "start_year cannot be after end_year",
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
