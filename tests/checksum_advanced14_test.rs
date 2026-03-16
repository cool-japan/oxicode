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

#[derive(Debug, PartialEq, Encode, Decode)]
enum SensorKind {
    Temperature,
    Humidity,
    Pressure,
    Light,
    Motion,
    Gas,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Reading {
    sensor_id: u32,
    kind: SensorKind,
    value_raw: i32,
    timestamp_s: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SensorReport {
    device_id: u64,
    readings: Vec<Reading>,
    battery_pct: u8,
    firmware_ver: String,
}

// Test 1: Reading with Temperature roundtrip via checksum
#[test]
fn test_reading_temperature_roundtrip_checksum() {
    let reading = Reading {
        sensor_id: 1,
        kind: SensorKind::Temperature,
        value_raw: 2350,
        timestamp_s: 1_700_000_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Reading, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(reading, decoded);
}

// Test 2: Reading with Humidity roundtrip via checksum
#[test]
fn test_reading_humidity_roundtrip_checksum() {
    let reading = Reading {
        sensor_id: 2,
        kind: SensorKind::Humidity,
        value_raw: 6520,
        timestamp_s: 1_700_001_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Reading, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(reading, decoded);
}

// Test 3: Reading with Pressure roundtrip via checksum
#[test]
fn test_reading_pressure_roundtrip_checksum() {
    let reading = Reading {
        sensor_id: 3,
        kind: SensorKind::Pressure,
        value_raw: 101325,
        timestamp_s: 1_700_002_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Reading, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(reading, decoded);
}

// Test 4: Reading with Light roundtrip via checksum
#[test]
fn test_reading_light_roundtrip_checksum() {
    let reading = Reading {
        sensor_id: 4,
        kind: SensorKind::Light,
        value_raw: 4800,
        timestamp_s: 1_700_003_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Reading, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(reading, decoded);
}

// Test 5: Reading with Motion roundtrip via checksum
#[test]
fn test_reading_motion_roundtrip_checksum() {
    let reading = Reading {
        sensor_id: 5,
        kind: SensorKind::Motion,
        value_raw: 1,
        timestamp_s: 1_700_004_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Reading, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(reading, decoded);
}

// Test 6: Reading with Gas roundtrip via checksum
#[test]
fn test_reading_gas_roundtrip_checksum() {
    let reading = Reading {
        sensor_id: 6,
        kind: SensorKind::Gas,
        value_raw: 450,
        timestamp_s: 1_700_005_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Reading, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(reading, decoded);
}

// Test 7: SensorReport roundtrip via checksum
#[test]
fn test_sensor_report_roundtrip_checksum() {
    let report = SensorReport {
        device_id: 0xDEAD_BEEF_1234_5678,
        readings: vec![
            Reading {
                sensor_id: 10,
                kind: SensorKind::Temperature,
                value_raw: -500,
                timestamp_s: 1_700_010_000,
            },
            Reading {
                sensor_id: 11,
                kind: SensorKind::Humidity,
                value_raw: 7800,
                timestamp_s: 1_700_010_001,
            },
        ],
        battery_pct: 87,
        firmware_ver: "v1.4.2-stable".to_string(),
    };
    let raw = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SensorReport, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(report, decoded);
}

// Test 8: HEADER_SIZE == 16
#[test]
fn test_header_size_is_16() {
    assert_eq!(HEADER_SIZE, 16);
}

// Test 9: Wrapped length == HEADER_SIZE + raw encoded length
#[test]
fn test_wrapped_length_equals_header_plus_encoded() {
    let reading = Reading {
        sensor_id: 42,
        kind: SensorKind::Temperature,
        value_raw: 2500,
        timestamp_s: 1_710_000_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    assert_eq!(wrapped.len(), HEADER_SIZE + raw.len());
}

// Test 10: Corruption detection — aggressive bit flip after index 4
#[test]
fn test_corruption_detected_aggressive_bit_flip() {
    let report = SensorReport {
        device_id: 0xCAFE_BABE,
        readings: vec![Reading {
            sensor_id: 99,
            kind: SensorKind::Gas,
            value_raw: 999,
            timestamp_s: 1_720_000_000,
        }],
        battery_pct: 50,
        firmware_ver: "v2.0.0".to_string(),
    };
    let raw = encode_to_vec(&report).expect("encode failed");
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

// Test 11: Truncation detection — remove last byte
#[test]
fn test_truncated_data_detected() {
    let reading = Reading {
        sensor_id: 77,
        kind: SensorKind::Pressure,
        value_raw: 100000,
        timestamp_s: 1_730_000_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let truncated = &wrapped[..wrapped.len() - 1];
    let result = unwrap_with_checksum(truncated);
    assert!(result.is_err(), "truncated data must return Err");
}

// Test 12: Zero-fill detection — replace entire wrapped buffer with zeros
#[test]
fn test_zero_fill_detected() {
    let report = SensorReport {
        device_id: 1,
        readings: vec![],
        battery_pct: 100,
        firmware_ver: "v0.0.1".to_string(),
    };
    let raw = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let zeroed = vec![0u8; wrapped.len()];
    let result = unwrap_with_checksum(&zeroed);
    assert!(result.is_err(), "zero-filled data must return Err");
}

// Test 13: Vec<Reading> roundtrip via checksum
#[test]
fn test_vec_of_readings_roundtrip_checksum() {
    let readings = vec![
        Reading {
            sensor_id: 101,
            kind: SensorKind::Temperature,
            value_raw: 2100,
            timestamp_s: 1_700_020_000,
        },
        Reading {
            sensor_id: 102,
            kind: SensorKind::Humidity,
            value_raw: 5500,
            timestamp_s: 1_700_020_001,
        },
        Reading {
            sensor_id: 103,
            kind: SensorKind::Light,
            value_raw: 3200,
            timestamp_s: 1_700_020_002,
        },
        Reading {
            sensor_id: 104,
            kind: SensorKind::Motion,
            value_raw: 0,
            timestamp_s: 1_700_020_003,
        },
    ];
    let raw = encode_to_vec(&readings).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Vec<Reading>, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(readings, decoded);
}

// Test 14: Option<SensorReport> Some roundtrip via checksum
#[test]
fn test_option_some_sensor_report_roundtrip_checksum() {
    let val: Option<SensorReport> = Some(SensorReport {
        device_id: 0xBEEF,
        readings: vec![Reading {
            sensor_id: 200,
            kind: SensorKind::Gas,
            value_raw: 350,
            timestamp_s: 1_740_000_000,
        }],
        battery_pct: 72,
        firmware_ver: "v3.1.0-iot".to_string(),
    });
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Option<SensorReport>, _) =
        decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 15: Option<SensorReport> None roundtrip via checksum
#[test]
fn test_option_none_sensor_report_roundtrip_checksum() {
    let val: Option<SensorReport> = None;
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Option<SensorReport>, _) =
        decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 16: All SensorKind variants encode to different bytes
#[test]
fn test_all_sensor_kind_variants_encode_differently() {
    let variants = [
        SensorKind::Temperature,
        SensorKind::Humidity,
        SensorKind::Pressure,
        SensorKind::Light,
        SensorKind::Motion,
        SensorKind::Gas,
    ];
    let encoded: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode variant failed"))
        .collect();
    for i in 0..encoded.len() {
        for j in (i + 1)..encoded.len() {
            assert_ne!(
                encoded[i], encoded[j],
                "SensorKind variants at index {} and {} must encode differently",
                i, j
            );
        }
    }
    assert_eq!(encoded.len(), 6);
}

// Test 17: Consumed bytes == HEADER_SIZE + payload bytes consumed
#[test]
fn test_consumed_bytes_from_decode() {
    let reading = Reading {
        sensor_id: 333,
        kind: SensorKind::Pressure,
        value_raw: 98765,
        timestamp_s: 1_780_000_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (_, consumed) = decode_from_slice::<Reading>(&payload).expect("decode failed");
    assert_eq!(wrapped.len(), HEADER_SIZE + consumed);
}

// Test 18: Unwrap gives back exact original encoded bytes
#[test]
fn test_unwrap_gives_back_exact_encoded_bytes() {
    let report = SensorReport {
        device_id: 0xFF00_FF00,
        readings: vec![Reading {
            sensor_id: 500,
            kind: SensorKind::Light,
            value_raw: 1234,
            timestamp_s: 1_790_000_000,
        }],
        battery_pct: 10,
        firmware_ver: "v0.9.9-beta".to_string(),
    };
    let raw = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    assert_eq!(
        raw, payload,
        "unwrapped payload must equal original encoded bytes"
    );
}

// Test 19: Large report with many readings roundtrip via checksum
#[test]
fn test_large_report_many_readings_roundtrip_checksum() {
    let readings: Vec<Reading> = (0..500u32)
        .map(|i| Reading {
            sensor_id: i,
            kind: match i % 6 {
                0 => SensorKind::Temperature,
                1 => SensorKind::Humidity,
                2 => SensorKind::Pressure,
                3 => SensorKind::Light,
                4 => SensorKind::Motion,
                _ => SensorKind::Gas,
            },
            value_raw: (i as i32) * 13 - 1000,
            timestamp_s: 1_700_000_000 + (i as u64) * 60,
        })
        .collect();
    let report = SensorReport {
        device_id: 0xAAAA_BBBB_CCCC_DDDD,
        readings,
        battery_pct: 44,
        firmware_ver: "v5.0.0-release-candidate".to_string(),
    };
    let raw = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SensorReport, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(report.device_id, decoded.device_id);
    assert_eq!(report.readings.len(), decoded.readings.len());
    assert_eq!(report.battery_pct, decoded.battery_pct);
    assert_eq!(report.firmware_ver, decoded.firmware_ver);
    assert_eq!(report, decoded);
}

// Test 20: Deterministic — wrap same data twice gives identical output
#[test]
fn test_wrap_same_data_twice_identical_output() {
    let reading = Reading {
        sensor_id: 9999,
        kind: SensorKind::Motion,
        value_raw: 1,
        timestamp_s: 1_770_000_000,
    };
    let raw1 = encode_to_vec(&reading).expect("encode first failed");
    let raw2 = encode_to_vec(&reading).expect("encode second failed");
    let wrapped1 = wrap_with_checksum(&raw1);
    let wrapped2 = wrap_with_checksum(&raw2);
    assert_eq!(
        wrapped1, wrapped2,
        "wrapping identical data must be deterministic"
    );
}

// Test 21: Reading with negative value_raw (below-zero temperature) roundtrip
#[test]
fn test_reading_negative_value_raw_roundtrip() {
    let reading = Reading {
        sensor_id: 55,
        kind: SensorKind::Temperature,
        value_raw: i32::MIN,
        timestamp_s: 1_800_000_000,
    };
    let raw = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Reading, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(reading, decoded);
    assert_eq!(decoded.value_raw, i32::MIN);
}

// Test 22: SensorReport with empty readings Vec and zero battery roundtrip
#[test]
fn test_sensor_report_empty_readings_zero_battery_roundtrip() {
    let report = SensorReport {
        device_id: u64::MAX,
        readings: vec![],
        battery_pct: 0,
        firmware_ver: String::new(),
    };
    let raw = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    assert_eq!(wrapped.len(), HEADER_SIZE + raw.len());
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SensorReport, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(report, decoded);
    assert!(decoded.readings.is_empty());
    assert_eq!(decoded.battery_pct, 0);
    assert_eq!(decoded.device_id, u64::MAX);
}
