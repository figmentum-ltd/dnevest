use serde::Deserialize;
use std::result::Result;

use crate::{
    bindings::ByteArray,
    errors::Error,
    newspaper::{DateDTO, NewspaperDTO},
};

//TODO! use Newspaper
#[derive(Deserialize)]
pub enum ExecuteMsg {
    CreateNewspaper { input: NewspaperDTO },
}

//TODO! use Date instead of DateDTO
#[derive(Deserialize)]
pub enum QueryMsg {
    NewspapersByDate { date: DateDTO },
}

//TODO! Parameterize the functions
pub(crate) fn deserialize_execute_msg(cmd: ByteArray) -> Result<ExecuteMsg, ByteArray> {
    serde_json::from_slice::<ExecuteMsg>(&cmd)
        .map_err(|err| Error::InvalidCommandRequest(err).to_byte_array())
}

pub(crate) fn deserialize_query_msg(req: ByteArray) -> Result<QueryMsg, ByteArray> {
    serde_json::from_slice::<QueryMsg>(&req)
        .map_err(|err| Error::InvalidQueryRequest(err).to_byte_array())
}
