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
use oxicode::checksum::{unwrap_with_checksum, wrap_with_checksum, HEADER_SIZE};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types: Nuclear plant / industrial safety checksum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SafetyLevel {
    Normal,
    Advisory,
    Caution,
    Warning,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorType {
    Temperature,
    Pressure,
    Radiation,
    Vibration,
    Flow,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SafetyReading {
    sensor_id: u64,
    sensor_type: SensorType,
    value: f64,
    safety_level: SafetyLevel,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SafetyReport {
    facility_id: u32,
    readings: Vec<SafetyReading>,
    alert_count: u32,
    report_time: u64,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_safety_reading(
    sensor_id: u64,
    sensor_type: SensorType,
    value: f64,
    safety_level: SafetyLevel,
) -> SafetyReading {
    SafetyReading {
        sensor_id,
        sensor_type,
        value,
        safety_level,
        timestamp: 1_700_000_000u64 + sensor_id,
    }
}

fn make_safety_report(
    facility_id: u32,
    readings: Vec<SafetyReading>,
    alert_count: u32,
) -> SafetyReport {
    SafetyReport {
        facility_id,
        readings,
        alert_count,
        report_time: 1_700_000_000u64 + facility_id as u64,
    }
}

// ---------------------------------------------------------------------------
// Test 1: HEADER_SIZE constant equals 16
// ---------------------------------------------------------------------------

#[test]
fn test_header_size_is_16() {
    assert_eq!(HEADER_SIZE, 16, "HEADER_SIZE must be exactly 16 bytes");
}

// ---------------------------------------------------------------------------
// Test 2: wrap_with_checksum produces output of len == HEADER_SIZE + payload
// ---------------------------------------------------------------------------

#[test]
fn test_wrap_output_length_matches_header_plus_payload() {
    let reading = make_safety_reading(1001, SensorType::Temperature, 320.5, SafetyLevel::Normal);
    let bytes = encode_to_vec(&reading).expect("encode SafetyReading failed");
    let wrapped = wrap_with_checksum(&bytes);
    assert_eq!(
        wrapped.len(),
        HEADER_SIZE + bytes.len(),
        "wrapped length must equal HEADER_SIZE + payload length"
    );
}

// ---------------------------------------------------------------------------
// Test 3: SafetyReading wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_safety_reading_wrap_unwrap_roundtrip() {
    let reading = make_safety_reading(2001, SensorType::Pressure, 150.0, SafetyLevel::Advisory);
    let bytes = encode_to_vec(&reading).expect("encode SafetyReading failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap SafetyReading failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode SafetyReading failed");
    assert_eq!(reading, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: SafetyReport wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_safety_report_wrap_unwrap_roundtrip() {
    let readings = vec![
        make_safety_reading(3001, SensorType::Radiation, 0.25, SafetyLevel::Caution),
        make_safety_reading(3002, SensorType::Vibration, 5.8, SafetyLevel::Warning),
        make_safety_reading(3003, SensorType::Flow, 200.0, SafetyLevel::Normal),
    ];
    let report = make_safety_report(1001, readings, 2);
    let bytes = encode_to_vec(&report).expect("encode SafetyReport failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap SafetyReport failed");
    let (decoded, _): (SafetyReport, usize) =
        decode_from_slice(&recovered).expect("decode SafetyReport failed");
    assert_eq!(report, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: SafetyLevel::Normal variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_safety_level_normal_roundtrip() {
    let reading = make_safety_reading(4001, SensorType::Temperature, 290.0, SafetyLevel::Normal);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.safety_level, SafetyLevel::Normal);
}

// ---------------------------------------------------------------------------
// Test 6: SafetyLevel::Advisory variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_safety_level_advisory_roundtrip() {
    let reading = make_safety_reading(4002, SensorType::Pressure, 180.0, SafetyLevel::Advisory);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.safety_level, SafetyLevel::Advisory);
}

// ---------------------------------------------------------------------------
// Test 7: SafetyLevel::Caution variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_safety_level_caution_roundtrip() {
    let reading = make_safety_reading(4003, SensorType::Radiation, 1.0, SafetyLevel::Caution);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.safety_level, SafetyLevel::Caution);
    assert!((decoded.value - 1.0_f64).abs() < 1e-10);
}

// ---------------------------------------------------------------------------
// Test 8: SafetyLevel::Warning variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_safety_level_warning_roundtrip() {
    let reading = make_safety_reading(4004, SensorType::Vibration, 12.3, SafetyLevel::Warning);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.safety_level, SafetyLevel::Warning);
}

// ---------------------------------------------------------------------------
// Test 9: SafetyLevel::Emergency variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_safety_level_emergency_roundtrip() {
    let reading = make_safety_reading(4005, SensorType::Radiation, 99.9, SafetyLevel::Emergency);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.safety_level, SafetyLevel::Emergency);
}

// ---------------------------------------------------------------------------
// Test 10: SensorType::Temperature variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_sensor_type_temperature_roundtrip() {
    let reading = make_safety_reading(5001, SensorType::Temperature, 350.0, SafetyLevel::Normal);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.sensor_type, SensorType::Temperature);
    assert!((decoded.value - 350.0_f64).abs() < 1e-10);
}

// ---------------------------------------------------------------------------
// Test 11: SensorType::Pressure variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_sensor_type_pressure_roundtrip() {
    let reading = make_safety_reading(5002, SensorType::Pressure, 220.5, SafetyLevel::Advisory);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.sensor_type, SensorType::Pressure);
}

// ---------------------------------------------------------------------------
// Test 12: SensorType::Radiation variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_sensor_type_radiation_roundtrip() {
    let reading = make_safety_reading(5003, SensorType::Radiation, 0.05, SafetyLevel::Normal);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.sensor_type, SensorType::Radiation);
    assert!((decoded.value - 0.05_f64).abs() < 1e-10);
}

// ---------------------------------------------------------------------------
// Test 13: SensorType::Vibration variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_sensor_type_vibration_roundtrip() {
    let reading = make_safety_reading(5004, SensorType::Vibration, 3.7, SafetyLevel::Caution);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.sensor_type, SensorType::Vibration);
}

// ---------------------------------------------------------------------------
// Test 14: SensorType::Flow variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_sensor_type_flow_roundtrip() {
    let reading = make_safety_reading(5005, SensorType::Flow, 450.0, SafetyLevel::Normal);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.sensor_type, SensorType::Flow);
}

// ---------------------------------------------------------------------------
// Test 15: corruption in wrapped[4..] is detected by unwrap_with_checksum
// ---------------------------------------------------------------------------

#[test]
fn test_corruption_after_byte4_detected() {
    let reading = make_safety_reading(6001, SensorType::Temperature, 310.0, SafetyLevel::Normal);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);

    // Corrupt according to spec: flip all bytes from index 4 onwards in wrapped
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }

    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption in wrapped[4..] must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 16: overhead check — wrapped is exactly HEADER_SIZE bytes larger
// ---------------------------------------------------------------------------

#[test]
fn test_wrap_overhead_is_exactly_header_size() {
    let reading = make_safety_reading(7001, SensorType::Pressure, 175.0, SafetyLevel::Advisory);
    let plain = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&plain);
    assert_eq!(
        wrapped.len(),
        plain.len() + HEADER_SIZE,
        "overhead must be exactly HEADER_SIZE ({}) bytes",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 17: double wrap/unwrap roundtrip (nested integrity checking)
// ---------------------------------------------------------------------------

#[test]
fn test_double_wrap_unwrap_roundtrip() {
    let reading = make_safety_reading(8001, SensorType::Radiation, 2.1, SafetyLevel::Warning);
    let bytes = encode_to_vec(&reading).expect("encode failed");

    // First wrap
    let wrapped_once = wrap_with_checksum(&bytes);
    // Second wrap — treat the already-wrapped bytes as the new payload
    let wrapped_twice = wrap_with_checksum(&wrapped_once);

    // Unwrap outer layer
    let after_first_unwrap = unwrap_with_checksum(&wrapped_twice).expect("first unwrap failed");
    // Unwrap inner layer
    let after_second_unwrap =
        unwrap_with_checksum(&after_first_unwrap).expect("second unwrap failed");

    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&after_second_unwrap).expect("decode failed");
    assert_eq!(reading, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: SafetyReport with 500 readings (large payload stress test)
// ---------------------------------------------------------------------------

#[test]
fn test_safety_report_500_readings_roundtrip() {
    let readings: Vec<SafetyReading> = (0u64..500)
        .map(|i| {
            let sensor_type = match i % 5 {
                0 => SensorType::Temperature,
                1 => SensorType::Pressure,
                2 => SensorType::Radiation,
                3 => SensorType::Vibration,
                _ => SensorType::Flow,
            };
            let safety_level = match i % 5 {
                0 => SafetyLevel::Normal,
                1 => SafetyLevel::Advisory,
                2 => SafetyLevel::Caution,
                3 => SafetyLevel::Warning,
                _ => SafetyLevel::Emergency,
            };
            make_safety_reading(9000 + i, sensor_type, 100.0 + i as f64 * 0.5, safety_level)
        })
        .collect();
    let alert_count = readings
        .iter()
        .filter(|r| r.safety_level != SafetyLevel::Normal)
        .count() as u32;
    let report = make_safety_report(2001, readings, alert_count);
    let bytes = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReport, usize) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(report, decoded);
    assert_eq!(decoded.readings.len(), 500);
}

// ---------------------------------------------------------------------------
// Test 19: SafetyReport with empty readings list
// ---------------------------------------------------------------------------

#[test]
fn test_safety_report_empty_readings_roundtrip() {
    let report = make_safety_report(3001, vec![], 0);
    let bytes = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReport, usize) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(report, decoded);
    assert!(decoded.readings.is_empty(), "readings must be empty");
}

// ---------------------------------------------------------------------------
// Test 20: unwrap_with_checksum on pristine data returns exact original bytes
// ---------------------------------------------------------------------------

#[test]
fn test_unwrap_returns_exact_original_bytes() {
    let reading = make_safety_reading(10001, SensorType::Vibration, 8.4, SafetyLevel::Warning);
    let original_bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&original_bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    assert_eq!(
        recovered, original_bytes,
        "recovered bytes must be byte-for-byte identical to original"
    );
}

// ---------------------------------------------------------------------------
// Test 21: all five SafetyLevel and all five SensorType variants in one report
// ---------------------------------------------------------------------------

#[test]
fn test_all_variants_in_one_report() {
    let readings = vec![
        make_safety_reading(11001, SensorType::Temperature, 300.0, SafetyLevel::Normal),
        make_safety_reading(11002, SensorType::Pressure, 190.0, SafetyLevel::Advisory),
        make_safety_reading(11003, SensorType::Radiation, 0.8, SafetyLevel::Caution),
        make_safety_reading(11004, SensorType::Vibration, 9.9, SafetyLevel::Warning),
        make_safety_reading(11005, SensorType::Flow, 350.0, SafetyLevel::Emergency),
    ];
    let report = make_safety_report(4001, readings.clone(), 4);

    let bytes = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SafetyReport, usize) = decode_from_slice(&recovered).expect("decode failed");

    assert_eq!(decoded.readings.len(), 5);
    for (original, decoded_reading) in readings.iter().zip(decoded.readings.iter()) {
        assert_eq!(original, decoded_reading);
    }
}

// ---------------------------------------------------------------------------
// Test 22: SafetyReading with extreme f64 and u64 boundary values survives roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_safety_reading_extreme_values_roundtrip() {
    let reading_max = SafetyReading {
        sensor_id: u64::MAX,
        sensor_type: SensorType::Radiation,
        value: f64::MAX,
        safety_level: SafetyLevel::Emergency,
        timestamp: u64::MAX,
    };
    let bytes = encode_to_vec(&reading_max).expect("encode MAX failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap MAX failed");
    let (decoded, _): (SafetyReading, usize) =
        decode_from_slice(&recovered).expect("decode MAX failed");
    assert_eq!(reading_max, decoded);

    let reading_min = SafetyReading {
        sensor_id: 0,
        sensor_type: SensorType::Flow,
        value: f64::MIN_POSITIVE,
        safety_level: SafetyLevel::Normal,
        timestamp: 0,
    };
    let bytes2 = encode_to_vec(&reading_min).expect("encode MIN failed");
    let wrapped2 = wrap_with_checksum(&bytes2);
    let recovered2 = unwrap_with_checksum(&wrapped2).expect("unwrap MIN failed");
    let (decoded2, _): (SafetyReading, usize) =
        decode_from_slice(&recovered2).expect("decode MIN failed");
    assert_eq!(reading_min, decoded2);
}
