#[cfg(test)]
use std::collections::HashMap;

use crate::Time;
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
pub(crate) struct MockHost {
    store: HashMap<String, ByteArray>,
}

#[cfg(test)]
impl MockHost {
    pub(crate) fn with_newspapers() -> Self {
        let mut host = Self::default();
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

impl Default for MockHost {
    fn default() -> Self {
        let mut host = Self {
            store: Default::default(),
        };

        let max_cards = serde_json::to_vec(&40).expect("Failed to serialize max_cards");
        host.persist("max_cards", &max_cards);
        host
    }
}

#[cfg(test)]
impl Storage for MockHost {
    fn persist(&mut self, key: &str, value: &ByteArray) {
        self.store.insert(key.to_string(), value.clone());
    }

    fn retrieve(&self, key: &str) -> Option<ByteArray> {
        self.store.get(key).cloned()
    }

    fn retrieve_range(&self, _start: &str, _end: &str) -> Vec<ByteArray> {
        self.store.values().cloned().collect()
    }
}

#[cfg(test)]
impl Time for MockHost {
    fn now() -> Clock {
        Clock {
            timestamp: 1732880395,
        }
    }
}
