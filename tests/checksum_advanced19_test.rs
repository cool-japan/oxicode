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
// Domain types: Medical device / patient monitoring
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VitalType {
    HeartRate,
    BloodPressure,
    Temperature,
    OxygenSaturation,
    RespiratoryRate,
    GlucoseLevel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VitalReading {
    patient_id: u64,
    vital_type: VitalType,
    value: f64,
    timestamp: u64,
    device_id: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PatientRecord {
    patient_id: u64,
    name: String,
    readings: Vec<VitalReading>,
    emergency_contact: String,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_vital_reading(patient_id: u64, vital_type: VitalType, value: f64) -> VitalReading {
    VitalReading {
        patient_id,
        vital_type,
        value,
        timestamp: 1_700_000_000u64 + patient_id,
        device_id: (patient_id % 100) as u32,
    }
}

fn make_patient_record(patient_id: u64, readings: Vec<VitalReading>) -> PatientRecord {
    PatientRecord {
        patient_id,
        name: format!("Patient_{}", patient_id),
        readings,
        emergency_contact: format!("+1-555-{:04}", patient_id % 10000),
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
    let reading = make_vital_reading(1001, VitalType::HeartRate, 72.0);
    let bytes = encode_to_vec(&reading).expect("encode VitalReading failed");
    let wrapped = wrap_with_checksum(&bytes);
    assert_eq!(
        wrapped.len(),
        HEADER_SIZE + bytes.len(),
        "wrapped length must equal HEADER_SIZE + payload length"
    );
}

// ---------------------------------------------------------------------------
// Test 3: VitalReading wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vital_reading_wrap_unwrap_roundtrip() {
    let reading = make_vital_reading(2001, VitalType::BloodPressure, 120.5);
    let bytes = encode_to_vec(&reading).expect("encode VitalReading failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap VitalReading failed");
    let (decoded, _): (VitalReading, usize) =
        decode_from_slice(&recovered).expect("decode VitalReading failed");
    assert_eq!(reading, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: PatientRecord wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_patient_record_wrap_unwrap_roundtrip() {
    let readings = vec![
        make_vital_reading(3001, VitalType::HeartRate, 68.0),
        make_vital_reading(3001, VitalType::Temperature, 36.8),
        make_vital_reading(3001, VitalType::OxygenSaturation, 98.0),
    ];
    let record = make_patient_record(3001, readings);
    let bytes = encode_to_vec(&record).expect("encode PatientRecord failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap PatientRecord failed");
    let (decoded, _): (PatientRecord, usize) =
        decode_from_slice(&recovered).expect("decode PatientRecord failed");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: VitalType::HeartRate variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_vital_type_heart_rate_roundtrip() {
    let reading = make_vital_reading(4001, VitalType::HeartRate, 85.0);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (VitalReading, usize) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.vital_type, VitalType::HeartRate);
    assert_eq!(decoded.value, 85.0_f64);
}

// ---------------------------------------------------------------------------
// Test 6: VitalType::BloodPressure variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_vital_type_blood_pressure_roundtrip() {
    let reading = make_vital_reading(4002, VitalType::BloodPressure, 130.0);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (VitalReading, usize) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.vital_type, VitalType::BloodPressure);
}

// ---------------------------------------------------------------------------
// Test 7: VitalType::Temperature variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_vital_type_temperature_roundtrip() {
    let reading = make_vital_reading(4003, VitalType::Temperature, 37.2);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (VitalReading, usize) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.vital_type, VitalType::Temperature);
    assert!((decoded.value - 37.2_f64).abs() < 1e-10);
}

// ---------------------------------------------------------------------------
// Test 8: VitalType::OxygenSaturation variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_vital_type_oxygen_saturation_roundtrip() {
    let reading = make_vital_reading(4004, VitalType::OxygenSaturation, 99.0);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (VitalReading, usize) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.vital_type, VitalType::OxygenSaturation);
}

// ---------------------------------------------------------------------------
// Test 9: VitalType::RespiratoryRate variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_vital_type_respiratory_rate_roundtrip() {
    let reading = make_vital_reading(4005, VitalType::RespiratoryRate, 16.0);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (VitalReading, usize) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.vital_type, VitalType::RespiratoryRate);
}

// ---------------------------------------------------------------------------
// Test 10: VitalType::GlucoseLevel variant roundtrips correctly
// ---------------------------------------------------------------------------

#[test]
fn test_vital_type_glucose_level_roundtrip() {
    let reading = make_vital_reading(4006, VitalType::GlucoseLevel, 5.8);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (VitalReading, usize) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.vital_type, VitalType::GlucoseLevel);
}

// ---------------------------------------------------------------------------
// Test 11: corruption in payload bytes[4..] is detected by unwrap_with_checksum
// ---------------------------------------------------------------------------

#[test]
fn test_corruption_after_byte4_detected() {
    let reading = make_vital_reading(5001, VitalType::HeartRate, 70.0);
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
// Test 12: double wrap/unwrap roundtrip (nested integrity checking)
// ---------------------------------------------------------------------------

#[test]
fn test_double_wrap_unwrap_roundtrip() {
    let reading = make_vital_reading(6001, VitalType::Temperature, 38.5);
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

    let (decoded, _): (VitalReading, usize) =
        decode_from_slice(&after_second_unwrap).expect("decode failed");
    assert_eq!(reading, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: PatientRecord with empty readings list
// ---------------------------------------------------------------------------

#[test]
fn test_patient_record_empty_readings_roundtrip() {
    let record = make_patient_record(7001, vec![]);
    let bytes = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (PatientRecord, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(record, decoded);
    assert!(decoded.readings.is_empty(), "readings must be empty");
}

// ---------------------------------------------------------------------------
// Test 14: PatientRecord with 500 readings (large payload stress test)
// ---------------------------------------------------------------------------

#[test]
fn test_patient_record_500_readings_roundtrip() {
    let readings: Vec<VitalReading> = (0u64..500)
        .map(|i| {
            let vital_type = match i % 6 {
                0 => VitalType::HeartRate,
                1 => VitalType::BloodPressure,
                2 => VitalType::Temperature,
                3 => VitalType::OxygenSaturation,
                4 => VitalType::RespiratoryRate,
                _ => VitalType::GlucoseLevel,
            };
            make_vital_reading(8001, vital_type, 60.0 + i as f64 * 0.1)
        })
        .collect();
    let record = make_patient_record(8001, readings);
    let bytes = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (PatientRecord, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(record, decoded);
    assert_eq!(decoded.readings.len(), 500);
}

// ---------------------------------------------------------------------------
// Test 15: overhead check — wrapped is exactly HEADER_SIZE bytes larger
// ---------------------------------------------------------------------------

#[test]
fn test_wrap_overhead_is_exactly_header_size() {
    let reading = make_vital_reading(9001, VitalType::OxygenSaturation, 97.5);
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
// Test 16: unwrap_with_checksum on pristine data returns exact original bytes
// ---------------------------------------------------------------------------

#[test]
fn test_unwrap_returns_exact_original_bytes() {
    let reading = make_vital_reading(10001, VitalType::RespiratoryRate, 18.0);
    let original_bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&original_bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    assert_eq!(
        recovered, original_bytes,
        "recovered bytes must be byte-for-byte identical to original"
    );
}

// ---------------------------------------------------------------------------
// Test 17: multiple patients, all records survive independent wrap/unwrap
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_patients_independent_wrap_unwrap() {
    let patients: Vec<PatientRecord> = (1u64..=10)
        .map(|id| {
            let readings = vec![
                make_vital_reading(id, VitalType::HeartRate, 60.0 + id as f64),
                make_vital_reading(id, VitalType::GlucoseLevel, 4.5 + id as f64 * 0.1),
            ];
            make_patient_record(id, readings)
        })
        .collect();

    for record in &patients {
        let bytes = encode_to_vec(record).expect("encode failed");
        let wrapped = wrap_with_checksum(&bytes);
        let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
        let (decoded, _): (PatientRecord, usize) =
            decode_from_slice(&recovered).expect("decode failed");
        assert_eq!(
            record, &decoded,
            "mismatch for patient_id {}",
            record.patient_id
        );
    }
}

// ---------------------------------------------------------------------------
// Test 18: tampering with a single byte inside payload (XOR 0x01) is detected
// ---------------------------------------------------------------------------

#[test]
fn test_single_byte_tamper_detected() {
    let reading = make_vital_reading(11001, VitalType::BloodPressure, 115.0);
    let bytes = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);

    // Flip the very first payload byte (just after the header)
    let mut tampered = wrapped.clone();
    tampered[HEADER_SIZE] ^= 0x01;

    let result = unwrap_with_checksum(&tampered);
    assert!(result.is_err(), "single-byte tamper must be detected");
}

// ---------------------------------------------------------------------------
// Test 19: all six VitalType variants can be wrapped and roundtripped together
// ---------------------------------------------------------------------------

#[test]
fn test_all_vital_type_variants_in_one_record() {
    let variants = vec![
        VitalType::HeartRate,
        VitalType::BloodPressure,
        VitalType::Temperature,
        VitalType::OxygenSaturation,
        VitalType::RespiratoryRate,
        VitalType::GlucoseLevel,
    ];
    let readings: Vec<VitalReading> = variants
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, vt)| make_vital_reading(12001, vt, 50.0 + i as f64))
        .collect();
    let record = make_patient_record(12001, readings.clone());

    let bytes = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (PatientRecord, usize) =
        decode_from_slice(&recovered).expect("decode failed");

    assert_eq!(decoded.readings.len(), 6);
    for (original, decoded_reading) in readings.iter().zip(decoded.readings.iter()) {
        assert_eq!(original, decoded_reading);
    }
}

// ---------------------------------------------------------------------------
// Test 20: zero-length patient name and empty emergency_contact survive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_patient_record_empty_string_fields_roundtrip() {
    let record = PatientRecord {
        patient_id: 13001,
        name: String::new(),
        readings: vec![make_vital_reading(13001, VitalType::Temperature, 36.6)],
        emergency_contact: String::new(),
    };
    let bytes = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (PatientRecord, usize) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(record, decoded);
    assert!(decoded.name.is_empty());
    assert!(decoded.emergency_contact.is_empty());
}

// ---------------------------------------------------------------------------
// Test 21: extreme vital values (f64::MAX, f64::MIN_POSITIVE) survive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vital_reading_extreme_float_values_roundtrip() {
    let reading_max = VitalReading {
        patient_id: 14001,
        vital_type: VitalType::GlucoseLevel,
        value: f64::MAX,
        timestamp: u64::MAX,
        device_id: u32::MAX,
    };
    let bytes = encode_to_vec(&reading_max).expect("encode MAX failed");
    let wrapped = wrap_with_checksum(&bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap MAX failed");
    let (decoded, _): (VitalReading, usize) =
        decode_from_slice(&recovered).expect("decode MAX failed");
    assert_eq!(reading_max, decoded);

    let reading_min = VitalReading {
        patient_id: 14002,
        vital_type: VitalType::HeartRate,
        value: f64::MIN_POSITIVE,
        timestamp: 0,
        device_id: 0,
    };
    let bytes2 = encode_to_vec(&reading_min).expect("encode MIN failed");
    let wrapped2 = wrap_with_checksum(&bytes2);
    let recovered2 = unwrap_with_checksum(&wrapped2).expect("unwrap MIN failed");
    let (decoded2, _): (VitalReading, usize) =
        decode_from_slice(&recovered2).expect("decode MIN failed");
    assert_eq!(reading_min, decoded2);
}

// ---------------------------------------------------------------------------
// Test 22: corrupting bytes at the very end of wrapped data is detected
// ---------------------------------------------------------------------------

#[test]
fn test_corruption_at_end_of_wrapped_data_detected() {
    let readings: Vec<VitalReading> = (0u64..10)
        .map(|i| make_vital_reading(15001, VitalType::OxygenSaturation, 95.0 + i as f64 * 0.2))
        .collect();
    let record = make_patient_record(15001, readings);
    let bytes = encode_to_vec(&record).expect("encode failed");
    let wrapped = wrap_with_checksum(&bytes);

    // Flip the last byte of the wrapped output (deepest payload byte)
    let mut corrupted = wrapped.clone();
    let last = corrupted.len() - 1;
    corrupted[last] ^= 0xFF;

    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption at the last payload byte must be detected"
    );
}
