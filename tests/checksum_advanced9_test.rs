//! Advanced checksum encoding tests – checksum_advanced9_test.rs

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
struct SensorData {
    device_id: u32,
    readings: Vec<f32>,
    unit: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SensorStatus {
    Online,
    Offline(String),
    Calibrating { progress: u8 },
    Error { code: u32, msg: String },
}

// ---------------------------------------------------------------------------
// Test 1: SensorData roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_sensor_data_checksum_roundtrip() {
    let sensor = SensorData {
        device_id: 101,
        readings: vec![1.1, 2.2, 3.3, 4.4],
        unit: "Celsius".to_string(),
    };
    let encoded = encode_with_checksum(&sensor).expect("encode SensorData failed");
    let (decoded, _): (SensorData, usize) =
        decode_with_checksum(&encoded).expect("decode SensorData failed");
    assert_eq!(sensor, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: SensorStatus::Online roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_sensor_status_online_checksum_roundtrip() {
    let status = SensorStatus::Online;
    let encoded = encode_with_checksum(&status).expect("encode SensorStatus::Online failed");
    let (decoded, _): (SensorStatus, usize) =
        decode_with_checksum(&encoded).expect("decode SensorStatus::Online failed");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: SensorStatus::Offline roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_sensor_status_offline_checksum_roundtrip() {
    let status = SensorStatus::Offline("network unreachable".to_string());
    let encoded = encode_with_checksum(&status).expect("encode SensorStatus::Offline failed");
    let (decoded, _): (SensorStatus, usize) =
        decode_with_checksum(&encoded).expect("decode SensorStatus::Offline failed");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: SensorStatus::Calibrating roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_sensor_status_calibrating_checksum_roundtrip() {
    let status = SensorStatus::Calibrating { progress: 73 };
    let encoded = encode_with_checksum(&status).expect("encode SensorStatus::Calibrating failed");
    let (decoded, _): (SensorStatus, usize) =
        decode_with_checksum(&encoded).expect("decode SensorStatus::Calibrating failed");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: SensorStatus::Error roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_sensor_status_error_checksum_roundtrip() {
    let status = SensorStatus::Error {
        code: 0xDEAD_BEEF,
        msg: "fatal hardware fault".to_string(),
    };
    let encoded = encode_with_checksum(&status).expect("encode SensorStatus::Error failed");
    let (decoded, _): (SensorStatus, usize) =
        decode_with_checksum(&encoded).expect("decode SensorStatus::Error failed");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Vec<SensorData> checksum roundtrip (3 items)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_sensor_data_checksum_roundtrip() {
    let sensors = vec![
        SensorData {
            device_id: 1,
            readings: vec![0.0, 0.5, 1.0],
            unit: "Volt".to_string(),
        },
        SensorData {
            device_id: 2,
            readings: vec![100.0, 200.0],
            unit: "Pascal".to_string(),
        },
        SensorData {
            device_id: 3,
            readings: vec![9.8],
            unit: "m/s^2".to_string(),
        },
    ];
    let encoded = encode_with_checksum(&sensors).expect("encode Vec<SensorData> failed");
    let (decoded, _): (Vec<SensorData>, usize) =
        decode_with_checksum(&encoded).expect("decode Vec<SensorData> failed");
    assert_eq!(sensors, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Vec<SensorStatus> checksum roundtrip (4 variants)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_sensor_status_checksum_roundtrip() {
    let statuses = vec![
        SensorStatus::Online,
        SensorStatus::Offline("timeout".to_string()),
        SensorStatus::Calibrating { progress: 50 },
        SensorStatus::Error {
            code: 42,
            msg: "overheated".to_string(),
        },
    ];
    let encoded = encode_with_checksum(&statuses).expect("encode Vec<SensorStatus> failed");
    let (decoded, _): (Vec<SensorStatus>, usize) =
        decode_with_checksum(&encoded).expect("decode Vec<SensorStatus> failed");
    assert_eq!(statuses, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: u32 checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_checksum_roundtrip() {
    let value: u32 = 0xCAFE_BABE;
    let encoded = encode_with_checksum(&value).expect("encode u32 failed");
    let (decoded, _): (u32, usize) = decode_with_checksum(&encoded).expect("decode u32 failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: u64 checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u64_checksum_roundtrip() {
    let value: u64 = u64::MAX - 1;
    let encoded = encode_with_checksum(&value).expect("encode u64 failed");
    let (decoded, _): (u64, usize) = decode_with_checksum(&encoded).expect("decode u64 failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: String checksum roundtrip with unicode
// ---------------------------------------------------------------------------
#[test]
fn test_string_unicode_checksum_roundtrip() {
    let value = "こんにちは世界 🌍 oxicode".to_string();
    let encoded = encode_with_checksum(&value).expect("encode unicode String failed");
    let (decoded, _): (String, usize) =
        decode_with_checksum(&encoded).expect("decode unicode String failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: bool checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_bool_checksum_roundtrip() {
    for val in [true, false] {
        let encoded = encode_with_checksum(&val).expect("encode bool failed");
        let (decoded, _): (bool, usize) =
            decode_with_checksum(&encoded).expect("decode bool failed");
        assert_eq!(val, decoded, "bool roundtrip mismatch for {}", val);
    }
}

// ---------------------------------------------------------------------------
// Test 12: i64 negative checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i64_negative_checksum_roundtrip() {
    let value: i64 = -9_223_372_036_854_775_807;
    let encoded = encode_with_checksum(&value).expect("encode negative i64 failed");
    let (decoded, _): (i64, usize) =
        decode_with_checksum(&encoded).expect("decode negative i64 failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Option<SensorData> Some checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_some_sensor_data_checksum_roundtrip() {
    let value: Option<SensorData> = Some(SensorData {
        device_id: 77,
        readings: vec![3.14, 2.71],
        unit: "Kelvin".to_string(),
    });
    let encoded = encode_with_checksum(&value).expect("encode Option<SensorData> Some failed");
    let (decoded, _): (Option<SensorData>, usize) =
        decode_with_checksum(&encoded).expect("decode Option<SensorData> Some failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Option<SensorData> None checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_none_sensor_data_checksum_roundtrip() {
    let value: Option<SensorData> = None;
    let encoded = encode_with_checksum(&value).expect("encode Option<SensorData> None failed");
    let (decoded, _): (Option<SensorData>, usize) =
        decode_with_checksum(&encoded).expect("decode Option<SensorData> None failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Checksum adds exactly HEADER_SIZE bytes overhead
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_header_size_overhead() {
    let sensor = SensorData {
        device_id: 55,
        readings: vec![1.0, 2.0, 3.0],
        unit: "Ohm".to_string(),
    };
    let raw_encoded = encode_to_vec(&sensor).expect("raw encode failed");
    let checksum_encoded = encode_with_checksum(&sensor).expect("checksum encode failed");

    let overhead = checksum_encoded.len() - raw_encoded.len();
    assert_eq!(
        overhead, HEADER_SIZE,
        "checksum overhead must equal HEADER_SIZE ({}) but got {}",
        HEADER_SIZE, overhead
    );
}

// ---------------------------------------------------------------------------
// Test 16: Flipping byte 1 of header causes decode error
// ---------------------------------------------------------------------------
#[test]
fn test_flip_header_byte1_causes_error() {
    let sensor = SensorData {
        device_id: 12,
        readings: vec![0.1],
        unit: "Hz".to_string(),
    };
    let mut encoded = encode_with_checksum(&sensor).expect("encode failed");
    encoded[1] ^= 0xFF;
    let result: Result<(SensorData, usize), _> = decode_with_checksum(&encoded);
    assert!(
        result.is_err(),
        "flipping header byte 1 must produce a decode error"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Flipping last byte of payload causes decode error
// ---------------------------------------------------------------------------
#[test]
fn test_flip_last_payload_byte_causes_error() {
    let sensor = SensorData {
        device_id: 99,
        readings: vec![5.0, 10.0, 15.0],
        unit: "Watt".to_string(),
    };
    let mut encoded = encode_with_checksum(&sensor).expect("encode failed");
    let last = encoded.len() - 1;
    encoded[last] ^= 0xFF;
    let result: Result<(SensorData, usize), _> = decode_with_checksum(&encoded);
    assert!(
        result.is_err(),
        "flipping last payload byte must produce a decode error"
    );
}

// ---------------------------------------------------------------------------
// Test 18: HEADER_SIZE is at least 4 (minimum CRC32 size)
// ---------------------------------------------------------------------------
#[test]
fn test_header_size_at_least_four() {
    assert!(
        HEADER_SIZE >= 4,
        "HEADER_SIZE must be at least 4 (CRC32 size), but got {}",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 19: Same SensorData encoded twice gives identical bytes
// ---------------------------------------------------------------------------
#[test]
fn test_deterministic_encoding() {
    let sensor = SensorData {
        device_id: 42,
        readings: vec![1.0, 2.0, 3.0],
        unit: "Ampere".to_string(),
    };
    let first = encode_with_checksum(&sensor).expect("first encode failed");
    let second = encode_with_checksum(&sensor).expect("second encode failed");
    assert_eq!(
        first, second,
        "encoding the same value twice must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Empty Vec<f32> in SensorData roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_sensor_data_empty_readings_checksum_roundtrip() {
    let sensor = SensorData {
        device_id: 0,
        readings: vec![],
        unit: "".to_string(),
    };
    let encoded = encode_with_checksum(&sensor).expect("encode empty readings failed");
    let (decoded, _): (SensorData, usize) =
        decode_with_checksum(&encoded).expect("decode empty readings failed");
    assert_eq!(sensor, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Large SensorData (1000 readings) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_large_sensor_data_checksum_roundtrip() {
    let sensor = SensorData {
        device_id: 9999,
        readings: (0..1000).map(|i| i as f32 * 0.001).collect(),
        unit: "Joule".to_string(),
    };
    let encoded = encode_with_checksum(&sensor).expect("encode large SensorData failed");
    let (decoded, consumed): (SensorData, usize) =
        decode_with_checksum(&encoded).expect("decode large SensorData failed");
    assert_eq!(sensor, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 22: SensorData with unicode unit string roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_sensor_data_unicode_unit_checksum_roundtrip() {
    let sensor = SensorData {
        device_id: 7,
        readings: vec![273.15, 293.15, 373.15],
        unit: "°C / μV · Ω⁻¹".to_string(),
    };
    let encoded =
        encode_with_checksum(&sensor).expect("encode SensorData with unicode unit failed");
    let (decoded, _): (SensorData, usize) =
        decode_with_checksum(&encoded).expect("decode SensorData with unicode unit failed");
    assert_eq!(sensor, decoded);
}
