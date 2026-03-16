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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Alert {
    id: u32,
    severity: u8,
    description: String,
    active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AlertLevel {
    Info,
    Warning(String),
    Critical { code: u32, details: String },
}

#[test]
fn test_u32_checksum_roundtrip() {
    let val: u32 = 42;
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (u32, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_string_checksum_roundtrip() {
    let val = String::from("Hello, OxiCode checksum!");
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (String, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_u8_checksum_roundtrip() {
    let val: Vec<u8> = vec![10, 20, 30, 40, 50, 200, 255, 0];
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (Vec<u8>, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_alert_struct_checksum_roundtrip() {
    let val = Alert {
        id: 9001,
        severity: 3,
        description: String::from("Disk usage critical"),
        active: true,
    };
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (Alert, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_alert_level_info_checksum_roundtrip() {
    let val = AlertLevel::Info;
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (AlertLevel, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_alert_level_warning_checksum_roundtrip() {
    let val = AlertLevel::Warning(String::from("Memory pressure high"));
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (AlertLevel, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_alert_level_critical_checksum_roundtrip() {
    let val = AlertLevel::Critical {
        code: 503,
        details: String::from("Service unavailable"),
    };
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (AlertLevel, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_bool_checksum_roundtrip() {
    let val = false;
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (bool, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_u64_checksum_roundtrip() {
    let val: u64 = 18_446_744_073_709_551_615u64;
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (u64, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_i32_negative_checksum_roundtrip() {
    let val: i32 = -987_654_321;
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (i32, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_some_checksum_roundtrip() {
    let val: Option<String> = Some(String::from("present value"));
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (Option<String>, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_none_checksum_roundtrip() {
    let val: Option<String> = None;
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (Option<String>, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_alert_checksum_roundtrip() {
    let val = vec![
        Alert {
            id: 1,
            severity: 1,
            description: String::from("Low memory"),
            active: true,
        },
        Alert {
            id: 2,
            severity: 5,
            description: String::from("Kernel panic"),
            active: false,
        },
        Alert {
            id: 3,
            severity: 2,
            description: String::from("High CPU"),
            active: true,
        },
    ];
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (Vec<Alert>, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_checksum_output_longer_than_plain_encoding() {
    let val = Alert {
        id: 77,
        severity: 2,
        description: String::from("Disk full"),
        active: true,
    };
    let plain = encode_to_vec(&val).expect("encode_to_vec failed");
    let with_checksum = encode_with_checksum(&val).expect("encode_with_checksum failed");
    assert!(
        with_checksum.len() > plain.len(),
        "checksum-wrapped output should be longer than plain encoding"
    );
    assert_eq!(
        with_checksum.len(),
        plain.len() + HEADER_SIZE,
        "overhead should equal exactly HEADER_SIZE"
    );
}

#[test]
fn test_corrupted_checksum_returns_error() {
    let val: u32 = 12345;
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let mut corrupted = encoded.clone();
    corrupted[0] ^= 0xFF; // flip bits in checksum header
    let result = decode_with_checksum::<u32>(&corrupted);
    assert!(result.is_err(), "corrupted checksum should fail");
}

#[test]
fn test_header_size_is_positive() {
    assert!(HEADER_SIZE > 0, "HEADER_SIZE must be greater than zero");
}

#[test]
fn test_checksum_of_same_data_is_deterministic() {
    let val: u32 = 99999;
    let encoded_a = encode_with_checksum(&val).expect("encode_with_checksum first call failed");
    let encoded_b = encode_with_checksum(&val).expect("encode_with_checksum second call failed");
    assert_eq!(
        encoded_a, encoded_b,
        "checksum encoding must be deterministic for identical input"
    );
}

#[test]
fn test_different_data_produces_different_checksum_bytes() {
    let val_a: u32 = 1;
    let val_b: u32 = 2;
    let encoded_a = encode_with_checksum(&val_a).expect("encode_with_checksum val_a failed");
    let encoded_b = encode_with_checksum(&val_b).expect("encode_with_checksum val_b failed");
    assert_ne!(
        encoded_a, encoded_b,
        "different data must produce different checksum-wrapped bytes"
    );
}

#[test]
fn test_empty_vec_u8_checksum_roundtrip() {
    let val: Vec<u8> = vec![];
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (Vec<u8>, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let val = Alert {
        id: 500,
        severity: 4,
        description: String::from("Network timeout"),
        active: false,
    };
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let total_len = encoded.len();
    let (_, consumed): (Alert, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(
        consumed, total_len,
        "consumed bytes should equal the total encoded length"
    );
}

#[test]
fn test_u128_checksum_roundtrip() {
    let val: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455u128;
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (u128, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_alert_level_checksum_roundtrip_four_items() {
    let val = vec![
        AlertLevel::Info,
        AlertLevel::Warning(String::from("Swap space low")),
        AlertLevel::Critical {
            code: 1001,
            details: String::from("Database connection lost"),
        },
        AlertLevel::Warning(String::from("Packet loss detected")),
    ];
    let encoded = encode_with_checksum(&val).expect("encode_with_checksum failed");
    let (decoded, _): (Vec<AlertLevel>, usize) =
        decode_with_checksum(&encoded).expect("decode_with_checksum failed");
    assert_eq!(val, decoded);
}
