//! Advanced nested struct encoding tests for OxiCode (set 3)

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
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Location {
    lat: f64,
    lon: f64,
    altitude: Option<f64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Vehicle {
    id: u32,
    vin: String,
    make: String,
    model: String,
    year: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Trip {
    id: u64,
    vehicle: Vehicle,
    start: Location,
    end: Option<Location>,
    waypoints: Vec<Location>,
    distance_km: f64,
    active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Fleet {
    name: String,
    trips: Vec<Trip>,
    vehicles: Vec<Vehicle>,
    region: String,
}

fn make_location(lat: f64, lon: f64, altitude: Option<f64>) -> Location {
    Location { lat, lon, altitude }
}

fn make_vehicle(id: u32, vin: &str, make: &str, model: &str, year: u16) -> Vehicle {
    Vehicle {
        id,
        vin: vin.to_string(),
        make: make.to_string(),
        model: model.to_string(),
        year,
    }
}

fn make_trip(
    id: u64,
    vehicle: Vehicle,
    start: Location,
    end: Option<Location>,
    waypoints: Vec<Location>,
    distance_km: f64,
    active: bool,
) -> Trip {
    Trip {
        id,
        vehicle,
        start,
        end,
        waypoints,
        distance_km,
        active,
    }
}

// Test 1: Location roundtrip with altitude
#[test]
fn test_location_with_altitude_roundtrip() {
    let loc = make_location(48.8566, 2.3522, Some(35.0));
    let bytes = encode_to_vec(&loc).expect("encode Location with altitude");
    let (decoded, _): (Location, usize) =
        decode_from_slice(&bytes).expect("decode Location with altitude");
    assert_eq!(loc, decoded);
}

// Test 2: Location roundtrip without altitude
#[test]
fn test_location_no_altitude_roundtrip() {
    let loc = make_location(51.5074, -0.1278, None);
    let bytes = encode_to_vec(&loc).expect("encode Location without altitude");
    let (decoded, _): (Location, usize) =
        decode_from_slice(&bytes).expect("decode Location without altitude");
    assert_eq!(loc, decoded);
}

// Test 3: Vehicle roundtrip
#[test]
fn test_vehicle_roundtrip() {
    let v = make_vehicle(1, "1HGBH41JXMN109186", "Honda", "Civic", 2021);
    let bytes = encode_to_vec(&v).expect("encode Vehicle");
    let (decoded, _): (Vehicle, usize) = decode_from_slice(&bytes).expect("decode Vehicle");
    assert_eq!(v, decoded);
}

// Test 4: Trip with end location roundtrip
#[test]
fn test_trip_with_end_location_roundtrip() {
    let v = make_vehicle(2, "2T1BURHE0JC012345", "Toyota", "Corolla", 2018);
    let start = make_location(40.7128, -74.0060, Some(10.0));
    let end = Some(make_location(34.0522, -118.2437, Some(71.0)));
    let trip = make_trip(100, v, start, end, vec![], 2789.5, false);
    let bytes = encode_to_vec(&trip).expect("encode Trip with end");
    let (decoded, _): (Trip, usize) = decode_from_slice(&bytes).expect("decode Trip with end");
    assert_eq!(trip, decoded);
}

// Test 5: Trip with no end location roundtrip
#[test]
fn test_trip_no_end_location_roundtrip() {
    let v = make_vehicle(3, "3VWFE21C04M000001", "Volkswagen", "Golf", 2004);
    let start = make_location(52.5200, 13.4050, None);
    let trip = make_trip(200, v, start, None, vec![], 0.0, true);
    let bytes = encode_to_vec(&trip).expect("encode Trip no end");
    let (decoded, _): (Trip, usize) = decode_from_slice(&bytes).expect("decode Trip no end");
    assert_eq!(trip, decoded);
}

// Test 6: Trip with 3 waypoints roundtrip
#[test]
fn test_trip_with_waypoints_roundtrip() {
    let v = make_vehicle(4, "4T1BF3EK8AU123456", "Toyota", "Camry", 2010);
    let start = make_location(48.2082, 16.3738, Some(151.0));
    let end = Some(make_location(47.8095, 13.0550, Some(425.0)));
    let waypoints = vec![
        make_location(48.1000, 15.0000, Some(200.0)),
        make_location(47.9500, 14.0000, Some(300.0)),
        make_location(47.8500, 13.5000, Some(380.0)),
    ];
    let trip = make_trip(300, v, start, end, waypoints, 305.2, false);
    let bytes = encode_to_vec(&trip).expect("encode Trip with waypoints");
    let (decoded, _): (Trip, usize) =
        decode_from_slice(&bytes).expect("decode Trip with waypoints");
    assert_eq!(trip, decoded);
}

// Test 7: Trip with empty waypoints roundtrip
#[test]
fn test_trip_empty_waypoints_roundtrip() {
    let v = make_vehicle(5, "5YJSA1DG9DFP14705", "Tesla", "Model S", 2013);
    let start = make_location(37.7749, -122.4194, Some(16.0));
    let trip = make_trip(400, v, start, None, vec![], 0.0, true);
    let bytes = encode_to_vec(&trip).expect("encode Trip empty waypoints");
    let (decoded, _): (Trip, usize) =
        decode_from_slice(&bytes).expect("decode Trip empty waypoints");
    assert_eq!(trip, decoded);
}

// Test 8: Fleet roundtrip (3 trips, 2 vehicles)
#[test]
fn test_fleet_roundtrip_3trips_2vehicles() {
    let v1 = make_vehicle(10, "WBADS63463AT32142", "BMW", "3 Series", 2003);
    let v2 = make_vehicle(11, "1FTFW1ET5DFC10312", "Ford", "F-150", 2013);
    let trip1 = make_trip(
        1,
        make_vehicle(10, "WBADS63463AT32142", "BMW", "3 Series", 2003),
        make_location(48.8566, 2.3522, None),
        Some(make_location(43.2965, 5.3698, None)),
        vec![],
        775.0,
        false,
    );
    let trip2 = make_trip(
        2,
        make_vehicle(11, "1FTFW1ET5DFC10312", "Ford", "F-150", 2013),
        make_location(40.7128, -74.0060, Some(5.0)),
        None,
        vec![make_location(41.0000, -73.0000, None)],
        120.0,
        true,
    );
    let trip3 = make_trip(
        3,
        make_vehicle(10, "WBADS63463AT32142", "BMW", "3 Series", 2003),
        make_location(51.5074, -0.1278, Some(11.0)),
        Some(make_location(53.4808, -2.2426, Some(38.0))),
        vec![],
        262.0,
        false,
    );
    let fleet = Fleet {
        name: "EuroFleet".to_string(),
        trips: vec![trip1, trip2, trip3],
        vehicles: vec![v1, v2],
        region: "Europe".to_string(),
    };
    let bytes = encode_to_vec(&fleet).expect("encode Fleet 3 trips");
    let (decoded, _): (Fleet, usize) = decode_from_slice(&bytes).expect("decode Fleet 3 trips");
    assert_eq!(fleet, decoded);
}

// Test 9: Vec<Location> roundtrip (5 items)
#[test]
fn test_vec_location_roundtrip() {
    let locs: Vec<Location> = vec![
        make_location(0.0, 0.0, None),
        make_location(10.0, 20.0, Some(100.0)),
        make_location(-33.8688, 151.2093, Some(5.0)),
        make_location(55.7558, 37.6173, None),
        make_location(35.6762, 139.6503, Some(40.0)),
    ];
    let bytes = encode_to_vec(&locs).expect("encode Vec<Location>");
    let (decoded, _): (Vec<Location>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Location>");
    assert_eq!(locs, decoded);
}

// Test 10: Vec<Vehicle> roundtrip (3 items)
#[test]
fn test_vec_vehicle_roundtrip() {
    let vehicles = vec![
        make_vehicle(1, "VIN001", "Audi", "A4", 2019),
        make_vehicle(2, "VIN002", "Mercedes", "C-Class", 2020),
        make_vehicle(3, "VIN003", "BMW", "5 Series", 2022),
    ];
    let bytes = encode_to_vec(&vehicles).expect("encode Vec<Vehicle>");
    let (decoded, _): (Vec<Vehicle>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Vehicle>");
    assert_eq!(vehicles, decoded);
}

// Test 11: Vec<Trip> roundtrip (2 trips)
#[test]
fn test_vec_trip_roundtrip() {
    let trips = vec![
        make_trip(
            1,
            make_vehicle(1, "VIN-A", "Volvo", "XC90", 2021),
            make_location(59.3293, 18.0686, Some(28.0)),
            Some(make_location(57.7089, 11.9746, None)),
            vec![],
            470.0,
            false,
        ),
        make_trip(
            2,
            make_vehicle(2, "VIN-B", "Saab", "9-3", 2007),
            make_location(60.1699, 24.9384, None),
            None,
            vec![make_location(60.5000, 25.0000, None)],
            55.0,
            true,
        ),
    ];
    let bytes = encode_to_vec(&trips).expect("encode Vec<Trip>");
    let (decoded, _): (Vec<Trip>, usize) = decode_from_slice(&bytes).expect("decode Vec<Trip>");
    assert_eq!(trips, decoded);
}

// Test 12: Option<Fleet> Some roundtrip
#[test]
fn test_option_fleet_some_roundtrip() {
    let fleet = Fleet {
        name: "NorthFleet".to_string(),
        trips: vec![make_trip(
            1,
            make_vehicle(1, "VIN-X", "Nissan", "Leaf", 2023),
            make_location(63.4305, 10.3951, None),
            None,
            vec![],
            12.5,
            true,
        )],
        vehicles: vec![make_vehicle(1, "VIN-X", "Nissan", "Leaf", 2023)],
        region: "Scandinavia".to_string(),
    };
    let opt: Option<Fleet> = Some(fleet);
    let bytes = encode_to_vec(&opt).expect("encode Option<Fleet> Some");
    let (decoded, _): (Option<Fleet>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Fleet> Some");
    assert_eq!(opt, decoded);
}

// Test 13: Option<Fleet> None roundtrip
#[test]
fn test_option_fleet_none_roundtrip() {
    let opt: Option<Fleet> = None;
    let bytes = encode_to_vec(&opt).expect("encode Option<Fleet> None");
    let (decoded, _): (Option<Fleet>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Fleet> None");
    assert_eq!(opt, decoded);
}

// Test 14: Fleet with empty trips and vehicles
#[test]
fn test_fleet_empty_collections_roundtrip() {
    let fleet = Fleet {
        name: "EmptyFleet".to_string(),
        trips: vec![],
        vehicles: vec![],
        region: "Unknown".to_string(),
    };
    let bytes = encode_to_vec(&fleet).expect("encode Fleet empty");
    let (decoded, _): (Fleet, usize) = decode_from_slice(&bytes).expect("decode Fleet empty");
    assert_eq!(fleet, decoded);
}

// Test 15: Trip with fixed-int config
#[test]
fn test_trip_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let v = make_vehicle(99, "VIN-FIXED", "Peugeot", "308", 2016);
    let trip = make_trip(
        999,
        v,
        make_location(48.8566, 2.3522, Some(50.0)),
        None,
        vec![],
        88.8,
        false,
    );
    let bytes = encode_to_vec_with_config(&trip, cfg).expect("encode Trip fixed-int");
    let (decoded, _): (Trip, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Trip fixed-int");
    assert_eq!(trip, decoded);
}

// Test 16: Consumed bytes equals encoded length for Fleet
#[test]
fn test_fleet_consumed_bytes_equals_encoded_len() {
    let fleet = Fleet {
        name: "ByteFleet".to_string(),
        trips: vec![make_trip(
            1,
            make_vehicle(1, "VIN-B1", "Renault", "Clio", 2015),
            make_location(48.5734, 7.7521, None),
            None,
            vec![],
            33.0,
            true,
        )],
        vehicles: vec![make_vehicle(1, "VIN-B1", "Renault", "Clio", 2015)],
        region: "Alsace".to_string(),
    };
    let bytes = encode_to_vec(&fleet).expect("encode Fleet for byte count");
    let (_, consumed): (Fleet, usize) =
        decode_from_slice(&bytes).expect("decode Fleet for byte count");
    assert_eq!(consumed, bytes.len());
}

// Test 17: Same Fleet encodes to same bytes both times
#[test]
fn test_fleet_deterministic_encoding() {
    let fleet = Fleet {
        name: "DeterministicFleet".to_string(),
        trips: vec![make_trip(
            1,
            make_vehicle(1, "VIN-D1", "Fiat", "500", 2019),
            make_location(41.9028, 12.4964, Some(21.0)),
            Some(make_location(43.7696, 11.2558, None)),
            vec![],
            272.0,
            false,
        )],
        vehicles: vec![make_vehicle(1, "VIN-D1", "Fiat", "500", 2019)],
        region: "Italy".to_string(),
    };
    let bytes1 = encode_to_vec(&fleet).expect("encode Fleet deterministic first");
    let bytes2 = encode_to_vec(&fleet).expect("encode Fleet deterministic second");
    assert_eq!(bytes1, bytes2);
}

// Test 18: Different Fleets produce different bytes
#[test]
fn test_different_fleets_different_bytes() {
    let fleet_a = Fleet {
        name: "FleetAlpha".to_string(),
        trips: vec![],
        vehicles: vec![make_vehicle(1, "VIN-A", "Alfa Romeo", "Giulia", 2020)],
        region: "South".to_string(),
    };
    let fleet_b = Fleet {
        name: "FleetBeta".to_string(),
        trips: vec![],
        vehicles: vec![make_vehicle(2, "VIN-B", "Lancia", "Delta", 1992)],
        region: "North".to_string(),
    };
    let bytes_a = encode_to_vec(&fleet_a).expect("encode FleetAlpha");
    let bytes_b = encode_to_vec(&fleet_b).expect("encode FleetBeta");
    assert_ne!(bytes_a, bytes_b);
}

// Test 19: Fleet with 10 trips and 5 vehicles roundtrip
#[test]
fn test_fleet_large_roundtrip() {
    let vehicles: Vec<Vehicle> = (0..5)
        .map(|i| {
            make_vehicle(
                i,
                &format!("VIN-L{i}"),
                "Generic",
                &format!("Model{i}"),
                2010 + i as u16,
            )
        })
        .collect();
    let trips: Vec<Trip> = (0..10)
        .map(|i| {
            make_trip(
                i as u64,
                make_vehicle(
                    i % 5,
                    &format!("VIN-L{}", i % 5),
                    "Generic",
                    &format!("Model{}", i % 5),
                    2010 + (i % 5) as u16,
                ),
                make_location(i as f64, i as f64 * 2.0, Some(i as f64 * 10.0)),
                if i % 2 == 0 {
                    Some(make_location(i as f64 + 1.0, i as f64 * 2.0 + 0.5, None))
                } else {
                    None
                },
                (0..i % 4)
                    .map(|j| make_location(j as f64, j as f64, None))
                    .collect(),
                i as f64 * 50.5,
                i % 3 == 0,
            )
        })
        .collect();
    let fleet = Fleet {
        name: "LargeFleet".to_string(),
        trips,
        vehicles,
        region: "Global".to_string(),
    };
    let bytes = encode_to_vec(&fleet).expect("encode large Fleet");
    let (decoded, _): (Fleet, usize) = decode_from_slice(&bytes).expect("decode large Fleet");
    assert_eq!(fleet, decoded);
}

// Test 20: Deeply nested: Fleet -> Trip -> Vehicle + Location roundtrip
#[test]
fn test_deeply_nested_fleet_trip_vehicle_location() {
    let inner_loc = make_location(35.6762, 139.6503, Some(40.0));
    let inner_vehicle = make_vehicle(42, "JN1AZ4EH9FM730109", "Nissan", "GT-R", 2015);
    let waypoints = vec![
        make_location(35.7000, 139.7000, Some(50.0)),
        make_location(35.6500, 139.6000, Some(30.0)),
    ];
    let trip = make_trip(
        9999,
        inner_vehicle,
        inner_loc,
        Some(make_location(34.6937, 135.5023, Some(15.0))),
        waypoints,
        515.0,
        false,
    );
    let fleet = Fleet {
        name: "JapanFleet".to_string(),
        trips: vec![trip],
        vehicles: vec![make_vehicle(
            42,
            "JN1AZ4EH9FM730109",
            "Nissan",
            "GT-R",
            2015,
        )],
        region: "Asia-Pacific".to_string(),
    };
    let bytes = encode_to_vec(&fleet).expect("encode deeply nested Fleet");
    let (decoded, _): (Fleet, usize) =
        decode_from_slice(&bytes).expect("decode deeply nested Fleet");
    assert_eq!(fleet, decoded);
}

// Test 21: Big-endian config with Vehicle
#[test]
fn test_vehicle_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let v = make_vehicle(77, "WBA3A5C51CF256551", "BMW", "3 Series", 2012);
    let bytes = encode_to_vec_with_config(&v, cfg).expect("encode Vehicle big-endian");
    let (decoded, _): (Vehicle, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Vehicle big-endian");
    assert_eq!(v, decoded);
}

// Test 22: Fleet with unicode region name roundtrip
#[test]
fn test_fleet_unicode_region_roundtrip() {
    let fleet = Fleet {
        name: "UniFleet".to_string(),
        trips: vec![make_trip(
            1,
            make_vehicle(1, "VIN-U1", "Mitsubishi", "Outlander", 2022),
            make_location(35.0116, 135.7681, None),
            None,
            vec![],
            88.0,
            true,
        )],
        vehicles: vec![make_vehicle(1, "VIN-U1", "Mitsubishi", "Outlander", 2022)],
        region: "日本・関西地方".to_string(),
    };
    let bytes = encode_to_vec(&fleet).expect("encode Fleet unicode region");
    let (decoded, _): (Fleet, usize) =
        decode_from_slice(&bytes).expect("decode Fleet unicode region");
    assert_eq!(fleet, decoded);
    assert_eq!(decoded.region, "日本・関西地方");
}
