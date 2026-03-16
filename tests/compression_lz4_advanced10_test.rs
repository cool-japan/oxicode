//! LZ4 compression tests with satellite imagery / remote sensing domain theme.
//!
//! Tests cover encode/decode round-trips combined with LZ4 compress/decompress
//! across a variety of satellite imagery data structures and edge cases.

#![cfg(feature = "compression-lz4")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum BandType {
    Red,
    Green,
    Blue,
    NIR,
    SWIR,
    Thermal,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct PixelValue {
    band: BandType,
    value: f32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ImageTile {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    pixels: Vec<PixelValue>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum SatelliteId {
    Sentinel1,
    Sentinel2,
    Landsat8,
    Landsat9,
    MODIS,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct SceneMetadata {
    satellite: SatelliteId,
    tile: ImageTile,
    cloud_coverage: f32,
    acquisition_date: u64,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn make_pixel(band: BandType, value: f32) -> PixelValue {
    PixelValue { band, value }
}

fn make_tile(x: u32, y: u32, width: u32, height: u32, pixels: Vec<PixelValue>) -> ImageTile {
    ImageTile {
        x,
        y,
        width,
        height,
        pixels,
    }
}

// ---------------------------------------------------------------------------
// Test 1: compress/decompress roundtrip for a small PixelValue struct
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_pixel_value_small_struct_roundtrip() {
    let pv = make_pixel(BandType::Red, 0.72_f32);
    let encoded = encode_to_vec(&pv).expect("encode PixelValue failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress PixelValue failed");
    let decompressed = decompress(&compressed).expect("decompress PixelValue failed");
    let (decoded, _): (PixelValue, usize) =
        decode_from_slice(&decompressed).expect("decode PixelValue failed");
    assert_eq!(pv, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: compress/decompress for large pixel arrays (1000+ elements)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_large_pixel_array_roundtrip() {
    let pixels: Vec<PixelValue> = (0..1200)
        .map(|i| {
            let band = match i % 6 {
                0 => BandType::Red,
                1 => BandType::Green,
                2 => BandType::Blue,
                3 => BandType::NIR,
                4 => BandType::SWIR,
                _ => BandType::Thermal,
            };
            make_pixel(band, (i as f32) * 0.001_f32)
        })
        .collect();

    let tile = make_tile(0, 0, 40, 30, pixels.clone());
    let encoded = encode_to_vec(&tile).expect("encode large tile failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large tile failed");
    let decompressed = decompress(&compressed).expect("decompress large tile failed");
    let (decoded, _): (ImageTile, usize) =
        decode_from_slice(&decompressed).expect("decode large tile failed");
    assert_eq!(tile, decoded);
    assert_eq!(decoded.pixels.len(), 1200);
}

// ---------------------------------------------------------------------------
// Test 3: compressed size < original for repetitive pixel data
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_repetitive_pixel_data_compresses_smaller() {
    // All pixels identical — highly compressible
    let pixels: Vec<PixelValue> = (0..2000)
        .map(|_| make_pixel(BandType::NIR, 0.5_f32))
        .collect();
    let tile = make_tile(0, 0, 50, 40, pixels);
    let encoded = encode_to_vec(&tile).expect("encode repetitive tile failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress repetitive tile failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be smaller than encoded ({} bytes) for repetitive data",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 4: compress empty pixel vec
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_empty_pixel_vec_roundtrip() {
    let tile = make_tile(0, 0, 0, 0, vec![]);
    let encoded = encode_to_vec(&tile).expect("encode empty tile failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress empty tile failed");
    let decompressed = decompress(&compressed).expect("decompress empty tile failed");
    let (decoded, _): (ImageTile, usize) =
        decode_from_slice(&decompressed).expect("decode empty tile failed");
    assert_eq!(tile, decoded);
    assert!(decoded.pixels.is_empty());
}

// ---------------------------------------------------------------------------
// Test 5: decompress after corrupt data returns error (flip bytes after index 4)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_corrupt_data_after_header_returns_error() {
    let pv = make_pixel(BandType::Thermal, 300.5_f32);
    let encoded = encode_to_vec(&pv).expect("encode for corruption test failed");
    let mut compressed =
        compress(&encoded, Compression::Lz4).expect("compress for corruption test failed");

    // Flip bytes after header (index 4 onwards) to corrupt the LZ4 payload
    for byte in compressed.iter_mut().skip(5) {
        *byte = byte.wrapping_add(0xFF);
    }

    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress should fail on corrupted LZ4 payload"
    );
}

// ---------------------------------------------------------------------------
// Test 6: multiple band types roundtrip through LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_all_band_types_roundtrip() {
    let bands = vec![
        BandType::Red,
        BandType::Green,
        BandType::Blue,
        BandType::NIR,
        BandType::SWIR,
        BandType::Thermal,
    ];

    for band in bands {
        let pv = make_pixel(band.clone(), 0.42_f32);
        let encoded = encode_to_vec(&pv).expect("encode band pixel failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress band pixel failed");
        let decompressed = decompress(&compressed).expect("decompress band pixel failed");
        let (decoded, _): (PixelValue, usize) =
            decode_from_slice(&decompressed).expect("decode band pixel failed");
        assert_eq!(pv, decoded, "roundtrip failed for band {:?}", band);
    }
}

// ---------------------------------------------------------------------------
// Test 7: scene metadata roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_scene_metadata_roundtrip() {
    let pixels: Vec<PixelValue> = (0..256)
        .map(|i| make_pixel(BandType::Green, i as f32 / 255.0_f32))
        .collect();
    let scene = SceneMetadata {
        satellite: SatelliteId::Sentinel2,
        tile: make_tile(100, 200, 16, 16, pixels),
        cloud_coverage: 12.5_f32,
        acquisition_date: 1_700_000_000_u64,
    };

    let encoded = encode_to_vec(&scene).expect("encode scene metadata failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress scene metadata failed");
    let decompressed = decompress(&compressed).expect("decompress scene metadata failed");
    let (decoded, _): (SceneMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode scene metadata failed");
    assert_eq!(scene, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: custom satellite id roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_custom_satellite_id_roundtrip() {
    let sat_id = SatelliteId::Custom("PlanetScope-PS2".to_string());
    let tile = make_tile(0, 0, 4, 4, vec![make_pixel(BandType::Red, 0.1_f32)]);
    let scene = SceneMetadata {
        satellite: sat_id,
        tile,
        cloud_coverage: 0.0_f32,
        acquisition_date: 1_710_000_000_u64,
    };

    let encoded = encode_to_vec(&scene).expect("encode custom sat scene failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress custom sat scene failed");
    let decompressed = decompress(&compressed).expect("decompress custom sat scene failed");
    let (decoded, _): (SceneMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode custom sat scene failed");
    assert_eq!(scene, decoded);
    assert_eq!(
        decoded.satellite,
        SatelliteId::Custom("PlanetScope-PS2".to_string())
    );
}

// ---------------------------------------------------------------------------
// Test 9: vec of image tiles roundtrip with compression
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_vec_of_image_tiles_roundtrip() {
    let tiles: Vec<ImageTile> = (0..10_u32)
        .map(|i| {
            let pixels = (0..100)
                .map(|j| make_pixel(BandType::Blue, (i * 100 + j) as f32 * 0.001_f32))
                .collect();
            make_tile(i * 10, 0, 10, 10, pixels)
        })
        .collect();

    let encoded = encode_to_vec(&tiles).expect("encode vec of tiles failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress vec of tiles failed");
    let decompressed = decompress(&compressed).expect("decompress vec of tiles failed");
    let (decoded, _): (Vec<ImageTile>, usize) =
        decode_from_slice(&decompressed).expect("decode vec of tiles failed");
    assert_eq!(tiles, decoded);
    assert_eq!(decoded.len(), 10);
}

// ---------------------------------------------------------------------------
// Test 10: idempotent decompression — compressing already-encoded data twice
//          and decompressing twice returns original
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_double_compress_decompress_idempotent() {
    let pv = make_pixel(BandType::SWIR, 1.5_f32);
    let encoded = encode_to_vec(&pv).expect("encode for double compress failed");

    let compressed_once = compress(&encoded, Compression::Lz4).expect("first compress failed");
    let compressed_twice =
        compress(&compressed_once, Compression::Lz4).expect("second compress failed");

    let decompressed_once = decompress(&compressed_twice).expect("first decompress failed");
    let decompressed_twice = decompress(&decompressed_once).expect("second decompress failed");

    let (decoded, _): (PixelValue, usize) =
        decode_from_slice(&decompressed_twice).expect("decode after double compress failed");
    assert_eq!(pv, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: different tile sizes (1x1 through 32x32) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_different_tile_sizes_roundtrip() {
    for side in [1_u32, 2, 4, 8, 16, 32] {
        let count = (side * side) as usize;
        let pixels: Vec<PixelValue> = (0..count)
            .map(|i| make_pixel(BandType::NIR, i as f32 * 0.01_f32))
            .collect();
        let tile = make_tile(0, 0, side, side, pixels);
        let encoded = encode_to_vec(&tile).expect("encode tile size failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress tile size failed");
        let decompressed = decompress(&compressed).expect("decompress tile size failed");
        let (decoded, _): (ImageTile, usize) =
            decode_from_slice(&decompressed).expect("decode tile size failed");
        assert_eq!(tile, decoded, "roundtrip failed for {}x{} tile", side, side);
    }
}

// ---------------------------------------------------------------------------
// Test 12: high cloud coverage values (100 %, NaN-adjacent boundary)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_high_cloud_coverage_scene_roundtrip() {
    let scene = SceneMetadata {
        satellite: SatelliteId::MODIS,
        tile: make_tile(0, 0, 8, 8, vec![make_pixel(BandType::Thermal, 280.0_f32)]),
        cloud_coverage: 99.9_f32,
        acquisition_date: 1_715_000_000_u64,
    };

    let encoded = encode_to_vec(&scene).expect("encode high cloud scene failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress high cloud scene failed");
    let decompressed = decompress(&compressed).expect("decompress high cloud scene failed");
    let (decoded, _): (SceneMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode high cloud scene failed");
    assert!((decoded.cloud_coverage - 99.9_f32).abs() < 1e-4_f32);
    assert_eq!(decoded.satellite, SatelliteId::MODIS);
}

// ---------------------------------------------------------------------------
// Test 13: zero-pixel image tile
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_zero_pixel_image_tile_roundtrip() {
    let tile = make_tile(512, 512, 0, 0, vec![]);
    let encoded = encode_to_vec(&tile).expect("encode zero-pixel tile failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress zero-pixel tile failed");
    let decompressed = decompress(&compressed).expect("decompress zero-pixel tile failed");
    let (decoded, _): (ImageTile, usize) =
        decode_from_slice(&decompressed).expect("decode zero-pixel tile failed");
    assert_eq!(tile, decoded);
    assert!(decoded.pixels.is_empty());
    assert_eq!(decoded.x, 512);
    assert_eq!(decoded.y, 512);
}

// ---------------------------------------------------------------------------
// Test 14: single pixel image tile
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_single_pixel_image_tile_roundtrip() {
    let tile = make_tile(
        1023,
        1023,
        1,
        1,
        vec![make_pixel(BandType::Green, 0.333_f32)],
    );
    let encoded = encode_to_vec(&tile).expect("encode single-pixel tile failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress single-pixel tile failed");
    let decompressed = decompress(&compressed).expect("decompress single-pixel tile failed");
    let (decoded, _): (ImageTile, usize) =
        decode_from_slice(&decompressed).expect("decode single-pixel tile failed");
    assert_eq!(tile, decoded);
    assert_eq!(decoded.pixels.len(), 1);
    assert!((decoded.pixels[0].value - 0.333_f32).abs() < 1e-6_f32);
}

// ---------------------------------------------------------------------------
// Test 15: Landsat8 and Landsat9 satellite id roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_landsat8_and_landsat9_satellite_ids_roundtrip() {
    for sat in [SatelliteId::Landsat8, SatelliteId::Landsat9] {
        let scene = SceneMetadata {
            satellite: sat.clone(),
            tile: make_tile(0, 0, 2, 2, vec![make_pixel(BandType::Red, 0.25_f32)]),
            cloud_coverage: 5.0_f32,
            acquisition_date: 1_720_000_000_u64,
        };
        let encoded = encode_to_vec(&scene).expect("encode landsat scene failed");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("compress landsat scene failed");
        let decompressed = decompress(&compressed).expect("decompress landsat scene failed");
        let (decoded, _): (SceneMetadata, usize) =
            decode_from_slice(&decompressed).expect("decode landsat scene failed");
        assert_eq!(scene, decoded, "roundtrip failed for satellite {:?}", sat);
    }
}

// ---------------------------------------------------------------------------
// Test 16: multispectral scene — tile with all 6 band types represented
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_multispectral_scene_all_bands_roundtrip() {
    let pixels = vec![
        make_pixel(BandType::Red, 0.10_f32),
        make_pixel(BandType::Green, 0.20_f32),
        make_pixel(BandType::Blue, 0.30_f32),
        make_pixel(BandType::NIR, 0.45_f32),
        make_pixel(BandType::SWIR, 0.55_f32),
        make_pixel(BandType::Thermal, 295.0_f32),
    ];
    let tile = make_tile(0, 0, 6, 1, pixels);
    let scene = SceneMetadata {
        satellite: SatelliteId::Sentinel2,
        tile,
        cloud_coverage: 2.3_f32,
        acquisition_date: 1_725_000_000_u64,
    };

    let encoded = encode_to_vec(&scene).expect("encode multispectral scene failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress multispectral scene failed");
    let decompressed = decompress(&compressed).expect("decompress multispectral scene failed");
    let (decoded, _): (SceneMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode multispectral scene failed");
    assert_eq!(scene, decoded);
    assert_eq!(decoded.tile.pixels.len(), 6);
}

// ---------------------------------------------------------------------------
// Test 17: large scene with Sentinel1 and 5000 thermal pixels
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_large_sentinel1_thermal_scene_roundtrip() {
    let pixels: Vec<PixelValue> = (0..5000_u32)
        .map(|i| make_pixel(BandType::Thermal, 200.0_f32 + (i % 100) as f32 * 0.5_f32))
        .collect();
    let scene = SceneMetadata {
        satellite: SatelliteId::Sentinel1,
        tile: make_tile(0, 0, 100, 50, pixels),
        cloud_coverage: 0.0_f32,
        acquisition_date: 1_730_000_000_u64,
    };

    let encoded = encode_to_vec(&scene).expect("encode large sentinel1 scene failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress large sentinel1 scene failed");
    let decompressed = decompress(&compressed).expect("decompress large sentinel1 scene failed");
    let (decoded, _): (SceneMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode large sentinel1 scene failed");
    assert_eq!(scene.satellite, decoded.satellite);
    assert_eq!(decoded.tile.pixels.len(), 5000);
    assert!(
        compressed.len() < encoded.len(),
        "large repetitive thermal data should compress"
    );
}

// ---------------------------------------------------------------------------
// Test 18: acquisition date boundary values (u64::MIN, u64::MAX)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_acquisition_date_boundary_values_roundtrip() {
    for &date in &[u64::MIN, u64::MAX, 0_u64, 1_u64, u64::MAX - 1] {
        let scene = SceneMetadata {
            satellite: SatelliteId::MODIS,
            tile: make_tile(0, 0, 1, 1, vec![make_pixel(BandType::NIR, 0.0_f32)]),
            cloud_coverage: 0.0_f32,
            acquisition_date: date,
        };
        let encoded = encode_to_vec(&scene).expect("encode boundary date scene failed");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("compress boundary date scene failed");
        let decompressed = decompress(&compressed).expect("decompress boundary date scene failed");
        let (decoded, _): (SceneMetadata, usize) =
            decode_from_slice(&decompressed).expect("decode boundary date scene failed");
        assert_eq!(
            decoded.acquisition_date, date,
            "date boundary {} failed",
            date
        );
    }
}

// ---------------------------------------------------------------------------
// Test 19: cloud coverage boundary values (0.0 and 100.0)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_cloud_coverage_boundary_values_roundtrip() {
    for &cov in &[0.0_f32, 50.0_f32, 100.0_f32] {
        let scene = SceneMetadata {
            satellite: SatelliteId::Landsat8,
            tile: make_tile(0, 0, 1, 1, vec![make_pixel(BandType::Blue, 0.5_f32)]),
            cloud_coverage: cov,
            acquisition_date: 1_700_000_000_u64,
        };
        let encoded = encode_to_vec(&scene).expect("encode cloud cov scene failed");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("compress cloud cov scene failed");
        let decompressed = decompress(&compressed).expect("decompress cloud cov scene failed");
        let (decoded, _): (SceneMetadata, usize) =
            decode_from_slice(&decompressed).expect("decode cloud cov scene failed");
        assert!(
            (decoded.cloud_coverage - cov).abs() < 1e-5_f32,
            "cloud coverage {} failed",
            cov
        );
    }
}

// ---------------------------------------------------------------------------
// Test 20: custom satellite with long name string
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_custom_satellite_long_name_roundtrip() {
    let long_name = "WorldView-3-Optical-High-Resolution-Commercial-Satellite-2014".to_string();
    let sat_id = SatelliteId::Custom(long_name.clone());
    let scene = SceneMetadata {
        satellite: sat_id,
        tile: make_tile(
            0,
            0,
            4,
            4,
            (0..16)
                .map(|i| make_pixel(BandType::Red, i as f32))
                .collect(),
        ),
        cloud_coverage: 7.7_f32,
        acquisition_date: 1_735_000_000_u64,
    };

    let encoded = encode_to_vec(&scene).expect("encode long-name sat scene failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress long-name sat scene failed");
    let decompressed = decompress(&compressed).expect("decompress long-name sat scene failed");
    let (decoded, _): (SceneMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode long-name sat scene failed");
    assert_eq!(decoded.satellite, SatelliteId::Custom(long_name));
}

// ---------------------------------------------------------------------------
// Test 21: verify compressed payload is valid by checking decoded pixel count
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_decoded_pixel_count_matches_tile_dimensions() {
    let width = 20_u32;
    let height = 15_u32;
    let pixels: Vec<PixelValue> = (0..(width * height))
        .map(|i| make_pixel(BandType::SWIR, i as f32 * 0.002_f32))
        .collect();
    let tile = make_tile(0, 0, width, height, pixels);
    let encoded = encode_to_vec(&tile).expect("encode dimension tile failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress dimension tile failed");
    let decompressed = decompress(&compressed).expect("decompress dimension tile failed");
    let (decoded, _): (ImageTile, usize) =
        decode_from_slice(&decompressed).expect("decode dimension tile failed");
    assert_eq!(
        decoded.pixels.len() as u32,
        decoded.width * decoded.height,
        "pixel count should equal width * height"
    );
}

// ---------------------------------------------------------------------------
// Test 22: vec of SceneMetadata with mixed satellites compresses and round-trips
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_vec_of_scene_metadata_mixed_satellites_roundtrip() {
    let scenes: Vec<SceneMetadata> = vec![
        SceneMetadata {
            satellite: SatelliteId::Sentinel1,
            tile: make_tile(
                0,
                0,
                8,
                8,
                (0..64)
                    .map(|i| make_pixel(BandType::Red, i as f32))
                    .collect(),
            ),
            cloud_coverage: 10.0_f32,
            acquisition_date: 1_700_000_000_u64,
        },
        SceneMetadata {
            satellite: SatelliteId::Sentinel2,
            tile: make_tile(
                8,
                0,
                8,
                8,
                (0..64)
                    .map(|i| make_pixel(BandType::NIR, i as f32 * 0.5_f32))
                    .collect(),
            ),
            cloud_coverage: 25.0_f32,
            acquisition_date: 1_705_000_000_u64,
        },
        SceneMetadata {
            satellite: SatelliteId::Custom("SPOT-7".to_string()),
            tile: make_tile(
                16,
                0,
                8,
                8,
                (0..64)
                    .map(|i| make_pixel(BandType::Thermal, 270.0_f32 + i as f32))
                    .collect(),
            ),
            cloud_coverage: 0.0_f32,
            acquisition_date: 1_710_000_000_u64,
        },
    ];

    let encoded = encode_to_vec(&scenes).expect("encode vec of scenes failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress vec of scenes failed");
    let decompressed = decompress(&compressed).expect("decompress vec of scenes failed");
    let (decoded, _): (Vec<SceneMetadata>, usize) =
        decode_from_slice(&decompressed).expect("decode vec of scenes failed");
    assert_eq!(scenes, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(
        decoded[2].satellite,
        SatelliteId::Custom("SPOT-7".to_string())
    );
}
