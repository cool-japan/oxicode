//! `#[oxicode(transparent)]` is a struct-only container attribute (it forwards
//! encode/decode to the struct's single field). Applying it to an enum has no
//! sensible meaning, so the derive macros must reject it with a clear error.

use oxicode::Encode;

#[derive(Encode)]
#[oxicode(transparent)]
enum Wrapper {
    A(u32),
}

fn main() {}
