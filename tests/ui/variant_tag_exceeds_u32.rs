//! `#[oxicode(variant = N)]` assigns an explicit discriminant, but the wire
//! width is controlled by the container's `tag_type` (default `u32`). A
//! discriminant that does not fit in the configured width must be a compile
//! error (rather than silently truncating the tag on the wire), and the
//! error should point the user at `#[oxicode(tag_type = "u64")]` as the fix.

use oxicode::Encode;

#[derive(Encode)]
enum Huge {
    #[oxicode(variant = 4294967296)] // u32::MAX + 1
    A,
}

fn main() {}
