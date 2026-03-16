#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use oxicode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode, Arbitrary)]
struct FuzzStruct {
    a: u32,
    b: String,
    c: bool,
    d: Option<u64>,
}

#[derive(Debug, PartialEq, Encode, Decode, Arbitrary)]
enum FuzzEnum {
    Unit,
    Tuple(u32, u32),
    Named { x: i64, y: String },
}

fuzz_target!(|data: (&[u8], FuzzStruct, FuzzEnum)| {
    let (_, s, e) = data;

    // Encode then decode must produce the same value
    if let Ok(enc) = oxicode::encode_to_vec(&s) {
        if let Ok((dec, _)) = oxicode::decode_from_slice::<FuzzStruct>(&enc) {
            assert_eq!(s, dec);
        }
    }

    if let Ok(enc) = oxicode::encode_to_vec(&e) {
        if let Ok((dec, _)) = oxicode::decode_from_slice::<FuzzEnum>(&enc) {
            assert_eq!(e, dec);
        }
    }
});
