#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::net::IpAddr;

// A representative serde struct/enum pair with the shapes the serde bridge has
// dedicated code paths for: primitives, String, Option, Vec, and all three
// enum-variant kinds.
#[derive(Debug, PartialEq, Serialize, Deserialize, Arbitrary)]
struct SerdeFuzzStruct {
    a: u32,
    b: String,
    c: Option<i64>,
    d: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Arbitrary)]
enum SerdeFuzzEnum {
    Unit,
    Tuple(u32, String),
    Named { x: i64, y: String },
}

// A self-referential type. Decoding attacker-controlled bytes into it must be
// stopped by the serde deserializer's recursion-depth guard and return an
// error — never recurse until the stack overflows and the process aborts.
#[derive(Debug, Serialize, Deserialize)]
enum Tree {
    Leaf,
    Node(Box<Tree>),
}

fuzz_target!(|data: (&[u8], SerdeFuzzStruct, SerdeFuzzEnum)| {
    let (raw, s, e) = data;
    let cfg = oxicode::config::standard();

    // 1. Feeding arbitrary bytes through the serde deserializers must never
    //    panic, abort, or run unbounded — this is the path carrying the serde
    //    recursion guard, the `usize::try_from` length conversion (no
    //    `usize::MAX` sentinel collision), and the seq/map container claims.
    let _ = oxicode::serde::decode_owned_from_slice::<SerdeFuzzStruct, _>(raw, cfg);
    let _ = oxicode::serde::decode_owned_from_slice::<SerdeFuzzEnum, _>(raw, cfg);
    // Zero-sized and map element types exercise the sentinel / claim edges.
    let _ = oxicode::serde::decode_owned_from_slice::<Vec<()>, _>(raw, cfg);
    let _ = oxicode::serde::decode_owned_from_slice::<BTreeMap<u16, String>, _>(raw, cfg);
    // `is_human_readable`-sensitive type.
    let _ = oxicode::serde::decode_owned_from_slice::<IpAddr, _>(raw, cfg);
    // Recursive type: must error under the depth guard, not overflow the stack.
    let _ = oxicode::serde::decode_owned_from_slice::<Tree, _>(raw, cfg);

    // Also fuzz the borrowed deserializer, which mirrors the owned one and
    // carries the same guards.
    let _ = oxicode::serde::decode_from_slice::<SerdeFuzzStruct, _>(raw, cfg);

    // 2. Round-trip: a value encoded through the serde bridge must decode back
    //    to itself.
    if let Ok(enc) = oxicode::serde::encode_to_vec(&s, cfg) {
        if let Ok((dec, _)) =
            oxicode::serde::decode_owned_from_slice::<SerdeFuzzStruct, _>(&enc, cfg)
        {
            assert_eq!(s, dec);
        }
    }
    if let Ok(enc) = oxicode::serde::encode_to_vec(&e, cfg) {
        if let Ok((dec, _)) = oxicode::serde::decode_owned_from_slice::<SerdeFuzzEnum, _>(&enc, cfg)
        {
            assert_eq!(e, dec);
        }
    }
});
