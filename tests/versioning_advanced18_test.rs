//! Smart home / IoT device management versioning tests for OxiCode (set 18).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and smart home domain structs (SmartDeviceV1, SmartDeviceV2,
//! SmartDeviceV3, AutomationRule) with the DeviceType and DeviceStatus enums
//! across all variants, various version tags, field verification, version
//! comparison, consumed bytes accounting, Vec of devices, firmware upgrade
//! scenario, patch/minor version increments, and multiple decode roundtrips.

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
enum DeviceType {
    SmartLight,
    Thermostat,
    DoorLock,
    Camera,
    MotionSensor,
    SmartPlug,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeviceStatus {
    Online,
    Offline,
    Error,
    Updating,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartDeviceV1 {
    device_id: u64,
    device_type: DeviceType,
    status: DeviceStatus,
    battery_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartDeviceV2 {
    device_id: u64,
    device_type: DeviceType,
    status: DeviceStatus,
    battery_pct: u8,
    firmware_version: u32,
    rssi_dbm: i8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartDeviceV3 {
    device_id: u64,
    device_type: DeviceType,
    status: DeviceStatus,
    battery_pct: u8,
    firmware_version: u32,
    rssi_dbm: i8,
    room_id: u32,
    group_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AutomationRule {
    rule_id: u32,
    trigger_device_id: u64,
    action_device_id: u64,
    condition: String,
}

// ── Scenario 1 ────────────────────────────────────────────────────────────────
// SmartDeviceV1 with SmartLight at version 1.0.0 — basic v1 roundtrip
#[test]
fn test_smart_device_v1_smart_light_v1_0_0() {
    let version = Version::new(1, 0, 0);
    let original = SmartDeviceV1 {
        device_id: 1001,
        device_type: DeviceType::SmartLight,
        status: DeviceStatus::Online,
        battery_pct: 95,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.device_type, DeviceType::SmartLight);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

// ── Scenario 2 ────────────────────────────────────────────────────────────────
// SmartDeviceV2 with Thermostat at version 2.0.0 — v2 struct roundtrip
#[test]
fn test_smart_device_v2_thermostat_v2_0_0() {
    let version = Version::new(2, 0, 0);
    let original = SmartDeviceV2 {
        device_id: 2001,
        device_type: DeviceType::Thermostat,
        status: DeviceStatus::Online,
        battery_pct: 80,
        firmware_version: 0x0200_0000,
        rssi_dbm: -55,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (SmartDeviceV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.device_type, DeviceType::Thermostat);
    assert_eq!(decoded.rssi_dbm, -55_i8);
}

// ── Scenario 3 ────────────────────────────────────────────────────────────────
// SmartDeviceV3 with DoorLock and group membership at version 3.0.0
#[test]
fn test_smart_device_v3_door_lock_v3_0_0() {
    let version = Version::new(3, 0, 0);
    let original = SmartDeviceV3 {
        device_id: 3001,
        device_type: DeviceType::DoorLock,
        status: DeviceStatus::Online,
        battery_pct: 72,
        firmware_version: 0x0300_0000,
        rssi_dbm: -62,
        room_id: 10,
        group_ids: vec![1, 2, 5],
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (SmartDeviceV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.device_type, DeviceType::DoorLock);
    assert_eq!(decoded.group_ids, vec![1u32, 2, 5]);
}

// ── Scenario 4 ────────────────────────────────────────────────────────────────
// DeviceType::Camera variant preserved through versioned encode/decode
#[test]
fn test_device_type_camera_variant_versioned() {
    let version = Version::new(1, 0, 0);
    let original = SmartDeviceV1 {
        device_id: 4001,
        device_type: DeviceType::Camera,
        status: DeviceStatus::Online,
        battery_pct: 100,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver, version);
    assert_eq!(decoded.device_type, DeviceType::Camera);
}

// ── Scenario 5 ────────────────────────────────────────────────────────────────
// DeviceType::MotionSensor variant preserved through versioned encode/decode
#[test]
fn test_device_type_motion_sensor_variant_versioned() {
    let version = Version::new(1, 0, 0);
    let original = SmartDeviceV1 {
        device_id: 5001,
        device_type: DeviceType::MotionSensor,
        status: DeviceStatus::Offline,
        battery_pct: 45,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver, version);
    assert_eq!(decoded.device_type, DeviceType::MotionSensor);
    assert_eq!(decoded.status, DeviceStatus::Offline);
}

// ── Scenario 6 ────────────────────────────────────────────────────────────────
// DeviceType::SmartPlug variant preserved through versioned encode/decode
#[test]
fn test_device_type_smart_plug_variant_versioned() {
    let version = Version::new(1, 0, 0);
    let original = SmartDeviceV1 {
        device_id: 6001,
        device_type: DeviceType::SmartPlug,
        status: DeviceStatus::Online,
        battery_pct: 0,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver, version);
    assert_eq!(decoded.device_type, DeviceType::SmartPlug);
}

// ── Scenario 7 ────────────────────────────────────────────────────────────────
// DeviceStatus::Error variant preserved through versioned encode/decode
#[test]
fn test_device_status_error_variant_versioned() {
    let version = Version::new(1, 0, 0);
    let original = SmartDeviceV1 {
        device_id: 7001,
        device_type: DeviceType::Camera,
        status: DeviceStatus::Error,
        battery_pct: 30,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver, version);
    assert_eq!(decoded.status, DeviceStatus::Error);
}

// ── Scenario 8 ────────────────────────────────────────────────────────────────
// DeviceStatus::Updating variant preserved through versioned encode/decode
#[test]
fn test_device_status_updating_variant_versioned() {
    let version = Version::new(2, 0, 0);
    let original = SmartDeviceV2 {
        device_id: 8001,
        device_type: DeviceType::Thermostat,
        status: DeviceStatus::Updating,
        battery_pct: 88,
        firmware_version: 0x0201_0000,
        rssi_dbm: -70,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (SmartDeviceV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver, version);
    assert_eq!(decoded.status, DeviceStatus::Updating);
}

// ── Scenario 9 ────────────────────────────────────────────────────────────────
// AutomationRule versioned encode/decode roundtrip at version 1.0.0
#[test]
fn test_automation_rule_versioned_v1_0_0() {
    let version = Version::new(1, 0, 0);
    let original = AutomationRule {
        rule_id: 9001,
        trigger_device_id: 1001,
        action_device_id: 2001,
        condition: String::from("motion_detected"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (AutomationRule, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.condition, "motion_detected");
}

// ── Scenario 10 ───────────────────────────────────────────────────────────────
// Version triple (major, minor, patch) survives encode/decode intact
#[test]
fn test_version_triple_preserved_after_decode() {
    let version = Version::new(7, 13, 42);
    let original = SmartDeviceV1 {
        device_id: 10001,
        device_type: DeviceType::SmartLight,
        status: DeviceStatus::Online,
        battery_pct: 60,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (_decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver.major, 7);
    assert_eq!(ver.minor, 13);
    assert_eq!(ver.patch, 42);
}

// ── Scenario 11 ───────────────────────────────────────────────────────────────
// Version comparison: v1.0.0 < v2.0.0 < v3.0.0 mirrors device schema generations
#[test]
fn test_version_comparison_v1_lt_v2_lt_v3() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);
    assert!(v1 < v2, "v1.0.0 must be less than v2.0.0");
    assert!(v2 < v3, "v2.0.0 must be less than v3.0.0");
    assert!(v1 < v3, "v1.0.0 must be less than v3.0.0");
    assert_ne!(v1, v2);
    assert_ne!(v2, v3);
    assert_ne!(v1, v3);
}

// ── Scenario 12 ───────────────────────────────────────────────────────────────
// Vec<SmartDeviceV1> — each element independently versioned and decoded correctly
#[test]
fn test_vec_smart_device_v1_versioned_independently() {
    let version = Version::new(1, 0, 0);
    let devices = vec![
        SmartDeviceV1 {
            device_id: 11001,
            device_type: DeviceType::SmartLight,
            status: DeviceStatus::Online,
            battery_pct: 90,
        },
        SmartDeviceV1 {
            device_id: 11002,
            device_type: DeviceType::Thermostat,
            status: DeviceStatus::Online,
            battery_pct: 75,
        },
        SmartDeviceV1 {
            device_id: 11003,
            device_type: DeviceType::DoorLock,
            status: DeviceStatus::Offline,
            battery_pct: 20,
        },
        SmartDeviceV1 {
            device_id: 11004,
            device_type: DeviceType::Camera,
            status: DeviceStatus::Online,
            battery_pct: 55,
        },
        SmartDeviceV1 {
            device_id: 11005,
            device_type: DeviceType::MotionSensor,
            status: DeviceStatus::Error,
            battery_pct: 10,
        },
        SmartDeviceV1 {
            device_id: 11006,
            device_type: DeviceType::SmartPlug,
            status: DeviceStatus::Online,
            battery_pct: 0,
        },
    ];
    for original in &devices {
        let encoded =
            encode_versioned_value(original, version).expect("encode_versioned_value failed");
        let (decoded, ver, consumed): (SmartDeviceV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(&decoded, original);
        assert_eq!(ver, version);
        assert!(consumed > 0);
    }
}

// ── Scenario 13 ───────────────────────────────────────────────────────────────
// Multiple decode roundtrips: encoding twice from the decoded value yields
// identical bytes and identical version metadata
#[test]
fn test_multiple_decode_roundtrips_preserving_version() {
    let version = Version::new(2, 0, 0);
    let original = SmartDeviceV2 {
        device_id: 13001,
        device_type: DeviceType::SmartLight,
        status: DeviceStatus::Online,
        battery_pct: 85,
        firmware_version: 0x0200_0001,
        rssi_dbm: -48,
    };
    let enc1 = encode_versioned_value(&original, version).expect("first encode failed");
    let (decoded1, ver1, _): (SmartDeviceV2, Version, usize) =
        decode_versioned_value(&enc1).expect("first decode failed");

    let enc2 = encode_versioned_value(&decoded1, ver1).expect("second encode failed");
    let (decoded2, ver2, consumed2): (SmartDeviceV2, Version, usize) =
        decode_versioned_value(&enc2).expect("second decode failed");

    assert_eq!(decoded2, original);
    assert_eq!(ver2, version);
    assert_eq!(enc1, enc2);
    assert!(consumed2 > 0);
}

// ── Scenario 14 ───────────────────────────────────────────────────────────────
// Device vec covering all DeviceType variants verifies enum exhaustiveness
#[test]
fn test_all_device_type_variants_in_vec_versioned() {
    let version = Version::new(1, 0, 0);
    let all_types = vec![
        DeviceType::SmartLight,
        DeviceType::Thermostat,
        DeviceType::DoorLock,
        DeviceType::Camera,
        DeviceType::MotionSensor,
        DeviceType::SmartPlug,
    ];
    for (idx, dtype) in all_types.into_iter().enumerate() {
        let original = SmartDeviceV1 {
            device_id: 14000 + idx as u64,
            device_type: dtype.clone(),
            status: DeviceStatus::Online,
            battery_pct: 50,
        };
        let encoded =
            encode_versioned_value(&original, version).expect("encode_versioned_value failed");
        let (decoded, ver, consumed): (SmartDeviceV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(decoded.device_type, dtype);
        assert_eq!(ver, version);
        assert!(consumed > 0);
    }
}

// ── Scenario 15 ───────────────────────────────────────────────────────────────
// Patch version increments: v1.0.0 → v1.0.1 → v1.0.5 ordering and preservation
#[test]
fn test_patch_version_increments_preserved() {
    let v1_0_0 = Version::new(1, 0, 0);
    let v1_0_1 = Version::new(1, 0, 1);
    let v1_0_5 = Version::new(1, 0, 5);
    assert!(v1_0_0 < v1_0_1, "patch 0 must be less than patch 1");
    assert!(v1_0_1 < v1_0_5, "patch 1 must be less than patch 5");

    let original = SmartDeviceV1 {
        device_id: 15001,
        device_type: DeviceType::MotionSensor,
        status: DeviceStatus::Online,
        battery_pct: 77,
    };
    for &ver in &[v1_0_0, v1_0_1, v1_0_5] {
        let encoded =
            encode_versioned_value(&original, ver).expect("encode_versioned_value failed");
        let (_decoded, decoded_ver, _): (SmartDeviceV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(decoded_ver, ver);
    }
}

// ── Scenario 16 ───────────────────────────────────────────────────────────────
// Minor version increments: v2.0.0 → v2.1.0 → v2.3.0 ordering and preservation
#[test]
fn test_minor_version_increments_preserved() {
    let v2_0_0 = Version::new(2, 0, 0);
    let v2_1_0 = Version::new(2, 1, 0);
    let v2_3_0 = Version::new(2, 3, 0);
    assert!(v2_0_0 < v2_1_0, "minor 0 must be less than minor 1");
    assert!(v2_1_0 < v2_3_0, "minor 1 must be less than minor 3");

    let original = SmartDeviceV2 {
        device_id: 16001,
        device_type: DeviceType::SmartPlug,
        status: DeviceStatus::Online,
        battery_pct: 0,
        firmware_version: 0x0201_0000,
        rssi_dbm: -60,
    };
    for &ver in &[v2_0_0, v2_1_0, v2_3_0] {
        let encoded =
            encode_versioned_value(&original, ver).expect("encode_versioned_value failed");
        let (_decoded, decoded_ver, _): (SmartDeviceV2, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(decoded_ver, ver);
        assert_eq!(decoded_ver.major, 2);
    }
}

// ── Scenario 17 ───────────────────────────────────────────────────────────────
// Firmware upgrade scenario: same device data tagged at v1.0.0 → v1.1.0 → v2.0.0
// — payload identical, only version metadata changes
#[test]
fn test_firmware_upgrade_scenario_v1_0_0_to_v1_1_0_to_v2_0_0() {
    let v1_0_0 = Version::new(1, 0, 0);
    let v1_1_0 = Version::new(1, 1, 0);
    let v2_0_0 = Version::new(2, 0, 0);

    let device = SmartDeviceV1 {
        device_id: 17001,
        device_type: DeviceType::Thermostat,
        status: DeviceStatus::Updating,
        battery_pct: 65,
    };

    let enc_v1 = encode_versioned_value(&device, v1_0_0).expect("encode v1.0.0 failed");
    let enc_v1_1 = encode_versioned_value(&device, v1_1_0).expect("encode v1.1.0 failed");
    let enc_v2 = encode_versioned_value(&device, v2_0_0).expect("encode v2.0.0 failed");

    let (dec_v1, ver_v1, _): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&enc_v1).expect("decode v1.0.0 failed");
    let (dec_v1_1, ver_v1_1, _): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&enc_v1_1).expect("decode v1.1.0 failed");
    let (dec_v2, ver_v2, _): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&enc_v2).expect("decode v2.0.0 failed");

    assert_eq!(dec_v1, device);
    assert_eq!(dec_v1_1, device);
    assert_eq!(dec_v2, device);
    assert_eq!(ver_v1, v1_0_0);
    assert_eq!(ver_v1_1, v1_1_0);
    assert_eq!(ver_v2, v2_0_0);
    assert!(ver_v1 < ver_v1_1);
    assert!(ver_v1_1 < ver_v2);
}

// ── Scenario 18 ───────────────────────────────────────────────────────────────
// Consumed bytes check: consumed <= total encoded buffer length and > 0
#[test]
fn test_consumed_bytes_within_encoded_buffer_bounds() {
    let version = Version::new(3, 0, 0);
    let original = SmartDeviceV3 {
        device_id: 18001,
        device_type: DeviceType::Camera,
        status: DeviceStatus::Online,
        battery_pct: 91,
        firmware_version: 0x0300_0005,
        rssi_dbm: -40,
        room_id: 7,
        group_ids: vec![10, 20, 30, 40],
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let total_len = encoded.len();
    let (_decoded, _ver, consumed): (SmartDeviceV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert!(consumed > 0, "consumed bytes must be positive");
    assert!(
        consumed <= total_len,
        "consumed ({consumed}) must not exceed total encoded length ({total_len})"
    );
}

// ── Scenario 19 ───────────────────────────────────────────────────────────────
// AutomationRule with complex condition string at version 2.0.0
#[test]
fn test_automation_rule_complex_condition_v2_0_0() {
    let version = Version::new(2, 0, 0);
    let original = AutomationRule {
        rule_id: 19001,
        trigger_device_id: 3001,
        action_device_id: 6001,
        condition: String::from("door_opened AND time_between(22:00,06:00) AND NOT alarm_silenced"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (AutomationRule, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert!(decoded.condition.contains("door_opened"));
    assert_eq!(ver.major, 2);
}

// ── Scenario 20 ───────────────────────────────────────────────────────────────
// SmartDeviceV3 with empty group_ids at version 3.0.0 — edge case for empty vec
#[test]
fn test_smart_device_v3_empty_group_ids_v3_0_0() {
    let version = Version::new(3, 0, 0);
    let original = SmartDeviceV3 {
        device_id: 20001,
        device_type: DeviceType::SmartLight,
        status: DeviceStatus::Online,
        battery_pct: 100,
        firmware_version: 0x0300_0000,
        rssi_dbm: -35,
        room_id: 1,
        group_ids: vec![],
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (SmartDeviceV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert!(decoded.group_ids.is_empty());
}

// ── Scenario 21 ───────────────────────────────────────────────────────────────
// Plain encode/decode baseline for SmartDeviceV1 (no versioning wrapper)
#[test]
fn test_smart_device_v1_plain_encode_decode_baseline() {
    let original = SmartDeviceV1 {
        device_id: 21001,
        device_type: DeviceType::DoorLock,
        status: DeviceStatus::Online,
        battery_pct: 43,
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (SmartDeviceV1, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.device_type, DeviceType::DoorLock);
    assert_eq!(decoded.battery_pct, 43);
}

// ── Scenario 22 ───────────────────────────────────────────────────────────────
// Plain encode/decode baseline for AutomationRule (no versioning wrapper)
#[test]
fn test_automation_rule_plain_encode_decode_baseline() {
    let original = AutomationRule {
        rule_id: 22001,
        trigger_device_id: 5001,
        action_device_id: 6001,
        condition: String::from("battery_low"),
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (AutomationRule, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.rule_id, 22001);
    assert_eq!(decoded.condition, "battery_low");
}
