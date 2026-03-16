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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CommandType {
    Maneuver,
    PowerCycle,
    PayloadOn,
    PayloadOff,
    ImageCapture,
    DataDownlink,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SatelliteBeacon {
    sat_id: u32,
    signal_freq_hz: u64,
    power_dbm: i8,
    beam_id: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TelemetryPacket {
    sat_id: u32,
    timestamp: u64,
    battery_pct: u8,
    solar_power_w: f32,
    attitude: [f32; 4],
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroundContact {
    station_id: u32,
    sat_id: u32,
    start_time: u64,
    end_time: u64,
    max_elevation_deg: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CommandUplink {
    command_id: u64,
    sat_id: u32,
    command_type: CommandType,
    payload: Vec<u8>,
    scheduled_time: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrbitElement {
    sat_id: u32,
    epoch: u64,
    semi_major_axis: f64,
    eccentricity: f64,
    inclination: f32,
}

// Test 1: SatelliteBeacon basic encode/decode roundtrip
#[test]
fn test_satellite_beacon_basic_roundtrip() {
    let beacon = SatelliteBeacon {
        sat_id: 1001,
        signal_freq_hz: 2_400_000_000,
        power_dbm: -50,
        beam_id: 7,
    };
    let encoded = encode_to_vec(&beacon).expect("encode SatelliteBeacon failed");
    let (decoded, _consumed): (SatelliteBeacon, usize) =
        decode_from_slice(&encoded).expect("decode SatelliteBeacon failed");
    assert_eq!(beacon, decoded);
}

// Test 2: SatelliteBeacon versioned encode with v1.0.0
#[test]
fn test_satellite_beacon_versioned_v1() {
    let beacon = SatelliteBeacon {
        sat_id: 2002,
        signal_freq_hz: 8_450_000_000,
        power_dbm: -30,
        beam_id: 3,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&beacon, ver).expect("versioned encode failed");
    let (decoded, decoded_ver, consumed): (SatelliteBeacon, Version, usize) =
        decode_versioned_value::<SatelliteBeacon>(&encoded).expect("versioned decode failed");
    assert_eq!(beacon, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert!(consumed > 0);
}

// Test 3: TelemetryPacket basic roundtrip
#[test]
fn test_telemetry_packet_basic_roundtrip() {
    let pkt = TelemetryPacket {
        sat_id: 3003,
        timestamp: 1_700_000_000,
        battery_pct: 87,
        solar_power_w: 1450.5,
        attitude: [0.0, 0.0, 0.0, 1.0],
    };
    let encoded = encode_to_vec(&pkt).expect("encode TelemetryPacket failed");
    let (decoded, _consumed): (TelemetryPacket, usize) =
        decode_from_slice(&encoded).expect("decode TelemetryPacket failed");
    assert_eq!(pkt, decoded);
}

// Test 4: TelemetryPacket versioned with v2.0.0
#[test]
fn test_telemetry_packet_versioned_v2() {
    let pkt = TelemetryPacket {
        sat_id: 4004,
        timestamp: 1_710_000_000,
        battery_pct: 62,
        solar_power_w: 980.25,
        attitude: [0.1, 0.2, 0.3, 0.9274],
    };
    let ver = Version::new(2, 0, 0);
    let encoded =
        encode_versioned_value(&pkt, ver).expect("versioned encode TelemetryPacket failed");
    let (decoded, decoded_ver, consumed): (TelemetryPacket, Version, usize) =
        decode_versioned_value::<TelemetryPacket>(&encoded)
            .expect("versioned decode TelemetryPacket failed");
    assert_eq!(pkt, decoded);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert!(consumed > 0);
}

// Test 5: GroundContact basic roundtrip
#[test]
fn test_ground_contact_basic_roundtrip() {
    let contact = GroundContact {
        station_id: 101,
        sat_id: 5005,
        start_time: 1_720_000_000,
        end_time: 1_720_000_600,
        max_elevation_deg: 45.3,
    };
    let encoded = encode_to_vec(&contact).expect("encode GroundContact failed");
    let (decoded, _consumed): (GroundContact, usize) =
        decode_from_slice(&encoded).expect("decode GroundContact failed");
    assert_eq!(contact, decoded);
}

// Test 6: GroundContact versioned with v1.5.0
#[test]
fn test_ground_contact_versioned_v1_5() {
    let contact = GroundContact {
        station_id: 202,
        sat_id: 6006,
        start_time: 1_730_000_000,
        end_time: 1_730_001_200,
        max_elevation_deg: 72.8,
    };
    let ver = Version::new(1, 5, 0);
    let encoded =
        encode_versioned_value(&contact, ver).expect("versioned encode GroundContact failed");
    let (decoded, decoded_ver, consumed): (GroundContact, Version, usize) =
        decode_versioned_value::<GroundContact>(&encoded)
            .expect("versioned decode GroundContact failed");
    assert_eq!(contact, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 5);
    assert_eq!(decoded_ver.patch, 0);
    assert!(consumed > 0);
}

// Test 7: CommandUplink with Maneuver command
#[test]
fn test_command_uplink_maneuver_roundtrip() {
    let cmd = CommandUplink {
        command_id: 9_000_000_001,
        sat_id: 7007,
        command_type: CommandType::Maneuver,
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
        scheduled_time: 1_740_000_000,
    };
    let encoded = encode_to_vec(&cmd).expect("encode CommandUplink failed");
    let (decoded, _consumed): (CommandUplink, usize) =
        decode_from_slice(&encoded).expect("decode CommandUplink failed");
    assert_eq!(cmd, decoded);
}

// Test 8: CommandUplink with ImageCapture versioned v1.0.0
#[test]
fn test_command_uplink_image_capture_versioned() {
    let cmd = CommandUplink {
        command_id: 9_000_000_002,
        sat_id: 8008,
        command_type: CommandType::ImageCapture,
        payload: vec![0x01, 0x02, 0x03],
        scheduled_time: 1_750_000_000,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&cmd, ver).expect("versioned encode CommandUplink failed");
    let (decoded, decoded_ver, consumed): (CommandUplink, Version, usize) =
        decode_versioned_value::<CommandUplink>(&encoded)
            .expect("versioned decode CommandUplink failed");
    assert_eq!(cmd, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(consumed > 0, true);
}

// Test 9: OrbitElement basic roundtrip
#[test]
fn test_orbit_element_basic_roundtrip() {
    let orbit = OrbitElement {
        sat_id: 9009,
        epoch: 1_760_000_000,
        semi_major_axis: 7_000_000.0,
        eccentricity: 0.001_234,
        inclination: 51.6,
    };
    let encoded = encode_to_vec(&orbit).expect("encode OrbitElement failed");
    let (decoded, _consumed): (OrbitElement, usize) =
        decode_from_slice(&encoded).expect("decode OrbitElement failed");
    assert_eq!(orbit, decoded);
}

// Test 10: OrbitElement versioned v2.0.0
#[test]
fn test_orbit_element_versioned_v2() {
    let orbit = OrbitElement {
        sat_id: 10010,
        epoch: 1_770_000_000,
        semi_major_axis: 42_164_000.0,
        eccentricity: 0.000_070,
        inclination: 0.05,
    };
    let ver = Version::new(2, 0, 0);
    let encoded =
        encode_versioned_value(&orbit, ver).expect("versioned encode OrbitElement failed");
    let (decoded, decoded_ver, consumed): (OrbitElement, Version, usize) =
        decode_versioned_value::<OrbitElement>(&encoded)
            .expect("versioned decode OrbitElement failed");
    assert_eq!(orbit, decoded);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert!(consumed > 0);
}

// Test 11: Version field access — patch field
#[test]
fn test_version_patch_field_access() {
    let ver = Version::new(1, 2, 3);
    let beacon = SatelliteBeacon {
        sat_id: 11011,
        signal_freq_hz: 5_800_000_000,
        power_dbm: -60,
        beam_id: 12,
    };
    let encoded =
        encode_versioned_value(&beacon, ver).expect("versioned encode beacon patch test failed");
    let (_decoded, decoded_ver, _consumed): (SatelliteBeacon, Version, usize) =
        decode_versioned_value::<SatelliteBeacon>(&encoded)
            .expect("versioned decode beacon patch test failed");
    assert_eq!(decoded_ver.patch, 3);
}

// Test 12: Version inequality — v1.0.0 vs v2.0.0
#[test]
fn test_version_inequality_major() {
    let ver1 = Version::new(1, 0, 0);
    let ver2 = Version::new(2, 0, 0);
    assert_ne!(ver1.major, ver2.major);
    assert_eq!(ver1.minor, ver2.minor);
    assert_eq!(ver1.patch, ver2.patch);
}

// Test 13: Version inequality — v1.0.0 vs v1.5.0
#[test]
fn test_version_inequality_minor() {
    let ver1 = Version::new(1, 0, 0);
    let ver2 = Version::new(1, 5, 0);
    assert_eq!(ver1.major, ver2.major);
    assert_ne!(ver1.minor, ver2.minor);
    assert_eq!(ver1.patch, ver2.patch);
}

// Test 14: Consumed bytes check for SatelliteBeacon
#[test]
fn test_satellite_beacon_consumed_bytes() {
    let beacon = SatelliteBeacon {
        sat_id: 14014,
        signal_freq_hz: 1_200_000_000,
        power_dbm: -45,
        beam_id: 5,
    };
    let encoded = encode_to_vec(&beacon).expect("encode beacon consumed bytes failed");
    let (_decoded, consumed): (SatelliteBeacon, usize) =
        decode_from_slice(&encoded).expect("decode beacon consumed bytes failed");
    assert_eq!(consumed, encoded.len());
}

// Test 15: Consumed bytes check for versioned TelemetryPacket
#[test]
fn test_telemetry_packet_versioned_consumed_bytes() {
    let pkt = TelemetryPacket {
        sat_id: 15015,
        timestamp: 1_780_000_000,
        battery_pct: 95,
        solar_power_w: 1600.0,
        attitude: [0.5, 0.5, 0.5, 0.5],
    };
    let ver = Version::new(1, 5, 0);
    let encoded =
        encode_versioned_value(&pkt, ver).expect("versioned encode telemetry consumed test failed");
    let (_decoded, _decoded_ver, consumed): (TelemetryPacket, Version, usize) =
        decode_versioned_value::<TelemetryPacket>(&encoded)
            .expect("versioned decode telemetry consumed test failed");
    assert_eq!(consumed, encoded.len());
}

// Test 16: Vec of TelemetryPackets versioned encoding
#[test]
fn test_vec_telemetry_packets_versioned() {
    let packets = vec![
        TelemetryPacket {
            sat_id: 16001,
            timestamp: 1_790_000_000,
            battery_pct: 70,
            solar_power_w: 1100.0,
            attitude: [0.0, 0.0, 1.0, 0.0],
        },
        TelemetryPacket {
            sat_id: 16002,
            timestamp: 1_790_001_000,
            battery_pct: 80,
            solar_power_w: 1200.0,
            attitude: [0.0, 1.0, 0.0, 0.0],
        },
        TelemetryPacket {
            sat_id: 16003,
            timestamp: 1_790_002_000,
            battery_pct: 90,
            solar_power_w: 1300.0,
            attitude: [1.0, 0.0, 0.0, 0.0],
        },
    ];
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&packets, ver)
        .expect("versioned encode Vec<TelemetryPacket> failed");
    let (decoded, decoded_ver, consumed): (Vec<TelemetryPacket>, Version, usize) =
        decode_versioned_value::<Vec<TelemetryPacket>>(&encoded)
            .expect("versioned decode Vec<TelemetryPacket> failed");
    assert_eq!(packets, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(consumed, encoded.len());
}

// Test 17: Vec of SatelliteBeacons basic roundtrip
#[test]
fn test_vec_satellite_beacons_roundtrip() {
    let beacons = vec![
        SatelliteBeacon {
            sat_id: 17001,
            signal_freq_hz: 2_100_000_000,
            power_dbm: -55,
            beam_id: 1,
        },
        SatelliteBeacon {
            sat_id: 17002,
            signal_freq_hz: 2_200_000_000,
            power_dbm: -52,
            beam_id: 2,
        },
    ];
    let encoded = encode_to_vec(&beacons).expect("encode Vec<SatelliteBeacon> failed");
    let (decoded, _consumed): (Vec<SatelliteBeacon>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<SatelliteBeacon> failed");
    assert_eq!(beacons, decoded);
}

// Test 18: All CommandType variants roundtrip
#[test]
fn test_all_command_types_roundtrip() {
    let variants = vec![
        CommandType::Maneuver,
        CommandType::PowerCycle,
        CommandType::PayloadOn,
        CommandType::PayloadOff,
        CommandType::ImageCapture,
        CommandType::DataDownlink,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode CommandType variant failed");
        let (decoded, _consumed): (CommandType, usize) =
            decode_from_slice(&encoded).expect("decode CommandType variant failed");
        assert_eq!(variant, &decoded);
    }
}

// Test 19: CommandUplink DataDownlink versioned v1.5.0
#[test]
fn test_command_uplink_data_downlink_versioned_v1_5() {
    let cmd = CommandUplink {
        command_id: 9_000_000_019,
        sat_id: 19019,
        command_type: CommandType::DataDownlink,
        payload: vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE],
        scheduled_time: 1_800_000_000,
    };
    let ver = Version::new(1, 5, 0);
    let encoded =
        encode_versioned_value(&cmd, ver).expect("versioned encode DataDownlink cmd failed");
    let (decoded, decoded_ver, consumed): (CommandUplink, Version, usize) =
        decode_versioned_value::<CommandUplink>(&encoded)
            .expect("versioned decode DataDownlink cmd failed");
    assert_eq!(cmd, decoded);
    assert_eq!(decoded_ver.minor, 5);
    assert!(consumed > 0);
}

// Test 20: OrbitElement GEO orbit versioned v1.5.0
#[test]
fn test_orbit_element_geo_versioned_v1_5() {
    let orbit = OrbitElement {
        sat_id: 20020,
        epoch: 1_810_000_000,
        semi_major_axis: 42_241_096.0,
        eccentricity: 0.000_100,
        inclination: 0.02,
    };
    let ver = Version::new(1, 5, 0);
    let encoded = encode_versioned_value(&orbit, ver).expect("versioned encode GEO orbit failed");
    let (decoded, decoded_ver, consumed): (OrbitElement, Version, usize) =
        decode_versioned_value::<OrbitElement>(&encoded)
            .expect("versioned decode GEO orbit failed");
    assert_eq!(orbit, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 5);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// Test 21: GroundContact consumed bytes matches encoded length
#[test]
fn test_ground_contact_versioned_consumed_equals_len() {
    let contact = GroundContact {
        station_id: 303,
        sat_id: 21021,
        start_time: 1_820_000_000,
        end_time: 1_820_001_800,
        max_elevation_deg: 88.5,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&contact, ver)
        .expect("versioned encode GroundContact len test failed");
    let (_decoded, _decoded_ver, consumed): (GroundContact, Version, usize) =
        decode_versioned_value::<GroundContact>(&encoded)
            .expect("versioned decode GroundContact len test failed");
    assert_eq!(consumed, encoded.len());
}

// Test 22: Version inequality patch — v1.0.0 vs v1.0.1
#[test]
fn test_version_inequality_patch() {
    let beacon = SatelliteBeacon {
        sat_id: 22022,
        signal_freq_hz: 9_600_000_000,
        power_dbm: -20,
        beam_id: 22,
    };
    let ver_a = Version::new(1, 0, 0);
    let ver_b = Version::new(1, 0, 1);
    let encoded_a =
        encode_versioned_value(&beacon, ver_a).expect("versioned encode beacon ver_a failed");
    let encoded_b =
        encode_versioned_value(&beacon, ver_b).expect("versioned encode beacon ver_b failed");
    let (_decoded_a, decoded_ver_a, _consumed_a): (SatelliteBeacon, Version, usize) =
        decode_versioned_value::<SatelliteBeacon>(&encoded_a)
            .expect("versioned decode beacon ver_a failed");
    let (_decoded_b, decoded_ver_b, _consumed_b): (SatelliteBeacon, Version, usize) =
        decode_versioned_value::<SatelliteBeacon>(&encoded_b)
            .expect("versioned decode beacon ver_b failed");
    assert_eq!(decoded_ver_a.major, decoded_ver_b.major);
    assert_eq!(decoded_ver_a.minor, decoded_ver_b.minor);
    assert_ne!(decoded_ver_a.patch, decoded_ver_b.patch);
    assert_eq!(decoded_ver_a.patch, 0);
    assert_eq!(decoded_ver_b.patch, 1);
}
