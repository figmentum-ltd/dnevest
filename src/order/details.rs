use serde::{Deserialize, Serialize};

use std::{marker::PhantomData, result::Result as StdResult};

use crate::{Host, Storage};

use super::{Error, Result};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(try_from = "UncheckedDetails<Host>")]
pub(super) struct Details {
    background: Rgb,
    frame: Frame,
    wish: String,
    font_type: String,
    font_size: u8,
    card_id: u8,
}

impl Details {
    pub(super) fn new_unchecked(
        background: Rgb,
        frame: Frame,
        wish: String,
        font_type: String,
        font_size: u8,
        card_id: u8,
    ) -> Self {
        Self {
            background,
            frame,
            wish,
            font_type,
            font_size,
            card_id,
        }
    }

    fn invariant_held(&self, max_cards: MaxCards) -> Result<()> {
        if self.card_id > max_cards.0 {
            Err(Error::InvalidCard)
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MaxCards(u8);

impl MaxCards {
    pub(crate) fn new(max: u8) -> Self {
        Self(max)
    }

    pub(crate) fn number(&self) -> u8 {
        self.0
    }
}
#[derive(Deserialize)]
struct UncheckedDetails<S>
where
    S: Storage,
{
    background: Rgb,
    frame: Frame,
    wish: String,
    font_type: String,
    font_size: u8,
    card_id: u8,
    #[serde(skip)]
    _storage: PhantomData<S>,
}

impl<S> UncheckedDetails<S>
where
    S: Storage,
{
    fn into_checked(self) -> Details {
        Details::new_unchecked(
            self.background,
            self.frame,
            self.wish,
            self.font_type,
            self.font_size,
            self.card_id,
        )
    }
}

impl<S> TryFrom<UncheckedDetails<S>> for Details
where
    S: Storage + Default,
{
    type Error = Error;

    fn try_from(unchecked: UncheckedDetails<S>) -> StdResult<Self, Self::Error> {
        S::default()
            .retrieve("max_cards")
            .ok_or(Error::NotFound("Failed to fetch the max cards."))
            .and_then(|max_cards| {
                serde_json::from_slice(&max_cards)
                    .map_err(Error::DeserializationFault)
                    .and_then(|max_cards| {
                        let obj = unchecked.into_checked();
                        obj.invariant_held(max_cards).map(|()| obj)
                    })
            })
    }
}

// Since Rbg only accepts `u8` values, there is no need to check the invariant
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(super) struct Rgb([u8; 3]);

impl Rgb {
    #[cfg(test)]
    pub(super) fn new(red: u8, green: u8, blue: u8) -> Self {
        Self([red, green, blue])
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(super) enum Frame {
    White,
    Black,
    Wooden,
}

#[cfg(test)]
mod test {
    use crate::services::MockHost;

    use super::{Details, Frame, Result, Rgb, UncheckedDetails};

    #[test]
    fn deserialize() {
        let json = r#"{"background":[255,0,0],"frame":"White","wish":"Честит рожден ден!","font_type":"Times New Roman","font_size":12,"card_id":10}"#;
        let unchecked: UncheckedDetails<MockHost> =
            serde_json::from_str(json).expect("failed to deserialize JSON");
        let expected = Details::new_unchecked(
            Rgb::new(255, 0, 0),
            Frame::White,
            "Честит рожден ден!".to_string(),
            "Times New Roman".to_string(),
            12,
            10,
        );

        assert_eq!(expected, unchecked.into_checked())
    }

    #[test]
    fn invalid_card() {
        let json = r#"{"background":[255,0,0],"frame":"White","wish":"Честит рожден ден!","font_type":"Times New Roman","font_size":12,"card_id":41}"#;
        let unchecked: UncheckedDetails<MockHost> =
            serde_json::from_str(json).expect("failed to deserialize JSON");

        assert_err(unchecked.try_into(), "The card number does not exist");
    }

    #[test]
    fn serialize() {
        let details = Details::new_unchecked(
            Rgb::new(123, 23, 255),
            Frame::Wooden,
            "Честит юбилей!".to_string(),
            "Arial".to_string(),
            16,
            11,
        );
        let serialized = serde_json::to_string(&details).expect("failed to serialize");
        assert_eq!(
            serialized,
            r#"{"background":[123,23,255],"frame":"Wooden","wish":"Честит юбилей!","font_type":"Arial","font_size":16,"card_id":11}"#
        )
    }

    fn assert_err(r: Result<Details>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
