//! A variant-level `#[oxicode(skip)]` may only alias a successor with an
//! identical field shape. Here the skipped `B(String)` would alias `C(u8)`,
//! whose fields differ, so encoding `B` would decode as a corrupted `C` and
//! desynchronize the stream. The derive must reject this at compile time.

use oxicode::Encode;

#[derive(Encode)]
enum Shape {
    A(u32),
    #[oxicode(skip)]
    B(String),
    C(u8),
}

fn main() {}
