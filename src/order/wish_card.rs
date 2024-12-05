use serde::{Deserialize, Serialize};

use super::{Error, Result};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
// TODO insert newspapers(as 'covers')
pub(super) struct WishCard {
    background: Rgb,
    frame: Frame,
    message: String,
    font_type: String,
    font_size: u8,
    template_id: u8,
}

impl WishCard {
    pub(super) fn new_unchecked(
        background: Rgb,
        frame: Frame,
        message: String,
        font_type: String,
        font_size: u8,
        template_id: u8,
    ) -> Self {
        Self {
            background,
            frame,
            message,
            font_type,
            font_size,
            template_id,
        }
    }

    pub(super) fn check(&self, max_cards: MaxCards) -> Result<()> {
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

#[cfg(test)]
mod test {
    use super::{Frame, Result, Rgb, WishCard};

    #[test]
    fn deserialize() {
        let json = r#"{"background":[255,0,0],"frame":"White","message":"Честит рожден ден!","font_type":"Times New Roman","font_size":12,"template_id":10}"#;
        let unchecked: WishCard = serde_json::from_str(json).expect("failed to deserialize JSON");
        let expected = WishCard::new_unchecked(
            Rgb::new(255, 0, 0),
            Frame::White,
            "Честит рожден ден!".to_string(),
            "Times New Roman".to_string(),
            12,
            10,
        );

        assert_eq!(expected, unchecked)
    }

    #[test]
    fn serialize() {
        let details = WishCard::new_unchecked(
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
            r#"{"background":[123,23,255],"frame":"Wooden","message":"Честит юбилей!","font_type":"Arial","font_size":16,"template_id":11}"#
        )
    }

    // fn assert_err(r: Result<WishCard>, msg: &str) {
    //     assert!(r.expect_err("expected an error").to_string().contains(msg))
    // }
}
