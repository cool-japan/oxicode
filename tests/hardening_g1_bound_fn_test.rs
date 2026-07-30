//! WP-G1 hardening: the `bound = "..."` splitter must handle parenthesized (Fn-trait) predicates
//! whose argument lists contain commas, instead of naively splitting on every top-level comma
//! (finding derive-audit#3, TODO line 424).

#![cfg(all(feature = "alloc", feature = "derive"))]
use oxicode::{config, Decode, Encode};
use std::marker::PhantomData;

// The custom bound REPLACES the auto-generated `F: Encode/Decode` bounds. The `Fn(u8, u8) -> u8`
// predicate contains a comma inside the parentheses; before the fix the splitter cut it into two
// invalid fragments and produced a spurious "invalid where predicate" error.
#[derive(Encode, Decode, PartialEq, Debug)]
#[oxicode(bound = "F: Fn(u8, u8) -> u8")]
struct Holder<F> {
    #[oxicode(skip)]
    callback: PhantomData<F>,
    value: u32,
}

type AddFn = fn(u8, u8) -> u8;

fn add(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

#[test]
fn bound_with_fn_trait_predicate_compiles_and_roundtrips() {
    let value: Holder<AddFn> = Holder {
        callback: PhantomData,
        value: 99,
    };
    let encoded = oxicode::encode_to_vec_with_config(&value, config::standard()).expect("encode");
    let (decoded, _): (Holder<AddFn>, usize) =
        oxicode::decode_from_slice_with_config(&encoded, config::standard()).expect("decode");
    assert_eq!(decoded, value);
    // Reference `add` so the Fn-typed parameter is genuinely exercised.
    assert_eq!(add(1, 2), 3);
}

// A multi-predicate bound whose predicates each contain nested `<>` and `()` must still split at
// the correct top-level commas.
#[derive(Encode, Decode, PartialEq, Debug)]
#[oxicode(bound = "T: Into<Vec<u8>> + Clone, T: core::fmt::Debug")]
struct Multi<T> {
    #[oxicode(skip)]
    marker: PhantomData<T>,
    payload: Vec<u16>,
}

#[test]
fn bound_multi_predicate_with_nested_generics_roundtrips() {
    let value: Multi<Vec<u8>> = Multi {
        marker: PhantomData,
        payload: vec![1, 2, 3],
    };
    let encoded = oxicode::encode_to_vec_with_config(&value, config::standard()).expect("encode");
    let (decoded, _): (Multi<Vec<u8>>, usize) =
        oxicode::decode_from_slice_with_config(&encoded, config::standard()).expect("decode");
    assert_eq!(decoded, value);
}
