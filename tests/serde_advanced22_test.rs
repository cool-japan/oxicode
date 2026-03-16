#![cfg(feature = "serde")]
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
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum CoordinateSystem {
    Wgs84,
    Utm,
    WebMercator,
    LocalGrid,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct Coordinate {
    lat_micro: i64, // latitude * 1_000_000
    lon_micro: i64, // longitude * 1_000_000
    alt_cm: Option<i32>,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct BoundingBox {
    min: Coordinate,
    max: Coordinate,
    crs: CoordinateSystem,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct MapFeature {
    id: u64,
    name: String,
    feature_type: String,
    centroid: Coordinate,
    bbox: BoundingBox,
    tags: Vec<String>,
}

// --- Test 1: CoordinateSystem::Wgs84 roundtrip ---
#[test]
fn test_coordinate_system_wgs84_roundtrip() {
    let cfg = config::standard();
    let val = CoordinateSystem::Wgs84;
    let bytes = encode_to_vec(&val, cfg).expect("encode CoordinateSystem::Wgs84");
    let (decoded, consumed): (CoordinateSystem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CoordinateSystem::Wgs84");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 2: CoordinateSystem::Utm roundtrip ---
#[test]
fn test_coordinate_system_utm_roundtrip() {
    let cfg = config::standard();
    let val = CoordinateSystem::Utm;
    let bytes = encode_to_vec(&val, cfg).expect("encode CoordinateSystem::Utm");
    let (decoded, consumed): (CoordinateSystem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CoordinateSystem::Utm");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 3: CoordinateSystem::WebMercator roundtrip ---
#[test]
fn test_coordinate_system_web_mercator_roundtrip() {
    let cfg = config::standard();
    let val = CoordinateSystem::WebMercator;
    let bytes = encode_to_vec(&val, cfg).expect("encode CoordinateSystem::WebMercator");
    let (decoded, consumed): (CoordinateSystem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CoordinateSystem::WebMercator");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 4: CoordinateSystem::LocalGrid roundtrip ---
#[test]
fn test_coordinate_system_local_grid_roundtrip() {
    let cfg = config::standard();
    let val = CoordinateSystem::LocalGrid;
    let bytes = encode_to_vec(&val, cfg).expect("encode CoordinateSystem::LocalGrid");
    let (decoded, consumed): (CoordinateSystem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CoordinateSystem::LocalGrid");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 5: Coordinate without altitude (alt_cm = None) ---
#[test]
fn test_coordinate_without_altitude_roundtrip() {
    let cfg = config::standard();
    let val = Coordinate {
        lat_micro: 35_681_236_i64,  // ~35.681236 degrees (Tokyo)
        lon_micro: 139_767_125_i64, // ~139.767125 degrees
        alt_cm: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Coordinate without altitude");
    let (decoded, consumed): (Coordinate, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Coordinate without altitude");
    assert_eq!(val.lat_micro, decoded.lat_micro);
    assert_eq!(val.lon_micro, decoded.lon_micro);
    assert!(decoded.alt_cm.is_none());
    assert_eq!(consumed, bytes.len());
}

// --- Test 6: Coordinate with altitude (alt_cm = Some) ---
#[test]
fn test_coordinate_with_altitude_roundtrip() {
    let cfg = config::standard();
    let val = Coordinate {
        lat_micro: 27_988_120_i64, // ~27.98812 degrees (Everest base camp area)
        lon_micro: 86_924_987_i64, // ~86.924987 degrees
        alt_cm: Some(884_800_i32), // 8848 metres in cm
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Coordinate with altitude");
    let (decoded, consumed): (Coordinate, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Coordinate with altitude");
    assert_eq!(val.lat_micro, decoded.lat_micro);
    assert_eq!(val.lon_micro, decoded.lon_micro);
    assert_eq!(val.alt_cm, decoded.alt_cm);
    assert_eq!(decoded.alt_cm, Some(884_800_i32));
    assert_eq!(consumed, bytes.len());
}

// --- Test 7: Coordinate at boundary latitude +90 degrees ---
#[test]
fn test_coordinate_north_pole_boundary_roundtrip() {
    let cfg = config::standard();
    let val = Coordinate {
        lat_micro: 90_000_000_i64, // exactly +90.000000 degrees
        lon_micro: 0_i64,
        alt_cm: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Coordinate north pole");
    let (decoded, consumed): (Coordinate, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Coordinate north pole");
    assert_eq!(val.lat_micro, decoded.lat_micro);
    assert_eq!(val.lon_micro, decoded.lon_micro);
    assert_eq!(consumed, bytes.len());
}

// --- Test 8: Coordinate at boundary latitude -90 degrees and longitude -180 ---
#[test]
fn test_coordinate_south_pole_boundary_roundtrip() {
    let cfg = config::standard();
    let val = Coordinate {
        lat_micro: -90_000_000_i64,  // exactly -90.000000 degrees
        lon_micro: -180_000_000_i64, // exactly -180.000000 degrees
        alt_cm: Some(-42_200_i32),   // Dead Sea depth ~ -422m in cm
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Coordinate south pole boundary");
    let (decoded, consumed): (Coordinate, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Coordinate south pole boundary");
    assert_eq!(val.lat_micro, decoded.lat_micro);
    assert_eq!(val.lon_micro, decoded.lon_micro);
    assert_eq!(val.alt_cm, decoded.alt_cm);
    assert_eq!(consumed, bytes.len());
}

// --- Test 9: BoundingBox with Wgs84 CRS roundtrip ---
#[test]
fn test_bounding_box_wgs84_roundtrip() {
    let cfg = config::standard();
    let val = BoundingBox {
        min: Coordinate {
            lat_micro: 51_460_000_i64, // SW London area
            lon_micro: -200_000_i64,
            alt_cm: None,
        },
        max: Coordinate {
            lat_micro: 51_560_000_i64, // NE London area
            lon_micro: 50_000_i64,
            alt_cm: None,
        },
        crs: CoordinateSystem::Wgs84,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BoundingBox Wgs84");
    let (decoded, consumed): (BoundingBox, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BoundingBox Wgs84");
    assert_eq!(val.min.lat_micro, decoded.min.lat_micro);
    assert_eq!(val.max.lat_micro, decoded.max.lat_micro);
    assert_eq!(val.crs, decoded.crs);
    assert_eq!(consumed, bytes.len());
}

// --- Test 10: BoundingBox with WebMercator CRS roundtrip ---
#[test]
fn test_bounding_box_web_mercator_roundtrip() {
    let cfg = config::standard();
    // WebMercator uses metres but we store microdegrees-equivalent integers here
    let val = BoundingBox {
        min: Coordinate {
            lat_micro: -2_037_508_i64,
            lon_micro: -20_037_508_i64,
            alt_cm: None,
        },
        max: Coordinate {
            lat_micro: 2_037_508_i64,
            lon_micro: 20_037_508_i64,
            alt_cm: None,
        },
        crs: CoordinateSystem::WebMercator,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BoundingBox WebMercator");
    let (decoded, consumed): (BoundingBox, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BoundingBox WebMercator");
    assert_eq!(val.min.lon_micro, decoded.min.lon_micro);
    assert_eq!(val.max.lon_micro, decoded.max.lon_micro);
    assert_eq!(val.crs, decoded.crs);
    assert_eq!(consumed, bytes.len());
}

// --- Test 11: BoundingBox with LocalGrid CRS and altitude ---
#[test]
fn test_bounding_box_local_grid_with_altitude_roundtrip() {
    let cfg = config::standard();
    let val = BoundingBox {
        min: Coordinate {
            lat_micro: 0_i64,
            lon_micro: 0_i64,
            alt_cm: Some(0_i32),
        },
        max: Coordinate {
            lat_micro: 1_000_000_i64,
            lon_micro: 1_000_000_i64,
            alt_cm: Some(10_000_i32), // 100m in cm
        },
        crs: CoordinateSystem::LocalGrid,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BoundingBox LocalGrid altitude");
    let (decoded, consumed): (BoundingBox, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BoundingBox LocalGrid altitude");
    assert_eq!(val.crs, decoded.crs);
    assert_eq!(val.min.alt_cm, decoded.min.alt_cm);
    assert_eq!(val.max.alt_cm, decoded.max.alt_cm);
    assert_eq!(consumed, bytes.len());
}

// --- Test 12: MapFeature with point centroid and empty tags roundtrip ---
#[test]
fn test_map_feature_point_empty_tags_roundtrip() {
    let cfg = config::standard();
    let centroid = Coordinate {
        lat_micro: 48_856_613_i64, // Paris
        lon_micro: 2_352_222_i64,
        alt_cm: None,
    };
    let bbox = BoundingBox {
        min: Coordinate {
            lat_micro: 48_856_000_i64,
            lon_micro: 2_351_000_i64,
            alt_cm: None,
        },
        max: Coordinate {
            lat_micro: 48_857_000_i64,
            lon_micro: 2_353_000_i64,
            alt_cm: None,
        },
        crs: CoordinateSystem::Wgs84,
    };
    let val = MapFeature {
        id: 1_u64,
        name: "Eiffel Tower".to_string(),
        feature_type: "landmark".to_string(),
        centroid,
        bbox,
        tags: vec![],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode MapFeature point empty tags");
    let (decoded, consumed): (MapFeature, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MapFeature point empty tags");
    assert_eq!(val.id, decoded.id);
    assert_eq!(val.name, decoded.name);
    assert_eq!(val.feature_type, decoded.feature_type);
    assert!(decoded.tags.is_empty());
    assert_eq!(consumed, bytes.len());
}

// --- Test 13: MapFeature with multiple tags roundtrip ---
#[test]
fn test_map_feature_with_multiple_tags_roundtrip() {
    let cfg = config::standard();
    let centroid = Coordinate {
        lat_micro: 40_712_776_i64, // New York City
        lon_micro: -74_005_974_i64,
        alt_cm: Some(1_000_i32), // ~10m
    };
    let bbox = BoundingBox {
        min: Coordinate {
            lat_micro: 40_477_399_i64,
            lon_micro: -74_259_090_i64,
            alt_cm: None,
        },
        max: Coordinate {
            lat_micro: 40_917_577_i64,
            lon_micro: -73_700_272_i64,
            alt_cm: None,
        },
        crs: CoordinateSystem::Wgs84,
    };
    let val = MapFeature {
        id: 2_u64,
        name: "New York City".to_string(),
        feature_type: "city".to_string(),
        centroid,
        bbox,
        tags: vec![
            "populated_place".to_string(),
            "capital".to_string(),
            "tourism".to_string(),
            "urban".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode MapFeature multiple tags");
    let (decoded, consumed): (MapFeature, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MapFeature multiple tags");
    assert_eq!(val.id, decoded.id);
    assert_eq!(val.tags.len(), decoded.tags.len());
    assert_eq!(val.tags, decoded.tags);
    assert_eq!(val.centroid.alt_cm, decoded.centroid.alt_cm);
    assert_eq!(consumed, bytes.len());
}

// --- Test 14: Vec<MapFeature> roundtrip ---
#[test]
fn test_vec_map_feature_roundtrip() {
    let cfg = config::standard();
    let make_feature = |id: u64, name: &str, lat: i64, lon: i64| -> MapFeature {
        MapFeature {
            id,
            name: name.to_string(),
            feature_type: "place".to_string(),
            centroid: Coordinate {
                lat_micro: lat,
                lon_micro: lon,
                alt_cm: None,
            },
            bbox: BoundingBox {
                min: Coordinate {
                    lat_micro: lat - 1000,
                    lon_micro: lon - 1000,
                    alt_cm: None,
                },
                max: Coordinate {
                    lat_micro: lat + 1000,
                    lon_micro: lon + 1000,
                    alt_cm: None,
                },
                crs: CoordinateSystem::Wgs84,
            },
            tags: vec!["place".to_string()],
        }
    };
    let val = vec![
        make_feature(10, "London", 51_507_351_i64, -127_588_i64),
        make_feature(11, "Berlin", 52_520_008_i64, 13_404_954_i64),
        make_feature(12, "Sydney", -33_868_820_i64, 151_209_296_i64),
        make_feature(13, "Cairo", 30_044_420_i64, 31_235_712_i64),
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<MapFeature>");
    let (decoded, consumed): (Vec<MapFeature>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<MapFeature>");
    assert_eq!(val.len(), decoded.len());
    for (orig, dec) in val.iter().zip(decoded.iter()) {
        assert_eq!(orig.id, dec.id);
        assert_eq!(orig.name, dec.name);
        assert_eq!(orig.centroid.lat_micro, dec.centroid.lat_micro);
        assert_eq!(orig.centroid.lon_micro, dec.centroid.lon_micro);
        assert_eq!(orig.bbox.crs, dec.bbox.crs);
    }
    assert_eq!(consumed, bytes.len());
}

// --- Test 15: consumed bytes equals encoded length for MapFeature ---
#[test]
fn test_map_feature_consumed_bytes_equals_encoded_length() {
    let cfg = config::standard();
    let val = MapFeature {
        id: 99_u64,
        name: "ConsumedBytesCheck".to_string(),
        feature_type: "test".to_string(),
        centroid: Coordinate {
            lat_micro: 0,
            lon_micro: 0,
            alt_cm: Some(0),
        },
        bbox: BoundingBox {
            min: Coordinate {
                lat_micro: -1_000_000,
                lon_micro: -1_000_000,
                alt_cm: None,
            },
            max: Coordinate {
                lat_micro: 1_000_000,
                lon_micro: 1_000_000,
                alt_cm: None,
            },
            crs: CoordinateSystem::Utm,
        },
        tags: vec!["check".to_string(), "consumed".to_string()],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode for consumed bytes check");
    let (_decoded, consumed): (MapFeature, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode for consumed bytes check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal encoded length"
    );
}

// --- Test 16: BoundingBox with big_endian config roundtrip ---
#[test]
fn test_bounding_box_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = BoundingBox {
        min: Coordinate {
            lat_micro: -34_603_722_i64, // Buenos Aires south
            lon_micro: -58_381_592_i64,
            alt_cm: None,
        },
        max: Coordinate {
            lat_micro: -34_553_722_i64,
            lon_micro: -58_331_592_i64,
            alt_cm: None,
        },
        crs: CoordinateSystem::Wgs84,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BoundingBox big_endian");
    let (decoded, consumed): (BoundingBox, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BoundingBox big_endian");
    assert_eq!(val.min.lat_micro, decoded.min.lat_micro);
    assert_eq!(val.max.lon_micro, decoded.max.lon_micro);
    assert_eq!(val.crs, decoded.crs);
    assert_eq!(consumed, bytes.len());
}

// --- Test 17: MapFeature with fixed_int_encoding config roundtrip ---
#[test]
fn test_map_feature_fixed_int_encoding_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = MapFeature {
        id: 9_000_000_000_u64,
        name: "FixedIntFeature".to_string(),
        feature_type: "road".to_string(),
        centroid: Coordinate {
            lat_micro: -22_906_847_i64, // Rio de Janeiro
            lon_micro: -43_172_897_i64,
            alt_cm: Some(2_800_i32), // ~28m elevation
        },
        bbox: BoundingBox {
            min: Coordinate {
                lat_micro: -23_100_000,
                lon_micro: -43_800_000,
                alt_cm: None,
            },
            max: Coordinate {
                lat_micro: -22_700_000,
                lon_micro: -43_000_000,
                alt_cm: None,
            },
            crs: CoordinateSystem::Wgs84,
        },
        tags: vec!["highway".to_string(), "primary".to_string()],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode MapFeature fixed_int");
    let (decoded, consumed): (MapFeature, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MapFeature fixed_int");
    assert_eq!(val.id, decoded.id);
    assert_eq!(val.centroid.lat_micro, decoded.centroid.lat_micro);
    assert_eq!(val.centroid.alt_cm, decoded.centroid.alt_cm);
    assert_eq!(consumed, bytes.len());
}

// --- Test 18: BoundingBox spanning antimeridian (lon crosses +180/-180) ---
#[test]
fn test_bounding_box_antimeridian_boundary_roundtrip() {
    let cfg = config::standard();
    // Bounding box that spans the antimeridian: min lon > max lon in signed space
    let val = BoundingBox {
        min: Coordinate {
            lat_micro: -10_000_000_i64,
            lon_micro: 179_000_000_i64, // near +179 E
            alt_cm: None,
        },
        max: Coordinate {
            lat_micro: 10_000_000_i64,
            lon_micro: -179_000_000_i64, // near -179 (= 181 E) — antimeridian crossing
            alt_cm: None,
        },
        crs: CoordinateSystem::Wgs84,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BoundingBox antimeridian");
    let (decoded, consumed): (BoundingBox, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BoundingBox antimeridian");
    assert_eq!(val.min.lon_micro, decoded.min.lon_micro);
    assert_eq!(val.max.lon_micro, decoded.max.lon_micro);
    assert_eq!(decoded.min.lon_micro, 179_000_000_i64);
    assert_eq!(decoded.max.lon_micro, -179_000_000_i64);
    assert_eq!(consumed, bytes.len());
}

// --- Test 19: Coordinate with maximum positive i64 boundary values ---
#[test]
fn test_coordinate_max_i64_boundary_roundtrip() {
    let cfg = config::standard();
    let val = Coordinate {
        lat_micro: i64::MAX,
        lon_micro: i64::MAX,
        alt_cm: Some(i32::MAX),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Coordinate i64::MAX");
    let (decoded, consumed): (Coordinate, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Coordinate i64::MAX");
    assert_eq!(val.lat_micro, decoded.lat_micro);
    assert_eq!(val.lon_micro, decoded.lon_micro);
    assert_eq!(val.alt_cm, decoded.alt_cm);
    assert_eq!(decoded.lat_micro, i64::MAX);
    assert_eq!(decoded.alt_cm, Some(i32::MAX));
    assert_eq!(consumed, bytes.len());
}

// --- Test 20: Coordinate with minimum negative i64 boundary values ---
#[test]
fn test_coordinate_min_i64_boundary_roundtrip() {
    let cfg = config::standard();
    let val = Coordinate {
        lat_micro: i64::MIN,
        lon_micro: i64::MIN,
        alt_cm: Some(i32::MIN),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Coordinate i64::MIN");
    let (decoded, consumed): (Coordinate, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Coordinate i64::MIN");
    assert_eq!(val.lat_micro, decoded.lat_micro);
    assert_eq!(val.lon_micro, decoded.lon_micro);
    assert_eq!(val.alt_cm, decoded.alt_cm);
    assert_eq!(decoded.lat_micro, i64::MIN);
    assert_eq!(decoded.alt_cm, Some(i32::MIN));
    assert_eq!(consumed, bytes.len());
}

// --- Test 21: Vec<MapFeature> large collection roundtrip ---
#[test]
fn test_large_vec_map_feature_roundtrip() {
    let cfg = config::standard();
    let features: Vec<MapFeature> = (0..30)
        .map(|i| {
            let crs = match i % 4 {
                0 => CoordinateSystem::Wgs84,
                1 => CoordinateSystem::Utm,
                2 => CoordinateSystem::WebMercator,
                _ => CoordinateSystem::LocalGrid,
            };
            let lat = -90_000_000_i64 + i as i64 * 6_000_000_i64;
            let lon = -180_000_000_i64 + i as i64 * 12_000_000_i64;
            MapFeature {
                id: i as u64,
                name: format!("Feature_{}", i),
                feature_type: "polygon".to_string(),
                centroid: Coordinate {
                    lat_micro: lat,
                    lon_micro: lon,
                    alt_cm: None,
                },
                bbox: BoundingBox {
                    min: Coordinate {
                        lat_micro: lat - 100_000,
                        lon_micro: lon - 100_000,
                        alt_cm: None,
                    },
                    max: Coordinate {
                        lat_micro: lat + 100_000,
                        lon_micro: lon + 100_000,
                        alt_cm: None,
                    },
                    crs,
                },
                tags: vec![format!("tag_{}", i % 5), "generated".to_string()],
            }
        })
        .collect();
    let bytes = encode_to_vec(&features, cfg).expect("encode large Vec<MapFeature>");
    let (decoded, consumed): (Vec<MapFeature>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large Vec<MapFeature>");
    assert_eq!(features.len(), decoded.len());
    for (orig, dec) in features.iter().zip(decoded.iter()) {
        assert_eq!(orig.id, dec.id);
        assert_eq!(orig.name, dec.name);
        assert_eq!(orig.centroid.lat_micro, dec.centroid.lat_micro);
        assert_eq!(orig.centroid.lon_micro, dec.centroid.lon_micro);
        assert_eq!(orig.tags.len(), dec.tags.len());
        assert_eq!(orig.tags, dec.tags);
    }
    assert_eq!(consumed, bytes.len());
}

// --- Test 22: MapFeature with UTM CRS, negative altitude, and unicode name ---
#[test]
fn test_map_feature_utm_negative_altitude_unicode_roundtrip() {
    let cfg = config::standard();
    let val = MapFeature {
        id: u64::MAX,
        name: "死海 (Dead Sea) — מלח ים".to_string(),
        feature_type: "水体".to_string(),
        centroid: Coordinate {
            lat_micro: 31_500_000_i64, // ~31.5N
            lon_micro: 35_500_000_i64, // ~35.5E
            alt_cm: Some(-42_650_i32), // ~-426.5m (below sea level)
        },
        bbox: BoundingBox {
            min: Coordinate {
                lat_micro: 31_000_000_i64,
                lon_micro: 35_300_000_i64,
                alt_cm: Some(-43_000_i32),
            },
            max: Coordinate {
                lat_micro: 32_000_000_i64,
                lon_micro: 35_700_000_i64,
                alt_cm: None,
            },
            crs: CoordinateSystem::Utm,
        },
        tags: vec![
            "natural=water".to_string(),
            "water=lake".to_string(),
            "salt_lake=yes".to_string(),
            "ele=-430".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode MapFeature UTM negative altitude unicode");
    let (decoded, consumed): (MapFeature, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode MapFeature UTM negative altitude unicode");
    assert_eq!(val.id, decoded.id);
    assert_eq!(val.name, decoded.name);
    assert_eq!(val.feature_type, decoded.feature_type);
    assert_eq!(val.centroid.alt_cm, decoded.centroid.alt_cm);
    assert_eq!(val.bbox.crs, decoded.bbox.crs);
    assert_eq!(val.bbox.min.alt_cm, decoded.bbox.min.alt_cm);
    assert!(decoded.bbox.max.alt_cm.is_none());
    assert_eq!(val.tags.len(), decoded.tags.len());
    assert_eq!(val.tags, decoded.tags);
    assert_eq!(consumed, bytes.len());
}
