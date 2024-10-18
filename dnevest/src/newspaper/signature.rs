use once_cell::sync::Lazy;
use regex::Regex;

use super::error::{Error, Result};

static REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^B\d{4}$").expect("Invalid regex pattern!"));

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(super) struct Signature {
    signature: String,
}

impl Signature {
    pub(super) fn new(signature: String) -> Self {
        let obj = Self { signature };
        debug_assert!(obj.invariant_held().is_ok());
        obj
    }

    #[cfg(test)]
    fn try_new(signature: String) -> Result<Self> {
        let obj = Self { signature };
        obj.invariant_held().map(|()| obj)
    }

    fn invariant_held(&self) -> Result<()> {
        if REGEX.is_match(&self.signature){
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
    fn valid_object() {
        assert!(Signature::try_new("B1234".to_string()).is_ok())
    }

    #[test]
    fn not_maching_pattern() {
        const MSG: &str = "Signature does not match the required pattern";

        assert_err(new_invalid("b2974"), MSG);
        assert_err(new_invalid("n2974"), MSG);
        assert_err(new_invalid("N0970"), MSG);
        assert_err(new_invalid("0000"), MSG);
        assert_err(new_invalid("B780"), MSG);
        assert_err(new_invalid("B34580"), MSG);
    }

    #[test]
    fn using_cyrillic_letter() {
        const MSG: &str = "Signature does not match the required pattern";
        assert_err(new_invalid("В3497"), MSG);
    }

    fn new_invalid(sign: &str) -> Result<Signature> {
        Signature::try_new(sign.to_string())
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
