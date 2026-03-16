//! Advanced tests for `ManuallyDrop<T>` encoding in OxiCode.
//!
//! `ManuallyDrop<T>` encodes/decodes the inner value identically to `T`.

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
use oxicode::{config, decode_from_slice, encode_to_vec};
use std::mem::ManuallyDrop;

// ---------------------------------------------------------------------------
// Basic roundtrip tests
// ---------------------------------------------------------------------------

#[test]
fn test_manually_drop_u32_roundtrip() {
    let val = ManuallyDrop::new(42u32);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<u32>");
    let (decoded, _): (ManuallyDrop<u32>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<u32>");
    assert_eq!(ManuallyDrop::into_inner(decoded), 42u32);
}

#[test]
fn test_manually_drop_string_roundtrip() {
    let val = ManuallyDrop::new("hello".to_string());
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<String>");
    let (decoded, _): (ManuallyDrop<String>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<String>");
    assert_eq!(ManuallyDrop::into_inner(decoded), "hello");
}

#[test]
fn test_manually_drop_vec_u8_roundtrip() {
    let val = ManuallyDrop::new(vec![1u8, 2, 3]);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<Vec<u8>>");
    let (decoded, _): (ManuallyDrop<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<Vec<u8>>");
    assert_eq!(ManuallyDrop::into_inner(decoded), vec![1u8, 2, 3]);
}

#[test]
fn test_manually_drop_bool_roundtrip() {
    let val = ManuallyDrop::new(true);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<bool>");
    let (decoded, _): (ManuallyDrop<bool>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<bool>");
    assert!(ManuallyDrop::into_inner(decoded));
}

#[test]
fn test_manually_drop_u64_max_roundtrip() {
    let val = ManuallyDrop::new(u64::MAX);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<u64::MAX>");
    let (decoded, _): (ManuallyDrop<u64>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<u64::MAX>");
    assert_eq!(ManuallyDrop::into_inner(decoded), u64::MAX);
}

#[test]
fn test_manually_drop_i64_min_roundtrip() {
    let val = ManuallyDrop::new(i64::MIN);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<i64::MIN>");
    let (decoded, _): (ManuallyDrop<i64>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<i64::MIN>");
    assert_eq!(ManuallyDrop::into_inner(decoded), i64::MIN);
}

// ---------------------------------------------------------------------------
// Byte-identity tests: ManuallyDrop<T> must encode identically to T
// ---------------------------------------------------------------------------

#[test]
fn test_manually_drop_u32_same_bytes_as_u32() {
    let raw: u32 = 12345;
    let wrapped = ManuallyDrop::new(raw);

    let raw_bytes = encode_to_vec(&raw).expect("encode u32");
    let wrapped_bytes = encode_to_vec(&wrapped).expect("encode ManuallyDrop<u32>");

    assert_eq!(
        raw_bytes, wrapped_bytes,
        "ManuallyDrop<u32> must produce identical bytes to u32"
    );
}

#[test]
fn test_manually_drop_string_same_bytes_as_string() {
    let raw = "oxicode".to_string();
    let wrapped = ManuallyDrop::new(raw.clone());

    let raw_bytes = encode_to_vec(&raw).expect("encode String");
    let wrapped_bytes = encode_to_vec(&wrapped).expect("encode ManuallyDrop<String>");

    assert_eq!(
        raw_bytes, wrapped_bytes,
        "ManuallyDrop<String> must produce identical bytes to String"
    );
}

// ---------------------------------------------------------------------------
// Compound type tests
// ---------------------------------------------------------------------------

#[test]
fn test_option_manually_drop_u64_roundtrip() {
    // Some variant
    let some_val: Option<ManuallyDrop<u64>> = Some(ManuallyDrop::new(999u64));
    let bytes = encode_to_vec(&some_val).expect("encode Some(ManuallyDrop<u64>)");
    let (decoded, _): (Option<ManuallyDrop<u64>>, usize) =
        decode_from_slice(&bytes).expect("decode Some(ManuallyDrop<u64>)");
    assert_eq!(decoded.map(ManuallyDrop::into_inner), Some(999u64));

    // None variant
    let none_val: Option<ManuallyDrop<u64>> = None;
    let bytes_none = encode_to_vec(&none_val).expect("encode None ManuallyDrop<u64>");
    let (decoded_none, _): (Option<ManuallyDrop<u64>>, usize) =
        decode_from_slice(&bytes_none).expect("decode None ManuallyDrop<u64>");
    assert!(decoded_none.is_none());
}

#[test]
fn test_vec_manually_drop_u32_roundtrip() {
    let val: Vec<ManuallyDrop<u32>> = vec![
        ManuallyDrop::new(10u32),
        ManuallyDrop::new(20u32),
        ManuallyDrop::new(30u32),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<ManuallyDrop<u32>>");
    let (decoded, _): (Vec<ManuallyDrop<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<ManuallyDrop<u32>>");
    let inner: Vec<u32> = decoded.into_iter().map(ManuallyDrop::into_inner).collect();
    assert_eq!(inner, vec![10u32, 20, 30]);
}

// ---------------------------------------------------------------------------
// Config variant tests
// ---------------------------------------------------------------------------

#[test]
fn test_manually_drop_with_legacy_config() {
    let val = ManuallyDrop::new(777u32);
    let bytes = oxicode::encode_to_vec_with_config(&val, config::legacy()).expect("legacy encode");
    let (decoded, _): (ManuallyDrop<u32>, usize) =
        oxicode::decode_from_slice_with_config(&bytes, config::legacy()).expect("legacy decode");
    assert_eq!(ManuallyDrop::into_inner(decoded), 777u32);
    // legacy uses fixed 4-byte encoding for u32
    assert_eq!(bytes.len(), 4, "legacy u32 must be 4 bytes");
}

#[test]
fn test_manually_drop_with_big_endian_config() {
    let val = ManuallyDrop::new(0x0102_0304u32);
    let cfg = config::standard().with_big_endian();
    let bytes = oxicode::encode_to_vec_with_config(&val, cfg).expect("big-endian encode");
    let (decoded, _): (ManuallyDrop<u32>, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("big-endian decode");
    assert_eq!(ManuallyDrop::into_inner(decoded), 0x0102_0304u32);
}

// ---------------------------------------------------------------------------
// Float, boundary, and extended type tests
// ---------------------------------------------------------------------------

#[test]
fn test_manually_drop_f64_pi_roundtrip() {
    let val = ManuallyDrop::new(std::f64::consts::PI);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<f64> PI");
    let (decoded, _): (ManuallyDrop<f64>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<f64> PI");
    let inner = ManuallyDrop::into_inner(decoded);
    assert!(
        (inner - std::f64::consts::PI).abs() < f64::EPSILON,
        "decoded PI must be within epsilon of std::f64::consts::PI"
    );
}

#[test]
fn test_manually_drop_u8_min_max_roundtrip() {
    for original in [u8::MIN, u8::MAX] {
        let val = ManuallyDrop::new(original);
        let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<u8>");
        let (decoded, _): (ManuallyDrop<u8>, usize) =
            decode_from_slice(&bytes).expect("decode ManuallyDrop<u8>");
        assert_eq!(ManuallyDrop::into_inner(decoded), original);
    }
}

#[test]
fn test_manually_drop_i128_min_roundtrip() {
    let val = ManuallyDrop::new(i128::MIN);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<i128::MIN>");
    let (decoded, _): (ManuallyDrop<i128>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<i128::MIN>");
    assert_eq!(ManuallyDrop::into_inner(decoded), i128::MIN);
}

#[test]
fn test_manually_drop_u128_max_roundtrip() {
    let val = ManuallyDrop::new(u128::MAX);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<u128::MAX>");
    let (decoded, _): (ManuallyDrop<u128>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<u128::MAX>");
    assert_eq!(ManuallyDrop::into_inner(decoded), u128::MAX);
}

// ---------------------------------------------------------------------------
// Tuple and nested type tests
// ---------------------------------------------------------------------------

#[test]
fn test_manually_drop_tuple_roundtrip() {
    let val: (ManuallyDrop<u32>, ManuallyDrop<String>) = (
        ManuallyDrop::new(42u32),
        ManuallyDrop::new("tuple".to_string()),
    );
    let bytes = encode_to_vec(&val).expect("encode (ManuallyDrop<u32>, ManuallyDrop<String>)");
    let (decoded, _): ((ManuallyDrop<u32>, ManuallyDrop<String>), usize) =
        decode_from_slice(&bytes).expect("decode (ManuallyDrop<u32>, ManuallyDrop<String>)");
    let (a, b) = decoded;
    assert_eq!(ManuallyDrop::into_inner(a), 42u32);
    assert_eq!(ManuallyDrop::into_inner(b), "tuple");
}

#[test]
fn test_manually_drop_option_inside_roundtrip() {
    let val = ManuallyDrop::new(Some(42u32));
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<Option<u32>>");
    let (decoded, _): (ManuallyDrop<Option<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<Option<u32>>");
    assert_eq!(ManuallyDrop::into_inner(decoded), Some(42u32));
}

#[test]
fn test_manually_drop_vec_string_roundtrip() {
    let val = ManuallyDrop::new(vec!["a".to_string(), "b".to_string()]);
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<Vec<String>>");
    let (decoded, _): (ManuallyDrop<Vec<String>>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<Vec<String>>");
    assert_eq!(
        ManuallyDrop::into_inner(decoded),
        vec!["a".to_string(), "b".to_string()]
    );
}

#[test]
fn test_manually_drop_nested_roundtrip() {
    let val = ManuallyDrop::new(ManuallyDrop::new(42u32));
    let bytes = encode_to_vec(&val).expect("encode ManuallyDrop<ManuallyDrop<u32>>");
    let (decoded, _): (ManuallyDrop<ManuallyDrop<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode ManuallyDrop<ManuallyDrop<u32>>");
    let inner = ManuallyDrop::into_inner(ManuallyDrop::into_inner(decoded));
    assert_eq!(inner, 42u32);
}

// ---------------------------------------------------------------------------
// Limit config and size verification
// ---------------------------------------------------------------------------

#[test]
fn test_manually_drop_with_limit_config() {
    let val = ManuallyDrop::new(123u32);
    let cfg = config::standard().with_limit::<1024>();
    let bytes = oxicode::encode_to_vec_with_config(&val, cfg).expect("limit encode");
    let (decoded, _): (ManuallyDrop<u32>, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("limit decode");
    assert_eq!(ManuallyDrop::into_inner(decoded), 123u32);
}

#[test]
fn test_manually_drop_encoded_size_equals_inner_size() {
    let raw: u32 = 255u32;
    let wrapped = ManuallyDrop::new(raw);

    let raw_bytes = encode_to_vec(&raw).expect("encode u32 for size check");
    let wrapped_bytes = encode_to_vec(&wrapped).expect("encode ManuallyDrop<u32> for size check");

    assert_eq!(
        raw_bytes.len(),
        wrapped_bytes.len(),
        "ManuallyDrop<u32> encoded size must equal u32 encoded size"
    );
}
