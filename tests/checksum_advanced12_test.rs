#![cfg(feature = "checksum")]
#![allow(unused_imports)]
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
use oxicode::checksum::{unwrap_with_checksum, wrap_with_checksum, ChecksumError, HEADER_SIZE};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
enum SensorType {
    Temperature,
    Pressure,
    Humidity,
    Light,
    Accelerometer,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CalibrationRecord {
    sensor_id: u32,
    sensor_type: SensorType,
    offset: f32,
    scale: f32,
    calibrated_at: u64,
    notes: Option<String>,
}

// Test 1: CalibrationRecord roundtrip via checksum
#[test]
fn test_calibration_record_roundtrip_checksum() {
    let record = CalibrationRecord {
        sensor_id: 1001,
        sensor_type: SensorType::Temperature,
        offset: -0.25,
        scale: 1.05,
        calibrated_at: 1_700_000_000,
        notes: Some("factory calibration".to_string()),
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (CalibrationRecord, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(record, decoded);
}

// Test 2: SensorType::Temperature roundtrip via checksum
#[test]
fn test_sensor_type_temperature_roundtrip() {
    let val = SensorType::Temperature;
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SensorType, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 3: SensorType::Pressure roundtrip via checksum
#[test]
fn test_sensor_type_pressure_roundtrip() {
    let val = SensorType::Pressure;
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SensorType, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 4: SensorType::Humidity roundtrip via checksum
#[test]
fn test_sensor_type_humidity_roundtrip() {
    let val = SensorType::Humidity;
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SensorType, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 5: SensorType::Light roundtrip via checksum
#[test]
fn test_sensor_type_light_roundtrip() {
    let val = SensorType::Light;
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SensorType, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 6: SensorType::Accelerometer roundtrip via checksum
#[test]
fn test_sensor_type_accelerometer_roundtrip() {
    let val = SensorType::Accelerometer;
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SensorType, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 7: HEADER_SIZE == 16
#[test]
fn test_header_size_is_16() {
    assert_eq!(HEADER_SIZE, 16);
}

// Test 8: Wrapped length == HEADER_SIZE + raw encoded length
#[test]
fn test_wrapped_length_equals_header_plus_raw() {
    let record = CalibrationRecord {
        sensor_id: 42,
        sensor_type: SensorType::Pressure,
        offset: 0.0,
        scale: 1.0,
        calibrated_at: 0,
        notes: None,
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    assert_eq!(wrapped.len(), HEADER_SIZE + raw.len());
}

// Test 9: Bit-flip corruption → Err (flip many bytes aggressively)
#[test]
fn test_corruption_detected_aggressive_bit_flip() {
    let record = CalibrationRecord {
        sensor_id: 99,
        sensor_type: SensorType::Humidity,
        offset: 1.23,
        scale: 0.99,
        calibrated_at: 12345678,
        notes: Some("test".to_string()),
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "aggressively corrupted data must return Err"
    );
}

// Test 10: Complete data replacement → Err
#[test]
fn test_complete_data_replacement_detected() {
    let record = CalibrationRecord {
        sensor_id: 7,
        sensor_type: SensorType::Light,
        offset: -1.0,
        scale: 2.0,
        calibrated_at: 999,
        notes: None,
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    // Replace with all-zeros of the same length
    let replacement = vec![0u8; wrapped.len()];
    let result = unwrap_with_checksum(&replacement);
    assert!(result.is_err(), "completely replaced data must return Err");
}

// Test 11: Truncated data → Err (remove last byte)
#[test]
fn test_truncated_data_detected() {
    let record = CalibrationRecord {
        sensor_id: 55,
        sensor_type: SensorType::Accelerometer,
        offset: 0.5,
        scale: 1.5,
        calibrated_at: 1_000_000,
        notes: Some("truncation test".to_string()),
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    // Remove the last byte
    let truncated = &wrapped[..wrapped.len() - 1];
    let result = unwrap_with_checksum(truncated);
    assert!(result.is_err(), "truncated data must return Err");
}

// Test 12: Vec<CalibrationRecord> roundtrip via checksum
#[test]
fn test_vec_of_calibration_records_roundtrip() {
    let records = vec![
        CalibrationRecord {
            sensor_id: 1,
            sensor_type: SensorType::Temperature,
            offset: 0.1,
            scale: 1.0,
            calibrated_at: 100,
            notes: None,
        },
        CalibrationRecord {
            sensor_id: 2,
            sensor_type: SensorType::Pressure,
            offset: -0.5,
            scale: 0.95,
            calibrated_at: 200,
            notes: Some("second sensor".to_string()),
        },
        CalibrationRecord {
            sensor_id: 3,
            sensor_type: SensorType::Humidity,
            offset: 0.0,
            scale: 1.1,
            calibrated_at: 300,
            notes: None,
        },
    ];
    let raw = encode_to_vec(&records).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Vec<CalibrationRecord>, _) =
        decode_from_slice(&payload).expect("decode failed");
    assert_eq!(records, decoded);
}

// Test 13: Option<CalibrationRecord> Some via checksum
#[test]
fn test_option_some_calibration_record_roundtrip() {
    let val: Option<CalibrationRecord> = Some(CalibrationRecord {
        sensor_id: 77,
        sensor_type: SensorType::Light,
        offset: 0.01,
        scale: 1.02,
        calibrated_at: 5_000_000,
        notes: Some("option test".to_string()),
    });
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Option<CalibrationRecord>, _) =
        decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 14: Option<CalibrationRecord> None via checksum
#[test]
fn test_option_none_calibration_record_roundtrip() {
    let val: Option<CalibrationRecord> = None;
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Option<CalibrationRecord>, _) =
        decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 15: CalibrationRecord with None notes
#[test]
fn test_calibration_record_none_notes_roundtrip() {
    let record = CalibrationRecord {
        sensor_id: 500,
        sensor_type: SensorType::Accelerometer,
        offset: -2.5,
        scale: 0.5,
        calibrated_at: 9_876_543_210,
        notes: None,
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (CalibrationRecord, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(record, decoded);
    assert!(decoded.notes.is_none());
}

// Test 16: CalibrationRecord with Some notes (long string)
#[test]
fn test_calibration_record_long_notes_roundtrip() {
    let long_notes = "calibration performed under strict laboratory conditions with NIST-traceable reference instruments. ".repeat(20);
    let record = CalibrationRecord {
        sensor_id: 8888,
        sensor_type: SensorType::Temperature,
        offset: 0.001,
        scale: 0.999,
        calibrated_at: 1_609_459_200,
        notes: Some(long_notes),
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (CalibrationRecord, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(record, decoded);
    assert!(decoded.notes.as_ref().expect("notes must be Some").len() > 100);
}

// Test 17: Encoding deterministic: wrap same data twice → identical bytes
#[test]
fn test_wrap_same_data_twice_identical_bytes() {
    let record = CalibrationRecord {
        sensor_id: 321,
        sensor_type: SensorType::Pressure,
        offset: 3.14,
        scale: 2.71,
        calibrated_at: 1_234_567_890,
        notes: Some("determinism check".to_string()),
    };
    let raw1 = encode_to_vec(&record).expect("encode first failed");
    let raw2 = encode_to_vec(&record).expect("encode second failed");
    let wrapped1 = wrap_with_checksum(&raw1);
    let wrapped2 = wrap_with_checksum(&raw2);
    assert_eq!(
        wrapped1, wrapped2,
        "wrapping identical data must be deterministic"
    );
}

// Test 18: u64::MAX calibrated_at roundtrip
#[test]
fn test_u64_max_calibrated_at_roundtrip() {
    let record = CalibrationRecord {
        sensor_id: 0,
        sensor_type: SensorType::Humidity,
        offset: 0.0,
        scale: 1.0,
        calibrated_at: u64::MAX,
        notes: None,
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (CalibrationRecord, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(record.calibrated_at, decoded.calibrated_at);
    assert_eq!(u64::MAX, decoded.calibrated_at);
}

// Test 19: Consumed bytes == wrapped length
#[test]
fn test_consumed_bytes_equals_wrapped_length() {
    let record = CalibrationRecord {
        sensor_id: 404,
        sensor_type: SensorType::Light,
        offset: 0.123,
        scale: 1.456,
        calibrated_at: 1_700_100_200,
        notes: Some("consumed bytes test".to_string()),
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (_, consumed) = decode_from_slice::<CalibrationRecord>(&payload).expect("decode failed");
    // consumed from decode_from_slice is the inner payload consumed bytes;
    // the total wrapped length should equal HEADER_SIZE + consumed
    assert_eq!(wrapped.len(), HEADER_SIZE + consumed);
}

// Test 20: Unwrap gives back original encoded bytes
#[test]
fn test_unwrap_gives_back_original_encoded_bytes() {
    let record = CalibrationRecord {
        sensor_id: 111,
        sensor_type: SensorType::Accelerometer,
        offset: -5.5,
        scale: 2.2,
        calibrated_at: 42,
        notes: None,
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    assert_eq!(
        raw, payload,
        "unwrapped payload must equal original encoded bytes"
    );
}

// Test 21: f32 offset = f32::NAN — encode/decode preserves bit pattern via to_bits
#[test]
fn test_f32_nan_offset_bit_pattern_preserved() {
    let nan_val = f32::NAN;
    let record = CalibrationRecord {
        sensor_id: 999,
        sensor_type: SensorType::Temperature,
        offset: nan_val,
        scale: 1.0,
        calibrated_at: 0,
        notes: None,
    };
    let raw = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (CalibrationRecord, _) = decode_from_slice(&payload).expect("decode failed");
    // NaN != NaN by definition, so compare bit patterns
    assert_eq!(
        nan_val.to_bits(),
        decoded.offset.to_bits(),
        "NaN bit pattern must be preserved through encode/decode"
    );
}

// Test 22: Vec of all 5 SensorType variants via checksum
#[test]
fn test_vec_all_sensor_type_variants_roundtrip() {
    let variants = vec![
        SensorType::Temperature,
        SensorType::Pressure,
        SensorType::Humidity,
        SensorType::Light,
        SensorType::Accelerometer,
    ];
    let raw = encode_to_vec(&variants).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Vec<SensorType>, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(variants, decoded);
    assert_eq!(decoded.len(), 5);
}
