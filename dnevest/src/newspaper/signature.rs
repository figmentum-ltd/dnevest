use serde::{Deserialize, Serialize};

use super::error::{Error, Result};

use crate::newspaper::dto;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(try_from = "dto::SignatureDTO", into = "dto::SignatureDTO")]
pub(crate) struct Signature {
    signature: String,
}

impl Signature {
    pub(crate) fn signature(&self) -> String {
        self.signature.clone()
    }

    pub(super) fn try_new(signature: &str) -> Result<Self> {
        let obj = Self::new_internal(signature);
        obj.invariant_held().map(|()| obj)
    }

    fn new_internal(signature: &str) -> Self {
        Self {
            signature: signature.to_string(),
        }
    }

    fn invariant_held(&self) -> Result<()> {
        let sign = self.signature.as_str();
        let mut chars = sign.chars();

        // the character 'B' in cyrillic takes 2 bytes, so the signature length is 6
        if sign.len() == 6
            && chars.next() == Some('В')
            && chars.take(4).all(|c| c.is_digit(10))
            && !sign.ends_with("0000")
        {
            Ok(())
        } else {
            Err(Error::SignatureMismatch)
        }
    }
}

#[cfg(test)]
mod test_invariant {
    use super::{Result, Signature};

    #[test]
    fn valid_signatures() {
        assert!(Signature::try_new("В1234").is_ok());
        assert!(Signature::try_new("В0001").is_ok());
        assert!(Signature::try_new("В9999").is_ok());
    }

    #[test]
    fn not_maching_pattern() {
        const MSG: &str = "Signature does not match the required pattern";

        assert_err(new_invalid("В0000"), MSG);
        assert_err(new_invalid("b2974"), MSG);
        assert_err(new_invalid("в2974"), MSG);
        assert_err(new_invalid("n2974"), MSG);
        assert_err(new_invalid("N0970"), MSG);
        assert_err(new_invalid("0000"), MSG);
        assert_err(new_invalid("В-780"), MSG);
        assert_err(new_invalid("В34580"), MSG);
        assert_err(new_invalid("В+450"), MSG);
    }

    #[test]
    fn using_latin_letter() {
        const MSG: &str = "Signature does not match the required pattern";
        assert_err(new_invalid("B3497"), MSG);
    }

    fn new_invalid(sign: &str) -> Result<Signature> {
        Signature::try_new(sign)
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
