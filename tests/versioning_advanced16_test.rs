//! Drone / UAV fleet management versioning tests for OxiCode (set 16).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and three drone domain structs (DroneV1, DroneV2, DroneV3) with the
//! DroneStatus enum across all its variants, various version tags, field verification,
//! version comparison, consumed bytes accounting, Vec of drones, version field
//! preservation, and plain encode/decode baseline.

#![cfg(feature = "versioning")]
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
use oxicode::versioning::Version;
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DroneStatus {
    Idle,
    Flying,
    Charging,
    Maintenance,
    Offline,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DroneV1 {
    drone_id: u64,
    status: DroneStatus,
    battery_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DroneV2 {
    drone_id: u64,
    status: DroneStatus,
    battery_pct: u8,
    altitude_m: f32,
    speed_ms: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DroneV3 {
    drone_id: u64,
    status: DroneStatus,
    battery_pct: u8,
    altitude_m: f32,
    speed_ms: f32,
    payload_kg: f32,
    last_mission: Option<String>,
}

// ── Scenario 1 ────────────────────────────────────────────────────────────────
// DroneV1 with DroneStatus::Idle at version 1.0.0
#[test]
fn test_drone_v1_status_idle_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = DroneV1 {
        drone_id: 1001,
        status: DroneStatus::Idle,
        battery_pct: 100,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.status, DroneStatus::Idle);
}

// ── Scenario 2 ────────────────────────────────────────────────────────────────
// DroneV1 with DroneStatus::Flying at version 1.0.0
#[test]
fn test_drone_v1_status_flying_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = DroneV1 {
        drone_id: 1002,
        status: DroneStatus::Flying,
        battery_pct: 73,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.status, DroneStatus::Flying);
}

// ── Scenario 3 ────────────────────────────────────────────────────────────────
// DroneV1 with DroneStatus::Charging at version 1.0.0
#[test]
fn test_drone_v1_status_charging_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = DroneV1 {
        drone_id: 1003,
        status: DroneStatus::Charging,
        battery_pct: 32,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.status, DroneStatus::Charging);
}

// ── Scenario 4 ────────────────────────────────────────────────────────────────
// DroneV1 with DroneStatus::Maintenance at version 1.0.0
#[test]
fn test_drone_v1_status_maintenance_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = DroneV1 {
        drone_id: 1004,
        status: DroneStatus::Maintenance,
        battery_pct: 0,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.status, DroneStatus::Maintenance);
}

// ── Scenario 5 ────────────────────────────────────────────────────────────────
// DroneV1 with DroneStatus::Offline at version 1.0.0
#[test]
fn test_drone_v1_status_offline_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = DroneV1 {
        drone_id: 1005,
        status: DroneStatus::Offline,
        battery_pct: 55,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.status, DroneStatus::Offline);
}

// ── Scenario 6 ────────────────────────────────────────────────────────────────
// DroneV2 roundtrip at version 2.0.0 with flying drone at altitude
#[test]
fn test_drone_v2_flying_at_altitude_v2_0_0() {
    let version = Version::new(2, 0, 0);
    let original = DroneV2 {
        drone_id: 2001,
        status: DroneStatus::Flying,
        battery_pct: 88,
        altitude_m: 150.5,
        speed_ms: 12.3,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(ver.major, 2);
    assert!((decoded.altitude_m - 150.5_f32).abs() < 1e-4);
}

// ── Scenario 7 ────────────────────────────────────────────────────────────────
// DroneV2 roundtrip at version 2.1.0 with idle drone at zero altitude
#[test]
fn test_drone_v2_idle_zero_altitude_v2_1_0() {
    let version = Version::new(2, 1, 0);
    let original = DroneV2 {
        drone_id: 2002,
        status: DroneStatus::Idle,
        battery_pct: 100,
        altitude_m: 0.0,
        speed_ms: 0.0,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver.minor, 1);
    assert!(consumed > 0);
    assert!((decoded.speed_ms - 0.0_f32).abs() < 1e-7);
}

// ── Scenario 8 ────────────────────────────────────────────────────────────────
// DroneV2 roundtrip at version 3.0.0 with high-speed drone
#[test]
fn test_drone_v2_high_speed_v3_0_0() {
    let version = Version::new(3, 0, 0);
    let original = DroneV2 {
        drone_id: 2003,
        status: DroneStatus::Flying,
        battery_pct: 45,
        altitude_m: 500.0,
        speed_ms: 28.7,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert!((decoded.speed_ms - 28.7_f32).abs() < 1e-4);
}

// ── Scenario 9 ────────────────────────────────────────────────────────────────
// DroneV3 with Option<String> Some(mission) at version 3.0.0
#[test]
fn test_drone_v3_with_optional_mission_some() {
    let version = Version::new(3, 0, 0);
    let original = DroneV3 {
        drone_id: 3001,
        status: DroneStatus::Flying,
        battery_pct: 62,
        altitude_m: 220.0,
        speed_ms: 15.5,
        payload_kg: 2.5,
        last_mission: Some(String::from("package_delivery_alpha")),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(
        decoded.last_mission,
        Some(String::from("package_delivery_alpha"))
    );
}

// ── Scenario 10 ───────────────────────────────────────────────────────────────
// DroneV3 with Option<String> None (no mission recorded) at version 3.0.0
#[test]
fn test_drone_v3_with_optional_mission_none() {
    let version = Version::new(3, 0, 0);
    let original = DroneV3 {
        drone_id: 3002,
        status: DroneStatus::Idle,
        battery_pct: 100,
        altitude_m: 0.0,
        speed_ms: 0.0,
        payload_kg: 0.0,
        last_mission: None,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.last_mission, None);
}

// ── Scenario 11 ───────────────────────────────────────────────────────────────
// Version ordering: v1 < v2 < v3 representing drone schema generations
#[test]
fn test_version_ordering_drone_generations_v1_lt_v2_lt_v3() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);
    assert!(v1 < v2, "DroneV1 schema version must be less than DroneV2");
    assert!(v2 < v3, "DroneV2 schema version must be less than DroneV3");
    assert!(v1 < v3, "DroneV1 schema version must be less than DroneV3");
    assert_ne!(v1, v2);
    assert_ne!(v2, v3);
    assert_ne!(v1, v3);
}

// ── Scenario 12 ───────────────────────────────────────────────────────────────
// Version comparison: minor ordering within the same major version
#[test]
fn test_version_minor_ordering_within_major() {
    let v2_0 = Version::new(2, 0, 0);
    let v2_1 = Version::new(2, 1, 0);
    let v2_5 = Version::new(2, 5, 0);
    assert!(v2_0 < v2_1);
    assert!(v2_1 < v2_5);
    assert!(v2_0 < v2_5);
}

// ── Scenario 13 ───────────────────────────────────────────────────────────────
// Version comparison: patch ordering within the same major.minor
#[test]
fn test_version_patch_ordering_within_minor() {
    let v3_2_0 = Version::new(3, 2, 0);
    let v3_2_1 = Version::new(3, 2, 1);
    let v3_2_7 = Version::new(3, 2, 7);
    assert!(v3_2_0 < v3_2_1);
    assert!(v3_2_1 < v3_2_7);
    assert_ne!(v3_2_0, v3_2_7);
}

// ── Scenario 14 ───────────────────────────────────────────────────────────────
// Version field preservation: major, minor, patch survive encode/decode roundtrip
#[test]
fn test_version_field_preservation_after_decode() {
    let version = Version::new(5, 12, 99);
    let original = DroneV1 {
        drone_id: 9999,
        status: DroneStatus::Charging,
        battery_pct: 42,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (_decoded, ver, _consumed): (DroneV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver.major, 5);
    assert_eq!(ver.minor, 12);
    assert_eq!(ver.patch, 99);
}

// ── Scenario 15 ───────────────────────────────────────────────────────────────
// Consumed bytes: positive and within total encoded buffer length
#[test]
fn test_consumed_bytes_within_encoded_buffer_bounds() {
    let version = Version::new(2, 0, 0);
    let original = DroneV2 {
        drone_id: 4001,
        status: DroneStatus::Maintenance,
        battery_pct: 5,
        altitude_m: 0.0,
        speed_ms: 0.0,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let total_len = encoded.len();
    let (_decoded, _ver, consumed): (DroneV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert!(consumed > 0, "consumed bytes must be positive");
    assert!(
        consumed <= total_len,
        "consumed ({consumed}) must not exceed total encoded length ({total_len})"
    );
}

// ── Scenario 16 ───────────────────────────────────────────────────────────────
// Vec of DroneV1 drones, each versioned independently and decoded correctly
#[test]
fn test_vec_of_drones_versioned_independently() {
    let version = Version::new(1, 0, 0);
    let fleet = vec![
        DroneV1 {
            drone_id: 5001,
            status: DroneStatus::Idle,
            battery_pct: 100,
        },
        DroneV1 {
            drone_id: 5002,
            status: DroneStatus::Flying,
            battery_pct: 78,
        },
        DroneV1 {
            drone_id: 5003,
            status: DroneStatus::Charging,
            battery_pct: 15,
        },
        DroneV1 {
            drone_id: 5004,
            status: DroneStatus::Offline,
            battery_pct: 0,
        },
    ];
    for original in &fleet {
        let encoded =
            encode_versioned_value(original, version).expect("encode_versioned_value failed");
        let (decoded, ver, consumed): (DroneV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(&decoded, original);
        assert_eq!(ver, version);
        assert!(consumed > 0);
    }
}

// ── Scenario 17 ───────────────────────────────────────────────────────────────
// Same DroneV2 data tagged at two different version tags: decoded data is identical
// but version fields differ
#[test]
fn test_drone_v2_same_data_different_version_tags() {
    let v_old = Version::new(2, 0, 0);
    let v_new = Version::new(2, 3, 1);
    let drone = DroneV2 {
        drone_id: 6001,
        status: DroneStatus::Flying,
        battery_pct: 90,
        altitude_m: 80.0,
        speed_ms: 10.0,
    };

    let enc_old = encode_versioned_value(&drone, v_old).expect("encode v_old failed");
    let enc_new = encode_versioned_value(&drone, v_new).expect("encode v_new failed");

    let (decoded_old, ver_old, _): (DroneV2, Version, usize) =
        decode_versioned_value(&enc_old).expect("decode v_old failed");
    let (decoded_new, ver_new, _): (DroneV2, Version, usize) =
        decode_versioned_value(&enc_new).expect("decode v_new failed");

    assert_eq!(decoded_old, drone);
    assert_eq!(decoded_new, drone);
    assert_eq!(ver_old, v_old);
    assert_eq!(ver_new, v_new);
    assert_ne!(ver_old, ver_new);
}

// ── Scenario 18 ───────────────────────────────────────────────────────────────
// DroneV3 with maximum payload and long mission string at version 3.1.0
#[test]
fn test_drone_v3_max_payload_long_mission_string() {
    let version = Version::new(3, 1, 0);
    let original = DroneV3 {
        drone_id: 7001,
        status: DroneStatus::Flying,
        battery_pct: 50,
        altitude_m: 300.0,
        speed_ms: 20.0,
        payload_kg: 25.0,
        last_mission: Some(String::from(
            "long_range_inspection_sector_bravo_gamma_northwest_corridor_pass_alpha",
        )),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert!((decoded.payload_kg - 25.0_f32).abs() < 1e-4);
    assert!(decoded.last_mission.is_some());
}

// ── Scenario 19 ───────────────────────────────────────────────────────────────
// DroneV3 with zero payload and Offline status at version 3.0.0
#[test]
fn test_drone_v3_offline_zero_payload() {
    let version = Version::new(3, 0, 0);
    let original = DroneV3 {
        drone_id: 7002,
        status: DroneStatus::Offline,
        battery_pct: 0,
        altitude_m: 0.0,
        speed_ms: 0.0,
        payload_kg: 0.0,
        last_mission: None,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (DroneV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.battery_pct, 0);
    assert_eq!(decoded.status, DroneStatus::Offline);
}

// ── Scenario 20 ───────────────────────────────────────────────────────────────
// Version equality: two identical Version values compare equal
#[test]
fn test_version_equality_identical_values() {
    let va = Version::new(2, 7, 33);
    let vb = Version::new(2, 7, 33);
    assert_eq!(va, vb);
    assert!(!(va < vb));
    assert!(!(va > vb));
    assert!(va <= vb);
    assert!(va >= vb);
}

// ── Scenario 21 ───────────────────────────────────────────────────────────────
// Plain encode/decode baseline for DroneV1 (no versioning wrapper)
#[test]
fn test_drone_v1_plain_encode_decode_baseline() {
    let original = DroneV1 {
        drone_id: 8001,
        status: DroneStatus::Idle,
        battery_pct: 99,
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (DroneV1, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.battery_pct, 99);
    assert_eq!(decoded.status, DroneStatus::Idle);
}

// ── Scenario 22 ───────────────────────────────────────────────────────────────
// Plain encode/decode baseline for DroneV3 (no versioning wrapper) confirming
// the encoding is independent of version metadata
#[test]
fn test_drone_v3_plain_encode_decode_baseline() {
    let original = DroneV3 {
        drone_id: 8002,
        status: DroneStatus::Maintenance,
        battery_pct: 12,
        altitude_m: 0.0,
        speed_ms: 0.0,
        payload_kg: 3.75,
        last_mission: Some(String::from("survey_zone_delta")),
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (DroneV3, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.status, DroneStatus::Maintenance);
    assert_eq!(
        decoded.last_mission,
        Some(String::from("survey_zone_delta"))
    );
    assert!((decoded.payload_kg - 3.75_f32).abs() < 1e-4);
}
