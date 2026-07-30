//! `#[oxicode(bytes)]` selects a bulk raw-byte encode/decode path that reads
//! the field back out of the value; `#[oxicode(skip)]` binds the field as `_`
//! (never constructed from the wire) instead. The two are mutually exclusive
//! on any field — including a named field inside an enum variant, which is
//! exactly the case that previously slipped through (the field is pattern-
//! bound as `_field` when skipped, but the `bytes` codegen path referenced
//! the un-underscored `field` identifier, causing an unbound-identifier
//! error deep in generated code instead of a clear message at the attribute
//! site).

use oxicode::Encode;

#[derive(Encode)]
enum Message {
    Payload {
        #[oxicode(skip, bytes)]
        data: Vec<u8>,
    },
}

fn main() {}
