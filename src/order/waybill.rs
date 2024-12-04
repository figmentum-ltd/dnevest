use serde::{Deserialize, Serialize};

use std::result::Result as StdResult;

use super::{Error, Result};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(try_from = "UncheckedWaybill")]
pub(super) struct Waybill {
    customer_names: String,
    phone_number: String,
    address: String,
    order_type: OrderType,
}

impl Waybill {
    pub(super) fn new_unchecked(
        customer_names: String,
        phone_number: String,
        address: String,
        order_type: OrderType,
    ) -> Self {
        Self {
            customer_names,
            phone_number,
            address,
            order_type,
        }
    }

    pub(super) fn phone(&self) -> &str {
        &self.phone_number
    }

    // TODO: check address
    fn invariant_held(&self) -> Result<()> {
        check_names(&self.customer_names).and_then(|()| check_phone(&self.phone_number))
    }
}

fn check_names(names: &str) -> Result<()> {
    if names.split_whitespace().count() >= 2 {
        Ok(())
    } else {
        Err(Error::InvalidWaybill(
            "The customer has to supply at least two names.",
        ))
    }
}

fn check_phone(number: &str) -> Result<()> {
    number
        .strip_prefix("+359")
        .or_else(|| number.strip_prefix("0"))
        .map_or_else(
            || {
                Err(Error::InvalidWaybill(
                    "Phone number must start with 0 or +359",
                ))
            },
            |digits_only| {
                if digits_only.len() == 9 && digits_only.chars().all(|c| c.is_ascii_digit()) {
                    Ok(())
                } else {
                    Err(Error::InvalidWaybill("Wrong number of digits"))
                }
            },
        )
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Serialize, Deserialize)]
pub(super) enum OrderType {
    Standart,
    Express,
}

#[derive(Deserialize)]
struct UncheckedWaybill {
    customer_names: String,
    phone_number: String,
    address: String,
    order_type: OrderType,
}

impl UncheckedWaybill {
    fn into_checked(self) -> Waybill {
        Waybill::new_unchecked(
            self.customer_names,
            self.phone_number,
            self.address,
            self.order_type,
        )
    }
}

impl TryFrom<UncheckedWaybill> for Waybill {
    type Error = Error;

    fn try_from(unchecked: UncheckedWaybill) -> StdResult<Self, Self::Error> {
        let obj = unchecked.into_checked();
        obj.invariant_held().map(|()| obj)
    }
}

#[cfg(test)]
mod test_invariant {
    use super::{check_names, check_phone, Result};

    #[test]
    fn valid_names() {
        assert!(check_names("Иван Костадинов").is_ok());
        assert!(check_names("Анна-Мария Йорданова").is_ok());
        assert!(check_names("Стоян Руменов Тодоров").is_ok());
        assert!(check_names("Христина Асенова-Петрова").is_ok());

        assert!(check_names("Hristo Petkov").is_ok());
        assert!(check_names("Svilena Manolova-Donkova").is_ok());
    }

    #[test]
    fn invalid_names() {
        let msg = "The customer has to supply at least two names.";
        assert_err(check_names("Генади"), msg);
        assert_err(check_names("Galka"), msg);
    }

    #[test]
    fn valid_phone() {
        assert!(check_phone("0893471823").is_ok());
        assert!(check_phone("+359461839203").is_ok());
    }

    #[test]
    fn invalid_phone() {
        assert_err(
            check_phone("9838774692"),
            "Phone number must start with 0 or +359",
        );

        let msg = "Wrong number of digits";
        assert_err(check_phone("038776492"), msg);
        assert_err(check_phone("+35938776492"), msg);
        assert_err(check_phone("+359 238776492"), msg);
    }

    fn assert_err(r: Result<()>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}

#[cfg(test)]
mod test {
    use super::{OrderType, Result, UncheckedWaybill, Waybill};

    #[test]
    fn deserialize() {
        let json = r#"{"customer_names":"Тодор Георгиев","phone_number":"0873528495","address":"Пловдив, ул.Тракия 12","order_type":"Standart"}"#;
        let unchecked: UncheckedWaybill =
            serde_json::from_str(json).expect("failed to deserialize JSON");
        assert_eq!(waybill(), unchecked.into_checked())
    }

    #[test]
    fn deserialization_err() {
        let json = r#"{"customer_names":"Тодор Георгиев","phone_number":"+358873528495","address":"Пловдив, ул.Тракия 12","order_type":"Standart"}"#;
        let unchecked: UncheckedWaybill =
            serde_json::from_str(json).expect("failed to deserialize JSON");
        assert_err(
            unchecked.try_into(),
            "Phone number must start with 0 or +359",
        )
    }

    #[test]
    fn serialize() {
        let waybill = waybill();
        let serialized = serde_json::to_string(&waybill).expect("failed to serialize");
        assert_eq!(
            serialized,
            r#"{"customer_names":"Тодор Георгиев","phone_number":"0873528495","address":"Пловдив, ул.Тракия 12","order_type":"Standart"}"#
        )
    }

    fn waybill() -> Waybill {
        Waybill::new_unchecked(
            "Тодор Георгиев".to_string(),
            "0873528495".to_string(),
            "Пловдив, ул.Тракия 12".to_string(),
            OrderType::Standart,
        )
    }

    fn assert_err(r: Result<Waybill>, msg: &str) {
        assert!(r.expect_err("expected an error").to_string().contains(msg))
    }
}
