//! Advanced tests for Arc<T> and Rc<T> encoding in OxiCode.

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
use oxicode::{decode_from_slice, encode_to_vec, encoded_size};
use std::rc::Rc;
use std::sync::Arc;

// ── Structs used in tests 21 & 22 ────────────────────────────────────────────

use oxicode_derive::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithArcField {
    label: String,
    value: Arc<u64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithRcField {
    label: String,
    value: Rc<u64>,
}

// ── Test 1 ────────────────────────────────────────────────────────────────────

#[test]
fn test_arc_u32_42_roundtrip() {
    let original: Arc<u32> = Arc::new(42u32);
    let bytes = encode_to_vec(&original).expect("encode Arc<u32>(42)");
    let (decoded, _): (Arc<u32>, usize) = decode_from_slice(&bytes).expect("decode Arc<u32>(42)");
    assert_eq!(*decoded, 42u32);
}

// ── Test 2 ────────────────────────────────────────────────────────────────────

#[test]
fn test_arc_u32_max_roundtrip() {
    let original: Arc<u32> = Arc::new(u32::MAX);
    let bytes = encode_to_vec(&original).expect("encode Arc<u32>(MAX)");
    let (decoded, _): (Arc<u32>, usize) = decode_from_slice(&bytes).expect("decode Arc<u32>(MAX)");
    assert_eq!(*decoded, u32::MAX);
}

// ── Test 3 ────────────────────────────────────────────────────────────────────

#[test]
fn test_arc_string_roundtrip() {
    let original: Arc<String> = Arc::new("oxicode_arc".to_string());
    let bytes = encode_to_vec(&original).expect("encode Arc<String>");
    let (decoded, _): (Arc<String>, usize) = decode_from_slice(&bytes).expect("decode Arc<String>");
    assert_eq!(*decoded, "oxicode_arc".to_string());
}

// ── Test 4 ────────────────────────────────────────────────────────────────────

#[test]
fn test_arc_vec_u8_roundtrip() {
    let original: Arc<Vec<u8>> = Arc::new(vec![1u8, 2, 3]);
    let bytes = encode_to_vec(&original).expect("encode Arc<Vec<u8>>");
    let (decoded, _): (Arc<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<Vec<u8>>");
    assert_eq!(*decoded, vec![1u8, 2, 3]);
}

// ── Test 5 ────────────────────────────────────────────────────────────────────

#[test]
fn test_arc_bool_roundtrip() {
    // true
    let original_true: Arc<bool> = Arc::new(true);
    let bytes_true = encode_to_vec(&original_true).expect("encode Arc<bool>(true)");
    let (decoded_true, _): (Arc<bool>, usize) =
        decode_from_slice(&bytes_true).expect("decode Arc<bool>(true)");
    assert!(*decoded_true);

    // false
    let original_false: Arc<bool> = Arc::new(false);
    let bytes_false = encode_to_vec(&original_false).expect("encode Arc<bool>(false)");
    let (decoded_false, _): (Arc<bool>, usize) =
        decode_from_slice(&bytes_false).expect("decode Arc<bool>(false)");
    assert!(!*decoded_false);
}

// ── Test 6 ────────────────────────────────────────────────────────────────────

#[test]
fn test_arc_option_u64_roundtrip() {
    // Some
    let original_some: Arc<Option<u64>> = Arc::new(Some(123456789u64));
    let bytes_some = encode_to_vec(&original_some).expect("encode Arc<Option<u64>>(Some)");
    let (decoded_some, _): (Arc<Option<u64>>, usize) =
        decode_from_slice(&bytes_some).expect("decode Arc<Option<u64>>(Some)");
    assert_eq!(*decoded_some, Some(123456789u64));

    // None
    let original_none: Arc<Option<u64>> = Arc::new(None);
    let bytes_none = encode_to_vec(&original_none).expect("encode Arc<Option<u64>>(None)");
    let (decoded_none, _): (Arc<Option<u64>>, usize) =
        decode_from_slice(&bytes_none).expect("decode Arc<Option<u64>>(None)");
    assert_eq!(*decoded_none, None::<u64>);
}

// ── Test 7 ────────────────────────────────────────────────────────────────────

#[test]
fn test_rc_u32_42_roundtrip() {
    let original: Rc<u32> = Rc::new(42u32);
    let bytes = encode_to_vec(&original).expect("encode Rc<u32>(42)");
    let (decoded, _): (Rc<u32>, usize) = decode_from_slice(&bytes).expect("decode Rc<u32>(42)");
    assert_eq!(*decoded, 42u32);
}

// ── Test 8 ────────────────────────────────────────────────────────────────────

#[test]
fn test_rc_string_roundtrip() {
    let original: Rc<String> = Rc::new("oxicode_rc".to_string());
    let bytes = encode_to_vec(&original).expect("encode Rc<String>");
    let (decoded, _): (Rc<String>, usize) = decode_from_slice(&bytes).expect("decode Rc<String>");
    assert_eq!(*decoded, "oxicode_rc".to_string());
}

// ── Test 9 ────────────────────────────────────────────────────────────────────

#[test]
fn test_rc_vec_u32_roundtrip() {
    let original: Rc<Vec<u32>> = Rc::new(vec![10u32, 20, 30, 40]);
    let bytes = encode_to_vec(&original).expect("encode Rc<Vec<u32>>");
    let (decoded, _): (Rc<Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Rc<Vec<u32>>");
    assert_eq!(*decoded, vec![10u32, 20, 30, 40]);
}

// ── Test 10 ───────────────────────────────────────────────────────────────────

#[test]
fn test_arc_u32_encodes_same_as_plain_u32() {
    let plain: u32 = 999u32;
    let wrapped: Arc<u32> = Arc::new(999u32);

    let bytes_plain = encode_to_vec(&plain).expect("encode u32");
    let bytes_wrapped = encode_to_vec(&wrapped).expect("encode Arc<u32>");

    assert_eq!(
        bytes_plain, bytes_wrapped,
        "Arc<u32> must encode identically to plain u32 (no overhead)"
    );
}

// ── Test 11 ───────────────────────────────────────────────────────────────────

#[test]
fn test_rc_string_encodes_same_as_plain_string() {
    let plain: String = "compare_me".to_string();
    let wrapped: Rc<String> = Rc::new("compare_me".to_string());

    let bytes_plain = encode_to_vec(&plain).expect("encode String");
    let bytes_wrapped = encode_to_vec(&wrapped).expect("encode Rc<String>");

    assert_eq!(
        bytes_plain, bytes_wrapped,
        "Rc<String> must encode identically to plain String (no overhead)"
    );
}

// ── Test 12 ───────────────────────────────────────────────────────────────────

#[test]
fn test_vec_of_arc_u32_roundtrip() {
    let original: Vec<Arc<u32>> = vec![Arc::new(1u32), Arc::new(2u32), Arc::new(3u32)];
    let bytes = encode_to_vec(&original).expect("encode Vec<Arc<u32>>");
    let (decoded, _): (Vec<Arc<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Arc<u32>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(*decoded[0], 1u32);
    assert_eq!(*decoded[1], 2u32);
    assert_eq!(*decoded[2], 3u32);
}

// ── Test 13 ───────────────────────────────────────────────────────────────────

#[test]
fn test_vec_of_rc_string_roundtrip() {
    let original: Vec<Rc<String>> = vec![
        Rc::new("alpha".to_string()),
        Rc::new("beta".to_string()),
        Rc::new("gamma".to_string()),
    ];
    let bytes = encode_to_vec(&original).expect("encode Vec<Rc<String>>");
    let (decoded, _): (Vec<Rc<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Rc<String>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(*decoded[0], "alpha".to_string());
    assert_eq!(*decoded[1], "beta".to_string());
    assert_eq!(*decoded[2], "gamma".to_string());
}

// ── Test 14 ───────────────────────────────────────────────────────────────────

#[test]
fn test_option_arc_u64_roundtrip() {
    // Some
    let original_some: Option<Arc<u64>> = Some(Arc::new(u64::MAX / 2));
    let bytes_some = encode_to_vec(&original_some).expect("encode Option<Arc<u64>>(Some)");
    let (decoded_some, _): (Option<Arc<u64>>, usize) =
        decode_from_slice(&bytes_some).expect("decode Option<Arc<u64>>(Some)");
    assert!(decoded_some.is_some());
    assert_eq!(*decoded_some.expect("inner Some"), u64::MAX / 2);

    // None
    let original_none: Option<Arc<u64>> = None;
    let bytes_none = encode_to_vec(&original_none).expect("encode Option<Arc<u64>>(None)");
    let (decoded_none, _): (Option<Arc<u64>>, usize) =
        decode_from_slice(&bytes_none).expect("decode Option<Arc<u64>>(None)");
    assert!(decoded_none.is_none());
}

// ── Test 15 ───────────────────────────────────────────────────────────────────

#[test]
fn test_arc_slice_u8_roundtrip() {
    let original: Arc<[u8]> = Arc::<[u8]>::from(vec![10u8, 20, 30].as_slice());
    let bytes = encode_to_vec(&original).expect("encode Arc<[u8]>");
    // Decode as Vec<u8> – wire format is length-prefixed bytes, same as Vec<u8>
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<[u8]> as Vec<u8>");
    assert_eq!(decoded, vec![10u8, 20, 30]);
}

// ── Test 16 ───────────────────────────────────────────────────────────────────

#[test]
fn test_rc_str_roundtrip() {
    let original: Rc<str> = Rc::from("hello_rc_str");
    let bytes = encode_to_vec(&original).expect("encode Rc<str>");
    // Decode as String – wire format is length-prefixed UTF-8, same as &str / String
    let (decoded, _): (String, usize) =
        decode_from_slice(&bytes).expect("decode Rc<str> as String");
    assert_eq!(decoded, "hello_rc_str");
}

// ── Test 17 ───────────────────────────────────────────────────────────────────

#[test]
fn test_encoded_size_arc_u32_no_overhead() {
    let plain = 42u32;
    let wrapped: Arc<u32> = Arc::new(42u32);

    let size_plain = encoded_size(&plain).expect("encoded_size u32");
    let size_wrapped = encoded_size(&wrapped).expect("encoded_size Arc<u32>");

    assert_eq!(
        size_plain, size_wrapped,
        "encoded_size(Arc<u32>) must equal encoded_size(u32) — no pointer overhead"
    );
}

// ── Test 18 ───────────────────────────────────────────────────────────────────

#[test]
fn test_encoded_size_rc_str_no_overhead() {
    let plain: &str = "hello";
    let wrapped: Rc<str> = Rc::from("hello");

    let size_plain = encoded_size(&plain).expect("encoded_size &str");
    let size_wrapped = encoded_size(&wrapped).expect("encoded_size Rc<str>");

    assert_eq!(
        size_plain, size_wrapped,
        "encoded_size(Rc<str>) must equal encoded_size(&str) — no pointer overhead"
    );
}

// ── Test 19 ───────────────────────────────────────────────────────────────────

#[test]
fn test_arc_i64_boundary_values_roundtrip() {
    let boundary_values: &[i64] = &[i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX];

    for &val in boundary_values {
        let original: Arc<i64> = Arc::new(val);
        let bytes = encode_to_vec(&original).expect("encode Arc<i64>");
        let (decoded, consumed): (Arc<i64>, usize) =
            decode_from_slice(&bytes).expect("decode Arc<i64>");
        assert_eq!(*decoded, val, "boundary value {val} did not round-trip");
        assert_eq!(consumed, bytes.len(), "all bytes should be consumed");
    }
}

// ── Test 20 ───────────────────────────────────────────────────────────────────

#[test]
fn test_rc_f64_pi_bit_exact_roundtrip() {
    let pi = std::f64::consts::PI;
    let original: Rc<f64> = Rc::new(pi);
    let bytes = encode_to_vec(&original).expect("encode Rc<f64>(PI)");
    let (decoded, _): (Rc<f64>, usize) = decode_from_slice(&bytes).expect("decode Rc<f64>(PI)");
    // Bit-exact comparison via integer representation
    assert_eq!(
        decoded.to_bits(),
        pi.to_bits(),
        "Rc<f64> round-trip must preserve PI bit-exactly"
    );
}

// ── Test 21 ───────────────────────────────────────────────────────────────────

#[test]
fn test_struct_with_arc_field_roundtrip() {
    let original = WithArcField {
        label: "arc_struct".to_string(),
        value: Arc::new(0xDEAD_BEEF_u64),
    };
    let bytes = encode_to_vec(&original).expect("encode WithArcField");
    let (decoded, _): (WithArcField, usize) =
        decode_from_slice(&bytes).expect("decode WithArcField");
    assert_eq!(decoded.label, original.label);
    assert_eq!(*decoded.value, *original.value);
}

// ── Test 22 ───────────────────────────────────────────────────────────────────

#[test]
fn test_struct_with_rc_field_roundtrip() {
    let original = WithRcField {
        label: "rc_struct".to_string(),
        value: Rc::new(0xCAFE_BABE_u64),
    };
    let bytes = encode_to_vec(&original).expect("encode WithRcField");
    let (decoded, _): (WithRcField, usize) = decode_from_slice(&bytes).expect("decode WithRcField");
    assert_eq!(decoded.label, original.label);
    assert_eq!(*decoded.value, *original.value);
}
