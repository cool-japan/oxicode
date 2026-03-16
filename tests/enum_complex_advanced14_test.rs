//! Tests for Public Transportation / Transit System — advanced enum roundtrip coverage.
//!
//! Domain types model a simplified transit network with transit modes, service statuses,
//! stop types, GPS coordinates, stops, routes, and vehicles — all exercising complex
//! enum variants (including named-field and newtype variants with data) across a variety
//! of OxiCode config options.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TransitMode {
    Bus,
    Subway,
    Tram,
    Ferry,
    CableCar,
    Monorail,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ServiceStatus {
    Running,
    Delayed { minutes: u32 },
    Cancelled { reason: String },
    PartialService,
    Normal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StopType {
    Regular,
    Terminal,
    Transfer,
    RequestStop,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GpsCoord {
    lat: f64,
    lon: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Stop {
    id: u32,
    name: String,
    location: GpsCoord,
    stop_type: StopType,
    accessibility: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Route {
    id: u32,
    name: String,
    mode: TransitMode,
    stops: Vec<Stop>,
    status: ServiceStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Vehicle {
    id: u64,
    route_id: u32,
    current_stop: u32,
    occupancy_pct: u8,
    status: ServiceStatus,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_coord(lat: f64, lon: f64) -> GpsCoord {
    GpsCoord { lat, lon }
}

fn make_stop(
    id: u32,
    name: &str,
    lat: f64,
    lon: f64,
    stop_type: StopType,
    accessibility: bool,
) -> Stop {
    Stop {
        id,
        name: name.to_string(),
        location: make_coord(lat, lon),
        stop_type,
        accessibility,
    }
}

fn make_route(
    id: u32,
    name: &str,
    mode: TransitMode,
    stops: Vec<Stop>,
    status: ServiceStatus,
) -> Route {
    Route {
        id,
        name: name.to_string(),
        mode,
        stops,
        status,
    }
}

fn make_vehicle(
    id: u64,
    route_id: u32,
    current_stop: u32,
    occupancy_pct: u8,
    status: ServiceStatus,
) -> Vehicle {
    Vehicle {
        id,
        route_id,
        current_stop,
        occupancy_pct,
        status,
    }
}

// ---------------------------------------------------------------------------
// Test 1: TransitMode — all 6 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_transit_mode_all_variants_roundtrip() {
    let modes = vec![
        TransitMode::Bus,
        TransitMode::Subway,
        TransitMode::Tram,
        TransitMode::Ferry,
        TransitMode::CableCar,
        TransitMode::Monorail,
    ];

    for mode in &modes {
        let bytes = encode_to_vec(mode).expect("encode TransitMode");
        let (decoded, consumed): (TransitMode, usize) =
            decode_from_slice(&bytes).expect("decode TransitMode");
        assert_eq!(mode, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for TransitMode"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 2: TransitMode — discriminant uniqueness across all 6 variants
// ---------------------------------------------------------------------------

#[test]
fn test_transit_mode_discriminant_uniqueness() {
    let modes = vec![
        TransitMode::Bus,
        TransitMode::Subway,
        TransitMode::Tram,
        TransitMode::Ferry,
        TransitMode::CableCar,
        TransitMode::Monorail,
    ];

    let encodings: Vec<Vec<u8>> = modes
        .iter()
        .map(|m| encode_to_vec(m).expect("encode TransitMode for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "TransitMode variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 3: ServiceStatus — all 5 variants roundtrip (including Delayed and Cancelled)
// ---------------------------------------------------------------------------

#[test]
fn test_service_status_all_variants_roundtrip() {
    let statuses = vec![
        ServiceStatus::Running,
        ServiceStatus::Delayed { minutes: 15 },
        ServiceStatus::Cancelled {
            reason: "Track maintenance".to_string(),
        },
        ServiceStatus::PartialService,
        ServiceStatus::Normal,
    ];

    for status in &statuses {
        let bytes = encode_to_vec(status).expect("encode ServiceStatus");
        let (decoded, consumed): (ServiceStatus, usize) =
            decode_from_slice(&bytes).expect("decode ServiceStatus");
        assert_eq!(status, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for ServiceStatus"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 4: ServiceStatus — discriminant uniqueness across all 5 variants
// ---------------------------------------------------------------------------

#[test]
fn test_service_status_discriminant_uniqueness() {
    let statuses = vec![
        ServiceStatus::Running,
        ServiceStatus::Delayed { minutes: 1 },
        ServiceStatus::Cancelled {
            reason: "X".to_string(),
        },
        ServiceStatus::PartialService,
        ServiceStatus::Normal,
    ];

    let encodings: Vec<Vec<u8>> = statuses
        .iter()
        .map(|s| encode_to_vec(s).expect("encode ServiceStatus for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "ServiceStatus variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 5: ServiceStatus::Delayed — various minute values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_service_status_delayed_minute_values_roundtrip() {
    let delayed_variants = vec![
        ServiceStatus::Delayed { minutes: 0 },
        ServiceStatus::Delayed { minutes: 1 },
        ServiceStatus::Delayed { minutes: 5 },
        ServiceStatus::Delayed { minutes: 60 },
        ServiceStatus::Delayed { minutes: u32::MAX },
    ];

    for status in &delayed_variants {
        let bytes = encode_to_vec(status).expect("encode ServiceStatus::Delayed");
        let (decoded, consumed): (ServiceStatus, usize) =
            decode_from_slice(&bytes).expect("decode ServiceStatus::Delayed");
        assert_eq!(status, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal ServiceStatus::Delayed encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 6: ServiceStatus::Cancelled — various reason strings roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_service_status_cancelled_reason_strings_roundtrip() {
    let cancelled_variants = vec![
        ServiceStatus::Cancelled {
            reason: String::new(),
        },
        ServiceStatus::Cancelled {
            reason: "Signal failure".to_string(),
        },
        ServiceStatus::Cancelled {
            reason: "Severe weather conditions — all services suspended".to_string(),
        },
        ServiceStatus::Cancelled {
            reason: "Emergency vehicle incident blocking route 42".to_string(),
        },
        ServiceStatus::Cancelled {
            reason: "Strike action by transport workers union".to_string(),
        },
    ];

    for status in &cancelled_variants {
        let bytes = encode_to_vec(status).expect("encode ServiceStatus::Cancelled");
        let (decoded, consumed): (ServiceStatus, usize) =
            decode_from_slice(&bytes).expect("decode ServiceStatus::Cancelled");
        assert_eq!(status, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal ServiceStatus::Cancelled encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 7: StopType — all 4 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_stop_type_all_variants_roundtrip() {
    let stop_types = vec![
        StopType::Regular,
        StopType::Terminal,
        StopType::Transfer,
        StopType::RequestStop,
    ];

    for st in &stop_types {
        let bytes = encode_to_vec(st).expect("encode StopType");
        let (decoded, consumed): (StopType, usize) =
            decode_from_slice(&bytes).expect("decode StopType");
        assert_eq!(st, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for StopType"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 8: StopType — discriminant uniqueness across all 4 variants
// ---------------------------------------------------------------------------

#[test]
fn test_stop_type_discriminant_uniqueness() {
    let stop_types = vec![
        StopType::Regular,
        StopType::Terminal,
        StopType::Transfer,
        StopType::RequestStop,
    ];

    let encodings: Vec<Vec<u8>> = stop_types
        .iter()
        .map(|st| encode_to_vec(st).expect("encode StopType for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "StopType variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 9: GpsCoord — boundary and typical coordinate values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gps_coord_boundary_values_roundtrip() {
    let coords = vec![
        make_coord(0.0, 0.0),
        make_coord(90.0, 180.0),
        make_coord(-90.0, -180.0),
        make_coord(51.5074, -0.1278),   // London
        make_coord(35.6762, 139.6503),  // Tokyo
        make_coord(-33.8688, 151.2093), // Sydney
    ];

    for coord in &coords {
        let bytes = encode_to_vec(coord).expect("encode GpsCoord");
        let (decoded, consumed): (GpsCoord, usize) =
            decode_from_slice(&bytes).expect("decode GpsCoord");
        assert_eq!(coord, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal GpsCoord encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10: Stop — all StopType variants with accessibility flags roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_stop_all_stop_types_roundtrip() {
    let stops = vec![
        make_stop(
            1,
            "Central Station",
            51.5074,
            -0.1278,
            StopType::Terminal,
            true,
        ),
        make_stop(
            2,
            "Market Square",
            51.5100,
            -0.1200,
            StopType::Transfer,
            true,
        ),
        make_stop(3, "Park Lane", 51.5050, -0.1350, StopType::Regular, false),
        make_stop(
            4,
            "Hilltop Lane",
            51.5200,
            -0.1400,
            StopType::RequestStop,
            false,
        ),
    ];

    for stop in &stops {
        let bytes = encode_to_vec(stop).expect("encode Stop");
        let (decoded, consumed): (Stop, usize) = decode_from_slice(&bytes).expect("decode Stop");
        assert_eq!(stop, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal Stop encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 11: Route — running bus route with multiple stops roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_route_running_bus_multiple_stops_roundtrip() {
    let stops = vec![
        make_stop(10, "Bus Depot", 48.8566, 2.3522, StopType::Terminal, true),
        make_stop(11, "Opera House", 48.8720, 2.3300, StopType::Regular, true),
        make_stop(
            12,
            "University Gate",
            48.8460,
            2.3440,
            StopType::Transfer,
            true,
        ),
        make_stop(13, "North Park", 48.8610, 2.3600, StopType::Regular, false),
        make_stop(
            14,
            "East Terminal",
            48.8550,
            2.3700,
            StopType::Terminal,
            true,
        ),
    ];
    let route = make_route(
        101,
        "Route 38 — City Centre",
        TransitMode::Bus,
        stops,
        ServiceStatus::Running,
    );

    let bytes = encode_to_vec(&route).expect("encode running bus Route");
    let (decoded, consumed): (Route, usize) =
        decode_from_slice(&bytes).expect("decode running bus Route");
    assert_eq!(route, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal running bus Route encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Route — delayed subway route with ServiceStatus::Delayed roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_route_delayed_subway_roundtrip() {
    let stops = vec![
        make_stop(
            20,
            "Metro North",
            52.5200,
            13.4050,
            StopType::Terminal,
            true,
        ),
        make_stop(21, "City Hall", 52.5180, 13.3980, StopType::Transfer, true),
        make_stop(
            22,
            "South Bridge",
            52.5090,
            13.3900,
            StopType::Regular,
            true,
        ),
        make_stop(
            23,
            "Airport Link",
            52.5300,
            13.4200,
            StopType::Terminal,
            true,
        ),
    ];
    let route = make_route(
        202,
        "U-Bahn Line 2",
        TransitMode::Subway,
        stops,
        ServiceStatus::Delayed { minutes: 12 },
    );

    let bytes = encode_to_vec(&route).expect("encode delayed subway Route");
    let (decoded, consumed): (Route, usize) =
        decode_from_slice(&bytes).expect("decode delayed subway Route");
    assert_eq!(route, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal delayed subway Route encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Route — cancelled tram route with ServiceStatus::Cancelled roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_route_cancelled_tram_roundtrip() {
    let stops = vec![
        make_stop(30, "Tram Yard", 53.4808, -2.2426, StopType::Terminal, false),
        make_stop(
            31,
            "Manchester Piccadilly",
            53.4773,
            -2.2309,
            StopType::Transfer,
            true,
        ),
        make_stop(
            32,
            "Salford Quays",
            53.4726,
            -2.2974,
            StopType::Regular,
            true,
        ),
    ];
    let route = make_route(
        303,
        "Metrolink Blue Line",
        TransitMode::Tram,
        stops,
        ServiceStatus::Cancelled {
            reason: "Overhead wire damage at junction".to_string(),
        },
    );

    let bytes = encode_to_vec(&route).expect("encode cancelled tram Route");
    let (decoded, consumed): (Route, usize) =
        decode_from_slice(&bytes).expect("decode cancelled tram Route");
    assert_eq!(route, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal cancelled tram Route encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Route — ferry partial service with empty stops roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_route_ferry_partial_service_empty_stops_roundtrip() {
    let route = make_route(
        404,
        "Harbour Ferry — East Pier",
        TransitMode::Ferry,
        vec![],
        ServiceStatus::PartialService,
    );

    let bytes = encode_to_vec(&route).expect("encode ferry PartialService Route");
    let (decoded, consumed): (Route, usize) =
        decode_from_slice(&bytes).expect("decode ferry PartialService Route");
    assert_eq!(route, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal ferry PartialService Route encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Vehicle — all ServiceStatus variants across different vehicles roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vehicle_all_service_status_variants_roundtrip() {
    let vehicles = vec![
        make_vehicle(1001, 101, 10, 75, ServiceStatus::Running),
        make_vehicle(1002, 202, 21, 90, ServiceStatus::Delayed { minutes: 7 }),
        make_vehicle(
            1003,
            303,
            30,
            0,
            ServiceStatus::Cancelled {
                reason: "Breakdown".to_string(),
            },
        ),
        make_vehicle(1004, 404, 0, 50, ServiceStatus::PartialService),
        make_vehicle(1005, 101, 14, 20, ServiceStatus::Normal),
    ];

    for vehicle in &vehicles {
        let bytes = encode_to_vec(vehicle).expect("encode Vehicle with ServiceStatus");
        let (decoded, consumed): (Vehicle, usize) =
            decode_from_slice(&bytes).expect("decode Vehicle with ServiceStatus");
        assert_eq!(vehicle, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal Vehicle encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 16: Vec<Route> — batch of multiple routes across all transit modes roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_routes_all_transit_modes_roundtrip() {
    let routes: Vec<Route> = vec![
        make_route(
            1,
            "Bus 42",
            TransitMode::Bus,
            vec![
                make_stop(1, "Start", 51.5, -0.1, StopType::Terminal, true),
                make_stop(2, "End", 51.6, -0.2, StopType::Terminal, true),
            ],
            ServiceStatus::Running,
        ),
        make_route(
            2,
            "Metro Line A",
            TransitMode::Subway,
            vec![make_stop(
                3,
                "Underground Central",
                51.5,
                -0.1,
                StopType::Transfer,
                true,
            )],
            ServiceStatus::Normal,
        ),
        make_route(
            3,
            "Tram 7",
            TransitMode::Tram,
            vec![make_stop(
                4,
                "Tram Stop Alpha",
                51.5,
                -0.1,
                StopType::Regular,
                false,
            )],
            ServiceStatus::Delayed { minutes: 3 },
        ),
        make_route(
            4,
            "River Crossing Ferry",
            TransitMode::Ferry,
            vec![
                make_stop(5, "West Dock", 51.4, -0.3, StopType::Terminal, true),
                make_stop(6, "East Dock", 51.4, -0.2, StopType::Terminal, true),
            ],
            ServiceStatus::Running,
        ),
        make_route(
            5,
            "Summit CableCar",
            TransitMode::CableCar,
            vec![
                make_stop(7, "Valley Base", 46.0, 7.5, StopType::Terminal, true),
                make_stop(8, "Peak Station", 46.1, 7.5, StopType::Terminal, false),
            ],
            ServiceStatus::Cancelled {
                reason: "High wind speeds".to_string(),
            },
        ),
        make_route(
            6,
            "Sky Monorail Express",
            TransitMode::Monorail,
            vec![make_stop(
                9,
                "Exhibition Centre",
                1.3,
                103.8,
                StopType::RequestStop,
                true,
            )],
            ServiceStatus::PartialService,
        ),
    ];

    let bytes = encode_to_vec(&routes).expect("encode Vec<Route> all transit modes");
    let (decoded, consumed): (Vec<Route>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Route> all transit modes");
    assert_eq!(routes, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal Vec<Route> encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Big-endian config — Route roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_route_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let stops = vec![
        make_stop(
            100,
            "Grand Central",
            40.7527,
            -73.9772,
            StopType::Terminal,
            true,
        ),
        make_stop(
            101,
            "Penn Station",
            40.7506,
            -73.9971,
            StopType::Transfer,
            true,
        ),
        make_stop(
            102,
            "Times Square",
            40.7580,
            -73.9855,
            StopType::Regular,
            true,
        ),
    ];
    let route = make_route(
        999,
        "Subway Line 1 — Big Endian",
        TransitMode::Subway,
        stops,
        ServiceStatus::Running,
    );

    let bytes = encode_to_vec_with_config(&route, cfg).expect("encode big-endian Route");
    let (decoded, consumed): (Route, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian Route");
    assert_eq!(route, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal big-endian Route encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Fixed-int config — Vehicle with boundary field values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vehicle_fixed_int_config_boundary_values_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let vehicle = make_vehicle(
        u64::MAX,
        u32::MAX,
        u32::MAX,
        u8::MAX,
        ServiceStatus::Delayed { minutes: u32::MAX },
    );

    let bytes =
        encode_to_vec_with_config(&vehicle, cfg).expect("encode fixed-int Vehicle boundary");
    let (decoded, consumed): (Vehicle, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed-int Vehicle boundary");
    assert_eq!(vehicle, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal fixed-int Vehicle boundary encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Big-endian + fixed-int combined config — Route with Cancelled status roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_route_big_endian_fixed_int_cancelled_status_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let stops = vec![
        make_stop(200, "Cable Base", 47.3769, 8.5417, StopType::Terminal, true),
        make_stop(
            201,
            "Mid Station",
            47.3800,
            8.5450,
            StopType::RequestStop,
            false,
        ),
        make_stop(
            202,
            "Summit Peak",
            47.3900,
            8.5500,
            StopType::Terminal,
            false,
        ),
    ];
    let route = make_route(
        777,
        "Zürich CableCar Express",
        TransitMode::CableCar,
        stops,
        ServiceStatus::Cancelled {
            reason: "Ice accumulation on cables".to_string(),
        },
    );

    let bytes = encode_to_vec_with_config(&route, cfg).expect("encode big-endian+fixed Route");
    let (decoded, consumed): (Route, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian+fixed Route");
    assert_eq!(route, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal big-endian+fixed Route encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Consumed bytes accuracy — sequential decode from concatenated buffer
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_accuracy_sequential_routes() {
    let route1 = make_route(
        10,
        "Night Bus N29",
        TransitMode::Bus,
        vec![
            make_stop(50, "Victoria", 51.4952, -0.1439, StopType::Terminal, true),
            make_stop(51, "Waterloo", 51.5031, -0.1132, StopType::Transfer, true),
        ],
        ServiceStatus::Running,
    );
    let route2 = make_route(
        20,
        "Jubilee Line",
        TransitMode::Subway,
        vec![make_stop(
            60,
            "Baker Street",
            51.5226,
            -0.1571,
            StopType::Transfer,
            true,
        )],
        ServiceStatus::Delayed { minutes: 4 },
    );
    let route3 = make_route(
        30,
        "Monorail Expo",
        TransitMode::Monorail,
        vec![],
        ServiceStatus::Normal,
    );

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(encode_to_vec(&route1).expect("encode route1"));
    buffer.extend(encode_to_vec(&route2).expect("encode route2"));
    buffer.extend(encode_to_vec(&route3).expect("encode route3"));

    let (decoded1, consumed1): (Route, usize) =
        decode_from_slice(&buffer).expect("decode route1 from concatenated buffer");
    assert_eq!(route1, decoded1);

    let (decoded2, consumed2): (Route, usize) =
        decode_from_slice(&buffer[consumed1..]).expect("decode route2 from concatenated buffer");
    assert_eq!(route2, decoded2);

    let (decoded3, consumed3): (Route, usize) = decode_from_slice(&buffer[consumed1 + consumed2..])
        .expect("decode route3 from concatenated buffer");
    assert_eq!(route3, decoded3);

    assert_eq!(
        consumed1 + consumed2 + consumed3,
        buffer.len(),
        "sum of consumed bytes must equal total buffer length"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Large route — many stops with varied stop types roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_route_many_stops_roundtrip() {
    let stop_types = [
        StopType::Regular,
        StopType::Terminal,
        StopType::Transfer,
        StopType::RequestStop,
    ];

    let stops: Vec<Stop> = (0_u32..60)
        .map(|i| {
            make_stop(
                i,
                &format!("Stop {i}"),
                51.5 + (i as f64) * 0.001,
                -0.1 - (i as f64) * 0.001,
                stop_types[(i as usize) % stop_types.len()].clone(),
                i % 3 != 0,
            )
        })
        .collect();

    let route = make_route(
        9001,
        "Comprehensive City Tram Route — Cross-City",
        TransitMode::Tram,
        stops,
        ServiceStatus::Running,
    );

    let bytes = encode_to_vec(&route).expect("encode large Route with many stops");
    let (decoded, consumed): (Route, usize) =
        decode_from_slice(&bytes).expect("decode large Route with many stops");
    assert_eq!(route, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal large Route encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Mixed Vec<Vehicle> — batch vehicles with varied statuses and modes roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_vec_vehicles_all_statuses_roundtrip() {
    let service_statuses = vec![
        ServiceStatus::Running,
        ServiceStatus::Delayed { minutes: 2 },
        ServiceStatus::Delayed { minutes: 45 },
        ServiceStatus::Cancelled {
            reason: "Driver shortage".to_string(),
        },
        ServiceStatus::Cancelled {
            reason: "Flood warning — all ferry services suspended until further notice".to_string(),
        },
        ServiceStatus::PartialService,
        ServiceStatus::Normal,
        ServiceStatus::Running,
    ];

    let vehicles: Vec<Vehicle> = service_statuses
        .into_iter()
        .enumerate()
        .map(|(i, status)| {
            make_vehicle(
                (i as u64) * 100 + 1,
                (i as u32) % 6 + 1,
                (i as u32) * 3,
                ((i as u8) * 13) % 101,
                status,
            )
        })
        .collect();

    let bytes = encode_to_vec(&vehicles).expect("encode Vec<Vehicle> mixed statuses");
    let (decoded, consumed): (Vec<Vehicle>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Vehicle> mixed statuses");
    assert_eq!(vehicles, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal Vec<Vehicle> encoding length"
    );
}
