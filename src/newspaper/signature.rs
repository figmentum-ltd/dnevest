use serde::{Deserialize, Serialize};
use std::result;

use super::error::{Error, Result};

pub(crate) const SIGN: char = 'В';

/// Brings invariant checking as a step in deserializing a Newspaper
#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(try_from = "String")]
pub(crate) struct Signature(String);

impl Signature {
    #[cfg(test)]
    pub(crate) fn new(signature: &str) -> Self {
        Self(signature.to_string())
    }

    pub(super) fn signature(&self) -> &str {
        self.0.as_str()
    }

    fn invariant_held(&self) -> Result<()> {
        let sign = self.0.as_str();
        let mut chars = sign.chars();

        // the character 'B' in cyrillic takes 2 bytes, so the signature length is 6
        if sign.len() == 6
            && chars.next() == Some(SIGN)
            && chars.take(4).all(|c| c.is_ascii_digit())
            && !sign.ends_with("0000")
        {
            Ok(())
        } else {
            Err(Error::SignatureMismatch)
        }
    }
}

impl TryFrom<String> for Signature {
    type Error = Error;

    fn try_from(value: String) -> result::Result<Self, Self::Error> {
        let obj = Signature(value);
        obj.invariant_held().map(|()| obj)
    }
}

pub(crate) fn next_letter(c: char) -> char {
    u32::try_from(c)
        .ok()
        .and_then(|val| char::try_from(val + 1).ok())
        .unwrap()
}

#[cfg(test)]
mod test_invariant {
    use super::{Result, Signature};

    #[test]
    fn valid_signatures() {
        assert!(new("В1234").is_ok());
        assert!(new("В0001").is_ok());
        assert!(new("В9999").is_ok());
    }

    #[test]
    fn not_maching_pattern() {
        const MSG: &str = "Signature does not match the required pattern";

        assert_err(new("В0000"), MSG);
        assert_err(new("b2974"), MSG);
        assert_err(new("в2974"), MSG);
        assert_err(new("n2974"), MSG);
        assert_err(new("N0970"), MSG);
        assert_err(new("0000"), MSG);
        assert_err(new("В-780"), MSG);
        assert_err(new("В34580"), MSG);
        assert_err(new("В+450"), MSG);
    }

    #[test]
    fn using_latin_letter() {
        const MSG: &str = "Signature does not match the required pattern";
        assert_err(new("B3497"), MSG);
    }

    #[test]
    fn serialize() {
        let signature = Signature::new("В3452");
        let serialized = serde_json::to_string(&signature).unwrap();

        assert_eq!(serialized, r#""В3452""#)
    }

    #[test]
    fn next_letter() {
        assert_eq!(super::next_letter('В'), 'Г');
        assert_eq!(super::next_letter('B'), 'C');
        assert_ne!(super::next_letter('B'), 'Г')
    }

    fn new(sign: &str) -> Result<Signature> {
        Signature::try_from(sign.to_string())
    }

    fn assert_err(r: Result<Signature>, msg: &str) {
        r.map_err(|err| {
            let error_message = err.to_string();
            assert!(
                error_message.contains(msg),
                "Expected error message to contain: '{}', but got: '{}'",
                msg,
                error_message
            );
        })
        .expect_err("Expected an error, but got a valid Signature.");
    }
}
