//! Advanced tests for atomic-like types and boolean combinations in OxiCode.
//!
//! Covers bool roundtrips, wire-byte verification, Option<bool>, tuple/struct/enum with bools,
//! large Vec<bool>, BTreeMap<bool, u8>, config variants, and nested/compound types.

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
use std::collections::BTreeMap;

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// --- Test 1: bool true roundtrip ---

#[test]
fn test_bool_true_roundtrip() {
    let original = true;
    let enc = encode_to_vec(&original).expect("encode bool true");
    let (dec, _bytes): (bool, usize) = decode_from_slice(&enc).expect("decode bool true");
    assert!(dec, "decoded bool should be true");
}

// --- Test 2: bool false roundtrip ---

#[test]
fn test_bool_false_roundtrip() {
    let original = false;
    let enc = encode_to_vec(&original).expect("encode bool false");
    let (dec, _bytes): (bool, usize) = decode_from_slice(&enc).expect("decode bool false");
    assert!(!dec, "decoded bool should be false");
}

// --- Test 3: Vec<bool> roundtrip ---

#[test]
fn test_vec_bool_roundtrip() {
    let original: Vec<bool> = vec![true, false, true, true, false];
    let enc = encode_to_vec(&original).expect("encode Vec<bool>");
    let (dec, _bytes): (Vec<bool>, usize) = decode_from_slice(&enc).expect("decode Vec<bool>");
    assert_eq!(original, dec, "Vec<bool> must roundtrip exactly");
}

// --- Test 4: [bool; 8] fixed array roundtrip ---

#[test]
fn test_bool_fixed_array_roundtrip() {
    let original: [bool; 8] = [true, false, true, false, true, true, false, true];
    let enc = encode_to_vec(&original).expect("encode [bool; 8]");
    let (dec, _bytes): ([bool; 8], usize) = decode_from_slice(&enc).expect("decode [bool; 8]");
    assert_eq!(original, dec, "[bool; 8] must roundtrip exactly");
}

// --- Test 5: bool wire size is 1 byte ---

#[test]
fn test_bool_wire_size_one_byte() {
    let enc_true = encode_to_vec(&true).expect("encode true");
    let enc_false = encode_to_vec(&false).expect("encode false");
    assert_eq!(enc_true.len(), 1, "bool true must encode to exactly 1 byte");
    assert_eq!(
        enc_false.len(),
        1,
        "bool false must encode to exactly 1 byte"
    );
}

// --- Test 6: bool true encodes to [1u8] ---

#[test]
fn test_bool_true_encodes_to_one() {
    let enc = encode_to_vec(&true).expect("encode true");
    assert_eq!(enc, vec![1u8], "bool true must encode to [1u8]");
}

// --- Test 7: bool false encodes to [0u8] ---

#[test]
fn test_bool_false_encodes_to_zero() {
    let enc = encode_to_vec(&false).expect("encode false");
    assert_eq!(enc, vec![0u8], "bool false must encode to [0u8]");
}

// --- Test 8: Option<bool> Some(true) roundtrip ---

#[test]
fn test_option_bool_some_true_roundtrip() {
    let original: Option<bool> = Some(true);
    let enc = encode_to_vec(&original).expect("encode Option<bool> Some(true)");
    let (dec, _bytes): (Option<bool>, usize) =
        decode_from_slice(&enc).expect("decode Option<bool> Some(true)");
    assert_eq!(
        original, dec,
        "Option<bool> Some(true) must roundtrip exactly"
    );
}

// --- Test 9: Option<bool> Some(false) roundtrip ---

#[test]
fn test_option_bool_some_false_roundtrip() {
    let original: Option<bool> = Some(false);
    let enc = encode_to_vec(&original).expect("encode Option<bool> Some(false)");
    let (dec, _bytes): (Option<bool>, usize) =
        decode_from_slice(&enc).expect("decode Option<bool> Some(false)");
    assert_eq!(
        original, dec,
        "Option<bool> Some(false) must roundtrip exactly"
    );
}

// --- Test 10: Option<bool> None roundtrip ---

#[test]
fn test_option_bool_none_roundtrip() {
    let original: Option<bool> = None;
    let enc = encode_to_vec(&original).expect("encode Option<bool> None");
    let (dec, _bytes): (Option<bool>, usize) =
        decode_from_slice(&enc).expect("decode Option<bool> None");
    assert_eq!(original, dec, "Option<bool> None must roundtrip exactly");
}

// --- Test 11: (bool, bool) tuple roundtrip ---

#[test]
fn test_bool_bool_tuple_roundtrip() {
    let original: (bool, bool) = (true, false);
    let enc = encode_to_vec(&original).expect("encode (bool, bool)");
    let (dec, _bytes): ((bool, bool), usize) =
        decode_from_slice(&enc).expect("decode (bool, bool)");
    assert_eq!(original, dec, "(bool, bool) must roundtrip exactly");
}

// --- Test 12: (bool, bool, bool) triple roundtrip ---

#[test]
fn test_bool_triple_tuple_roundtrip() {
    let original: (bool, bool, bool) = (false, true, false);
    let enc = encode_to_vec(&original).expect("encode (bool, bool, bool)");
    let (dec, _bytes): ((bool, bool, bool), usize) =
        decode_from_slice(&enc).expect("decode (bool, bool, bool)");
    assert_eq!(original, dec, "(bool, bool, bool) must roundtrip exactly");
}

// --- Test 13: Struct with multiple bool fields roundtrip ---

#[derive(Debug, PartialEq, Encode, Decode)]
struct Flags {
    active: bool,
    visible: bool,
    enabled: bool,
    locked: bool,
}

#[test]
fn test_struct_multiple_bool_fields_roundtrip() {
    let original = Flags {
        active: true,
        visible: false,
        enabled: true,
        locked: false,
    };
    let enc = encode_to_vec(&original).expect("encode Flags struct");
    let (dec, _bytes): (Flags, usize) = decode_from_slice(&enc).expect("decode Flags struct");
    assert_eq!(original, dec, "Flags struct must roundtrip exactly");
}

// --- Test 14: Enum with bool payload roundtrip ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum BoolVariant {
    On(bool),
    Off,
    Toggle { value: bool, force: bool },
}

#[test]
fn test_enum_with_bool_payload_roundtrip() {
    let cases = [
        BoolVariant::On(true),
        BoolVariant::On(false),
        BoolVariant::Off,
        BoolVariant::Toggle {
            value: true,
            force: false,
        },
        BoolVariant::Toggle {
            value: false,
            force: true,
        },
    ];
    for original in &cases {
        let enc = encode_to_vec(original).expect("encode BoolVariant");
        let (dec, _bytes): (BoolVariant, usize) =
            decode_from_slice(&enc).expect("decode BoolVariant");
        assert_eq!(original, &dec, "BoolVariant must roundtrip exactly");
    }
}

// --- Test 15: Vec<bool> empty roundtrip ---

#[test]
fn test_vec_bool_empty_roundtrip() {
    let original: Vec<bool> = Vec::new();
    let enc = encode_to_vec(&original).expect("encode empty Vec<bool>");
    let (dec, _bytes): (Vec<bool>, usize) =
        decode_from_slice(&enc).expect("decode empty Vec<bool>");
    assert!(dec.is_empty(), "empty Vec<bool> must roundtrip as empty");
}

// --- Test 16: Vec<bool> 100 elements roundtrip ---

#[test]
fn test_vec_bool_100_elements_roundtrip() {
    let original: Vec<bool> = (0..100).map(|i| i % 3 != 0).collect();
    let enc = encode_to_vec(&original).expect("encode Vec<bool> 100 elements");
    let (dec, _bytes): (Vec<bool>, usize) =
        decode_from_slice(&enc).expect("decode Vec<bool> 100 elements");
    assert_eq!(
        original, dec,
        "Vec<bool> with 100 elements must roundtrip exactly"
    );
}

// --- Test 17: BTreeMap<bool, u8> roundtrip ---

#[test]
fn test_btreemap_bool_key_roundtrip() {
    let mut original: BTreeMap<bool, u8> = BTreeMap::new();
    original.insert(false, 0u8);
    original.insert(true, 1u8);
    let enc = encode_to_vec(&original).expect("encode BTreeMap<bool, u8>");
    let (dec, _bytes): (BTreeMap<bool, u8>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<bool, u8>");
    assert_eq!(original, dec, "BTreeMap<bool, u8> must roundtrip exactly");
}

// --- Test 18: bool big-endian same as little-endian (bools aren't byte-swapped) ---

#[test]
fn test_bool_big_endian_same_as_little_endian() {
    let cfg_be = config::standard().with_big_endian();
    let cfg_le = config::standard();

    let enc_be_true = encode_to_vec_with_config(&true, cfg_be).expect("encode true big-endian");
    let enc_le_true = encode_to_vec_with_config(&true, cfg_le).expect("encode true little-endian");
    assert_eq!(
        enc_be_true, enc_le_true,
        "bool true: big-endian and little-endian must produce identical bytes"
    );

    let enc_be_false = encode_to_vec_with_config(&false, cfg_be).expect("encode false big-endian");
    let enc_le_false =
        encode_to_vec_with_config(&false, cfg_le).expect("encode false little-endian");
    assert_eq!(
        enc_be_false, enc_le_false,
        "bool false: big-endian and little-endian must produce identical bytes"
    );
}

// --- Test 19: bool fixed-int config roundtrip ---

#[test]
fn test_bool_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();

    for &val in &[true, false] {
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode bool with fixed-int config");
        let (dec, _bytes): (bool, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode bool with fixed-int config");
        assert_eq!(val, dec, "bool must roundtrip with fixed-int config");
    }
}

// --- Test 20: Nested struct with bool fields roundtrip ---

#[derive(Debug, PartialEq, Encode, Decode)]
struct Inner {
    flag: bool,
    value: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Outer {
    enabled: bool,
    inner: Inner,
    tag: u8,
}

#[test]
fn test_nested_struct_with_bool_fields_roundtrip() {
    let original = Outer {
        enabled: true,
        inner: Inner {
            flag: false,
            value: 42,
        },
        tag: 7u8,
    };
    let enc = encode_to_vec(&original).expect("encode nested Outer struct");
    let (dec, _bytes): (Outer, usize) =
        decode_from_slice(&enc).expect("decode nested Outer struct");
    assert_eq!(
        original, dec,
        "nested struct with bool fields must roundtrip exactly"
    );
}

// --- Test 21: Result<bool, bool> roundtrip ---

#[test]
fn test_result_bool_bool_roundtrip() {
    let cases: [Result<bool, bool>; 4] = [Ok(true), Ok(false), Err(true), Err(false)];
    for original in &cases {
        let enc = encode_to_vec(original).expect("encode Result<bool, bool>");
        let (dec, _bytes): (Result<bool, bool>, usize) =
            decode_from_slice(&enc).expect("decode Result<bool, bool>");
        assert_eq!(original, &dec, "Result<bool, bool> must roundtrip exactly");
    }
}

// --- Test 22: Vec<(bool, u32)> roundtrip ---

#[test]
fn test_vec_bool_u32_tuple_roundtrip() {
    let original: Vec<(bool, u32)> = vec![
        (true, 0u32),
        (false, 1u32),
        (true, 100u32),
        (false, u32::MAX),
        (true, 42u32),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<(bool, u32)>");
    let (dec, _bytes): (Vec<(bool, u32)>, usize) =
        decode_from_slice(&enc).expect("decode Vec<(bool, u32)>");
    assert_eq!(original, dec, "Vec<(bool, u32)> must roundtrip exactly");
}
