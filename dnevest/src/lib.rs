use bindings::{ByteArray, Guest};


#[allow(warnings)]
#[rustfmt::skip]
mod bindings;

struct Component;

impl Guest for Component {
    fn execute(cmd: ByteArray) -> Result<Vec<bindings::Event>, ()> {
        todo!()
    }

    fn query(req: ByteArray) -> Result<Vec<ByteArray>, ()> {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
