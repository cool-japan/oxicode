//! Smart home / IoT configuration versioning tests for OxiCode (set 12).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and three generations of SmartDevice structs (V1/V2/V3) with all
//! DeviceCategory variants, HomeConfig, Vec of versioned items, version
//! comparison, consumed bytes, and plain encode/decode baselines.

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
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value,
    versioning::Version, Decode, Encode,
};

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeviceCategory {
    Light,
    Thermostat,
    Lock,
    Camera,
    Speaker,
    Sensor,
    Hub,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartDeviceV1 {
    id: u32,
    category: DeviceCategory,
    name: String,
    enabled: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartDeviceV2 {
    id: u32,
    category: DeviceCategory,
    name: String,
    enabled: bool,
    firmware: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartDeviceV3 {
    id: u32,
    category: DeviceCategory,
    name: String,
    enabled: bool,
    firmware: String,
    room: String,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HomeConfig {
    house_id: u64,
    devices: Vec<SmartDeviceV1>,
    timezone: String,
}

// ── Scenario 1 ────────────────────────────────────────────────────────────────
// SmartDeviceV1 basic roundtrip — Light category
#[test]
fn test_smart_device_v1_light_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = SmartDeviceV1 {
        id: 1001,
        category: DeviceCategory::Light,
        name: String::from("Living Room Light"),
        enabled: true,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 2 ────────────────────────────────────────────────────────────────
// SmartDeviceV2 roundtrip — Thermostat with firmware
#[test]
fn test_smart_device_v2_thermostat_versioned_roundtrip() {
    let version = Version::new(2, 0, 0);
    let original = SmartDeviceV2 {
        id: 2002,
        category: DeviceCategory::Thermostat,
        name: String::from("Hall Thermostat"),
        enabled: true,
        firmware: String::from("v2.4.1"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (SmartDeviceV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 3 ────────────────────────────────────────────────────────────────
// SmartDeviceV3 roundtrip — Lock with room and tags
#[test]
fn test_smart_device_v3_lock_versioned_roundtrip() {
    let version = Version::new(3, 0, 0);
    let original = SmartDeviceV3 {
        id: 3003,
        category: DeviceCategory::Lock,
        name: String::from("Front Door Lock"),
        enabled: true,
        firmware: String::from("v3.0.0"),
        room: String::from("Entrance"),
        tags: vec![String::from("security"), String::from("outdoor")],
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (SmartDeviceV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 4 ────────────────────────────────────────────────────────────────
// Version is exactly preserved — Version::new(1, 3, 7)
#[test]
fn test_smart_device_v1_version_preserved_exactly() {
    let version = Version::new(1, 3, 7);
    let original = SmartDeviceV1 {
        id: 42,
        category: DeviceCategory::Sensor,
        name: String::from("Motion Sensor"),
        enabled: false,
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (_decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 7);
    assert_eq!(ver, version);
}

// ── Scenario 5 ────────────────────────────────────────────────────────────────
// Camera category roundtrip via SmartDeviceV2 — disabled device
#[test]
fn test_smart_device_v2_camera_disabled() {
    let version = Version::new(2, 1, 0);
    let original = SmartDeviceV2 {
        id: 5005,
        category: DeviceCategory::Camera,
        name: String::from("Backyard Camera"),
        enabled: false,
        firmware: String::from("cam-fw-1.0"),
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _consumed): (SmartDeviceV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded.category, DeviceCategory::Camera);
    assert!(!decoded.enabled);
    assert_eq!(decoded.firmware, "cam-fw-1.0");
    assert_eq!(ver, version);
}

// ── Scenario 6 ────────────────────────────────────────────────────────────────
// Speaker category roundtrip via SmartDeviceV3 — multiple tags
#[test]
fn test_smart_device_v3_speaker_multiple_tags() {
    let version = Version::new(3, 0, 1);
    let original = SmartDeviceV3 {
        id: 6006,
        category: DeviceCategory::Speaker,
        name: String::from("Kitchen Speaker"),
        enabled: true,
        firmware: String::from("spk-fw-2.1"),
        room: String::from("Kitchen"),
        tags: vec![
            String::from("audio"),
            String::from("voice-assistant"),
            String::from("wifi"),
        ],
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _consumed): (SmartDeviceV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded.category, DeviceCategory::Speaker);
    assert_eq!(decoded.tags.len(), 3);
    assert_eq!(decoded.room, "Kitchen");
    assert_eq!(ver, version);
}

// ── Scenario 7 ────────────────────────────────────────────────────────────────
// All seven DeviceCategory variants roundtrip via SmartDeviceV1
#[test]
fn test_all_device_categories_roundtrip_v1() {
    let version = Version::new(1, 0, 0);
    let categories = [
        DeviceCategory::Light,
        DeviceCategory::Thermostat,
        DeviceCategory::Lock,
        DeviceCategory::Camera,
        DeviceCategory::Speaker,
        DeviceCategory::Sensor,
        DeviceCategory::Hub,
    ];
    let names = ["light", "thermo", "lock", "cam", "speaker", "sensor", "hub"];
    for (i, (cat, name)) in categories.into_iter().zip(names.iter()).enumerate() {
        let device = SmartDeviceV1 {
            id: i as u32,
            category: cat,
            name: String::from(*name),
            enabled: i % 2 == 0,
        };
        let encoded = encode_versioned_value(&device, version).expect("encode failed");
        let (decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, device);
        assert_eq!(ver, version);
    }
}

// ── Scenario 8 ────────────────────────────────────────────────────────────────
// consumed bytes is positive and does not exceed total encoded length
#[test]
fn test_smart_device_v1_consumed_bytes_within_bounds() {
    let version = Version::new(1, 0, 0);
    let original = SmartDeviceV1 {
        id: 8008,
        category: DeviceCategory::Hub,
        name: String::from("Central Hub"),
        enabled: true,
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (_decoded, _ver, consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert!(consumed > 0, "consumed must be positive");
    assert!(
        consumed <= encoded.len(),
        "consumed ({}) must not exceed encoded length ({})",
        consumed,
        encoded.len()
    );
}

// ── Scenario 9 ────────────────────────────────────────────────────────────────
// V1 encoded size < V2 encoded size < V3 encoded size (for equivalent fields)
#[test]
fn test_v1_smaller_than_v2_smaller_than_v3_encoded_size() {
    let v1 = SmartDeviceV1 {
        id: 1,
        category: DeviceCategory::Light,
        name: String::from("lamp"),
        enabled: true,
    };
    let v2 = SmartDeviceV2 {
        id: 1,
        category: DeviceCategory::Light,
        name: String::from("lamp"),
        enabled: true,
        firmware: String::from("fw"),
    };
    let v3 = SmartDeviceV3 {
        id: 1,
        category: DeviceCategory::Light,
        name: String::from("lamp"),
        enabled: true,
        firmware: String::from("fw"),
        room: String::from("rm"),
        tags: vec![String::from("t")],
    };
    let bytes_v1 = encode_to_vec(&v1).expect("encode v1 failed");
    let bytes_v2 = encode_to_vec(&v2).expect("encode v2 failed");
    let bytes_v3 = encode_to_vec(&v3).expect("encode v3 failed");
    assert!(
        bytes_v1.len() < bytes_v2.len(),
        "V1 ({}) should be smaller than V2 ({})",
        bytes_v1.len(),
        bytes_v2.len()
    );
    assert!(
        bytes_v2.len() < bytes_v3.len(),
        "V2 ({}) should be smaller than V3 ({})",
        bytes_v2.len(),
        bytes_v3.len()
    );
}

// ── Scenario 10 ───────────────────────────────────────────────────────────────
// Version tuple accessor returns correct (major, minor, patch)
#[test]
fn test_version_tuple_accessor_iot() {
    let version = Version::new(4, 8, 12);
    let original = SmartDeviceV1 {
        id: 10010,
        category: DeviceCategory::Sensor,
        name: String::from("Humidity Sensor"),
        enabled: true,
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (_decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    let (maj, min, pat) = ver.tuple();
    assert_eq!(maj, 4);
    assert_eq!(min, 8);
    assert_eq!(pat, 12);
}

// ── Scenario 11 ───────────────────────────────────────────────────────────────
// HomeConfig roundtrip — plain encode/decode with multiple devices
#[test]
fn test_home_config_plain_encode_decode_roundtrip() {
    let config = HomeConfig {
        house_id: 9_876_543_210,
        devices: vec![
            SmartDeviceV1 {
                id: 101,
                category: DeviceCategory::Light,
                name: String::from("Bedroom Light"),
                enabled: true,
            },
            SmartDeviceV1 {
                id: 102,
                category: DeviceCategory::Thermostat,
                name: String::from("HVAC Controller"),
                enabled: true,
            },
        ],
        timezone: String::from("America/New_York"),
    };
    let bytes = encode_to_vec(&config).expect("encode_to_vec failed");
    let (decoded, consumed): (HomeConfig, usize) =
        decode_from_slice(&bytes).expect("decode_from_slice failed");
    assert_eq!(decoded, config);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.devices.len(), 2);
    assert_eq!(decoded.timezone, "America/New_York");
}

// ── Scenario 12 ───────────────────────────────────────────────────────────────
// HomeConfig versioned roundtrip — Version::new(1, 5, 0)
#[test]
fn test_home_config_versioned_roundtrip() {
    let version = Version::new(1, 5, 0);
    let original = HomeConfig {
        house_id: 1,
        devices: vec![SmartDeviceV1 {
            id: 1,
            category: DeviceCategory::Hub,
            name: String::from("Smart Hub"),
            enabled: true,
        }],
        timezone: String::from("Europe/London"),
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _consumed): (HomeConfig, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert_eq!(decoded.timezone, "Europe/London");
}

// ── Scenario 13 ───────────────────────────────────────────────────────────────
// Vec of SmartDeviceV1 versioned roundtrip
#[test]
fn test_vec_of_smart_device_v1_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let devices = vec![
        SmartDeviceV1 {
            id: 1,
            category: DeviceCategory::Light,
            name: String::from("light-1"),
            enabled: true,
        },
        SmartDeviceV1 {
            id: 2,
            category: DeviceCategory::Sensor,
            name: String::from("sensor-1"),
            enabled: false,
        },
        SmartDeviceV1 {
            id: 3,
            category: DeviceCategory::Lock,
            name: String::from("lock-1"),
            enabled: true,
        },
    ];
    let encoded = encode_versioned_value(&devices, version).expect("encode failed");
    let (decoded, ver, _consumed): (Vec<SmartDeviceV1>, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, devices);
    assert_eq!(ver, version);
    assert_eq!(decoded.len(), 3);
}

// ── Scenario 14 ───────────────────────────────────────────────────────────────
// Vec of SmartDeviceV3 versioned roundtrip — mixed tags
#[test]
fn test_vec_of_smart_device_v3_versioned_roundtrip_mixed_tags() {
    let version = Version::new(3, 2, 0);
    let devices = vec![
        SmartDeviceV3 {
            id: 10,
            category: DeviceCategory::Camera,
            name: String::from("Front Cam"),
            enabled: true,
            firmware: String::from("cam-3.0"),
            room: String::from("Porch"),
            tags: vec![String::from("outdoor"), String::from("4k")],
        },
        SmartDeviceV3 {
            id: 11,
            category: DeviceCategory::Sensor,
            name: String::from("CO2 Sensor"),
            enabled: true,
            firmware: String::from("sens-1.2"),
            room: String::from("Living Room"),
            tags: vec![],
        },
    ];
    let encoded = encode_versioned_value(&devices, version).expect("encode failed");
    let (decoded, ver, _consumed): (Vec<SmartDeviceV3>, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, devices);
    assert_eq!(ver, version);
    assert_eq!(decoded[0].tags.len(), 2);
    assert_eq!(decoded[1].tags.len(), 0);
}

// ── Scenario 15 ───────────────────────────────────────────────────────────────
// SmartDeviceV3 with empty tags — roundtrip preserves empty vec
#[test]
fn test_smart_device_v3_empty_tags_preserved() {
    let version = Version::new(3, 0, 0);
    let original = SmartDeviceV3 {
        id: 15015,
        category: DeviceCategory::Thermostat,
        name: String::from("Smart Thermostat"),
        enabled: true,
        firmware: String::from("therm-fw-5.0"),
        room: String::from("Bedroom"),
        tags: vec![],
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _consumed): (SmartDeviceV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert!(decoded.tags.is_empty(), "tags should be empty");
    assert_eq!(ver, version);
}

// ── Scenario 16 ───────────────────────────────────────────────────────────────
// SmartDeviceV1 with id boundary values: 0 and u32::MAX
#[test]
fn test_smart_device_v1_id_boundary_values() {
    let version = Version::new(1, 0, 0);
    for id in [0u32, u32::MAX] {
        let original = SmartDeviceV1 {
            id,
            category: DeviceCategory::Light,
            name: String::from("boundary device"),
            enabled: true,
        };
        let encoded = encode_versioned_value(&original, version).expect("encode failed");
        let (decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded.id, id);
        assert_eq!(ver, version);
    }
}

// ── Scenario 17 ───────────────────────────────────────────────────────────────
// Version ordering: V1 version < V2 version < V3 version
#[test]
fn test_version_ordering_v1_v2_v3_iot() {
    let v1_ver = Version::new(1, 0, 0);
    let v2_ver = Version::new(2, 0, 0);
    let v3_ver = Version::new(3, 0, 0);
    assert!(v1_ver < v2_ver, "V1 version should be less than V2 version");
    assert!(v2_ver < v3_ver, "V2 version should be less than V3 version");
    assert!(v1_ver < v3_ver, "V1 version should be less than V3 version");
    assert_ne!(v1_ver, v2_ver);
    assert_ne!(v2_ver, v3_ver);
}

// ── Scenario 18 ───────────────────────────────────────────────────────────────
// Plain encode/decode baseline for SmartDeviceV2
#[test]
fn test_smart_device_v2_plain_encode_decode_baseline() {
    let original = SmartDeviceV2 {
        id: 18018,
        category: DeviceCategory::Speaker,
        name: String::from("Patio Speaker"),
        enabled: false,
        firmware: String::from("spk-fw-3.3"),
    };
    let bytes = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, consumed): (SmartDeviceV2, usize) =
        decode_from_slice(&bytes).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ── Scenario 19 ───────────────────────────────────────────────────────────────
// Multiple versions for same SmartDeviceV1 type — version extracted correctly
#[test]
fn test_multiple_versions_for_same_struct_type() {
    let versions = [
        Version::new(1, 0, 0),
        Version::new(1, 1, 0),
        Version::new(1, 2, 3),
    ];
    let device = SmartDeviceV1 {
        id: 19019,
        category: DeviceCategory::Sensor,
        name: String::from("Door Sensor"),
        enabled: true,
    };
    for version in versions {
        let encoded = encode_versioned_value(&device, version).expect("encode failed");
        let (decoded, ver, _consumed): (SmartDeviceV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, device);
        assert_eq!(
            ver, version,
            "extracted version should match encoded version"
        );
    }
}

// ── Scenario 20 ───────────────────────────────────────────────────────────────
// Three separate versioned buffers (V1/V2/V3) decoded independently
#[test]
fn test_three_iot_versions_decoded_independently() {
    let ver1 = Version::new(1, 0, 0);
    let ver2 = Version::new(2, 0, 0);
    let ver3 = Version::new(3, 0, 0);

    let dev1 = SmartDeviceV1 {
        id: 1,
        category: DeviceCategory::Light,
        name: String::from("v1 light"),
        enabled: true,
    };
    let dev2 = SmartDeviceV2 {
        id: 2,
        category: DeviceCategory::Lock,
        name: String::from("v2 lock"),
        enabled: false,
        firmware: String::from("lock-fw-1.0"),
    };
    let dev3 = SmartDeviceV3 {
        id: 3,
        category: DeviceCategory::Hub,
        name: String::from("v3 hub"),
        enabled: true,
        firmware: String::from("hub-fw-3.0"),
        room: String::from("Office"),
        tags: vec![String::from("zigbee"), String::from("zwave")],
    };

    let enc1 = encode_versioned_value(&dev1, ver1).expect("encode v1 failed");
    let enc2 = encode_versioned_value(&dev2, ver2).expect("encode v2 failed");
    let enc3 = encode_versioned_value(&dev3, ver3).expect("encode v3 failed");

    let (dec1, v1_out, _c1): (SmartDeviceV1, Version, usize) =
        decode_versioned_value(&enc1).expect("decode v1 failed");
    let (dec2, v2_out, _c2): (SmartDeviceV2, Version, usize) =
        decode_versioned_value(&enc2).expect("decode v2 failed");
    let (dec3, v3_out, _c3): (SmartDeviceV3, Version, usize) =
        decode_versioned_value(&enc3).expect("decode v3 failed");

    assert_eq!(dec1, dev1);
    assert_eq!(dec2, dev2);
    assert_eq!(dec3, dev3);
    assert_eq!(v1_out, ver1);
    assert_eq!(v2_out, ver2);
    assert_eq!(v3_out, ver3);
    assert_ne!(enc1, enc2);
    assert_ne!(enc2, enc3);
}

// ── Scenario 21 ───────────────────────────────────────────────────────────────
// HomeConfig house_id boundary values: 0 and u64::MAX
#[test]
fn test_home_config_house_id_boundary_values() {
    let version = Version::new(1, 0, 0);
    for house_id in [0u64, u64::MAX] {
        let original = HomeConfig {
            house_id,
            devices: vec![],
            timezone: String::from("UTC"),
        };
        let encoded = encode_versioned_value(&original, version).expect("encode failed");
        let (decoded, ver, _consumed): (HomeConfig, Version, usize) =
            decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded.house_id, house_id);
        assert!(decoded.devices.is_empty());
        assert_eq!(ver, version);
    }
}

// ── Scenario 22 ───────────────────────────────────────────────────────────────
// Version semver properties across IoT device generations
#[test]
fn test_version_semver_properties_across_iot_generations() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);

    // Each generation is a breaking change from the previous
    assert!(v2.is_breaking_change_from(&v1), "V2 breaks V1");
    assert!(v3.is_breaking_change_from(&v2), "V3 breaks V2");

    // Different major versions are not compatible with each other
    assert!(
        !v1.is_compatible_with(&v2),
        "V1 and V2 major version mismatch"
    );
    assert!(
        !v2.is_compatible_with(&v3),
        "V2 and V3 major version mismatch"
    );
    assert!(
        !v1.is_compatible_with(&v3),
        "V1 and V3 major version mismatch"
    );

    // Each version satisfies itself
    assert!(v1.satisfies(&v1), "V1 satisfies V1");
    assert!(v2.satisfies(&v2), "V2 satisfies V2");
    assert!(v3.satisfies(&v3), "V3 satisfies V3");

    // V3 satisfies V2 and V1 as minimum (it's larger)
    assert!(v3.satisfies(&v2), "V3 satisfies min V2");
    assert!(v3.satisfies(&v1), "V3 satisfies min V1");

    // V1 does NOT satisfy V2 or V3 as minimum
    assert!(!v1.satisfies(&v2), "V1 does not satisfy min V2");
    assert!(!v1.satisfies(&v3), "V1 does not satisfy min V3");

    // Patch-only version is compatible with same major/minor
    let v1_patch = Version::new(1, 0, 5);
    assert!(
        v1_patch.is_compatible_with(&v1),
        "patch version is compatible"
    );
    assert!(v1_patch.is_patch_update_from(&v1), "is patch update");
}
