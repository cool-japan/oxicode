//! Advanced derive macro tests — set 4
//!
//! Covers `encode_with`/`decode_with`, `bytes`, `skip`+`default`, generic
//! structs, unit structs, newtype wrappers, collection roundtrips, and
//! configuration-aware encoding.

#![allow(
    clippy::approx_constant,
    clippy::useless_vec,
    clippy::len_zero,
    clippy::unnecessary_cast,
    clippy::redundant_closure,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_borrow,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms,
    clippy::inconsistent_digit_grouping,
    clippy::unit_cmp,
    clippy::assertions_on_constants,
    clippy::iter_on_single_items,
    clippy::expect_fun_call,
    clippy::redundant_pattern_matching,
    variant_size_differences,
    clippy::absurd_extreme_comparisons,
    clippy::nonminimal_bool,
    clippy::for_kv_map,
    clippy::needless_range_loop,
    clippy::single_match,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::redundant_clone,
    clippy::map_entry,
    clippy::match_single_binding,
    clippy::bool_comparison,
    clippy::derivable_impls,
    clippy::manual_range_contains,
    clippy::needless_borrows_for_generic_args,
    clippy::manual_map,
    clippy::vec_init_then_push,
    clippy::identity_op,
    clippy::manual_flatten,
    clippy::single_char_pattern,
    clippy::search_is_some,
    clippy::option_map_unit_fn,
    clippy::while_let_on_iterator,
    clippy::clone_on_copy,
    clippy::box_collection,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::large_enum_variant,
    clippy::match_ref_pats,
    clippy::needless_pass_by_value,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::manual_strip,
    clippy::match_like_matches_macro,
    clippy::from_over_into,
    clippy::wrong_self_convention,
    clippy::inherent_to_string,
    clippy::new_without_default,
    clippy::unnecessary_wraps,
    clippy::field_reassign_with_default,
    clippy::manual_find,
    clippy::unnecessary_lazy_evaluations,
    clippy::should_implement_trait,
    clippy::missing_safety_doc,
    clippy::unusual_byte_groupings,
    clippy::bool_assert_comparison,
    clippy::zero_prefixed_literal,
    clippy::await_holding_lock,
    clippy::manual_saturating_arithmetic,
    clippy::explicit_counter_loop,
    clippy::needless_lifetimes,
    clippy::single_component_path_imports,
    clippy::uninlined_format_args,
    clippy::iter_cloned_collect,
    clippy::manual_str_repeat,
    clippy::excessive_precision,
    clippy::precedence,
    clippy::unnecessary_literal_unwrap
)]
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// encode_with / decode_with helpers
// ---------------------------------------------------------------------------

fn encode_uppercase<E: oxicode::enc::Encoder>(
    s: &String,
    enc: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let upper = s.to_uppercase();
    upper.encode(enc)
}

fn decode_uppercase<D: oxicode::de::Decoder<Context = ()>>(
    dec: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    String::decode(dec)
}

// ---------------------------------------------------------------------------
// Structs under test
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithEncoderOverride {
    #[oxicode(encode_with = "encode_uppercase", decode_with = "decode_uppercase")]
    label: String,
    value: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithBytesAttr {
    #[oxicode(bytes)]
    raw: Vec<u8>,
    id: u64,
}

fn default_count() -> u32 {
    42
}

#[derive(Debug, Encode, Decode)]
struct WithSkip {
    name: String,
    // `default = "fn"` implies skip during encode and calls fn() on decode.
    #[oxicode(default = "default_count")]
    internal_count: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair<A, B> {
    first: A,
    second: B,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UnitStruct;

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptWrapper(Option<Vec<u32>>);

// ---------------------------------------------------------------------------
// Test 1 — WithEncoderOverride: label gets uppercased during encode
// ---------------------------------------------------------------------------

#[test]
fn test_encoder_override_uppercase_on_encode() {
    let val = WithEncoderOverride {
        label: "hello world".to_string(),
        value: 7,
    };
    let bytes = encode_to_vec(&val).expect("encode WithEncoderOverride failed");
    // Re-decode the label only: the stored bytes must represent the uppercased form.
    let (decoded, _): (WithEncoderOverride, usize) =
        decode_from_slice(&bytes).expect("decode WithEncoderOverride failed");
    assert_eq!(
        decoded.label, "HELLO WORLD",
        "label must be uppercased in the wire format"
    );
}

// ---------------------------------------------------------------------------
// Test 2 — WithEncoderOverride: roundtrip preserves uppercase label
// ---------------------------------------------------------------------------

#[test]
fn test_encoder_override_roundtrip_preserves_uppercase() {
    let val = WithEncoderOverride {
        label: "oxicode".to_string(),
        value: 42,
    };
    let bytes = encode_to_vec(&val).expect("encode failed");
    let (decoded, consumed): (WithEncoderOverride, usize) =
        decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(decoded.label, "OXICODE");
    assert_eq!(decoded.value, 42);
    assert_eq!(consumed, bytes.len(), "all encoded bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 3 — WithBytesAttr: basic roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_basic_roundtrip() {
    let val = WithBytesAttr {
        raw: vec![0xDE, 0xAD, 0xBE, 0xEF],
        id: 1001,
    };
    let bytes = encode_to_vec(&val).expect("encode WithBytesAttr failed");
    let (decoded, consumed): (WithBytesAttr, usize) =
        decode_from_slice(&bytes).expect("decode WithBytesAttr failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 4 — WithBytesAttr: empty bytes vec
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_empty_vec() {
    let val = WithBytesAttr { raw: vec![], id: 0 };
    let bytes = encode_to_vec(&val).expect("encode WithBytesAttr empty failed");
    let (decoded, _): (WithBytesAttr, usize) =
        decode_from_slice(&bytes).expect("decode WithBytesAttr empty failed");
    assert_eq!(decoded, val);
    assert!(decoded.raw.is_empty());
}

// ---------------------------------------------------------------------------
// Test 5 — WithBytesAttr: bytes encode efficiently (no length doubling)
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_encodes_efficiently() {
    // The #[oxicode(bytes)] attribute must not inflate the payload.
    // A Vec<u8> of N bytes should encode to roughly N + overhead bytes,
    // not to 2*N or more.
    let payload: Vec<u8> = (0u8..64).collect();
    let val = WithBytesAttr {
        raw: payload.clone(),
        id: 99,
    };
    let bytes = encode_to_vec(&val).expect("encode failed");
    // The raw bytes (64) plus a small varint length prefix and the u64 id
    // must not exceed 64 * 2 bytes.
    assert!(
        bytes.len() <= payload.len() * 2,
        "encoded size {} should not exceed double the payload length {}",
        bytes.len(),
        payload.len() * 2,
    );
}

// ---------------------------------------------------------------------------
// Test 6 — WithSkip: decode produces default_count = 42 for skipped field
// ---------------------------------------------------------------------------

#[test]
fn test_skip_with_default_fn_produces_42() {
    let val = WithSkip {
        name: "probe".to_string(),
        internal_count: 9999, // must not be encoded
    };
    let bytes = encode_to_vec(&val).expect("encode WithSkip failed");
    let (decoded, _): (WithSkip, usize) =
        decode_from_slice(&bytes).expect("decode WithSkip failed");
    assert_eq!(
        decoded.internal_count, 42,
        "skipped field must be populated by default_count() = 42"
    );
}

// ---------------------------------------------------------------------------
// Test 7 — WithSkip: name field preserved correctly
// ---------------------------------------------------------------------------

#[test]
fn test_skip_name_field_preserved() {
    let val = WithSkip {
        name: "persistent".to_string(),
        internal_count: 0,
    };
    let bytes = encode_to_vec(&val).expect("encode failed");
    let (decoded, consumed): (WithSkip, usize) = decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(
        decoded.name, "persistent",
        "non-skipped name must survive roundtrip"
    );
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 8 — Pair<u32, String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_u32_string_roundtrip() {
    let val = Pair::<u32, String> {
        first: 123,
        second: "hello".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Pair<u32, String> failed");
    let (decoded, consumed): (Pair<u32, String>, usize) =
        decode_from_slice(&bytes).expect("decode Pair<u32, String> failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 9 — Pair<Vec<u8>, bool> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_vec_u8_bool_roundtrip() {
    let val = Pair::<Vec<u8>, bool> {
        first: vec![1, 2, 3, 4, 5],
        second: true,
    };
    let bytes = encode_to_vec(&val).expect("encode Pair<Vec<u8>, bool> failed");
    let (decoded, _): (Pair<Vec<u8>, bool>, usize) =
        decode_from_slice(&bytes).expect("decode Pair<Vec<u8>, bool> failed");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 10 — Pair<Option<u64>, Option<String>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_option_u64_option_string_roundtrip() {
    let val = Pair::<Option<u64>, Option<String>> {
        first: Some(0xDEAD_BEEF_CAFE_1234u64),
        second: Some("option-string".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode Pair<Option<u64>, Option<String>> failed");
    let (decoded, consumed): (Pair<Option<u64>, Option<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Pair<Option<u64>, Option<String>> failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());

    // also test with None values
    let none_val = Pair::<Option<u64>, Option<String>> {
        first: None,
        second: None,
    };
    let none_bytes = encode_to_vec(&none_val).expect("encode None pair failed");
    let (none_decoded, _): (Pair<Option<u64>, Option<String>>, usize) =
        decode_from_slice(&none_bytes).expect("decode None pair failed");
    assert_eq!(none_decoded, none_val);
}

// ---------------------------------------------------------------------------
// Test 11 — UnitStruct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_unit_struct_roundtrip() {
    let val = UnitStruct;
    let bytes = encode_to_vec(&val).expect("encode UnitStruct failed");
    let (decoded, consumed): (UnitStruct, usize) =
        decode_from_slice(&bytes).expect("decode UnitStruct failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 12 — UnitStruct encodes to 0 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_unit_struct_encodes_to_zero_bytes() {
    let val = UnitStruct;
    let bytes = encode_to_vec(&val).expect("encode UnitStruct failed");
    assert_eq!(bytes.len(), 0, "UnitStruct must encode to exactly 0 bytes");
}

// ---------------------------------------------------------------------------
// Test 13 — OptWrapper(Some(vec![])) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_opt_wrapper_some_empty_vec_roundtrip() {
    let val = OptWrapper(Some(vec![]));
    let bytes = encode_to_vec(&val).expect("encode OptWrapper(Some([])) failed");
    let (decoded, consumed): (OptWrapper, usize) =
        decode_from_slice(&bytes).expect("decode OptWrapper(Some([])) failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 14 — OptWrapper(None) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_opt_wrapper_none_roundtrip() {
    let val = OptWrapper(None);
    let bytes = encode_to_vec(&val).expect("encode OptWrapper(None) failed");
    let (decoded, consumed): (OptWrapper, usize) =
        decode_from_slice(&bytes).expect("decode OptWrapper(None) failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 15 — OptWrapper(Some(vec![1,2,3])) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_opt_wrapper_some_vec_roundtrip() {
    let val = OptWrapper(Some(vec![1, 2, 3]));
    let bytes = encode_to_vec(&val).expect("encode OptWrapper(Some([1,2,3])) failed");
    let (decoded, _): (OptWrapper, usize) =
        decode_from_slice(&bytes).expect("decode OptWrapper(Some([1,2,3])) failed");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 16 — Vec<Pair<u32, String>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_pairs_roundtrip() {
    let val: Vec<Pair<u32, String>> = vec![
        Pair {
            first: 1,
            second: "alpha".to_string(),
        },
        Pair {
            first: 2,
            second: "beta".to_string(),
        },
        Pair {
            first: 3,
            second: "gamma".to_string(),
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Pair<u32, String>> failed");
    let (decoded, consumed): (Vec<Pair<u32, String>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Pair<u32, String>> failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 17 — Vec<OptWrapper> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_opt_wrappers_roundtrip() {
    let val: Vec<OptWrapper> = vec![
        OptWrapper(Some(vec![10, 20])),
        OptWrapper(None),
        OptWrapper(Some(vec![])),
        OptWrapper(Some(vec![100, 200, 300])),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<OptWrapper> failed");
    let (decoded, consumed): (Vec<OptWrapper>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<OptWrapper> failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 18 — WithBytesAttr with 100-byte payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_hundred_byte_payload_roundtrip() {
    let payload: Vec<u8> = (0u8..100).collect();
    let val = WithBytesAttr {
        raw: payload.clone(),
        id: 0xCAFE_BABE_0000_0001u64,
    };
    let bytes = encode_to_vec(&val).expect("encode 100-byte payload failed");
    let (decoded, consumed): (WithBytesAttr, usize) =
        decode_from_slice(&bytes).expect("decode 100-byte payload failed");
    assert_eq!(decoded.raw, payload);
    assert_eq!(decoded.id, 0xCAFE_BABE_0000_0001u64);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 19 — Big-endian config Pair<u64, u64> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_u64_u64_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = Pair::<u64, u64> {
        first: 0xDEAD_BEEF_CAFE_0001u64,
        second: 0x0102_0304_0506_0708u64,
    };
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Pair<u64,u64> big-endian failed");
    let (decoded, consumed): (Pair<u64, u64>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Pair<u64,u64> big-endian failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 20 — Fixed-int config Pair<u32, u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_u32_u32_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = Pair::<u32, u32> {
        first: u32::MAX,
        second: 0,
    };
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Pair<u32,u32> fixed-int failed");
    let (decoded, consumed): (Pair<u32, u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Pair<u32,u32> fixed-int failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
    // With fixed-int encoding, two u32 fields must consume exactly 8 bytes.
    assert_eq!(
        bytes.len(),
        8,
        "two fixed u32 fields must occupy exactly 8 bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 21 — Consumed bytes == encoded length for OptWrapper
// ---------------------------------------------------------------------------

#[test]
fn test_opt_wrapper_consumed_bytes_equals_encoded_length() {
    let variants = [
        OptWrapper(None),
        OptWrapper(Some(vec![])),
        OptWrapper(Some(vec![0, 1, 2, 3, 4])),
        OptWrapper(Some((0u32..50).collect())),
    ];
    for val in &variants {
        let bytes = encode_to_vec(val).expect("encode OptWrapper variant failed");
        let (_, consumed): (OptWrapper, usize) =
            decode_from_slice(&bytes).expect("decode OptWrapper variant failed");
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed bytes must equal the encoded length for {:?}",
            val
        );
    }
}

// ---------------------------------------------------------------------------
// Test 22 — Deterministic encoding for Pair<u32, String>
// ---------------------------------------------------------------------------

#[test]
fn test_pair_u32_string_encoding_is_deterministic() {
    let val = Pair::<u32, String> {
        first: 0xABCD_1234u32,
        second: "deterministic".to_string(),
    };
    let bytes_a = encode_to_vec(&val).expect("first encode failed");
    let bytes_b = encode_to_vec(&val).expect("second encode failed");
    assert_eq!(
        bytes_a, bytes_b,
        "encoding the same value twice must produce identical byte sequences"
    );

    // Also verify roundtrip after the determinism check.
    let (decoded, consumed): (Pair<u32, String>, usize) =
        decode_from_slice(&bytes_a).expect("decode failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes_a.len());
}
