#![cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]
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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SatelliteBand {
    Blue,
    Green,
    Red,
    NIR,
    SWIR,
    TIR,
    Pan,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CloudCover {
    Clear,
    PartlyCloudy,
    Cloudy,
    Overcast,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ImageResolution {
    VeryHigh,
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProcessingLevel {
    L0Raw,
    L1Radiometric,
    L2Atmospheric,
    L3Analysis,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SceneMetadata {
    scene_id: u64,
    satellite_id: u32,
    acquisition_time: u64,
    cloud_cover: CloudCover,
    resolution: ImageResolution,
    level: ProcessingLevel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectralBand {
    scene_id: u64,
    band: SatelliteBand,
    min_val: u16,
    max_val: u16,
    mean_x100: u32,
    stddev_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroundControlPoint {
    gcp_id: u32,
    lat_x1e8: i64,
    lon_x1e8: i64,
    elevation_m: f32,
    pixel_x: u32,
    pixel_y: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NdviTile {
    tile_id: u64,
    x: u16,
    y: u16,
    zoom: u8,
    values: Vec<i16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AtmosphericCorrection {
    scene_id: u64,
    aerosol_optical_depth_x1000: u32,
    water_vapor_mm_x100: u32,
    ozone_du_x10: u16,
    method: String,
}

// Test 1: SceneMetadata roundtrip with LZ4
#[test]
fn test_scene_metadata_roundtrip_lz4() {
    let scene = SceneMetadata {
        scene_id: 100200300400,
        satellite_id: 7,
        acquisition_time: 1737000000,
        cloud_cover: CloudCover::PartlyCloudy,
        resolution: ImageResolution::High,
        level: ProcessingLevel::L2Atmospheric,
    };
    let encoded = encode_to_vec(&scene).expect("encode SceneMetadata");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress SceneMetadata");
    let decompressed = decompress(&compressed).expect("lz4 decompress SceneMetadata");
    let (decoded, _) =
        decode_from_slice::<SceneMetadata>(&decompressed).expect("decode SceneMetadata");
    assert_eq!(scene, decoded);
}

// Test 2: SceneMetadata roundtrip with Zstd
#[test]
fn test_scene_metadata_roundtrip_zstd() {
    let scene = SceneMetadata {
        scene_id: 999888777666,
        satellite_id: 3,
        acquisition_time: 1738500000,
        cloud_cover: CloudCover::Clear,
        resolution: ImageResolution::VeryHigh,
        level: ProcessingLevel::L3Analysis,
    };
    let encoded = encode_to_vec(&scene).expect("encode SceneMetadata zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress SceneMetadata");
    let decompressed = decompress(&compressed).expect("zstd decompress SceneMetadata");
    let (decoded, _) =
        decode_from_slice::<SceneMetadata>(&decompressed).expect("decode SceneMetadata zstd");
    assert_eq!(scene, decoded);
}

// Test 3: SpectralBand NIR roundtrip with LZ4
#[test]
fn test_spectral_band_nir_roundtrip_lz4() {
    let band = SpectralBand {
        scene_id: 555444333222,
        band: SatelliteBand::NIR,
        min_val: 0,
        max_val: 10000,
        mean_x100: 450000,
        stddev_x100: 120000,
    };
    let encoded = encode_to_vec(&band).expect("encode SpectralBand NIR");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress SpectralBand");
    let decompressed = decompress(&compressed).expect("lz4 decompress SpectralBand");
    let (decoded, _) =
        decode_from_slice::<SpectralBand>(&decompressed).expect("decode SpectralBand NIR");
    assert_eq!(band, decoded);
}

// Test 4: SpectralBand SWIR roundtrip with Zstd
#[test]
fn test_spectral_band_swir_roundtrip_zstd() {
    let band = SpectralBand {
        scene_id: 111222333444,
        band: SatelliteBand::SWIR,
        min_val: 50,
        max_val: 8500,
        mean_x100: 300000,
        stddev_x100: 95000,
    };
    let encoded = encode_to_vec(&band).expect("encode SpectralBand SWIR");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress SpectralBand");
    let decompressed = decompress(&compressed).expect("zstd decompress SpectralBand");
    let (decoded, _) =
        decode_from_slice::<SpectralBand>(&decompressed).expect("decode SpectralBand SWIR");
    assert_eq!(band, decoded);
}

// Test 5: GroundControlPoint roundtrip with LZ4
#[test]
fn test_ground_control_point_roundtrip_lz4() {
    let gcp = GroundControlPoint {
        gcp_id: 42,
        lat_x1e8: 3576543210,
        lon_x1e8: 13998765432,
        elevation_m: 312.5,
        pixel_x: 1024,
        pixel_y: 768,
    };
    let encoded = encode_to_vec(&gcp).expect("encode GroundControlPoint");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress GCP");
    let decompressed = decompress(&compressed).expect("lz4 decompress GCP");
    let (decoded, _) =
        decode_from_slice::<GroundControlPoint>(&decompressed).expect("decode GCP lz4");
    assert_eq!(gcp, decoded);
}

// Test 6: GroundControlPoint roundtrip with Zstd
#[test]
fn test_ground_control_point_roundtrip_zstd() {
    let gcp = GroundControlPoint {
        gcp_id: 99,
        lat_x1e8: -1234567890,
        lon_x1e8: 4567890123,
        elevation_m: 5600.0,
        pixel_x: 4096,
        pixel_y: 4096,
    };
    let encoded = encode_to_vec(&gcp).expect("encode GroundControlPoint zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress GCP");
    let decompressed = decompress(&compressed).expect("zstd decompress GCP");
    let (decoded, _) =
        decode_from_slice::<GroundControlPoint>(&decompressed).expect("decode GCP zstd");
    assert_eq!(gcp, decoded);
}

// Test 7: NdviTile (small, <=10 values) roundtrip with LZ4
#[test]
fn test_ndvi_tile_small_roundtrip_lz4() {
    let tile = NdviTile {
        tile_id: 9876543210,
        x: 15,
        y: 22,
        zoom: 10,
        values: vec![800, 750, 820, 790, 810, 780, 760, 830, 840, 770],
    };
    let encoded = encode_to_vec(&tile).expect("encode NdviTile lz4");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress NdviTile");
    let decompressed = decompress(&compressed).expect("lz4 decompress NdviTile");
    let (decoded, _) = decode_from_slice::<NdviTile>(&decompressed).expect("decode NdviTile lz4");
    assert_eq!(tile, decoded);
}

// Test 8: NdviTile (small, <=10 values) roundtrip with Zstd
#[test]
fn test_ndvi_tile_small_roundtrip_zstd() {
    let tile = NdviTile {
        tile_id: 1234567890,
        x: 8,
        y: 5,
        zoom: 12,
        values: vec![-200, -100, 0, 100, 200, 300, 400, 500, 600, 700],
    };
    let encoded = encode_to_vec(&tile).expect("encode NdviTile zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress NdviTile");
    let decompressed = decompress(&compressed).expect("zstd decompress NdviTile");
    let (decoded, _) = decode_from_slice::<NdviTile>(&decompressed).expect("decode NdviTile zstd");
    assert_eq!(tile, decoded);
}

// Test 9: AtmosphericCorrection roundtrip with LZ4
#[test]
fn test_atmospheric_correction_roundtrip_lz4() {
    let atm = AtmosphericCorrection {
        scene_id: 777666555444,
        aerosol_optical_depth_x1000: 87,
        water_vapor_mm_x100: 1520,
        ozone_du_x10: 3210,
        method: "6SV2".to_string(),
    };
    let encoded = encode_to_vec(&atm).expect("encode AtmosphericCorrection lz4");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress AtmCorrection");
    let decompressed = decompress(&compressed).expect("lz4 decompress AtmCorrection");
    let (decoded, _) = decode_from_slice::<AtmosphericCorrection>(&decompressed)
        .expect("decode AtmCorrection lz4");
    assert_eq!(atm, decoded);
}

// Test 10: AtmosphericCorrection roundtrip with Zstd
#[test]
fn test_atmospheric_correction_roundtrip_zstd() {
    let atm = AtmosphericCorrection {
        scene_id: 333222111000,
        aerosol_optical_depth_x1000: 210,
        water_vapor_mm_x100: 3300,
        ozone_du_x10: 2880,
        method: "MODTRAN5".to_string(),
    };
    let encoded = encode_to_vec(&atm).expect("encode AtmosphericCorrection zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress AtmCorrection");
    let decompressed = decompress(&compressed).expect("zstd decompress AtmCorrection");
    let (decoded, _) = decode_from_slice::<AtmosphericCorrection>(&decompressed)
        .expect("decode AtmCorrection zstd");
    assert_eq!(atm, decoded);
}

// Test 11: Large NDVI tile array (1000+ repetitive i16 values) - LZ4 compression ratio
#[test]
fn test_large_ndvi_values_lz4_compression_ratio() {
    // 1024 repetitive values simulating a near-uniform vegetation tile
    let values: Vec<i16> = (0..1024).map(|i| 750_i16 + (i % 5) as i16).collect();
    let large_ndvi_bytes = encode_to_vec(&values).expect("encode large ndvi values");
    let compressed =
        compress(&large_ndvi_bytes, Compression::Lz4).expect("lz4 compress large ndvi");
    // Repetitive data must compress to less than 90% of original
    assert!(
        compressed.len() < large_ndvi_bytes.len() * 9 / 10,
        "LZ4 should achieve >10% compression on repetitive NDVI data: original={}, compressed={}",
        large_ndvi_bytes.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("lz4 decompress large ndvi");
    let (decoded, _) = decode_from_slice::<Vec<i16>>(&decompressed).expect("decode large ndvi lz4");
    assert_eq!(values, decoded);
}

// Test 12: Large NDVI tile array (1000+ repetitive i16 values) - Zstd compression ratio
#[test]
fn test_large_ndvi_values_zstd_compression_ratio() {
    let values: Vec<i16> = (0..1200).map(|i| 600_i16 + (i % 8) as i16).collect();
    let large_ndvi_bytes = encode_to_vec(&values).expect("encode large ndvi values zstd");
    let compressed =
        compress(&large_ndvi_bytes, Compression::Zstd).expect("zstd compress large ndvi");
    assert!(
        compressed.len() < large_ndvi_bytes.len() * 9 / 10,
        "Zstd should achieve >10% compression on repetitive NDVI data: original={}, compressed={}",
        large_ndvi_bytes.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress large ndvi");
    let (decoded, _) =
        decode_from_slice::<Vec<i16>>(&decompressed).expect("decode large ndvi zstd");
    assert_eq!(values, decoded);
}

// Test 13: Large scene metadata list (1000+ scenes) - LZ4 compression ratio
#[test]
fn test_large_scene_metadata_list_lz4_compression_ratio() {
    let scenes: Vec<SceneMetadata> = (0..1000u64)
        .map(|i| SceneMetadata {
            scene_id: 1_000_000_000 + i,
            satellite_id: (i % 8) as u32,
            acquisition_time: 1_700_000_000 + i * 86400,
            cloud_cover: match i % 4 {
                0 => CloudCover::Clear,
                1 => CloudCover::PartlyCloudy,
                2 => CloudCover::Cloudy,
                _ => CloudCover::Overcast,
            },
            resolution: match i % 4 {
                0 => ImageResolution::VeryHigh,
                1 => ImageResolution::High,
                2 => ImageResolution::Medium,
                _ => ImageResolution::Low,
            },
            level: match i % 4 {
                0 => ProcessingLevel::L0Raw,
                1 => ProcessingLevel::L1Radiometric,
                2 => ProcessingLevel::L2Atmospheric,
                _ => ProcessingLevel::L3Analysis,
            },
        })
        .collect();
    let encoded = encode_to_vec(&scenes).expect("encode large scene list lz4");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large scene list");
    assert!(
        compressed.len() < encoded.len() * 9 / 10,
        "LZ4 should compress repetitive scene list: original={}, compressed={}",
        encoded.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("lz4 decompress large scene list");
    let (decoded, _) = decode_from_slice::<Vec<SceneMetadata>>(&decompressed)
        .expect("decode large scene list lz4");
    assert_eq!(scenes, decoded);
}

// Test 14: Large scene metadata list (1000+ scenes) - Zstd compression ratio
#[test]
fn test_large_scene_metadata_list_zstd_compression_ratio() {
    let scenes: Vec<SceneMetadata> = (0..1000u64)
        .map(|i| SceneMetadata {
            scene_id: 2_000_000_000 + i,
            satellite_id: (i % 6) as u32,
            acquisition_time: 1_710_000_000 + i * 3600,
            cloud_cover: match i % 4 {
                0 => CloudCover::Clear,
                1 => CloudCover::PartlyCloudy,
                2 => CloudCover::Cloudy,
                _ => CloudCover::Overcast,
            },
            resolution: ImageResolution::High,
            level: ProcessingLevel::L2Atmospheric,
        })
        .collect();
    let encoded = encode_to_vec(&scenes).expect("encode large scene list zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress large scene list");
    assert!(
        compressed.len() < encoded.len() * 9 / 10,
        "Zstd should compress repetitive scene list: original={}, compressed={}",
        encoded.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress large scene list");
    let (decoded, _) = decode_from_slice::<Vec<SceneMetadata>>(&decompressed)
        .expect("decode large scene list zstd");
    assert_eq!(scenes, decoded);
}

// Test 15: LZ4 and Zstd produce different compressed bytes but decode to same value
#[test]
fn test_lz4_vs_zstd_differ_but_decode_equal() {
    let gcp = GroundControlPoint {
        gcp_id: 256,
        lat_x1e8: 5123456789,
        lon_x1e8: -3456789012,
        elevation_m: 87.3,
        pixel_x: 2048,
        pixel_y: 1536,
    };
    let encoded = encode_to_vec(&gcp).expect("encode GCP for codec comparison");
    let compressed_lz4 =
        compress(&encoded, Compression::Lz4).expect("lz4 compress GCP for comparison");
    let compressed_zstd =
        compress(&encoded, Compression::Zstd).expect("zstd compress GCP for comparison");
    // Compressed representations should differ between codecs
    assert_ne!(
        compressed_lz4, compressed_zstd,
        "LZ4 and Zstd should produce different compressed bytes"
    );
    // But both must decompress to the same original encoded bytes
    let decompressed_lz4 = decompress(&compressed_lz4).expect("lz4 decompress GCP comparison");
    let decompressed_zstd = decompress(&compressed_zstd).expect("zstd decompress GCP comparison");
    assert_eq!(
        decompressed_lz4, decompressed_zstd,
        "LZ4 and Zstd must decompress to equal bytes"
    );
    let (decoded_lz4, _) =
        decode_from_slice::<GroundControlPoint>(&decompressed_lz4).expect("decode GCP from lz4");
    let (decoded_zstd, _) =
        decode_from_slice::<GroundControlPoint>(&decompressed_zstd).expect("decode GCP from zstd");
    assert_eq!(decoded_lz4, decoded_zstd);
    assert_eq!(gcp, decoded_lz4);
}

// Test 16: Empty vector of SpectralBands with LZ4
#[test]
fn test_empty_spectral_band_vec_lz4() {
    let bands: Vec<SpectralBand> = vec![];
    let encoded = encode_to_vec(&bands).expect("encode empty SpectralBand vec");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress empty band vec");
    let decompressed = decompress(&compressed).expect("lz4 decompress empty band vec");
    let (decoded, _) =
        decode_from_slice::<Vec<SpectralBand>>(&decompressed).expect("decode empty band vec");
    assert_eq!(bands, decoded);
    assert!(decoded.is_empty());
}

// Test 17: Empty vector of SpectralBands with Zstd
#[test]
fn test_empty_spectral_band_vec_zstd() {
    let bands: Vec<SpectralBand> = vec![];
    let encoded = encode_to_vec(&bands).expect("encode empty SpectralBand vec zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress empty band vec");
    let decompressed = decompress(&compressed).expect("zstd decompress empty band vec");
    let (decoded, _) =
        decode_from_slice::<Vec<SpectralBand>>(&decompressed).expect("decode empty band vec zstd");
    assert_eq!(bands, decoded);
    assert!(decoded.is_empty());
}

// Test 18: Multiple compress/decompress cycles with LZ4 (idempotency)
#[test]
fn test_multiple_compress_decompress_cycles_lz4() {
    let scene = SceneMetadata {
        scene_id: 88877766655,
        satellite_id: 1,
        acquisition_time: 1740000000,
        cloud_cover: CloudCover::Cloudy,
        resolution: ImageResolution::Medium,
        level: ProcessingLevel::L1Radiometric,
    };
    let encoded = encode_to_vec(&scene).expect("encode for multi-cycle lz4");
    // First cycle
    let c1 = compress(&encoded, Compression::Lz4).expect("lz4 cycle1 compress");
    let d1 = decompress(&c1).expect("lz4 cycle1 decompress");
    // Second cycle on already-decompressed bytes (re-compress the decoded bytes)
    let c2 = compress(&d1, Compression::Lz4).expect("lz4 cycle2 compress");
    let d2 = decompress(&c2).expect("lz4 cycle2 decompress");
    // Third cycle
    let c3 = compress(&d2, Compression::Lz4).expect("lz4 cycle3 compress");
    let d3 = decompress(&c3).expect("lz4 cycle3 decompress");
    assert_eq!(
        encoded, d3,
        "Data must survive 3 LZ4 compress/decompress cycles unchanged"
    );
    let (decoded, _) =
        decode_from_slice::<SceneMetadata>(&d3).expect("decode after multi-cycle lz4");
    assert_eq!(scene, decoded);
}

// Test 19: Multiple compress/decompress cycles with Zstd (idempotency)
#[test]
fn test_multiple_compress_decompress_cycles_zstd() {
    let atm = AtmosphericCorrection {
        scene_id: 444333222111,
        aerosol_optical_depth_x1000: 55,
        water_vapor_mm_x100: 1100,
        ozone_du_x10: 3000,
        method: "DOS1".to_string(),
    };
    let encoded = encode_to_vec(&atm).expect("encode for multi-cycle zstd");
    let c1 = compress(&encoded, Compression::Zstd).expect("zstd cycle1 compress");
    let d1 = decompress(&c1).expect("zstd cycle1 decompress");
    let c2 = compress(&d1, Compression::Zstd).expect("zstd cycle2 compress");
    let d2 = decompress(&c2).expect("zstd cycle2 decompress");
    let c3 = compress(&d2, Compression::Zstd).expect("zstd cycle3 compress");
    let d3 = decompress(&c3).expect("zstd cycle3 decompress");
    assert_eq!(
        encoded, d3,
        "Data must survive 3 Zstd compress/decompress cycles unchanged"
    );
    let (decoded, _) =
        decode_from_slice::<AtmosphericCorrection>(&d3).expect("decode after multi-cycle zstd");
    assert_eq!(atm, decoded);
}

// Test 20: Cross-codec round trip: encode → LZ4 compress → LZ4 decompress → Zstd compress → Zstd decompress → decode
#[test]
fn test_cross_codec_round_trip_lz4_then_zstd() {
    let band = SpectralBand {
        scene_id: 666555444333,
        band: SatelliteBand::TIR,
        min_val: 200,
        max_val: 9999,
        mean_x100: 650000,
        stddev_x100: 200000,
    };
    let encoded = encode_to_vec(&band).expect("encode for cross-codec roundtrip");
    let lz4_compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress cross-codec");
    let lz4_decompressed = decompress(&lz4_compressed).expect("lz4 decompress cross-codec");
    assert_eq!(
        encoded, lz4_decompressed,
        "LZ4 decompressed bytes must match original encoded"
    );
    let zstd_compressed =
        compress(&lz4_decompressed, Compression::Zstd).expect("zstd compress cross-codec");
    let zstd_decompressed = decompress(&zstd_compressed).expect("zstd decompress cross-codec");
    assert_eq!(
        encoded, zstd_decompressed,
        "Zstd decompressed bytes must match original encoded"
    );
    let (decoded, _) =
        decode_from_slice::<SpectralBand>(&zstd_decompressed).expect("decode after cross-codec");
    assert_eq!(band, decoded);
}

// Test 21: All SatelliteBand variants encode/decode via Zstd
#[test]
fn test_all_satellite_band_variants_zstd() {
    let all_bands = vec![
        SatelliteBand::Blue,
        SatelliteBand::Green,
        SatelliteBand::Red,
        SatelliteBand::NIR,
        SatelliteBand::SWIR,
        SatelliteBand::TIR,
        SatelliteBand::Pan,
    ];
    for band_variant in &all_bands {
        let spectral = SpectralBand {
            scene_id: 111000111000,
            band: band_variant.clone(),
            min_val: 10,
            max_val: 9990,
            mean_x100: 500000,
            stddev_x100: 150000,
        };
        let encoded = encode_to_vec(&spectral).expect("encode band variant");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress band variant");
        let decompressed = decompress(&compressed).expect("zstd decompress band variant");
        let (decoded, _) =
            decode_from_slice::<SpectralBand>(&decompressed).expect("decode band variant");
        assert_eq!(
            spectral, decoded,
            "Band variant {:?} failed roundtrip",
            band_variant
        );
    }
}

// Test 22: ProcessingLevel and CloudCover enum combinations with LZ4
#[test]
fn test_processing_level_cloud_cover_combinations_lz4() {
    let levels = vec![
        ProcessingLevel::L0Raw,
        ProcessingLevel::L1Radiometric,
        ProcessingLevel::L2Atmospheric,
        ProcessingLevel::L3Analysis,
    ];
    let covers = vec![
        CloudCover::Clear,
        CloudCover::PartlyCloudy,
        CloudCover::Cloudy,
        CloudCover::Overcast,
    ];
    let resolutions = vec![
        ImageResolution::VeryHigh,
        ImageResolution::High,
        ImageResolution::Medium,
        ImageResolution::Low,
    ];
    let mut scene_id: u64 = 500_000_000_000;
    for level in &levels {
        for cover in &covers {
            for resolution in &resolutions {
                let scene = SceneMetadata {
                    scene_id,
                    satellite_id: 5,
                    acquisition_time: 1_730_000_000 + scene_id % 10000,
                    cloud_cover: cover.clone(),
                    resolution: resolution.clone(),
                    level: level.clone(),
                };
                let encoded = encode_to_vec(&scene).expect("encode combination scene");
                let compressed =
                    compress(&encoded, Compression::Lz4).expect("lz4 compress combination");
                let decompressed = decompress(&compressed).expect("lz4 decompress combination");
                let (decoded, _) = decode_from_slice::<SceneMetadata>(&decompressed)
                    .expect("decode combination scene");
                assert_eq!(
                    scene, decoded,
                    "Combination ({:?}, {:?}, {:?}) failed",
                    level, cover, resolution
                );
                scene_id += 1;
            }
        }
    }
}
