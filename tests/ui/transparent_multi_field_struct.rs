//! `#[oxicode(transparent)]` requires the struct to have exactly one field
//! (the wrapped value that encoding/decoding is forwarded to). A struct with
//! more than one field has no unambiguous "the" field to forward to, so this
//! must be rejected at compile time rather than silently picking one.

use oxicode::Encode;

#[derive(Encode)]
#[oxicode(transparent)]
struct Pair {
    a: u32,
    b: u32,
}

fn main() {}
