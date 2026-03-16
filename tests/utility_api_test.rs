//! Tests for utility API functions: encode_to_fixed_array, decode_value, encode_bytes,
//! and the #[oxicode(bytes)] derive attribute.

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
use oxicode::{decode_value, encode_to_fixed_array, Decode, Encode};

// ---------------------------------------------------------------------------
// encode_to_fixed_array tests
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_fixed_array_u32() {
    let value = 42u32;
    let (arr, n): ([u8; 16], _) = encode_to_fixed_array(&value).expect("encode should succeed");
    let decoded: u32 = decode_value(&arr[..n]).expect("decode should succeed");
    assert_eq!(value, decoded);
}

#[test]
fn test_encode_to_fixed_array_u64() {
    let value = 0xDEAD_BEEF_CAFE_BABEu64;
    let (arr, n): ([u8; 32], _) = encode_to_fixed_array(&value).expect("encode should succeed");
    let decoded: u64 = decode_value(&arr[..n]).expect("decode should succeed");
    assert_eq!(value, decoded);
}

#[test]
fn test_encode_to_fixed_array_too_small() {
    // A 1000-element Vec<u8> needs more than 4 bytes
    let value: Vec<u8> = vec![0u8; 1000];
    let result: Result<([u8; 4], _), _> = encode_to_fixed_array(&value);
    assert!(result.is_err(), "should fail when array too small");
}

#[test]
fn test_encode_to_fixed_array_exact_fit() {
    // u8 encodes to exactly 1 byte
    let value: u8 = 255;
    let (arr, n): ([u8; 1], _) = encode_to_fixed_array(&value).expect("should fit in 1 byte");
    assert_eq!(n, 1);
    let decoded: u8 = decode_value(&arr[..n]).expect("decode");
    assert_eq!(decoded, 255u8);
}

#[test]
fn test_encode_to_fixed_array_with_config() {
    use oxicode::config;
    let value = 100u32;
    let (arr, n): ([u8; 16], _) =
        oxicode::encode_to_fixed_array_with_config(&value, config::standard())
            .expect("encode should succeed");
    let decoded: u32 = decode_value(&arr[..n]).expect("decode should succeed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// decode_value tests
// ---------------------------------------------------------------------------

#[test]
fn test_decode_value_u64() {
    let encoded = oxicode::encode_to_vec(&123u64).expect("encode");
    let decoded: u64 = decode_value(&encoded).expect("decode");
    assert_eq!(decoded, 123u64);
}

#[test]
fn test_decode_value_string() {
    let original = String::from("hello oxicode");
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let decoded: String = decode_value(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

#[test]
fn test_decode_value_vec() {
    let original: Vec<u32> = vec![1, 2, 3, 4, 5];
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let decoded: Vec<u32> = decode_value(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

#[test]
fn test_decode_value_invalid_data() {
    // Empty slice should fail to decode u32
    let result: Result<u32, _> = decode_value(&[]);
    assert!(result.is_err(), "decoding u32 from empty slice should fail");
}

// ---------------------------------------------------------------------------
// encode_bytes convenience alias test
// ---------------------------------------------------------------------------

#[test]
fn test_encode_bytes_alias() {
    let value = 42u32;
    let encoded = oxicode::encode_bytes(&value).expect("encode_bytes should succeed");
    let decoded: u32 = decode_value(&encoded).expect("decode");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// #[oxicode(bytes)] derive attribute tests
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, Debug, PartialEq)]
struct BlobStruct {
    id: u32,
    #[oxicode(bytes)]
    data: Vec<u8>,
    name: String,
}

#[test]
fn test_bytes_attr_roundtrip() {
    let original = BlobStruct {
        id: 42,
        data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
        name: String::from("test"),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let decoded: BlobStruct = decode_value(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_bytes_attr_empty_vec() {
    let original = BlobStruct {
        id: 0,
        data: vec![],
        name: String::from("empty"),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let decoded: BlobStruct = decode_value(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_bytes_attr_large_data() {
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    let original = BlobStruct {
        id: 99,
        data,
        name: String::from("large"),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let decoded: BlobStruct = decode_value(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_bytes_attr_produces_same_as_normal_encode() {
    // The #[oxicode(bytes)] attribute should produce the same wire format
    // as the regular Vec<u8> encoding (length prefix + raw bytes).
    // We verify this by comparing with a struct that does NOT use the attribute.

    #[derive(Encode, Decode, Debug, PartialEq)]
    struct NormalStruct {
        id: u32,
        data: Vec<u8>,
        name: String,
    }

    let bytes_struct = BlobStruct {
        id: 7,
        data: vec![1, 2, 3, 4, 5],
        name: String::from("wire"),
    };
    let normal_struct = NormalStruct {
        id: 7,
        data: vec![1, 2, 3, 4, 5],
        name: String::from("wire"),
    };

    let encoded_bytes_attr = oxicode::encode_to_vec(&bytes_struct).expect("encode bytes_attr");
    let encoded_normal = oxicode::encode_to_vec(&normal_struct).expect("encode normal");

    assert_eq!(
        encoded_bytes_attr, encoded_normal,
        "#[oxicode(bytes)] should produce identical wire format to normal Vec<u8> encoding"
    );
}

#[derive(Encode, Decode, Debug, PartialEq)]
struct MultiBytes {
    #[oxicode(bytes)]
    header: Vec<u8>,
    version: u8,
    #[oxicode(bytes)]
    payload: Vec<u8>,
}

#[test]
fn test_bytes_attr_multiple_fields() {
    let original = MultiBytes {
        header: vec![0xFF, 0xFE],
        version: 1,
        payload: vec![10, 20, 30],
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let decoded: MultiBytes = decode_value(&encoded).expect("decode");
    assert_eq!(original, decoded);
}
