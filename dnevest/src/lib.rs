use bindings::{ByteArray, Guest};

use msgs::{ExecuteMsg, QueryMsg};

#[allow(warnings)]
#[rustfmt::skip]
mod bindings;
mod errors;
mod msgs;
mod newspaper;
mod response;
mod services;

struct Component;

impl Guest for Component {
    fn execute(cmd: ByteArray) -> Result<Vec<bindings::Event>, ByteArray> {
        msgs::deserialize_execute_msg(cmd).and_then(|msg| match msg {
            ExecuteMsg::CreateNewspaper { input } => services::create_newspaper(input),
        })
    }

    fn query(req: ByteArray) -> Result<Vec<ByteArray>, ByteArray> {
        msgs::deserialize_query_msg(req).and_then(|msg| match msg {
            QueryMsg::NewspapersByDate { date } => services::newspapers_by_date(date),
        })
    }
}

bindings::export!(Component with_types_in bindings);
