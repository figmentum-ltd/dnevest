use bindings::{
    component::dnevest::{
        storage,
        time::{self, Clock},
    },
    ByteArray, Guest,
};

use msgs::{ExecuteMsg, QueryMsg};

#[allow(warnings)]
#[rustfmt::skip]
mod bindings;
mod errors;
mod msgs;
mod newspaper;
mod order;
mod response;
mod services;

struct Component;

impl Guest for Component {
    fn execute(cmd: ByteArray) -> Result<Vec<bindings::Event>, Vec<u8>> {
        msgs::deserialize_msg(cmd).and_then(|msg| match msg {
            ExecuteMsg::CreateNewspaper { input } => services::create_newspaper::<Host>(input),
            ExecuteMsg::AddFinalYear {
                signature,
                final_year,
            } => services::add_final_year::<Host, Host>(signature, final_year),
            ExecuteMsg::SpecifyMaxCards { max_number } => {
                services::specify_max_cards::<Host>(max_number)
            }
            ExecuteMsg::CreateOrder { order } => services::create_order(order),
        })
    }

    fn query(req: ByteArray) -> Result<ByteArray, ByteArray> {
        msgs::deserialize_msg(req).and_then(|msg| match msg {
            QueryMsg::NewspapersByDate { date } => services::newspapers_by_date::<Host, Host>(date),
        })
    }
}

#[derive(Default)]
struct Host;

trait Storage {
    fn persist(&mut self, key: &str, req: &ByteArray);

    fn retrieve(&self, key: &str) -> Option<ByteArray>;

    fn retrieve_range(&self, start: &str, end: &str) -> Vec<ByteArray>;
}

impl Storage for Host {
    fn persist(&mut self, key: &str, req: &ByteArray) {
        storage::persist(key, req)
    }

    fn retrieve(&self, key: &str) -> Option<ByteArray> {
        storage::retrieve(key)
    }

    fn retrieve_range(&self, start: &str, end: &str) -> Vec<ByteArray> {
        storage::retrieve_range(start, end)
    }
}

trait Time {
    fn now() -> Clock;
}

impl Time for Host {
    fn now() -> Clock {
        time::now()
    }
}

bindings::export!(Component with_types_in bindings);
