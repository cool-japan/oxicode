#![cfg(feature = "versioning")]

//! Fleet management / telematics domain tests for OxiCode versioning.
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version field access, Vec/Option roundtrips, bytes consumed, and backward
//! compatibility scenarios using vehicle, GPS, fuel, maintenance, and delivery data.

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

// ─── Domain structs ──────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GpsCoordinate {
    latitude: i64,  // stored as microdegrees (×1_000_000)
    longitude: i64, // stored as microdegrees (×1_000_000)
    altitude_m: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Vehicle {
    id: u32,
    vin: String,
    make: String,
    model: String,
    year: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuelReading {
    vehicle_id: u32,
    timestamp_unix: u64,
    fuel_level_pct: u8,
    odometer_km: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceRecord {
    vehicle_id: u32,
    service_type: String,
    due_km: u32,
    completed: bool,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DriverEvent {
    driver_id: u32,
    event_code: u8,
    speed_kmh: u16,
    timestamp_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RouteWaypoint {
    sequence: u16,
    coord: GpsCoordinate,
    eta_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CargoManifest {
    shipment_id: u32,
    description: String,
    weight_kg: u32,
    hazardous: bool,
    pieces: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DeliveryEvent {
    shipment_id: u32,
    vehicle_id: u32,
    status_code: u8,
    timestamp_unix: u64,
    location: Option<GpsCoordinate>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FleetAnalyticsSummary {
    fleet_id: u32,
    total_vehicles: u32,
    active_vehicles: u32,
    avg_fuel_pct: u8,
    total_distance_km: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TelematicsPacket {
    vehicle_id: u32,
    timestamp_unix: u64,
    coord: GpsCoordinate,
    speed_kmh: u16,
    heading_deg: u16,
    fuel_level_pct: u8,
}

// ─── Test 1: basic Vehicle roundtrip via encode_to_vec / decode_from_slice ───

#[test]
fn test_vehicle_basic_roundtrip() {
    let original = Vehicle {
        id: 101,
        vin: String::from("1HGCM82633A004352"),
        make: String::from("Honda"),
        model: String::from("Accord"),
        year: 2003,
    };
    let bytes = encode_to_vec(&original).expect("encode Vehicle failed");
    let (decoded, _consumed): (Vehicle, usize) =
        decode_from_slice(&bytes).expect("decode Vehicle failed");
    assert_eq!(decoded, original);
}

// ─── Test 2: GpsCoordinate versioned encode/decode ───────────────────────────

#[test]
fn test_gps_coordinate_versioned_encode_decode() {
    let version = Version::new(1, 0, 0);
    let coord = GpsCoordinate {
        latitude: 48_856_614, // 48.856614° N (Paris)
        longitude: 2_352_222, // 2.352222° E
        altitude_m: 35,
    };
    let encoded = oxicode::encode_versioned_value(&coord, version)
        .expect("encode_versioned_value GpsCoordinate failed");
    let (decoded, ver, _consumed): (GpsCoordinate, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value GpsCoordinate failed");
    assert_eq!(decoded, coord);
    assert_eq!(ver, version);
}

// ─── Test 3: Version field access (major / minor / patch) ────────────────────

#[test]
fn test_version_field_access_major_minor_patch() {
    let ver = Version::new(3, 7, 15);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 15);
}

// ─── Test 4: FuelReading versioned roundtrip with version 2.1.0 ──────────────

#[test]
fn test_fuel_reading_versioned_roundtrip_v2_1_0() {
    let version = Version::new(2, 1, 0);
    let reading = FuelReading {
        vehicle_id: 42,
        timestamp_unix: 1_700_000_000,
        fuel_level_pct: 78,
        odometer_km: 123_456,
    };
    let encoded = oxicode::encode_versioned_value(&reading, version)
        .expect("encode_versioned_value FuelReading failed");
    let (decoded, ver, _consumed): (FuelReading, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value FuelReading failed");
    assert_eq!(decoded, reading);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
}

// ─── Test 5: MaintenanceRecord with Some notes ───────────────────────────────

#[test]
fn test_maintenance_record_with_some_notes_roundtrip() {
    let version = Version::new(1, 2, 3);
    let record = MaintenanceRecord {
        vehicle_id: 7,
        service_type: String::from("Oil Change"),
        due_km: 200_000,
        completed: false,
        notes: Some(String::from("Synthetic 5W-30 required")),
    };
    let encoded = oxicode::encode_versioned_value(&record, version)
        .expect("encode_versioned_value MaintenanceRecord failed");
    let (decoded, ver, _consumed): (MaintenanceRecord, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value MaintenanceRecord failed");
    assert_eq!(decoded, record);
    assert_eq!(ver, version);
    assert!(decoded.notes.is_some());
}

// ─── Test 6: MaintenanceRecord with None notes (Option field) ────────────────

#[test]
fn test_maintenance_record_none_notes_option_field() {
    let version = Version::new(1, 0, 0);
    let record = MaintenanceRecord {
        vehicle_id: 99,
        service_type: String::from("Tyre Rotation"),
        due_km: 30_000,
        completed: true,
        notes: None,
    };
    let encoded = oxicode::encode_versioned_value(&record, version)
        .expect("encode_versioned_value None notes failed");
    let (decoded, _ver, _consumed): (MaintenanceRecord, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value None notes failed");
    assert_eq!(decoded, record);
    assert!(decoded.notes.is_none());
    assert!(decoded.completed);
}

// ─── Test 7: Vec<FuelReading> versioned roundtrip ────────────────────────────

#[test]
fn test_vec_fuel_readings_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let readings = vec![
        FuelReading {
            vehicle_id: 1,
            timestamp_unix: 1_000,
            fuel_level_pct: 90,
            odometer_km: 0,
        },
        FuelReading {
            vehicle_id: 1,
            timestamp_unix: 2_000,
            fuel_level_pct: 75,
            odometer_km: 150,
        },
        FuelReading {
            vehicle_id: 1,
            timestamp_unix: 3_000,
            fuel_level_pct: 60,
            odometer_km: 305,
        },
    ];
    let encoded = oxicode::encode_versioned_value(&readings, version)
        .expect("encode_versioned_value Vec<FuelReading> failed");
    let (decoded, ver, _consumed): (Vec<FuelReading>, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value Vec<FuelReading> failed");
    assert_eq!(decoded, readings);
    assert_eq!(decoded.len(), 3);
    assert_eq!(ver, version);
}

// ─── Test 8: DriverEvent multiple versions — patch bump preserves data ────────

#[test]
fn test_driver_event_patch_bump_backward_compat() {
    // Data encoded at v1.0.0 must decode successfully when we hand-verify at v1.0.5.
    let encode_ver = Version::new(1, 0, 0);
    let event = DriverEvent {
        driver_id: 55,
        event_code: 0x03, // harsh braking
        speed_kmh: 110,
        timestamp_unix: 1_600_000_000,
    };
    let encoded = oxicode::encode_versioned_value(&event, encode_ver)
        .expect("encode_versioned_value DriverEvent v1.0.0 failed");
    // Decode and check stored version is still the original v1.0.0
    let (decoded, stored_ver, _consumed): (DriverEvent, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value DriverEvent failed");
    assert_eq!(decoded, event);
    assert_eq!(stored_ver.major, 1);
    assert_eq!(stored_ver.minor, 0);
    assert_eq!(stored_ver.patch, 0);
}

// ─── Test 9: RouteWaypoint with nested GpsCoordinate ─────────────────────────

#[test]
fn test_route_waypoint_nested_struct_versioned_roundtrip() {
    let version = Version::new(2, 0, 0);
    let waypoint = RouteWaypoint {
        sequence: 3,
        coord: GpsCoordinate {
            latitude: 51_507_351,
            longitude: -122_129,
            altitude_m: 11,
        },
        eta_unix: 1_700_005_000,
    };
    let encoded = oxicode::encode_versioned_value(&waypoint, version)
        .expect("encode_versioned_value RouteWaypoint failed");
    let (decoded, ver, _consumed): (RouteWaypoint, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value RouteWaypoint failed");
    assert_eq!(decoded, waypoint);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
}

// ─── Test 10: CargoManifest hazardous flag roundtrip ─────────────────────────

#[test]
fn test_cargo_manifest_hazardous_flag_roundtrip() {
    let version = Version::new(1, 1, 0);
    let manifest = CargoManifest {
        shipment_id: 8888,
        description: String::from("Lithium battery packs"),
        weight_kg: 1_200,
        hazardous: true,
        pieces: 48,
    };
    let encoded = oxicode::encode_versioned_value(&manifest, version)
        .expect("encode_versioned_value CargoManifest failed");
    let (decoded, ver, _consumed): (CargoManifest, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value CargoManifest failed");
    assert_eq!(decoded, manifest);
    assert!(decoded.hazardous);
    assert_eq!(ver, version);
}

// ─── Test 11: DeliveryEvent with Some GPS location ───────────────────────────

#[test]
fn test_delivery_event_with_some_location_roundtrip() {
    let version = Version::new(1, 0, 0);
    let event = DeliveryEvent {
        shipment_id: 4001,
        vehicle_id: 12,
        status_code: 0x05, // delivered
        timestamp_unix: 1_700_100_000,
        location: Some(GpsCoordinate {
            latitude: 40_712_776,
            longitude: -74_005_974,
            altitude_m: 10,
        }),
    };
    let encoded = oxicode::encode_versioned_value(&event, version)
        .expect("encode_versioned_value DeliveryEvent Some failed");
    let (decoded, _ver, _consumed): (DeliveryEvent, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value DeliveryEvent Some failed");
    assert_eq!(decoded, event);
    assert!(decoded.location.is_some());
}

// ─── Test 12: DeliveryEvent with None location ───────────────────────────────

#[test]
fn test_delivery_event_none_location_option_field() {
    let version = Version::new(1, 0, 0);
    let event = DeliveryEvent {
        shipment_id: 5001,
        vehicle_id: 88,
        status_code: 0x01, // in_transit
        timestamp_unix: 1_700_200_000,
        location: None,
    };
    let encoded = oxicode::encode_versioned_value(&event, version)
        .expect("encode_versioned_value DeliveryEvent None failed");
    let (decoded, ver, _consumed): (DeliveryEvent, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value DeliveryEvent None failed");
    assert_eq!(decoded, event);
    assert!(decoded.location.is_none());
    assert_eq!(ver, version);
}

// ─── Test 13: FleetAnalyticsSummary versioned with large counters ─────────────

#[test]
fn test_fleet_analytics_summary_large_counters_roundtrip() {
    let version = Version::new(3, 2, 1);
    let summary = FleetAnalyticsSummary {
        fleet_id: 1,
        total_vehicles: 500,
        active_vehicles: 487,
        avg_fuel_pct: 62,
        total_distance_km: 15_000_000,
    };
    let encoded = oxicode::encode_versioned_value(&summary, version)
        .expect("encode_versioned_value FleetAnalyticsSummary failed");
    let (decoded, ver, _consumed): (FleetAnalyticsSummary, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value FleetAnalyticsSummary failed");
    assert_eq!(decoded, summary);
    assert_eq!(ver.patch, 1);
}

// ─── Test 14: TelematicsPacket versioned roundtrip ───────────────────────────

#[test]
fn test_telematics_packet_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let packet = TelematicsPacket {
        vehicle_id: 23,
        timestamp_unix: 1_700_300_000,
        coord: GpsCoordinate {
            latitude: 52_520_008,
            longitude: 13_404_954,
            altitude_m: 34,
        },
        speed_kmh: 87,
        heading_deg: 270,
        fuel_level_pct: 45,
    };
    let encoded = oxicode::encode_versioned_value(&packet, version)
        .expect("encode_versioned_value TelematicsPacket failed");
    let (decoded, ver, _consumed): (TelematicsPacket, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value TelematicsPacket failed");
    assert_eq!(decoded, packet);
    assert_eq!(ver, version);
}

// ─── Test 15: bytes_consumed from decode_versioned_value is non-zero ──────────

#[test]
fn test_decode_versioned_value_bytes_consumed_is_nonzero() {
    let version = Version::new(1, 0, 0);
    let value: u32 = 99_999;
    let encoded = oxicode::encode_versioned_value(&value, version)
        .expect("encode_versioned_value u32 failed");
    let (_decoded, _ver, consumed): (u32, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode_versioned_value u32 failed");
    assert!(consumed > 0, "bytes_consumed must be greater than zero");
}

// ─── Test 16: multiple versions of same struct encoded separately ─────────────

#[test]
fn test_multiple_versions_of_vehicle_encoded_separately() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    let vehicle_v1 = Vehicle {
        id: 1,
        vin: String::from("VIN1"),
        make: String::from("Ford"),
        model: String::from("Focus"),
        year: 2010,
    };
    let vehicle_v2 = Vehicle {
        id: 2,
        vin: String::from("VIN2"),
        make: String::from("BMW"),
        model: String::from("X5"),
        year: 2022,
    };

    let enc1 = oxicode::encode_versioned_value(&vehicle_v1, v1).expect("encode vehicle v1 failed");
    let enc2 = oxicode::encode_versioned_value(&vehicle_v2, v2).expect("encode vehicle v2 failed");

    let (dec1, ver1, _): (Vehicle, Version, usize) =
        oxicode::decode_versioned_value(&enc1).expect("decode vehicle v1 failed");
    let (dec2, ver2, _): (Vehicle, Version, usize) =
        oxicode::decode_versioned_value(&enc2).expect("decode vehicle v2 failed");

    assert_eq!(dec1, vehicle_v1);
    assert_eq!(dec2, vehicle_v2);
    assert_eq!(ver1, v1);
    assert_eq!(ver2, v2);
    assert_ne!(ver1, ver2);
}

// ─── Test 17: Vec<GpsCoordinate> versioned roundtrip (GPS track) ──────────────

#[test]
fn test_vec_gps_coordinates_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let track = vec![
        GpsCoordinate {
            latitude: 48_000_000,
            longitude: 2_000_000,
            altitude_m: 50,
        },
        GpsCoordinate {
            latitude: 48_100_000,
            longitude: 2_100_000,
            altitude_m: 55,
        },
        GpsCoordinate {
            latitude: 48_200_000,
            longitude: 2_200_000,
            altitude_m: 48,
        },
        GpsCoordinate {
            latitude: 48_300_000,
            longitude: 2_300_000,
            altitude_m: 52,
        },
    ];
    let encoded = oxicode::encode_versioned_value(&track, version)
        .expect("encode_versioned_value Vec<GpsCoordinate> failed");
    let (decoded, ver, _consumed): (Vec<GpsCoordinate>, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value Vec<GpsCoordinate> failed");
    assert_eq!(decoded, track);
    assert_eq!(decoded.len(), 4);
    assert_eq!(ver, version);
}

// ─── Test 18: backward compatibility — v1.0.0 data decoded at v1.1.0 reader ──

#[test]
fn test_backward_compat_v1_0_0_data_at_v1_1_0_reader() {
    // Simulate: data was written at schema v1.0.0; reader is now at v1.1.0.
    // The versioning header stores the writer version; reader can inspect it.
    let write_ver = Version::new(1, 0, 0);
    let read_ver = Version::new(1, 1, 0);

    let record = MaintenanceRecord {
        vehicle_id: 5,
        service_type: String::from("Brake Inspection"),
        due_km: 50_000,
        completed: false,
        notes: None,
    };

    let encoded = oxicode::encode_versioned_value(&record, write_ver)
        .expect("encode MaintenanceRecord v1.0.0 failed");

    // Decode and check the stored version is still write_ver
    let (decoded, stored_ver, _consumed): (MaintenanceRecord, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode MaintenanceRecord failed");

    assert_eq!(decoded, record);
    // Stored version reflects what was written, not the reader version
    assert_eq!(stored_ver, write_ver);
    assert_ne!(stored_ver, read_ver);
    assert!(
        stored_ver.major == read_ver.major,
        "same major → compatible format family"
    );
}

// ─── Test 19: version equality and ordering ───────────────────────────────────

#[test]
fn test_version_equality_and_ordering() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 0, 1);
    let v3 = Version::new(1, 1, 0);
    let v4 = Version::new(2, 0, 0);

    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v3 < v4);
    assert_eq!(v1, Version::new(1, 0, 0));
    assert_ne!(v1, v4);
}

// ─── Test 20: Vec<DeliveryEvent> versioned roundtrip (delivery log) ───────────

#[test]
fn test_vec_delivery_events_versioned_roundtrip() {
    let version = Version::new(2, 3, 0);
    let events = vec![
        DeliveryEvent {
            shipment_id: 1,
            vehicle_id: 10,
            status_code: 0x01,
            timestamp_unix: 100,
            location: None,
        },
        DeliveryEvent {
            shipment_id: 1,
            vehicle_id: 10,
            status_code: 0x02,
            timestamp_unix: 200,
            location: Some(GpsCoordinate {
                latitude: 1,
                longitude: 2,
                altitude_m: 0,
            }),
        },
        DeliveryEvent {
            shipment_id: 1,
            vehicle_id: 10,
            status_code: 0x05,
            timestamp_unix: 300,
            location: None,
        },
    ];
    let encoded = oxicode::encode_versioned_value(&events, version)
        .expect("encode_versioned_value Vec<DeliveryEvent> failed");
    let (decoded, ver, _consumed): (Vec<DeliveryEvent>, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value Vec<DeliveryEvent> failed");
    assert_eq!(decoded, events);
    assert_eq!(decoded.len(), 3);
    assert_eq!(ver, version);
    // Check middle event has Some location
    assert!(decoded[1].location.is_some());
}

// ─── Test 21: FleetAnalyticsSummary with zero active vehicles ─────────────────

#[test]
fn test_fleet_analytics_zero_active_vehicles() {
    let version = Version::new(1, 0, 0);
    let summary = FleetAnalyticsSummary {
        fleet_id: 77,
        total_vehicles: 10,
        active_vehicles: 0,
        avg_fuel_pct: 0,
        total_distance_km: 0,
    };
    let encoded = oxicode::encode_versioned_value(&summary, version)
        .expect("encode_versioned_value zero active vehicles failed");
    let (decoded, ver, consumed): (FleetAnalyticsSummary, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value zero active vehicles failed");
    assert_eq!(decoded, summary);
    assert_eq!(decoded.active_vehicles, 0);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    // consumed now includes the full versioned envelope (header + payload)
    assert_eq!(consumed, encoded.len());
}

// ─── Test 22: TelematicsPacket Vec roundtrip with version field check ─────────

#[test]
fn test_telematics_packet_vec_roundtrip_version_fields() {
    let version = Version::new(4, 11, 99);
    let packets: Vec<TelematicsPacket> = vec![
        TelematicsPacket {
            vehicle_id: 1,
            timestamp_unix: 1_000_000,
            coord: GpsCoordinate {
                latitude: 10_000_000,
                longitude: 20_000_000,
                altitude_m: 5,
            },
            speed_kmh: 60,
            heading_deg: 90,
            fuel_level_pct: 80,
        },
        TelematicsPacket {
            vehicle_id: 2,
            timestamp_unix: 1_000_010,
            coord: GpsCoordinate {
                latitude: 11_000_000,
                longitude: 21_000_000,
                altitude_m: 8,
            },
            speed_kmh: 0,
            heading_deg: 0,
            fuel_level_pct: 15,
        },
    ];
    let encoded = oxicode::encode_versioned_value(&packets, version)
        .expect("encode_versioned_value Vec<TelematicsPacket> failed");
    let (decoded, ver, _consumed): (Vec<TelematicsPacket>, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value Vec<TelematicsPacket> failed");
    assert_eq!(decoded, packets);
    assert_eq!(decoded.len(), 2);
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 11);
    assert_eq!(ver.patch, 99);
    assert_eq!(decoded[1].fuel_level_pct, 15);
}
