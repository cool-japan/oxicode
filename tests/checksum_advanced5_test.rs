//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced5_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, HEADER_SIZE};
use oxicode::{encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Shared helper types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Measurement {
    sensor_id: u32,
    value: f64,
    unit: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Error(u32),
}

// ---------------------------------------------------------------------------
// Test 1: u32 checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_checksum_roundtrip() {
    let value: u32 = 3_141_592_653;
    let encoded = encode_with_checksum(&value).expect("encode u32 failed");
    let (decoded, consumed): (u32, _) = decode_with_checksum(&encoded).expect("decode u32 failed");
    assert_eq!(decoded, value, "decoded u32 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 2: String checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_string_checksum_roundtrip() {
    let value = String::from("Hello, OxiCode checksum world!");
    let encoded = encode_with_checksum(&value).expect("encode String failed");
    let (decoded, consumed): (String, _) =
        decode_with_checksum(&encoded).expect("decode String failed");
    assert_eq!(decoded, value, "decoded String must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 3: bool checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_bool_checksum_roundtrip() {
    let value_true: bool = true;
    let encoded_true = encode_with_checksum(&value_true).expect("encode bool true failed");
    let (decoded_true, consumed_true): (bool, _) =
        decode_with_checksum(&encoded_true).expect("decode bool true failed");
    assert!(decoded_true, "decoded bool must be true");
    assert_eq!(
        consumed_true,
        encoded_true.len(),
        "consumed must equal encoded length"
    );

    let value_false: bool = false;
    let encoded_false = encode_with_checksum(&value_false).expect("encode bool false failed");
    let (decoded_false, consumed_false): (bool, _) =
        decode_with_checksum(&encoded_false).expect("decode bool false failed");
    assert!(!decoded_false, "decoded bool must be false");
    assert_eq!(
        consumed_false,
        encoded_false.len(),
        "consumed must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Vec<u8> checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_checksum_roundtrip() {
    let value: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let encoded = encode_with_checksum(&value).expect("encode Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<u8> failed");
    assert_eq!(decoded, value, "decoded Vec<u8> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 5: f64 checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_f64_checksum_roundtrip() {
    let value: f64 = std::f64::consts::E;
    let encoded = encode_with_checksum(&value).expect("encode f64 failed");
    let (decoded, consumed): (f64, _) = decode_with_checksum(&encoded).expect("decode f64 failed");
    assert_eq!(
        decoded.to_bits(),
        value.to_bits(),
        "decoded f64 must be bit-exact equal to original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Measurement struct checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_measurement_struct_checksum_roundtrip() {
    let value = Measurement {
        sensor_id: 42,
        value: -273.15,
        unit: String::from("celsius"),
    };
    let encoded = encode_with_checksum(&value).expect("encode Measurement failed");
    let (decoded, consumed): (Measurement, _) =
        decode_with_checksum(&encoded).expect("decode Measurement failed");
    assert_eq!(decoded, value, "decoded Measurement must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Status::Active checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_status_active_checksum_roundtrip() {
    let value = Status::Active;
    let encoded = encode_with_checksum(&value).expect("encode Status::Active failed");
    let (decoded, consumed): (Status, _) =
        decode_with_checksum(&encoded).expect("decode Status::Active failed");
    assert_eq!(decoded, value, "decoded Status::Active must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Status::Error(404) checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_status_error_404_checksum_roundtrip() {
    let value = Status::Error(404);
    let encoded = encode_with_checksum(&value).expect("encode Status::Error(404) failed");
    let (decoded, consumed): (Status, _) =
        decode_with_checksum(&encoded).expect("decode Status::Error(404) failed");
    assert_eq!(
        decoded, value,
        "decoded Status::Error(404) must equal original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Vec<Measurement> checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_measurement_checksum_roundtrip() {
    let value: Vec<Measurement> = vec![
        Measurement {
            sensor_id: 1,
            value: 0.0,
            unit: String::from("kelvin"),
        },
        Measurement {
            sensor_id: 2,
            value: 100.0,
            unit: String::from("fahrenheit"),
        },
        Measurement {
            sensor_id: 3,
            value: -40.0,
            unit: String::from("celsius"),
        },
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<Measurement> failed");
    let (decoded, consumed): (Vec<Measurement>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<Measurement> failed");
    assert_eq!(
        decoded, value,
        "decoded Vec<Measurement> must equal original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Option<String> Some checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_string_some_checksum_roundtrip() {
    let value: Option<String> = Some(String::from("some string value"));
    let encoded = encode_with_checksum(&value).expect("encode Option<String> Some failed");
    let (decoded, consumed): (Option<String>, _) =
        decode_with_checksum(&encoded).expect("decode Option<String> Some failed");
    assert_eq!(
        decoded, value,
        "decoded Option<String> Some must equal original"
    );
    assert!(decoded.is_some(), "decoded option must be Some");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Option<String> None checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_string_none_checksum_roundtrip() {
    let value: Option<String> = None;
    let encoded = encode_with_checksum(&value).expect("encode Option<String> None failed");
    let (decoded, consumed): (Option<String>, _) =
        decode_with_checksum(&encoded).expect("decode Option<String> None failed");
    assert_eq!(
        decoded, value,
        "decoded Option<String> None must equal original"
    );
    assert!(decoded.is_none(), "decoded option must be None");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 12: u128 checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u128_checksum_roundtrip() {
    let value: u128 = u128::MAX / 3;
    let encoded = encode_with_checksum(&value).expect("encode u128 failed");
    let (decoded, consumed): (u128, _) =
        decode_with_checksum(&encoded).expect("decode u128 failed");
    assert_eq!(decoded, value, "decoded u128 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Corrupted checksum data returns error (flip a byte in the encoded data)
// ---------------------------------------------------------------------------
#[test]
fn test_corrupted_checksum_data_returns_error() {
    let value: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let mut encoded = encode_with_checksum(&value).expect("encode u64 failed");
    // Flip a byte in the payload region (after the header) to corrupt the data
    let flip_idx = HEADER_SIZE + 1;
    encoded[flip_idx] ^= 0xFF;
    let result = decode_with_checksum::<u64>(&encoded);
    assert!(
        result.is_err(),
        "decode_with_checksum must return Err when payload byte is corrupted"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Empty Vec<u8> checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_empty_vec_u8_checksum_roundtrip() {
    let value: Vec<u8> = Vec::new();
    let encoded = encode_with_checksum(&value).expect("encode empty Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode empty Vec<u8> failed");
    assert_eq!(decoded, value, "decoded empty Vec<u8> must equal original");
    assert!(decoded.is_empty(), "decoded Vec<u8> must be empty");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 15: HEADER_SIZE is positive (> 0)
// ---------------------------------------------------------------------------
#[test]
fn test_header_size_is_positive() {
    assert!(
        HEADER_SIZE > 0,
        "HEADER_SIZE must be greater than 0, got {}",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 16: Encoded-with-checksum size > encode_to_vec size
// ---------------------------------------------------------------------------
#[test]
fn test_checksummed_size_greater_than_plain_size() {
    let value: u64 = 123_456_789;
    let plain = encode_to_vec(&value).expect("plain encode failed");
    let checked = encode_with_checksum(&value).expect("checksum encode failed");
    assert!(
        checked.len() > plain.len(),
        "checksummed encoding ({} bytes) must be larger than plain encoding ({} bytes)",
        checked.len(),
        plain.len()
    );
    assert_eq!(
        checked.len() - plain.len(),
        HEADER_SIZE,
        "difference must be exactly HEADER_SIZE ({}) bytes",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 17: Two different values produce different checksummed encodings
// ---------------------------------------------------------------------------
#[test]
fn test_two_different_values_produce_different_checksummed_encodings() {
    let value_a = Measurement {
        sensor_id: 1,
        value: 10.0,
        unit: String::from("unit_a"),
    };
    let value_b = Measurement {
        sensor_id: 2,
        value: 20.0,
        unit: String::from("unit_b"),
    };
    let encoded_a = encode_with_checksum(&value_a).expect("encode value_a failed");
    let encoded_b = encode_with_checksum(&value_b).expect("encode value_b failed");
    assert_ne!(
        encoded_a, encoded_b,
        "different values must produce different checksummed byte sequences"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Same value checksummed twice produces identical bytes
// ---------------------------------------------------------------------------
#[test]
fn test_same_value_checksummed_twice_produces_identical_bytes() {
    let value = Measurement {
        sensor_id: 99,
        value: std::f64::consts::PI,
        unit: String::from("rad"),
    };
    let encoded_first = encode_with_checksum(&value).expect("first checksum encode failed");
    let encoded_second = encode_with_checksum(&value).expect("second checksum encode failed");
    assert_eq!(
        encoded_first, encoded_second,
        "encoding the same value twice must produce identical checksummed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 19: i64::MIN checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i64_min_checksum_roundtrip() {
    let value: i64 = i64::MIN;
    let encoded = encode_with_checksum(&value).expect("encode i64::MIN failed");
    let (decoded, consumed): (i64, _) =
        decode_with_checksum(&encoded).expect("decode i64::MIN failed");
    assert_eq!(decoded, value, "decoded i64::MIN must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Vec<String> checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_string_checksum_roundtrip() {
    let value: Vec<String> = vec![
        String::from("first"),
        String::from("second string"),
        String::new(),
        String::from("fourth with unicode: 日本語"),
        String::from("fifth"),
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<String> failed");
    let (decoded, consumed): (Vec<String>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<String> failed");
    assert_eq!(decoded, value, "decoded Vec<String> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Vec<Status> checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_status_checksum_roundtrip() {
    let value: Vec<Status> = vec![
        Status::Active,
        Status::Inactive,
        Status::Error(500),
        Status::Active,
        Status::Error(0),
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<Status> failed");
    let (decoded, consumed): (Vec<Status>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<Status> failed");
    assert_eq!(decoded, value, "decoded Vec<Status> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Large data (Vec<u8> 5000 bytes) checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_large_data_5000_bytes_checksum_roundtrip() {
    let value: Vec<u8> = (0u8..=255).cycle().take(5000).collect();
    assert_eq!(value.len(), 5000, "test data must be exactly 5000 bytes");
    let encoded = encode_with_checksum(&value).expect("encode 5000-byte Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode 5000-byte Vec<u8> failed");
    assert_eq!(
        decoded, value,
        "decoded 5000-byte Vec<u8> must equal original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}
