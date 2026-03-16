//! Advanced checksum tests — space exploration / mission control domain theme.
//! Tests OxiCode's checksum API using spacecraft telemetry and mission data types.

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum SpacecraftStatus {
    Nominal,
    Warning,
    Critical,
    Safe,
    LostContact,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum SubsystemId {
    Power,
    Communications,
    ThermalControl,
    ADCS,
    Propulsion,
    Payload,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct TelemetryPoint {
    subsystem: SubsystemId,
    parameter: String,
    value: f64,
    unit: String,
    timestamp_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Spacecraft {
    mission_id: String,
    name: String,
    status: SpacecraftStatus,
    altitude_km: f64,
    velocity_ms: f64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct MissionReport {
    spacecraft: Spacecraft,
    telemetry: Vec<TelemetryPoint>,
    ground_station: String,
    contact_time_ms: u64,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_spacecraft(status: SpacecraftStatus) -> Spacecraft {
    Spacecraft {
        mission_id: "MISSION-ARTEMIS-07".to_string(),
        name: "Artemis VII Command Module".to_string(),
        status,
        altitude_km: 408.5,
        velocity_ms: 7660.0,
    }
}

fn make_telemetry_point(subsystem: SubsystemId, idx: u64) -> TelemetryPoint {
    TelemetryPoint {
        subsystem,
        parameter: format!("param_{}", idx),
        value: idx as f64 * 1.23,
        unit: "SI".to_string(),
        timestamp_ms: 1_700_000_000_000 + idx * 1000,
    }
}

fn make_mission_report(telemetry: Vec<TelemetryPoint>) -> MissionReport {
    MissionReport {
        spacecraft: make_spacecraft(SpacecraftStatus::Nominal),
        telemetry,
        ground_station: "Johnson Space Center".to_string(),
        contact_time_ms: 1_700_000_000_000,
    }
}

// ---------------------------------------------------------------------------
// Test 1: HEADER_SIZE constant equals 16
// ---------------------------------------------------------------------------

#[test]
fn test_header_size_is_16() {
    assert_eq!(
        HEADER_SIZE, 16,
        "HEADER_SIZE must be exactly 16 bytes (MAGIC(3)+VERSION(1)+LEN(8)+CRC32(4))"
    );
}

// ---------------------------------------------------------------------------
// Test 2: wrap/unwrap roundtrip for TelemetryPoint
// ---------------------------------------------------------------------------

#[test]
fn test_telemetry_point_wrap_unwrap_roundtrip() {
    let tp = make_telemetry_point(SubsystemId::Power, 42);
    let bytes = encode_to_vec(&tp).expect("encode TelemetryPoint failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap TelemetryPoint failed");
    let (decoded, _): (TelemetryPoint, usize) =
        decode_from_slice(&unwrapped).expect("decode TelemetryPoint failed");
    assert_eq!(
        tp, decoded,
        "TelemetryPoint roundtrip must preserve all fields"
    );
}

// ---------------------------------------------------------------------------
// Test 3: wrap/unwrap roundtrip for Spacecraft (Nominal status)
// ---------------------------------------------------------------------------

#[test]
fn test_spacecraft_nominal_roundtrip() {
    let sc = make_spacecraft(SpacecraftStatus::Nominal);
    let bytes = encode_to_vec(&sc).expect("encode Spacecraft failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap Spacecraft failed");
    let (decoded, _): (Spacecraft, usize) =
        decode_from_slice(&unwrapped).expect("decode Spacecraft failed");
    assert_eq!(sc, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: wrap/unwrap roundtrip for Spacecraft (Warning status)
// ---------------------------------------------------------------------------

#[test]
fn test_spacecraft_warning_roundtrip() {
    let sc = make_spacecraft(SpacecraftStatus::Warning);
    let bytes = encode_to_vec(&sc).expect("encode Spacecraft Warning failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap Spacecraft Warning failed");
    let (decoded, _): (Spacecraft, usize) =
        decode_from_slice(&unwrapped).expect("decode Spacecraft Warning failed");
    assert_eq!(sc, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: wrap/unwrap roundtrip for Spacecraft (Critical status)
// ---------------------------------------------------------------------------

#[test]
fn test_spacecraft_critical_roundtrip() {
    let sc = make_spacecraft(SpacecraftStatus::Critical);
    let bytes = encode_to_vec(&sc).expect("encode Spacecraft Critical failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap Spacecraft Critical failed");
    let (decoded, _): (Spacecraft, usize) =
        decode_from_slice(&unwrapped).expect("decode Spacecraft Critical failed");
    assert_eq!(sc, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: wrap/unwrap roundtrip for Spacecraft (Safe status)
// ---------------------------------------------------------------------------

#[test]
fn test_spacecraft_safe_roundtrip() {
    let sc = make_spacecraft(SpacecraftStatus::Safe);
    let bytes = encode_to_vec(&sc).expect("encode Spacecraft Safe failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap Spacecraft Safe failed");
    let (decoded, _): (Spacecraft, usize) =
        decode_from_slice(&unwrapped).expect("decode Spacecraft Safe failed");
    assert_eq!(sc, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: wrap/unwrap roundtrip for Spacecraft (LostContact status)
// ---------------------------------------------------------------------------

#[test]
fn test_spacecraft_lost_contact_roundtrip() {
    let sc = make_spacecraft(SpacecraftStatus::LostContact);
    let bytes = encode_to_vec(&sc).expect("encode Spacecraft LostContact failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap Spacecraft LostContact failed");
    let (decoded, _): (Spacecraft, usize) =
        decode_from_slice(&unwrapped).expect("decode Spacecraft LostContact failed");
    assert_eq!(sc, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: all SubsystemId variants survive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_subsystem_ids_roundtrip() {
    let subsystems = vec![
        SubsystemId::Power,
        SubsystemId::Communications,
        SubsystemId::ThermalControl,
        SubsystemId::ADCS,
        SubsystemId::Propulsion,
        SubsystemId::Payload,
    ];
    for (idx, subsystem) in subsystems.into_iter().enumerate() {
        let tp = make_telemetry_point(subsystem, idx as u64);
        let bytes = encode_to_vec(&tp).expect("encode SubsystemId variant failed");
        let wrapped = wrap_with_checksum(&bytes);
        let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap SubsystemId variant failed");
        let (decoded, _): (TelemetryPoint, usize) =
            decode_from_slice(&unwrapped).expect("decode SubsystemId variant failed");
        assert_eq!(tp, decoded, "SubsystemId variant at index {} failed", idx);
    }
}

// ---------------------------------------------------------------------------
// Test 9: MissionReport with empty telemetry roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_mission_report_empty_telemetry_roundtrip() {
    let report = make_mission_report(vec![]);
    let bytes = encode_to_vec(&report).expect("encode MissionReport (empty telemetry) failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped =
        unwrap_with_checksum(&wrapped).expect("unwrap MissionReport (empty telemetry) failed");
    let (decoded, _): (MissionReport, usize) =
        decode_from_slice(&unwrapped).expect("decode MissionReport (empty telemetry) failed");
    assert_eq!(report, decoded);
    assert!(
        decoded.telemetry.is_empty(),
        "telemetry must be empty after roundtrip"
    );
}

// ---------------------------------------------------------------------------
// Test 10: MissionReport with large telemetry (200+ points)
// ---------------------------------------------------------------------------

#[test]
fn test_mission_report_large_telemetry_roundtrip() {
    let subsystems = [
        SubsystemId::Power,
        SubsystemId::Communications,
        SubsystemId::ThermalControl,
        SubsystemId::ADCS,
        SubsystemId::Propulsion,
        SubsystemId::Payload,
    ];
    let telemetry: Vec<TelemetryPoint> = (0u64..210)
        .map(|i| make_telemetry_point(subsystems[(i as usize) % subsystems.len()].clone(), i))
        .collect();
    assert!(
        telemetry.len() >= 200,
        "test requires at least 200 telemetry points"
    );
    let report = make_mission_report(telemetry);
    let bytes = encode_to_vec(&report).expect("encode MissionReport (large telemetry) failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped =
        unwrap_with_checksum(&wrapped).expect("unwrap MissionReport (large telemetry) failed");
    let (decoded, _): (MissionReport, usize) =
        decode_from_slice(&unwrapped).expect("decode MissionReport (large telemetry) failed");
    assert_eq!(report, decoded);
    assert_eq!(decoded.telemetry.len(), 210);
}

// ---------------------------------------------------------------------------
// Test 11: wrapped output is exactly HEADER_SIZE bytes longer than plain encoding
// ---------------------------------------------------------------------------

#[test]
fn test_wrapped_output_length_overhead() {
    let sc = make_spacecraft(SpacecraftStatus::Nominal);
    let plain = encode_to_vec(&sc).expect("plain encode failed");
    let wrapped = wrap_with_checksum(&plain);
    assert_eq!(
        wrapped.len(),
        plain.len() + HEADER_SIZE,
        "wrapped output must be exactly HEADER_SIZE bytes longer than the payload"
    );
}

// ---------------------------------------------------------------------------
// Test 12: corruption detection — flip ALL bytes after index 4
// ---------------------------------------------------------------------------

#[test]
fn test_corruption_detected_after_index_4() {
    let sc = make_spacecraft(SpacecraftStatus::Nominal);
    let bytes = encode_to_vec(&sc).expect("encode for corruption test failed");
    let wrapped = wrap_with_checksum(&bytes);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption after index 4 must be detected, but unwrap returned Ok"
    );
}

// ---------------------------------------------------------------------------
// Test 13: corruption detection for TelemetryPoint
// ---------------------------------------------------------------------------

#[test]
fn test_telemetry_corruption_detected() {
    let tp = make_telemetry_point(SubsystemId::Communications, 99);
    let bytes = encode_to_vec(&tp).expect("encode TelemetryPoint for corruption test failed");
    let wrapped = wrap_with_checksum(&bytes);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption in TelemetryPoint payload must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 14: double wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_double_wrap_unwrap_roundtrip() {
    let sc = make_spacecraft(SpacecraftStatus::Warning);
    let bytes = encode_to_vec(&sc).expect("encode for double-wrap failed");
    // First wrap
    let wrapped_once = wrap_with_checksum(&bytes);
    // Second wrap (wrapping the already-wrapped bytes)
    let wrapped_twice = wrap_with_checksum(&wrapped_once);
    // First unwrap
    let after_first_unwrap =
        unwrap_with_checksum(&wrapped_twice).expect("first unwrap of double-wrapped data failed");
    // Second unwrap
    let after_second_unwrap =
        unwrap_with_checksum(&after_first_unwrap).expect("second unwrap failed");
    let (decoded, _): (Spacecraft, usize) =
        decode_from_slice(&after_second_unwrap).expect("decode after double unwrap failed");
    assert_eq!(
        sc, decoded,
        "double wrap/unwrap must restore original value"
    );
}

// ---------------------------------------------------------------------------
// Test 15: MissionReport with full mixed-subsystem telemetry
// ---------------------------------------------------------------------------

#[test]
fn test_mission_report_full_telemetry_roundtrip() {
    let telemetry = vec![
        TelemetryPoint {
            subsystem: SubsystemId::Power,
            parameter: "bus_voltage_v".to_string(),
            value: 28.4,
            unit: "V".to_string(),
            timestamp_ms: 1_700_000_001_000,
        },
        TelemetryPoint {
            subsystem: SubsystemId::ThermalControl,
            parameter: "panel_temp_c".to_string(),
            value: -40.0,
            unit: "°C".to_string(),
            timestamp_ms: 1_700_000_002_000,
        },
        TelemetryPoint {
            subsystem: SubsystemId::Propulsion,
            parameter: "thruster_delta_v".to_string(),
            value: 12.7,
            unit: "m/s".to_string(),
            timestamp_ms: 1_700_000_003_000,
        },
        TelemetryPoint {
            subsystem: SubsystemId::ADCS,
            parameter: "attitude_error_deg".to_string(),
            value: 0.003,
            unit: "deg".to_string(),
            timestamp_ms: 1_700_000_004_000,
        },
        TelemetryPoint {
            subsystem: SubsystemId::Communications,
            parameter: "downlink_bitrate_bps".to_string(),
            value: 4_000_000.0,
            unit: "bps".to_string(),
            timestamp_ms: 1_700_000_005_000,
        },
        TelemetryPoint {
            subsystem: SubsystemId::Payload,
            parameter: "camera_exposure_ms".to_string(),
            value: 250.0,
            unit: "ms".to_string(),
            timestamp_ms: 1_700_000_006_000,
        },
    ];
    let report = MissionReport {
        spacecraft: Spacecraft {
            mission_id: "DEEP-SPACE-9".to_string(),
            name: "Pioneer Spirit".to_string(),
            status: SpacecraftStatus::Nominal,
            altitude_km: 150_000_000.0,
            velocity_ms: 29_800.0,
        },
        telemetry,
        ground_station: "Deep Space Network — Goldstone".to_string(),
        contact_time_ms: 1_700_000_000_000,
    };
    let bytes = encode_to_vec(&report).expect("encode full MissionReport failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap full MissionReport failed");
    let (decoded, _): (MissionReport, usize) =
        decode_from_slice(&unwrapped).expect("decode full MissionReport failed");
    assert_eq!(report, decoded);
    assert_eq!(decoded.telemetry.len(), 6);
}

// ---------------------------------------------------------------------------
// Test 16: unwrap of valid data returns a Vec equal to original bytes
// ---------------------------------------------------------------------------

#[test]
fn test_unwrap_returns_exact_payload_bytes() {
    let sc = make_spacecraft(SpacecraftStatus::Safe);
    let original_bytes = encode_to_vec(&sc).expect("encode for payload bytes test failed");
    let wrapped = wrap_with_checksum(&original_bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    assert_eq!(
        recovered, original_bytes,
        "unwrap_with_checksum must return the exact original payload bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 17: wrapped bytes start with OXH magic
// ---------------------------------------------------------------------------

#[test]
fn test_wrapped_bytes_have_oxh_magic() {
    let sc = make_spacecraft(SpacecraftStatus::LostContact);
    let bytes = encode_to_vec(&sc).expect("encode for magic test failed");
    let wrapped = wrap_with_checksum(&bytes);
    assert!(
        wrapped.len() >= 3,
        "wrapped output must contain at least the magic bytes"
    );
    assert_eq!(
        &wrapped[..3],
        &[0x4F, 0x58, 0x48],
        "wrapped output must begin with OXH magic bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 18: TelemetryPoint with extreme float values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_telemetry_extreme_float_values_roundtrip() {
    let tp = TelemetryPoint {
        subsystem: SubsystemId::Propulsion,
        parameter: "exhaust_velocity_ms".to_string(),
        value: f64::MAX,
        unit: "m/s".to_string(),
        timestamp_ms: u64::MAX,
    };
    let bytes = encode_to_vec(&tp).expect("encode extreme float TelemetryPoint failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped =
        unwrap_with_checksum(&wrapped).expect("unwrap extreme float TelemetryPoint failed");
    let (decoded, _): (TelemetryPoint, usize) =
        decode_from_slice(&unwrapped).expect("decode extreme float TelemetryPoint failed");
    assert_eq!(tp, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Spacecraft with very long mission_id and name roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_spacecraft_long_string_fields_roundtrip() {
    let sc = Spacecraft {
        mission_id: "ARTEMIS-".repeat(100),
        name: "Long Distance Explorer Probe Mark VII Extended Range Edition".to_string(),
        status: SpacecraftStatus::Nominal,
        altitude_km: 384_400.0, // lunar distance
        velocity_ms: 1_023.0,
    };
    let bytes = encode_to_vec(&sc).expect("encode long-string Spacecraft failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap long-string Spacecraft failed");
    let (decoded, _): (Spacecraft, usize) =
        decode_from_slice(&unwrapped).expect("decode long-string Spacecraft failed");
    assert_eq!(sc, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: MissionReport with 200 telemetry points, all Power subsystem
// ---------------------------------------------------------------------------

#[test]
fn test_mission_report_200_power_telemetry_points() {
    let telemetry: Vec<TelemetryPoint> = (0u64..200)
        .map(|i| TelemetryPoint {
            subsystem: SubsystemId::Power,
            parameter: format!("power_channel_{}", i),
            value: 3.3 + (i as f64) * 0.01,
            unit: "W".to_string(),
            timestamp_ms: 1_700_000_000_000 + i * 500,
        })
        .collect();
    let report = make_mission_report(telemetry);
    let bytes = encode_to_vec(&report).expect("encode 200-point MissionReport failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap 200-point MissionReport failed");
    let (decoded, _): (MissionReport, usize) =
        decode_from_slice(&unwrapped).expect("decode 200-point MissionReport failed");
    assert_eq!(report, decoded);
    assert_eq!(decoded.telemetry.len(), 200);
}

// ---------------------------------------------------------------------------
// Test 21: corruption of MissionReport large payload is detected
// ---------------------------------------------------------------------------

#[test]
fn test_large_mission_report_corruption_detected() {
    let telemetry: Vec<TelemetryPoint> = (0u64..50)
        .map(|i| make_telemetry_point(SubsystemId::Payload, i))
        .collect();
    let report = make_mission_report(telemetry);
    let bytes = encode_to_vec(&report).expect("encode MissionReport for corruption test failed");
    let wrapped = wrap_with_checksum(&bytes);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption of large MissionReport payload must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 22: TelemetryPoint with SubsystemId::ADCS and zero values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_telemetry_adcs_zero_values_roundtrip() {
    let tp = TelemetryPoint {
        subsystem: SubsystemId::ADCS,
        parameter: "attitude_quaternion_w".to_string(),
        value: 0.0,
        unit: "dimensionless".to_string(),
        timestamp_ms: 0,
    };
    let bytes = encode_to_vec(&tp).expect("encode ADCS zero-value TelemetryPoint failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped =
        unwrap_with_checksum(&wrapped).expect("unwrap ADCS zero-value TelemetryPoint failed");
    let (decoded, _): (TelemetryPoint, usize) =
        decode_from_slice(&unwrapped).expect("decode ADCS zero-value TelemetryPoint failed");
    assert_eq!(tp, decoded);
}
