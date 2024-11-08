use serde::Serialize;

#[cfg(test)]
use serde::Deserialize;

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq, Hash, Deserialize))]
pub struct QueryNewspaperDTO {
    signature: String,
    name: String,
}

impl QueryNewspaperDTO {
    pub(crate) fn new(signature: String, name: String) -> Self {
        Self { signature, name }
    }

    #[cfg(test)]
    pub(crate) fn new_test(signature: &str, name: &str) -> Self {
        Self::new(signature.to_string(), name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::QueryNewspaperDTO;
    use serde_json::to_string;

    #[test]
    fn valid_serialize() {
        let newspaper = QueryNewspaperDTO::new_test("B1645", "Стършел");
        let serialized = to_string(&newspaper).expect("Failed to serialize");

        let expected_json = r#"{"signature":"B1645","name":"Стършел"}"#;

        assert_eq!(serialized, expected_json);
    }
}
