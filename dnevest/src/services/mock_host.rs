use std::collections::HashMap;

use crate::{
    bindings::ByteArray,
    newspaper::{Signature, WeeklyFrequency},
    HostImports,
};

use super::Newspaper;

pub(crate) struct MockHost {
    store: HashMap<String, ByteArray>,
}

impl MockHost {
    pub(crate) fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn load_newspapers() -> Vec<ByteArray> {
        let newspapers = vec![
            Newspaper::new(
                Signature::new("В4667"),
                "Орбита".to_string(),
                1969,
                Some(1991),
                WeeklyFrequency::new([false, false, false, false, false, true, false]),
            ),
            Newspaper::new(
                Signature::new("В1616"),
                "Народен спор".to_string(),
                1944,
                Some(1989),
                WeeklyFrequency::new([true, false, false, true, false, true, false]),
            ),
            Newspaper::new(
                Signature::new("В1612"),
                "Труд".to_string(),
                1946,
                None,
                WeeklyFrequency::new([true, true, true, true, true, true, true]),
            ),
        ];
        newspapers
            .into_iter()
            .map(|newspaper| serde_json::to_vec(&newspaper).expect("Failed to serialize Newspaper"))
            .collect()
    }
}

impl HostImports for MockHost {
    fn persist(&mut self, key: &str, req: &ByteArray) {
        self.store.insert(key.to_string(), req.clone());
    }

    fn retrieve_range(&mut self, _start: &str, _end: &str) -> Vec<ByteArray> {
        MockHost::load_newspapers()
    }
}
