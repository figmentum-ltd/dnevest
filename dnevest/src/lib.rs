use bindings::{ByteArray, Guest};


#[allow(warnings)]
#[rustfmt::skip]
mod bindings;
mod newspaper;

struct Component;

impl Guest for Component {
    fn execute(cmd: ByteArray) -> Result<Vec<bindings::Event>, ByteArray> {
        todo!()
    }

    fn query(_req: ByteArray) -> Result<Vec<ByteArray>, ByteArray> {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
