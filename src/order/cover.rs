use serde::{Deserialize, Serialize};

use std::{marker::PhantomData, result::Result as StdResult};

use crate::{newspaper::Signature, Storage};

use super::{Error, Result};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
pub(super) struct Cover {
    preference: Signature,
    options: [Option<Signature>; 2],
}

impl Cover {
    pub(super) fn new_unchecked(preference: Signature, options: [Option<Signature>; 2]) -> Self {
        Self {
            preference,
            options,
        }
    }

    fn invariant_held<S>(&self, storage: S) -> Result<()>
    where
        S: Storage,
    {
        storage
            .retrieve(self.preference.as_str())
            .ok_or_else(|| {
                Error::NotFound(format!(
                    "The signature {} is not found",
                    self.preference.as_str()
                ))
            })
            .and_then(|_| {
                if self.options.iter().any(|opt| {
                    opt.as_ref().map_or(false, |signature| {
                        storage.retrieve(signature.as_str()).is_none()
                    })
                }) {
                    Err(Error::NotFound("The signature is not found".into()))
                } else {
                    Ok(())
                }
            })
    }
}

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
pub(super) struct UncheckedCover<S>
where
    S: Storage + Default,
{
    preference: Signature,
    options: [Option<Signature>; 2],
    #[serde(skip)]
    _storage: PhantomData<S>,
}

impl<S> TryFrom<UncheckedCover<S>> for Cover
where
    S: Storage + Default,
{
    type Error = Error;

    fn try_from(unchecked: UncheckedCover<S>) -> StdResult<Self, Self::Error> {
        let obj = Self::new_unchecked(unchecked.preference, unchecked.options);
        obj.invariant_held(S::default()).map(|()| obj)
    }
}

#[cfg(test)]
mod test {
    use crate::{order::cover::UncheckedCover, services::MockHost};

    use super::{Cover, Result, Signature};

    #[test]
    fn unchecked_deserialization() {
        let json = r#"{"preference":"В2364","options":["В4780",null]}"#;
        let unchecked: Cover = serde_json::from_str(json).expect("failed to deserialize JSON");
        assert_eq!(
            cover("В2364", [Some(Signature::new("В4780")), None]),
            unchecked
        )
    }

    #[test]
    fn checked_deserialization() {
        let _host = MockHost::default();

        let json = r#"{"preference":"В1616","options":["В4667",null]}"#;
        let unchecked: UncheckedCover<MockHost> =
            serde_json::from_str(json).expect("failed to deserialize JSON");
        assert_eq!(
            cover("В1616", [Some(Signature::new("В4667")), None]),
            unchecked.try_into().unwrap()
        )
    }

    #[test]
    fn checked_deserialization_failed() {
        let _host = MockHost::default();

        failed_checked_deserialization(r#"{"preference":"В2364","options":[null,null]}"#);
        failed_checked_deserialization(r#"{"preference":"В4667","options":[null,"В2364"]}"#);
    }

    fn failed_checked_deserialization(json: &str) {
        let unchecked: UncheckedCover<MockHost> =
            serde_json::from_str(json).expect("failed to deserialize JSON");
        assert_err(unchecked.try_into(), "The signature")
    }

    fn assert_err(r: Result<Cover>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }

    fn cover(preference: &str, options: [Option<Signature>; 2]) -> Cover {
        Cover::new_unchecked(Signature::new(preference), options)
    }
}
