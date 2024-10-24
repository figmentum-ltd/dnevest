use std::result::Result as StdResult;

use chrono::Weekday;

use crate::{
    bindings::{self, ByteArray},
    newspaper::{self, Date, DateDTO, Newspaper, NewspaperDTO},
    response::Event,
};

mod error;

pub(super) use error::Error as ServiceError;

pub fn create_newspaper(input: NewspaperDTO) -> StdResult<bindings::Event, ByteArray> {
    input
        .try_into()
        .map_err(|err| ServiceError::InvalidNewspaper(err))
        .and_then(new_newspaper)
        .or_else(|error| error::serialize_errors(vec![error]))
}

pub fn newspapers_by_date(date_dto: DateDTO) -> Result<ByteArray, ByteArray> {
    // date_dto
    //     .try_into()
    //     .map_err(|err| ServiceError::InvalidDate(err))
    //     .and_then(find_newspapers)
    //     .or_else(|error| {
    //         //serialize_errors(vec![error])

    //     })
    todo!()
}
// parse_date_from_json(json_data).and_then(|date| {
//     //call to import all stored newspapers
//     let newspapers: Vec<Newspaper> = load_newspapers();
//     let published_newspapers = newspapers_by_date(json_data);
// })

// TODO! - do we need 'newspaper' to pe present in every name
fn new_newspaper(newspaper: Newspaper) -> StdResult<bindings::Event, ServiceError> {
    // TODO! remove the cloning
    let obj = newspaper.clone();
    let signature = obj.signature_str();
    let dto: NewspaperDTO = newspaper.into();

    let serialized_newspaper =
        serde_json::to_vec(&dto).map_err(|_| ServiceError::SerializationFault)?;
    // TODO! abstract persistence API
    bindings::persist("dto.signature", &serialized_newspaper);

    let serialized_event = Event::NewspaperCreated(signature).serialize_event()?;
    Ok(bindings::Event {
        id: "dnevest_n_n".to_string(),
        content: serialized_event,
    })
}

// fn find_newspapers(date: Date) -> Result<Vec<ByteArray>, ServiceError> {
//     let newspapers = newspaper::load_newspapers();
// }
