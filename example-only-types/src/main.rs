include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

use std::io::Cursor;
use types::*;

fn main() -> anyhow::Result<()> {
    let b: EnrolBody = EnrolBody::builder()
        .host("localhost")
        .key("abc")
        .try_into()
        .unwrap();

    let mut byte_buffer: Vec<u8> = vec![];

    // Now let's check that de(ser(b)) == b, using CBOR.

    ciborium::into_writer(&b, &mut byte_buffer)?;

    let b1: EnrolBody = ciborium::from_reader(Cursor::new(&byte_buffer))?;

    assert_eq!(b, b1);

    Ok(())
}
