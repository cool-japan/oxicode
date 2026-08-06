//! Two decodable variants resolving to the same discriminant is a compile
//! error: `A` takes positional discriminant 0 and `B` explicitly requests 0,
//! so `B` would be unreachable on decode and silently mis-decode as `A`.

use oxicode::Encode;

#[derive(Encode)]
enum Dup {
    A,
    #[oxicode(variant = 0)]
    B,
}

fn main() {}
