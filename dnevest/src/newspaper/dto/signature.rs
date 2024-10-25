use serde::{Deserialize, Serialize};

use crate::newspaper::{Error, Signature};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(transparent)]
pub(crate) struct SignatureDTO(pub(crate) String);

impl TryFrom<SignatureDTO> for Signature {
    type Error = Error;

    fn try_from(value: SignatureDTO) -> Result<Self, Self::Error> {
        Self::try_new(value.0.as_str())
    }
}

impl From<Signature> for SignatureDTO {
    fn from(value: Signature) -> Self {
        Self(value.signature().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::newspaper::Signature;

    use super::SignatureDTO;

    #[test]
    fn serialize() {
        let signature = Signature::try_new("В3452").unwrap();
        let dto: SignatureDTO = signature.into();
        let serialized = serde_json::to_string(&dto).unwrap();

        assert_eq!(serialized, r#""В3452""#)
    }

    #[test]
    fn deserialize() {
        let dto: SignatureDTO = serde_json::from_str(r#""В2749""#).unwrap();
        let signature = Signature::try_from(dto).unwrap();

        assert_eq!(signature.signature(), "В2749")
    }
}
