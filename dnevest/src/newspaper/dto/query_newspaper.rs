use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct QueryNewspaperDTO {
    signature: String,
    name: String,
}

impl QueryNewspaperDTO {
    fn new(signature: String, name: String) -> Self {
        Self { signature, name }
    }

    fn new_internal(signature: &str, name: &str) -> Self {
        Self {
            signature: signature.to_string(),
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QueryNewspaperDTO;
    use serde_json::to_string;

    #[test]
    fn valid_serialize() {
        let newspaper = QueryNewspaperDTO::new_internal("B1645", "Стършел");
        let serialized = to_string(&newspaper).expect("Failed to serialize");

        let expected_json = r#"{"signature":"B1645","name":"Стършел"}"#;

        assert_eq!(serialized, expected_json);
    }

}
