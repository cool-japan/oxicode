#![cfg(feature = "versioning")]

//! Urban mobility / smart transportation domain — versioning feature tests.
//!
//! 22 test functions covering ride-sharing, micromobility, transit, and
//! traffic-management scenarios.  Each test exercises `encode_versioned_value`
//! and `decode_versioned_value` (or Version semantics directly) with the domain
//! types defined below.

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

// ── Domain type definitions ───────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum VehicleMode {
    Car,
    Bicycle,
    Scooter,
    Bus,
    Subway,
    Ferry,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TripStatus {
    Requested,
    Matched,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GpsCoord {
    lat_micro: i32,
    lon_micro: i32,
    alt_cm: i32,
}

/// Schema version 1: core trip fields only.
#[derive(Debug, PartialEq, Encode, Decode)]
struct TripV1 {
    trip_id: u64,
    mode: VehicleMode,
    status: TripStatus,
    origin: GpsCoord,
    destination: GpsCoord,
    distance_m: u32,
}

/// Schema version 2: adds fare, CO₂ estimate and duration.
#[derive(Debug, PartialEq, Encode, Decode)]
struct TripV2 {
    trip_id: u64,
    mode: VehicleMode,
    status: TripStatus,
    origin: GpsCoord,
    destination: GpsCoord,
    distance_m: u32,
    fare_cents: u32,
    co2_g: u32,
    duration_s: u32,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn downtown_coord() -> GpsCoord {
    // 37.7749° N, 122.4194° W  (San Francisco downtown) in microdegrees
    GpsCoord {
        lat_micro: 37_774_900,
        lon_micro: -122_419_400,
        alt_cm: 1600,
    }
}

fn airport_coord() -> GpsCoord {
    // 37.6213° N, 122.3790° W  (SFO airport) in microdegrees
    GpsCoord {
        lat_micro: 37_621_300,
        lon_micro: -122_379_000,
        alt_cm: 400,
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 1 — TripV1 round-trip at version 1.0.0
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_trip_v1_roundtrip_version_1_0_0() {
    let trip = TripV1 {
        trip_id: 1_000_001,
        mode: VehicleMode::Car,
        status: TripStatus::Completed,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 19_800,
    };
    let v = Version::new(1, 0, 0);
    let encoded = oxicode::encode_versioned_value(&trip, v)
        .expect("encode_versioned_value TripV1 v1.0.0 failed");
    let (decoded, ver, _consumed): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value TripV1 v1.0.0 failed");
    assert_eq!(decoded, trip);
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 2 — TripV2 round-trip at version 2.0.0
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_trip_v2_roundtrip_version_2_0_0() {
    let trip = TripV2 {
        trip_id: 2_000_001,
        mode: VehicleMode::Bus,
        status: TripStatus::Completed,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 19_800,
        fare_cents: 350,
        co2_g: 4_800,
        duration_s: 2_400,
    };
    let v = Version::new(2, 0, 0);
    let encoded = oxicode::encode_versioned_value(&trip, v)
        .expect("encode_versioned_value TripV2 v2.0.0 failed");
    let (decoded, ver, _consumed): (TripV2, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value TripV2 v2.0.0 failed");
    assert_eq!(decoded, trip);
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 3 — each VehicleMode survives a versioned round-trip
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_each_vehicle_mode_versioned() {
    let modes = [
        VehicleMode::Car,
        VehicleMode::Bicycle,
        VehicleMode::Scooter,
        VehicleMode::Bus,
        VehicleMode::Subway,
        VehicleMode::Ferry,
    ];
    let v = Version::new(1, 0, 0);
    for mode in modes {
        let payload = encode_to_vec(&mode).expect("encode_to_vec VehicleMode failed");
        let versioned = oxicode::versioning::encode_versioned(&payload, v)
            .expect("encode_versioned VehicleMode failed");
        let (raw, decoded_ver) = oxicode::versioning::decode_versioned(&versioned)
            .expect("decode_versioned VehicleMode failed");
        let (decoded_mode, _): (VehicleMode, usize) =
            decode_from_slice(&raw).expect("decode_from_slice VehicleMode failed");
        assert_eq!(decoded_mode, mode);
        assert_eq!(decoded_ver, v);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 4 — each TripStatus survives a versioned round-trip
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_each_trip_status_versioned() {
    let statuses = [
        TripStatus::Requested,
        TripStatus::Matched,
        TripStatus::InProgress,
        TripStatus::Completed,
        TripStatus::Cancelled,
    ];
    let v = Version::new(1, 0, 0);
    for status in statuses {
        let payload = encode_to_vec(&status).expect("encode_to_vec TripStatus failed");
        let versioned = oxicode::versioning::encode_versioned(&payload, v)
            .expect("encode_versioned TripStatus failed");
        let (raw, decoded_ver) = oxicode::versioning::decode_versioned(&versioned)
            .expect("decode_versioned TripStatus failed");
        let (decoded_status, _): (TripStatus, usize) =
            decode_from_slice(&raw).expect("decode_from_slice TripStatus failed");
        assert_eq!(decoded_status, status);
        assert_eq!(decoded_ver, v);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 5 — GpsCoord versioned round-trip
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_gps_coord_versioned_roundtrip() {
    let coord = GpsCoord {
        lat_micro: 51_507_400,   // 51.5074° N (London)
        lon_micro: -122_419_400, // reuse lon
        alt_cm: 1_100,
    };
    let v = Version::new(1, 2, 0);
    let encoded =
        oxicode::encode_versioned_value(&coord, v).expect("encode_versioned_value GpsCoord failed");
    let (decoded, ver, _): (GpsCoord, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode_versioned_value GpsCoord failed");
    assert_eq!(decoded, coord);
    assert_eq!(ver, v);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 0);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 6 — version triple (major, minor, patch) is preserved exactly
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_version_triple_preserved() {
    let trip = TripV1 {
        trip_id: 9_000_001,
        mode: VehicleMode::Scooter,
        status: TripStatus::Matched,
        origin: downtown_coord(),
        destination: downtown_coord(),
        distance_m: 800,
    };
    let v = Version::new(3, 7, 14);
    let encoded = oxicode::encode_versioned_value(&trip, v)
        .expect("encode_versioned_value triple-version failed");
    let (_, ver, _): (TripV1, Version, usize) = oxicode::decode_versioned_value(&encoded)
        .expect("decode_versioned_value triple-version failed");
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 14);
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 7 — v1.0.0 < v2.0.0 comparison reflects schema evolution
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_v1_less_than_v2_comparison() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v1 < v2, "schema v1 must be strictly older than v2");
    assert!(v2 > v1, "schema v2 must be strictly newer than v1");
    assert!(v2.is_breaking_change_from(&v1));
    assert!(!v1.is_compatible_with(&v2));
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 8 — Vec<TripV1> versioned round-trip (fleet snapshot)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_vec_of_trip_v1_versioned() {
    let fleet: Vec<TripV1> = (0..4)
        .map(|i| TripV1 {
            trip_id: 100 + i as u64,
            mode: VehicleMode::Car,
            status: TripStatus::InProgress,
            origin: downtown_coord(),
            destination: airport_coord(),
            distance_m: 5_000 + i * 500,
        })
        .collect();
    let v = Version::new(1, 0, 0);
    let encoded = oxicode::encode_versioned_value(&fleet, v)
        .expect("encode_versioned_value Vec<TripV1> failed");
    let (decoded, ver, _): (Vec<TripV1>, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode_versioned_value Vec<TripV1> failed");
    assert_eq!(decoded, fleet);
    assert_eq!(ver, v);
    assert_eq!(decoded.len(), 4);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 9 — transit schema upgrade: decode TripV1 payload, re-encode as TripV2
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_transit_upgrade_v1_to_v2() {
    // Encode a TripV1 at schema version 1.0.0
    let trip_v1 = TripV1 {
        trip_id: 7_001,
        mode: VehicleMode::Subway,
        status: TripStatus::Completed,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 22_000,
    };
    let v1 = Version::new(1, 0, 0);
    let encoded_v1 = oxicode::encode_versioned_value(&trip_v1, v1)
        .expect("encode TripV1 for upgrade test failed");

    // Simulate migration: decode the v1 payload, fill new fields, re-encode as v2
    let (old_trip, old_ver, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded_v1)
            .expect("decode TripV1 for upgrade test failed");
    assert_eq!(old_ver, v1);

    let trip_v2 = TripV2 {
        trip_id: old_trip.trip_id,
        mode: VehicleMode::Subway,
        status: TripStatus::Completed,
        origin: old_trip.origin,
        destination: old_trip.destination,
        distance_m: old_trip.distance_m,
        fare_cents: 250, // filled-in default
        co2_g: 0,        // subway has zero direct emissions
        duration_s: 1_800,
    };
    let v2 = Version::new(2, 0, 0);
    let encoded_v2 =
        oxicode::encode_versioned_value(&trip_v2, v2).expect("encode TripV2 after upgrade failed");
    let (decoded_v2, new_ver, _): (TripV2, Version, usize) =
        oxicode::decode_versioned_value(&encoded_v2).expect("decode TripV2 after upgrade failed");
    assert_eq!(new_ver, v2);
    assert_eq!(decoded_v2.trip_id, 7_001);
    assert_eq!(decoded_v2.co2_g, 0);
    assert_eq!(decoded_v2.fare_cents, 250);
    assert!(v2.is_breaking_change_from(&v1));
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 10 — zero-fare trip (promotional / free ride)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_zero_fare_trip_v2() {
    let trip = TripV2 {
        trip_id: 5_555,
        mode: VehicleMode::Bicycle,
        status: TripStatus::Completed,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 15_200,
        fare_cents: 0,
        co2_g: 0,
        duration_s: 3_600,
    };
    let v = Version::new(2, 0, 0);
    let encoded =
        oxicode::encode_versioned_value(&trip, v).expect("encode zero-fare TripV2 failed");
    let (decoded, ver, _): (TripV2, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode zero-fare TripV2 failed");
    assert_eq!(decoded.fare_cents, 0);
    assert_eq!(ver, v);
    assert_eq!(decoded.co2_g, 0);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 11 — maximum-distance trip (cross-country ferry route)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_max_distance_trip() {
    let trip = TripV1 {
        trip_id: u64::MAX / 2,
        mode: VehicleMode::Ferry,
        status: TripStatus::InProgress,
        origin: GpsCoord {
            lat_micro: 1_000_000,
            lon_micro: -1_000_000,
            alt_cm: 0,
        },
        destination: GpsCoord {
            lat_micro: 60_000_000,
            lon_micro: 30_000_000,
            alt_cm: 0,
        },
        distance_m: u32::MAX,
    };
    let v = Version::new(1, 0, 0);
    let encoded =
        oxicode::encode_versioned_value(&trip, v).expect("encode max-distance TripV1 failed");
    let (decoded, ver, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode max-distance TripV1 failed");
    assert_eq!(decoded.distance_m, u32::MAX);
    assert_eq!(decoded.trip_id, u64::MAX / 2);
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 12 — origin equals destination (round-trip / loop ride)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_origin_equals_destination_trip() {
    let coord = downtown_coord();
    let trip = TripV1 {
        trip_id: 3_333,
        mode: VehicleMode::Scooter,
        status: TripStatus::Completed,
        origin: GpsCoord {
            lat_micro: coord.lat_micro,
            lon_micro: coord.lon_micro,
            alt_cm: coord.alt_cm,
        },
        destination: GpsCoord {
            lat_micro: coord.lat_micro,
            lon_micro: coord.lon_micro,
            alt_cm: coord.alt_cm,
        },
        distance_m: 0,
    };
    let v = Version::new(1, 0, 0);
    let encoded =
        oxicode::encode_versioned_value(&trip, v).expect("encode origin=destination trip failed");
    let (decoded, ver, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode origin=destination trip failed");
    assert_eq!(decoded.origin, decoded.destination);
    assert_eq!(decoded.distance_m, 0);
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 13 — trip in the southern hemisphere (negative latitude)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_southern_hemisphere_trip() {
    // Sydney CBD: -33.8688° S, 151.2093° E  (microdegrees)
    let sydney = GpsCoord {
        lat_micro: -33_868_800,
        lon_micro: 151_209_300,
        alt_cm: 500,
    };
    // Sydney Airport: -33.9399° S
    let sydney_airport = GpsCoord {
        lat_micro: -33_939_900,
        lon_micro: 151_175_200,
        alt_cm: 200,
    };
    let trip = TripV1 {
        trip_id: 8_001,
        mode: VehicleMode::Car,
        status: TripStatus::Completed,
        origin: sydney,
        destination: sydney_airport,
        distance_m: 8_700,
    };
    let v = Version::new(1, 0, 0);
    let encoded =
        oxicode::encode_versioned_value(&trip, v).expect("encode southern hemisphere trip failed");
    let (decoded, ver, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode southern hemisphere trip failed");
    assert!(
        decoded.origin.lat_micro < 0,
        "latitude must be negative in southern hemisphere"
    );
    assert_eq!(decoded.destination.lat_micro, -33_939_900);
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 14 — trip near the international date line (longitude ≈ ±180°)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_trip_near_international_date_line() {
    // Fiji islands — straddles the date line
    let suva = GpsCoord {
        lat_micro: -18_141_600,
        lon_micro: 178_441_200, // 178.4412° E
        alt_cm: 600,
    };
    let nadi = GpsCoord {
        lat_micro: -17_756_700,
        lon_micro: -177_446_200, // 177.4462° W (west side of date line)
        alt_cm: 200,
    };
    let trip = TripV1 {
        trip_id: 9_900,
        mode: VehicleMode::Ferry,
        status: TripStatus::InProgress,
        origin: suva,
        destination: nadi,
        distance_m: 350_000,
    };
    let v = Version::new(1, 1, 0);
    let encoded = oxicode::encode_versioned_value(&trip, v).expect("encode date-line trip failed");
    let (decoded, ver, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode date-line trip failed");
    assert!(
        decoded.origin.lon_micro > 0,
        "Suva must be east of prime meridian"
    );
    assert!(
        decoded.destination.lon_micro < 0,
        "Nadi (west side) must be negative longitude"
    );
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 15 — bicycle trip with zero CO₂ emissions
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_bicycle_trip_zero_co2() {
    let trip = TripV2 {
        trip_id: 4_200,
        mode: VehicleMode::Bicycle,
        status: TripStatus::Completed,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 12_000,
        fare_cents: 0,
        co2_g: 0,
        duration_s: 2_700,
    };
    let v = Version::new(2, 0, 0);
    let encoded =
        oxicode::encode_versioned_value(&trip, v).expect("encode bicycle zero-CO2 trip failed");
    let (decoded, ver, _): (TripV2, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode bicycle zero-CO2 trip failed");
    assert_eq!(decoded.co2_g, 0, "bicycle trips must have zero CO2");
    assert!(matches!(decoded.mode, VehicleMode::Bicycle));
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 16 — ferry trip with long duration
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_ferry_trip_with_duration() {
    let trip = TripV2 {
        trip_id: 6_666,
        mode: VehicleMode::Ferry,
        status: TripStatus::Completed,
        origin: GpsCoord {
            lat_micro: 37_774_900,
            lon_micro: -122_419_400,
            alt_cm: 0,
        },
        destination: GpsCoord {
            lat_micro: 37_856_300,
            lon_micro: -122_486_700,
            alt_cm: 0,
        },
        distance_m: 18_500,
        fare_cents: 1_250,
        co2_g: 2_200,
        duration_s: 5_400, // 90-minute ferry ride
    };
    let v = Version::new(2, 0, 0);
    let encoded =
        oxicode::encode_versioned_value(&trip, v).expect("encode ferry trip with duration failed");
    let (decoded, ver, _): (TripV2, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode ferry trip with duration failed");
    assert_eq!(decoded.duration_s, 5_400);
    assert!(matches!(decoded.mode, VehicleMode::Ferry));
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 17 — cancelled trip (rider changed plans)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_cancelled_trip() {
    let trip = TripV1 {
        trip_id: 1_001,
        mode: VehicleMode::Car,
        status: TripStatus::Cancelled,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 0, // trip never started
    };
    let v = Version::new(1, 0, 0);
    let encoded = oxicode::encode_versioned_value(&trip, v).expect("encode cancelled trip failed");
    let (decoded, ver, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode cancelled trip failed");
    assert!(matches!(decoded.status, TripStatus::Cancelled));
    assert_eq!(decoded.distance_m, 0);
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 18 — in-progress trip: real-time coordinate snapshot
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_in_progress_trip_coordinates() {
    let current_position = GpsCoord {
        lat_micro: 37_750_000, // midway between downtown and airport
        lon_micro: -122_400_000,
        alt_cm: 800,
    };
    let trip = TripV1 {
        trip_id: 2_222,
        mode: VehicleMode::Car,
        status: TripStatus::InProgress,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 10_000, // halfway through 20 km route
    };
    let v = Version::new(1, 0, 0);

    // Encode both the live trip and the real-time coordinate snapshot
    let encoded_trip =
        oxicode::encode_versioned_value(&trip, v).expect("encode in-progress trip failed");
    let encoded_pos = oxicode::encode_versioned_value(&current_position, v)
        .expect("encode position snapshot failed");

    let (decoded_trip, ver_trip, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded_trip).expect("decode in-progress trip failed");
    let (decoded_pos, ver_pos, _): (GpsCoord, Version, usize) =
        oxicode::decode_versioned_value(&encoded_pos).expect("decode position snapshot failed");

    assert!(matches!(decoded_trip.status, TripStatus::InProgress));
    assert_eq!(decoded_pos.lat_micro, 37_750_000);
    assert_eq!(ver_trip, v);
    assert_eq!(ver_pos, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 19 — multi-modal journey chain (walk → subway → scooter)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_multi_modal_journey_chain() {
    let leg1 = TripV2 {
        trip_id: 10_001,
        mode: VehicleMode::Subway,
        status: TripStatus::Completed,
        origin: downtown_coord(),
        destination: GpsCoord {
            lat_micro: 37_780_000,
            lon_micro: -122_410_000,
            alt_cm: 0,
        },
        distance_m: 3_200,
        fare_cents: 250,
        co2_g: 0,
        duration_s: 720,
    };
    let leg2 = TripV2 {
        trip_id: 10_002,
        mode: VehicleMode::Scooter,
        status: TripStatus::Completed,
        origin: GpsCoord {
            lat_micro: 37_780_000,
            lon_micro: -122_410_000,
            alt_cm: 0,
        },
        destination: airport_coord(),
        distance_m: 16_600,
        fare_cents: 1_800,
        co2_g: 320,
        duration_s: 2_100,
    };
    let v = Version::new(2, 0, 0);
    let journey = vec![leg1, leg2];
    let encoded =
        oxicode::encode_versioned_value(&journey, v).expect("encode multi-modal journey failed");
    let (decoded, ver, _): (Vec<TripV2>, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode multi-modal journey failed");
    assert_eq!(decoded.len(), 2);
    assert!(matches!(decoded[0].mode, VehicleMode::Subway));
    assert!(matches!(decoded[1].mode, VehicleMode::Scooter));
    let total_fare: u32 = decoded.iter().map(|t| t.fare_cents).sum();
    assert_eq!(total_fare, 2_050);
    assert_eq!(ver, v);
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 20 — patch version bump for a bug fix (1.0.0 → 1.0.1)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_patch_version_for_bug_fix() {
    let trip = TripV1 {
        trip_id: 11_001,
        mode: VehicleMode::Bus,
        status: TripStatus::Completed,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 19_800,
    };
    let v_before = Version::new(1, 0, 0);
    let v_after = Version::new(1, 0, 1);

    let encoded =
        oxicode::encode_versioned_value(&trip, v_after).expect("encode TripV1 at 1.0.1 failed");
    let (decoded, ver, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode TripV1 at 1.0.1 failed");
    assert_eq!(decoded, trip);
    assert_eq!(ver, v_after);
    assert!(v_after.is_patch_update_from(&v_before));
    assert!(v_after.is_compatible_with(&v_before));
    assert!(!v_after.is_breaking_change_from(&v_before));
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 21 — minor version bump for new optional field (1.0.0 → 1.1.0)
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_minor_version_for_new_field() {
    let trip = TripV1 {
        trip_id: 12_001,
        mode: VehicleMode::Car,
        status: TripStatus::Requested,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 19_800,
    };
    let v_before = Version::new(1, 0, 0);
    let v_after = Version::new(1, 1, 0);

    let encoded =
        oxicode::encode_versioned_value(&trip, v_after).expect("encode TripV1 at 1.1.0 failed");
    let (decoded, ver, _): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode TripV1 at 1.1.0 failed");
    assert_eq!(decoded, trip);
    assert_eq!(ver, v_after);
    assert!(v_after.is_minor_update_from(&v_before));
    assert!(v_after.is_compatible_with(&v_before));
    assert!(!v_after.is_breaking_change_from(&v_before));
}

// ═════════════════════════════════════════════════════════════════════════════
// Test 22 — consumed bytes check: header + payload == total encoded length
// ═════════════════════════════════════════════════════════════════════════════
#[test]
fn test_consumed_bytes_check() {
    let trip = TripV1 {
        trip_id: 13_001,
        mode: VehicleMode::Subway,
        status: TripStatus::Completed,
        origin: downtown_coord(),
        destination: airport_coord(),
        distance_m: 4_500,
    };
    let v = Version::new(1, 0, 0);
    let encoded = oxicode::encode_versioned_value(&trip, v)
        .expect("encode TripV1 for consumed-bytes check failed");

    // decode_versioned_value returns consumed = bytes read from the payload slice
    let (decoded, ver, consumed): (TripV1, Version, usize) =
        oxicode::decode_versioned_value(&encoded)
            .expect("decode TripV1 for consumed-bytes check failed");
    assert_eq!(decoded, trip);
    assert_eq!(ver, v);

    // consumed now includes the full versioned envelope (header + payload).
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal the full encoded length"
    );
    assert!(
        consumed > 0,
        "non-empty struct must consume at least one byte"
    );
}
