use serde::{Deserialize, Serialize};

/// Represents the frequency of publication of a newspaper over the course of a week.
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(transparent)]
pub(crate) struct WeeklyFrequency([bool; 7]);

impl WeeklyFrequency {
    #[cfg(test)]
    pub(crate) fn new(days: [bool; 7]) -> Self {
        Self(days)
    }

    pub(super) fn published_on(&self, day_index: usize) -> bool {
        debug_assert!(
            day_index < 7,
            "Index out of bounds: {}. Expected 0-6.",
            day_index
        );

        self.0[day_index]
    }
}

#[cfg(test)]
mod tests {
    use super::WeeklyFrequency;

    #[test]
    fn published_on() {
        let newspaper_fr = WeeklyFrequency::new([false, true, false, true, false, false, false]);

        assert!(
            newspaper_fr.published_on(1),
            "Newspaper should be published on Tuesday"
        );
        assert!(
            newspaper_fr.published_on(3),
            "Newspaper should be published on Thursday"
        );
        assert!(
            !newspaper_fr.published_on(5),
            "Newspaper should not be published on Saturday"
        );
    }
}
