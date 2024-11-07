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
    fn execute(cmd: ByteArray) -> Result<bindings::Event, ByteArray> {
        msgs::deserialize_msg(cmd).and_then(|msg| match msg {
            ExecuteMsg::CreateNewspaper { input } => {
                services::create_newspaper(&mut Component, input)
            }
        })
    }

    fn query(req: ByteArray) -> Result<ByteArray, ByteArray> {
        msgs::deserialize_msg(req).and_then(|msg| match msg {
            QueryMsg::NewspapersByDate { date } => {
                services::newspapers_by_date(&mut Component, date)
            }
        })
    }
}

trait HostImports {
    fn persist(&mut self, key: &str, req: &ByteArray);

    fn retrieve(&mut self, key: &str) -> Option<ByteArray>;

    fn retrieve_range(&mut self, start: &str, end: &str) -> Vec<ByteArray>;
}

impl HostImports for Component {
    fn persist(&mut self, key: &str, req: &ByteArray) {
        bindings::persist(key, req)
    }

    fn retrieve(&mut self, key: &str) -> Option<ByteArray> {
        bindings::retrieve(key)
    }

    fn retrieve_range(&mut self, start: &str, end: &str) -> Vec<ByteArray> {
        bindings::retrieve_range(start, end)
    }
}

bindings::export!(Component with_types_in bindings);
