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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};

fn tmp(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("{}_{}", name, std::process::id()))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GeoPoint {
    lat: f64,
    lon: f64,
    elevation: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BoundingBox {
    min_lat: f64,
    min_lon: f64,
    max_lat: f64,
    max_lon: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LineString {
    id: u64,
    name: String,
    points: Vec<GeoPoint>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Polygon {
    exterior: Vec<GeoPoint>,
    holes: Vec<Vec<GeoPoint>>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MapTile {
    x: u32,
    y: u32,
    zoom: u8,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum GeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GeoFeature {
    id: u64,
    name: String,
    geometry_type: GeometryType,
    coordinates: Vec<GeoPoint>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SpatialIndex {
    cell_id: u64,
    feature_ids: Vec<u64>,
}

// 1. Basic GeoPoint roundtrip
#[test]
fn test_geopoint_basic_roundtrip() {
    let point = GeoPoint {
        lat: 48.8566,
        lon: 2.3522,
        elevation: 35.0,
    };
    let bytes = encode_to_vec(&point).expect("encode GeoPoint");
    let (decoded, _): (GeoPoint, usize) = decode_from_slice(&bytes).expect("decode GeoPoint");
    assert_eq!(point, decoded);
}

// 2. GeoPoint at poles
#[test]
fn test_geopoint_poles() {
    let north_pole = GeoPoint {
        lat: 90.0,
        lon: 0.0,
        elevation: 2835.0,
    };
    let bytes = encode_to_vec(&north_pole).expect("encode north pole");
    let (decoded, _): (GeoPoint, usize) = decode_from_slice(&bytes).expect("decode north pole");
    assert_eq!(north_pole, decoded);

    let south_pole = GeoPoint {
        lat: -90.0,
        lon: 0.0,
        elevation: 2835.0,
    };
    let bytes2 = encode_to_vec(&south_pole).expect("encode south pole");
    let (decoded2, _): (GeoPoint, usize) = decode_from_slice(&bytes2).expect("decode south pole");
    assert_eq!(south_pole, decoded2);
}

// 3. BoundingBox roundtrip
#[test]
fn test_bounding_box_roundtrip() {
    let bbox = BoundingBox {
        min_lat: -33.8688,
        min_lon: 151.2093,
        max_lat: -33.7688,
        max_lon: 151.3093,
    };
    let bytes = encode_to_vec(&bbox).expect("encode BoundingBox");
    let (decoded, _): (BoundingBox, usize) = decode_from_slice(&bytes).expect("decode BoundingBox");
    assert_eq!(bbox, decoded);
}

// 4. BoundingBox file I/O
#[test]
fn test_bounding_box_file_io() {
    let bbox = BoundingBox {
        min_lat: 51.4,
        min_lon: -0.2,
        max_lat: 51.6,
        max_lon: 0.1,
    };
    let path = tmp("oxicode_bbox_28.bin");
    encode_to_file(&bbox, &path).expect("encode BoundingBox to file");
    let decoded: BoundingBox = decode_from_file(&path).expect("decode BoundingBox from file");
    assert_eq!(bbox, decoded);
    std::fs::remove_file(&path).expect("remove bbox temp file");
}

// 5. LineString roundtrip
#[test]
fn test_linestring_roundtrip() {
    let line = LineString {
        id: 1001,
        name: String::from("Rhine River"),
        points: vec![
            GeoPoint {
                lat: 47.6,
                lon: 7.6,
                elevation: 245.0,
            },
            GeoPoint {
                lat: 48.0,
                lon: 7.8,
                elevation: 200.0,
            },
            GeoPoint {
                lat: 49.0,
                lon: 8.0,
                elevation: 150.0,
            },
            GeoPoint {
                lat: 50.0,
                lon: 8.3,
                elevation: 80.0,
            },
        ],
    };
    let bytes = encode_to_vec(&line).expect("encode LineString");
    let (decoded, _): (LineString, usize) = decode_from_slice(&bytes).expect("decode LineString");
    assert_eq!(line, decoded);
}

// 6. Polygon with exterior ring
#[test]
fn test_polygon_exterior_ring() {
    let polygon = Polygon {
        exterior: vec![
            GeoPoint {
                lat: 0.0,
                lon: 0.0,
                elevation: 0.0,
            },
            GeoPoint {
                lat: 1.0,
                lon: 0.0,
                elevation: 0.0,
            },
            GeoPoint {
                lat: 1.0,
                lon: 1.0,
                elevation: 0.0,
            },
            GeoPoint {
                lat: 0.0,
                lon: 1.0,
                elevation: 0.0,
            },
            GeoPoint {
                lat: 0.0,
                lon: 0.0,
                elevation: 0.0,
            },
        ],
        holes: vec![],
    };
    let bytes = encode_to_vec(&polygon).expect("encode Polygon");
    let (decoded, _): (Polygon, usize) = decode_from_slice(&bytes).expect("decode Polygon");
    assert_eq!(polygon, decoded);
}

// 7. Polygon with holes
#[test]
fn test_polygon_with_holes() {
    let hole = vec![
        GeoPoint {
            lat: 0.2,
            lon: 0.2,
            elevation: 0.0,
        },
        GeoPoint {
            lat: 0.4,
            lon: 0.2,
            elevation: 0.0,
        },
        GeoPoint {
            lat: 0.4,
            lon: 0.4,
            elevation: 0.0,
        },
        GeoPoint {
            lat: 0.2,
            lon: 0.4,
            elevation: 0.0,
        },
        GeoPoint {
            lat: 0.2,
            lon: 0.2,
            elevation: 0.0,
        },
    ];
    let polygon = Polygon {
        exterior: vec![
            GeoPoint {
                lat: 0.0,
                lon: 0.0,
                elevation: 0.0,
            },
            GeoPoint {
                lat: 1.0,
                lon: 0.0,
                elevation: 0.0,
            },
            GeoPoint {
                lat: 1.0,
                lon: 1.0,
                elevation: 0.0,
            },
            GeoPoint {
                lat: 0.0,
                lon: 1.0,
                elevation: 0.0,
            },
            GeoPoint {
                lat: 0.0,
                lon: 0.0,
                elevation: 0.0,
            },
        ],
        holes: vec![hole],
    };
    let bytes = encode_to_vec(&polygon).expect("encode Polygon with holes");
    let (decoded, _): (Polygon, usize) =
        decode_from_slice(&bytes).expect("decode Polygon with holes");
    assert_eq!(polygon, decoded);
}

// 8. Large polygon with many vertices
#[test]
fn test_large_polygon_many_vertices() {
    let count = 1000usize;
    let exterior: Vec<GeoPoint> = (0..count)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (count as f64);
            GeoPoint {
                lat: angle.sin() * 10.0,
                lon: angle.cos() * 10.0,
                elevation: (i as f64) * 0.1,
            }
        })
        .collect();
    let polygon = Polygon {
        exterior,
        holes: vec![],
    };
    let bytes = encode_to_vec(&polygon).expect("encode large polygon");
    let (decoded, _): (Polygon, usize) = decode_from_slice(&bytes).expect("decode large polygon");
    assert_eq!(polygon, decoded);
}

// 9. MapTile roundtrip
#[test]
fn test_maptile_roundtrip() {
    let tile = MapTile {
        x: 17602,
        y: 11731,
        zoom: 15,
        data: vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a],
    };
    let bytes = encode_to_vec(&tile).expect("encode MapTile");
    let (decoded, _): (MapTile, usize) = decode_from_slice(&bytes).expect("decode MapTile");
    assert_eq!(tile, decoded);
}

// 10. MapTile file I/O with large data payload
#[test]
fn test_maptile_file_io_large_data() {
    let tile_data: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
    let tile = MapTile {
        x: 8801,
        y: 5865,
        zoom: 14,
        data: tile_data,
    };
    let path = tmp("oxicode_maptile_28.bin");
    encode_to_file(&tile, &path).expect("encode MapTile to file");
    let decoded: MapTile = decode_from_file(&path).expect("decode MapTile from file");
    assert_eq!(tile, decoded);
    std::fs::remove_file(&path).expect("remove maptile temp file");
}

// 11. GeoFeature with Point geometry
#[test]
fn test_geofeature_point_geometry() {
    let feature = GeoFeature {
        id: 42,
        name: String::from("Eiffel Tower"),
        geometry_type: GeometryType::Point,
        coordinates: vec![GeoPoint {
            lat: 48.8584,
            lon: 2.2945,
            elevation: 330.0,
        }],
    };
    let bytes = encode_to_vec(&feature).expect("encode GeoFeature Point");
    let (decoded, _): (GeoFeature, usize) =
        decode_from_slice(&bytes).expect("decode GeoFeature Point");
    assert_eq!(feature, decoded);
}

// 12. GeoFeature with Polygon geometry
#[test]
fn test_geofeature_polygon_geometry() {
    let feature = GeoFeature {
        id: 99,
        name: String::from("Central Park"),
        geometry_type: GeometryType::Polygon,
        coordinates: vec![
            GeoPoint {
                lat: 40.7641,
                lon: -73.9736,
                elevation: 10.0,
            },
            GeoPoint {
                lat: 40.8007,
                lon: -73.9581,
                elevation: 15.0,
            },
            GeoPoint {
                lat: 40.7968,
                lon: -73.9491,
                elevation: 12.0,
            },
            GeoPoint {
                lat: 40.7641,
                lon: -73.9736,
                elevation: 10.0,
            },
        ],
    };
    let bytes = encode_to_vec(&feature).expect("encode GeoFeature Polygon");
    let (decoded, _): (GeoFeature, usize) =
        decode_from_slice(&bytes).expect("decode GeoFeature Polygon");
    assert_eq!(feature, decoded);
}

// 13. GeoFeature file I/O
#[test]
fn test_geofeature_file_io() {
    let feature = GeoFeature {
        id: 7,
        name: String::from("Tokyo Tower"),
        geometry_type: GeometryType::Point,
        coordinates: vec![GeoPoint {
            lat: 35.6586,
            lon: 139.7454,
            elevation: 332.9,
        }],
    };
    let path = tmp("oxicode_geofeature_28.bin");
    encode_to_file(&feature, &path).expect("encode GeoFeature to file");
    let decoded: GeoFeature = decode_from_file(&path).expect("decode GeoFeature from file");
    assert_eq!(feature, decoded);
    std::fs::remove_file(&path).expect("remove geofeature temp file");
}

// 14. Collection of GeoFeatures
#[test]
fn test_collection_of_geofeatures() {
    let features: Vec<GeoFeature> = vec![
        GeoFeature {
            id: 1,
            name: String::from("Big Ben"),
            geometry_type: GeometryType::Point,
            coordinates: vec![GeoPoint {
                lat: 51.5007,
                lon: -0.1246,
                elevation: 96.0,
            }],
        },
        GeoFeature {
            id: 2,
            name: String::from("Thames Path"),
            geometry_type: GeometryType::LineString,
            coordinates: vec![
                GeoPoint {
                    lat: 51.5050,
                    lon: -0.1200,
                    elevation: 5.0,
                },
                GeoPoint {
                    lat: 51.5070,
                    lon: -0.1150,
                    elevation: 5.0,
                },
            ],
        },
        GeoFeature {
            id: 3,
            name: String::from("Hyde Park"),
            geometry_type: GeometryType::Polygon,
            coordinates: vec![
                GeoPoint {
                    lat: 51.5073,
                    lon: -0.1657,
                    elevation: 20.0,
                },
                GeoPoint {
                    lat: 51.5138,
                    lon: -0.1657,
                    elevation: 22.0,
                },
                GeoPoint {
                    lat: 51.5138,
                    lon: -0.1544,
                    elevation: 18.0,
                },
                GeoPoint {
                    lat: 51.5073,
                    lon: -0.1657,
                    elevation: 20.0,
                },
            ],
        },
    ];
    let bytes = encode_to_vec(&features).expect("encode Vec<GeoFeature>");
    let (decoded, _): (Vec<GeoFeature>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<GeoFeature>");
    assert_eq!(features, decoded);
}

// 15. SpatialIndex roundtrip
#[test]
fn test_spatial_index_roundtrip() {
    let index = SpatialIndex {
        cell_id: 0x4E6F72_0000_0000,
        feature_ids: vec![101, 202, 303, 404, 505],
    };
    let bytes = encode_to_vec(&index).expect("encode SpatialIndex");
    let (decoded, _): (SpatialIndex, usize) =
        decode_from_slice(&bytes).expect("decode SpatialIndex");
    assert_eq!(index, decoded);
}

// 16. SpatialIndex file I/O
#[test]
fn test_spatial_index_file_io() {
    let index = SpatialIndex {
        cell_id: 0xDEAD_BEEF_CAFE_F00D,
        feature_ids: (1000..1050).collect(),
    };
    let path = tmp("oxicode_spatialindex_28.bin");
    encode_to_file(&index, &path).expect("encode SpatialIndex to file");
    let decoded: SpatialIndex = decode_from_file(&path).expect("decode SpatialIndex from file");
    assert_eq!(index, decoded);
    std::fs::remove_file(&path).expect("remove spatial index temp file");
}

// 17. Collection of SpatialIndex entries
#[test]
fn test_spatial_index_collection() {
    let indices: Vec<SpatialIndex> = (0u64..10)
        .map(|i| SpatialIndex {
            cell_id: i * 1000,
            feature_ids: (i * 10..(i + 1) * 10).collect(),
        })
        .collect();
    let bytes = encode_to_vec(&indices).expect("encode Vec<SpatialIndex>");
    let (decoded, _): (Vec<SpatialIndex>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<SpatialIndex>");
    assert_eq!(indices, decoded);
}

// 18. Coordinate precision roundtrip (high-precision GPS)
#[test]
fn test_high_precision_coordinate_roundtrip() {
    let point = GeoPoint {
        lat: 37.774929_f64,
        lon: -122.419418_f64,
        elevation: 16.00001_f64,
    };
    let bytes = encode_to_vec(&point).expect("encode high-precision GeoPoint");
    let (decoded, _): (GeoPoint, usize) =
        decode_from_slice(&bytes).expect("decode high-precision GeoPoint");
    assert!(
        (decoded.lat - point.lat).abs() < 1e-10,
        "latitude precision lost"
    );
    assert!(
        (decoded.lon - point.lon).abs() < 1e-10,
        "longitude precision lost"
    );
    assert!(
        (decoded.elevation - point.elevation).abs() < 1e-10,
        "elevation precision lost"
    );
}

// 19. GeometryType enum variants roundtrip
#[test]
fn test_geometry_type_all_variants() {
    let variants = vec![
        GeometryType::Point,
        GeometryType::LineString,
        GeometryType::Polygon,
        GeometryType::MultiPoint,
        GeometryType::MultiLineString,
        GeometryType::MultiPolygon,
    ];
    for variant in variants {
        let bytes = encode_to_vec(&variant).expect("encode GeometryType variant");
        let (decoded, _): (GeometryType, usize) =
            decode_from_slice(&bytes).expect("decode GeometryType variant");
        assert_eq!(variant, decoded);
    }
}

// 20. Bounding box intersection data file I/O
#[test]
fn test_bounding_box_intersection_file_io() {
    let boxes: Vec<BoundingBox> = vec![
        BoundingBox {
            min_lat: 10.0,
            min_lon: 10.0,
            max_lat: 20.0,
            max_lon: 20.0,
        },
        BoundingBox {
            min_lat: 15.0,
            min_lon: 15.0,
            max_lat: 25.0,
            max_lon: 25.0,
        },
        BoundingBox {
            min_lat: 18.0,
            min_lon: 18.0,
            max_lat: 22.0,
            max_lon: 22.0,
        },
    ];
    let path = tmp("oxicode_bbox_intersect_28.bin");
    encode_to_file(&boxes, &path).expect("encode bounding boxes to file");
    let decoded: Vec<BoundingBox> =
        decode_from_file(&path).expect("decode bounding boxes from file");
    assert_eq!(boxes, decoded);
    std::fs::remove_file(&path).expect("remove bounding box intersection temp file");
}

// 21. LineString file I/O with unicode name
#[test]
fn test_linestring_file_io_unicode_name() {
    let line = LineString {
        id: 9999,
        name: String::from("黄河 Yellow River 黄河"),
        points: vec![
            GeoPoint {
                lat: 35.46,
                lon: 96.20,
                elevation: 4000.0,
            },
            GeoPoint {
                lat: 34.90,
                lon: 104.5,
                elevation: 1500.0,
            },
            GeoPoint {
                lat: 37.72,
                lon: 118.0,
                elevation: 4.0,
            },
        ],
    };
    let path = tmp("oxicode_linestring_28.bin");
    encode_to_file(&line, &path).expect("encode LineString to file");
    let decoded: LineString = decode_from_file(&path).expect("decode LineString from file");
    assert_eq!(line, decoded);
    std::fs::remove_file(&path).expect("remove linestring temp file");
}

// 22. Mixed geospatial dataset: nested structures roundtrip
#[test]
fn test_mixed_geospatial_dataset_roundtrip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct GeoDataset {
        bbox: BoundingBox,
        features: Vec<GeoFeature>,
        tiles: Vec<MapTile>,
        indices: Vec<SpatialIndex>,
    }

    let dataset = GeoDataset {
        bbox: BoundingBox {
            min_lat: -90.0,
            min_lon: -180.0,
            max_lat: 90.0,
            max_lon: 180.0,
        },
        features: vec![
            GeoFeature {
                id: 1,
                name: String::from("Null Island"),
                geometry_type: GeometryType::Point,
                coordinates: vec![GeoPoint {
                    lat: 0.0,
                    lon: 0.0,
                    elevation: 0.0,
                }],
            },
            GeoFeature {
                id: 2,
                name: String::from("Equator"),
                geometry_type: GeometryType::LineString,
                coordinates: vec![
                    GeoPoint {
                        lat: 0.0,
                        lon: -180.0,
                        elevation: 0.0,
                    },
                    GeoPoint {
                        lat: 0.0,
                        lon: 0.0,
                        elevation: 0.0,
                    },
                    GeoPoint {
                        lat: 0.0,
                        lon: 180.0,
                        elevation: 0.0,
                    },
                ],
            },
        ],
        tiles: vec![
            MapTile {
                x: 0,
                y: 0,
                zoom: 0,
                data: vec![1, 2, 3],
            },
            MapTile {
                x: 1,
                y: 1,
                zoom: 1,
                data: vec![4, 5, 6],
            },
        ],
        indices: vec![
            SpatialIndex {
                cell_id: 0,
                feature_ids: vec![1],
            },
            SpatialIndex {
                cell_id: 1,
                feature_ids: vec![2],
            },
        ],
    };

    let path = tmp("oxicode_geodataset_28.bin");
    encode_to_file(&dataset, &path).expect("encode GeoDataset to file");
    let decoded: GeoDataset = decode_from_file(&path).expect("decode GeoDataset from file");
    assert_eq!(dataset, decoded);
    std::fs::remove_file(&path).expect("remove geodataset temp file");
}
