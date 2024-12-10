use serde::{Deserialize, Serialize};

use std::{marker::PhantomData, result::Result as StdResult};

use crate::Storage;

use super::{
    cover::{Cover, UncheckedCover},
    Error, Result,
};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
pub(super) struct WishCard {
    covers: Cover,
    background: Rgb,
    frame: Frame,
    message: String,
    font_type: String,
    font_size: u8,
    template_id: u8,
}

impl WishCard {
    pub(super) fn new_unchecked(
        covers: Cover,
        background: Rgb,
        frame: Frame,
        message: String,
        font_type: String,
        font_size: u8,
        template_id: u8,
    ) -> Self {
        Self {
            covers,
            background,
            frame,
            message,
            font_type,
            font_size,
            template_id,
        }
    }

    pub(super) fn invariant_held(&self, max_cards: MaxCards) -> Result<()> {
        if self.template_id > max_cards.0 {
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

#[derive(Deserialize)]
#[serde(bound = "")]
pub(super) struct UncheckedWishCard<S>
where
    S: Storage + Default,
{
    covers: UncheckedCover<S>,
    background: Rgb,
    frame: Frame,
    message: String,
    font_type: String,
    font_size: u8,
    template_id: u8,
    #[serde(skip)]
    _storage: PhantomData<S>,
}

impl<S> UncheckedWishCard<S>
where
    S: Storage + Default,
{
    fn into_checked(self) -> Result<WishCard> {
        self.covers.try_into().map(|covers| {
            WishCard::new_unchecked(
                covers,
                self.background,
                self.frame,
                self.message,
                self.font_type,
                self.font_size,
                self.template_id,
            )
        })
    }
}

impl<S> TryFrom<UncheckedWishCard<S>> for WishCard
where
    S: Storage + Default,
{
    type Error = Error;

    fn try_from(unchecked: UncheckedWishCard<S>) -> StdResult<Self, Self::Error> {
        S::default()
            .retrieve("max_cards")
            .ok_or(Error::NotFound("Failed to fetch the max cards.".into()))
            .and_then(|max_cards| {
                serde_json::from_slice(&max_cards)
                    .map_err(Error::DeserializationFault)
                    .and_then(|max_cards| {
                        unchecked
                            .into_checked()
                            .and_then(|obj| obj.invariant_held(max_cards).map(|()| obj))
                    })
            })
    }
}

#[cfg(test)]
mod test {
    use crate::{newspaper::Signature, order::cover::Cover, services::MockHost};

    use super::{Frame, Result, Rgb, UncheckedWishCard, WishCard};

    #[test]
    fn unchecked_deserialization() {
        let json = r#"{"covers":{"preference":"В1616","options":["В4667",null]},"background":[255,0,0],"frame":"White","message":"Честит рожден ден!","font_type":"Times New Roman","font_size":12,"template_id":10}"#;
        let unchecked: WishCard = serde_json::from_str(json).expect("failed to deserialize JSON");
        let expected = WishCard::new_unchecked(
            covers(),
            Rgb::new(255, 0, 0),
            Frame::White,
            "Честит рожден ден!".into(),
            "Times New Roman".into(),
            12,
            10,
        );

        assert_eq!(expected, unchecked)
    }

    #[test]
    fn checked_deserialization() {
        let _host = MockHost::default();

        let json = r#"{"covers":{"preference":"В1616","options":["В4667",null]},"background":[255,0,0],"frame":"White","message":"Честит рожден ден!","font_type":"Times New Roman","font_size":12,"template_id":41}"#;
        let unchecked: UncheckedWishCard<MockHost> =
            serde_json::from_str(json).expect("failed to deserialize JSON");

        let res: Result<WishCard> = unchecked.try_into();

        assert_err(res, "The card number does not exist");
    }

    #[test]
    fn serialize() {
        let details = WishCard::new_unchecked(
            covers(),
            Rgb::new(123, 23, 255),
            Frame::Wooden,
            "Честит юбилей!".into(),
            "Arial".into(),
            16,
            11,
        );
        let serialized = serde_json::to_string(&details).expect("failed to serialize");
        assert_eq!(
            serialized,
            r#"{"covers":{"preference":"В1616","options":["В4667",null]},"background":[123,23,255],"frame":"Wooden","message":"Честит юбилей!","font_type":"Arial","font_size":16,"template_id":11}"#
        )
    }

    fn assert_err(r: Result<WishCard>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }

    fn covers() -> Cover {
        Cover::new_unchecked(
            Signature::new("В1616"),
            [Some(Signature::new("В4667")), None],
        )
    }
}
