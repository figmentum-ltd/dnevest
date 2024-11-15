#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
use crate::{
    bindings::{component::dnevest::time::Clock, ByteArray},
    Storage,
};

#[cfg(test)]
use super::Newspaper;

#[cfg(test)]
pub(crate) const CURRENT_YEAR: crate::newspaper::Year = 2024;

#[cfg(test)]
pub(crate) fn current_year() -> Clock {
    Clock { year: CURRENT_YEAR }
}

#[cfg(test)]
pub(crate) struct MockHost {
    store: HashMap<String, ByteArray>,
}

#[cfg(test)]
impl MockHost {
    pub(crate) fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub(crate) fn with_newspapers() -> Self {
        let mut host = Self::new();
        Self::load_newspapers().into_iter().for_each(|newspaper| {
            let serialized = serde_json::to_vec(&newspaper).expect("Failed to serialize Newspaper");
            host.persist(newspaper.identificator(), &serialized);
        });
        host
    }

    fn load_newspapers() -> Vec<Newspaper> {
        vec![
            Newspaper::new_unchecked(
                "В4667",
                "Орбита",
                1969,
                Some(1991),
                [false, false, false, false, false, true, false],
            ),
            Newspaper::new_unchecked(
                "В1616",
                "Народен спор",
                1944,
                Some(1989),
                [true, false, false, true, false, true, false],
            ),
            Newspaper::new_unchecked(
                "В1612",
                "Труд",
                1946,
                None,
                [true, true, true, true, true, true, true],
            ),
        ]
    }
}

#[cfg(test)]
impl Storage for MockHost {
    fn persist(&mut self, key: &str, req: &ByteArray) {
        self.store.insert(key.to_string(), req.clone());
    }

    fn retrieve(&mut self, key: &str) -> Option<ByteArray> {
        self.store.get(key).cloned()
    }

    fn retrieve_range(&mut self, _start: &str, _end: &str) -> Vec<ByteArray> {
        self.store.values().cloned().collect()
    }
}
