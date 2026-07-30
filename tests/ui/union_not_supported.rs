//! `#[derive(Encode, Decode, BorrowDecode)]` must be rejected on unions: none
//! of the three traits can be safely derived for a `union` (no way to know
//! which field is active), so each derive macro must surface a clear
//! compile error instead of generating unsound code.

use oxicode::{BorrowDecode, Decode, Encode};

#[derive(Encode, Decode, BorrowDecode)]
union Overlapped {
    a: u32,
    b: f32,
}

fn main() {}
