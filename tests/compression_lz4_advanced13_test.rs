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
// Domain types: Geospatial / map tile compression
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TileFormat {
    PNG,
    JPEG,
    WebP,
    GeoTIFF,
    VectorMVT,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ZoomLevel {
    Z0,
    Z4,
    Z8,
    Z12,
    Z16,
    Z20,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MapTile {
    tile_x: u32,
    tile_y: u32,
    zoom: ZoomLevel,
    format: TileFormat,
    data: Vec<u8>,
    size_bytes: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TileSet {
    tileset_id: u64,
    name: String,
    tiles: Vec<MapTile>,
    total_tiles: u64,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_tile(
    tile_x: u32,
    tile_y: u32,
    zoom: ZoomLevel,
    format: TileFormat,
    payload: Vec<u8>,
) -> MapTile {
    let size = payload.len() as u64;
    MapTile {
        tile_x,
        tile_y,
        zoom,
        format,
        data: payload,
        size_bytes: size,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Basic MapTile compress / decompress roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_map_tile_compress_decompress_roundtrip() {
    let tile = make_tile(10, 20, ZoomLevel::Z8, TileFormat::PNG, vec![0xAB; 256]);

    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");

    assert_eq!(tile, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: TileSet roundtrip with multiple tiles
// ---------------------------------------------------------------------------

#[test]
fn test_tileset_roundtrip_multiple_tiles() {
    let tiles = vec![
        make_tile(0, 0, ZoomLevel::Z0, TileFormat::PNG, vec![1u8; 64]),
        make_tile(1, 1, ZoomLevel::Z4, TileFormat::JPEG, vec![2u8; 128]),
        make_tile(2, 2, ZoomLevel::Z8, TileFormat::WebP, vec![3u8; 256]),
        make_tile(3, 3, ZoomLevel::Z12, TileFormat::GeoTIFF, vec![4u8; 512]),
        make_tile(4, 4, ZoomLevel::Z16, TileFormat::VectorMVT, vec![5u8; 1024]),
    ];
    let total = tiles.len() as u64;
    let ts = TileSet {
        tileset_id: 42,
        name: "world-basemap".to_string(),
        tiles,
        total_tiles: total,
    };

    let encoded = encode_to_vec(&ts).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (TileSet, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");

    assert_eq!(ts, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: TileFormat::PNG roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tile_format_png_roundtrip() {
    let tile = make_tile(5, 5, ZoomLevel::Z4, TileFormat::PNG, vec![0xFF; 128]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.format, TileFormat::PNG);
    assert_eq!(tile, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: TileFormat::JPEG roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tile_format_jpeg_roundtrip() {
    let tile = make_tile(6, 7, ZoomLevel::Z8, TileFormat::JPEG, vec![0xAA; 200]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.format, TileFormat::JPEG);
    assert_eq!(tile, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: TileFormat::WebP roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tile_format_webp_roundtrip() {
    let tile = make_tile(8, 9, ZoomLevel::Z12, TileFormat::WebP, vec![0x55; 300]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.format, TileFormat::WebP);
    assert_eq!(tile, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: TileFormat::GeoTIFF roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tile_format_geotiff_roundtrip() {
    let tile = make_tile(10, 11, ZoomLevel::Z16, TileFormat::GeoTIFF, vec![0x33; 400]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.format, TileFormat::GeoTIFF);
    assert_eq!(tile, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: TileFormat::VectorMVT roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tile_format_vector_mvt_roundtrip() {
    let tile = make_tile(
        12,
        13,
        ZoomLevel::Z20,
        TileFormat::VectorMVT,
        vec![0x77; 512],
    );
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.format, TileFormat::VectorMVT);
    assert_eq!(tile, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: ZoomLevel::Z0 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zoom_level_z0_roundtrip() {
    let tile = make_tile(0, 0, ZoomLevel::Z0, TileFormat::PNG, vec![0x11; 64]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.zoom, ZoomLevel::Z0);
}

// ---------------------------------------------------------------------------
// Test 9: ZoomLevel::Z4 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zoom_level_z4_roundtrip() {
    let tile = make_tile(1, 2, ZoomLevel::Z4, TileFormat::JPEG, vec![0x22; 64]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.zoom, ZoomLevel::Z4);
}

// ---------------------------------------------------------------------------
// Test 10: ZoomLevel::Z12 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zoom_level_z12_roundtrip() {
    let tile = make_tile(100, 200, ZoomLevel::Z12, TileFormat::WebP, vec![0x44; 64]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.zoom, ZoomLevel::Z12);
}

// ---------------------------------------------------------------------------
// Test 11: ZoomLevel::Z16 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zoom_level_z16_roundtrip() {
    let tile = make_tile(
        1024,
        2048,
        ZoomLevel::Z16,
        TileFormat::GeoTIFF,
        vec![0x66; 64],
    );
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.zoom, ZoomLevel::Z16);
}

// ---------------------------------------------------------------------------
// Test 12: ZoomLevel::Z20 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zoom_level_z20_roundtrip() {
    let tile = make_tile(
        524288,
        786432,
        ZoomLevel::Z20,
        TileFormat::VectorMVT,
        vec![0x88; 64],
    );
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(decoded.zoom, ZoomLevel::Z20);
}

// ---------------------------------------------------------------------------
// Test 13: Large repetitive tile (4096 bytes) — verify compression ratio
// ---------------------------------------------------------------------------

#[test]
fn test_large_repetitive_tile_compression_ratio() {
    // 4096 bytes of a repeating 4-byte pattern — highly compressible raster data
    let pattern: Vec<u8> = std::iter::repeat([0xDE, 0xAD, 0xBE, 0xEF].iter().copied())
        .flatten()
        .take(4096)
        .collect();
    let tile = make_tile(0, 0, ZoomLevel::Z8, TileFormat::PNG, pattern);

    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

    assert!(
        compressed.len() < encoded.len(),
        "LZ4 compressed ({} bytes) should be smaller than encoded ({} bytes) for repetitive tile data",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 14: Empty tile data roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_empty_tile_data_roundtrip() {
    let tile = make_tile(0, 0, ZoomLevel::Z0, TileFormat::PNG, vec![]);

    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");

    assert_eq!(tile, decoded);
    assert_eq!(decoded.data.len(), 0);
    assert_eq!(decoded.size_bytes, 0);
}

// ---------------------------------------------------------------------------
// Test 15: Truncated compressed bytes return an error
// ---------------------------------------------------------------------------

#[test]
fn test_truncated_compressed_data_returns_error() {
    let tile = make_tile(7, 7, ZoomLevel::Z8, TileFormat::JPEG, vec![0xCC; 512]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

    // Truncate to half — the frame is now malformed
    let half = compressed.len() / 2;
    let truncated = compressed[..half].to_vec();

    let result = decompress(&truncated);
    assert!(
        result.is_err(),
        "decompress() must return Err for truncated LZ4 data, but got Ok"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Single tile roundtrip with all fields verified
// ---------------------------------------------------------------------------

#[test]
fn test_single_tile_all_fields_roundtrip() {
    let payload: Vec<u8> = (0u8..=255).cycle().take(512).collect();
    let tile = MapTile {
        tile_x: 12345,
        tile_y: 67890,
        zoom: ZoomLevel::Z16,
        format: TileFormat::VectorMVT,
        data: payload.clone(),
        size_bytes: payload.len() as u64,
    };

    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, consumed): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");

    assert_eq!(tile, decoded);
    assert_eq!(decoded.tile_x, 12345);
    assert_eq!(decoded.tile_y, 67890);
    assert_eq!(decoded.zoom, ZoomLevel::Z16);
    assert_eq!(decoded.format, TileFormat::VectorMVT);
    assert_eq!(decoded.size_bytes, 512);
    assert!(consumed > 0);
}

// ---------------------------------------------------------------------------
// Test 17: encode_to_vec + compress + decompress + decode_from_slice chain
// ---------------------------------------------------------------------------

#[test]
fn test_full_pipeline_encode_compress_decompress_decode() {
    let ts = TileSet {
        tileset_id: 999,
        name: "pipeline-test".to_string(),
        tiles: vec![
            make_tile(0, 0, ZoomLevel::Z4, TileFormat::PNG, vec![0xAA; 128]),
            make_tile(1, 0, ZoomLevel::Z4, TileFormat::PNG, vec![0xBB; 128]),
            make_tile(0, 1, ZoomLevel::Z4, TileFormat::JPEG, vec![0xCC; 256]),
        ],
        total_tiles: 3,
    };

    // Step 1: encode
    let raw_bytes = encode_to_vec(&ts).expect("encode_to_vec failed");
    // Step 2: compress
    let compressed = compress(&raw_bytes, Compression::Lz4).expect("compress failed");
    // Step 3: decompress
    let decompressed = decompress(&compressed).expect("decompress failed");
    // Step 4: decode
    let (result, _): (TileSet, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");

    assert_eq!(ts, result);
    assert_eq!(result.total_tiles, 3);
    assert_eq!(result.tiles.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 18: Large TileSet with many tiles roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_tileset_many_tiles_roundtrip() {
    let formats = [
        TileFormat::PNG,
        TileFormat::JPEG,
        TileFormat::WebP,
        TileFormat::GeoTIFF,
        TileFormat::VectorMVT,
    ];
    let zooms = [
        ZoomLevel::Z0,
        ZoomLevel::Z4,
        ZoomLevel::Z8,
        ZoomLevel::Z12,
        ZoomLevel::Z16,
    ];

    let tiles: Vec<MapTile> = (0u32..50)
        .map(|i| {
            let fmt = formats[(i as usize) % formats.len()].clone();
            let zoom = zooms[(i as usize) % zooms.len()].clone();
            make_tile(i, i * 2, zoom, fmt, vec![i as u8; 64])
        })
        .collect();

    let total = tiles.len() as u64;
    let ts = TileSet {
        tileset_id: 1000,
        name: "large-tileset".to_string(),
        tiles,
        total_tiles: total,
    };

    let encoded = encode_to_vec(&ts).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (TileSet, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");

    assert_eq!(ts, decoded);
    assert_eq!(decoded.tiles.len(), 50);
}

// ---------------------------------------------------------------------------
// Test 19: MapTile with 1000+ byte repetitive pattern verifies compression
// ---------------------------------------------------------------------------

#[test]
fn test_maptile_repetitive_1000_bytes_compresses_smaller() {
    // 1200 bytes of uniform data (maximally compressible)
    let uniform_data = vec![0x7Fu8; 1200];
    let tile = make_tile(50, 75, ZoomLevel::Z12, TileFormat::GeoTIFF, uniform_data);

    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress uniform 1200-byte tile payload: compressed={} encoded={}",
        compressed.len(),
        encoded.len()
    );

    // Also verify roundtrip integrity
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MapTile, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    assert_eq!(tile, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: TileSet with empty tiles list roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tileset_empty_tiles_list_roundtrip() {
    let ts = TileSet {
        tileset_id: 0,
        name: "empty-set".to_string(),
        tiles: vec![],
        total_tiles: 0,
    };

    let encoded = encode_to_vec(&ts).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (TileSet, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");

    assert_eq!(ts, decoded);
    assert_eq!(decoded.tiles.len(), 0);
    assert_eq!(decoded.total_tiles, 0);
}

// ---------------------------------------------------------------------------
// Test 21: Compressed tile data is different bytes from original (not a no-op)
// ---------------------------------------------------------------------------

#[test]
fn test_compressed_output_differs_from_input() {
    // Use sufficiently large repetitive data so LZ4 actually transforms the bytes
    let tile = make_tile(3, 3, ZoomLevel::Z8, TileFormat::PNG, vec![0xEE; 2048]);
    let encoded = encode_to_vec(&tile).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

    // The compressed buffer must not be bit-for-bit identical to encoded
    assert_ne!(
        encoded, compressed,
        "compress() must produce output different from its input"
    );
}

// ---------------------------------------------------------------------------
// Test 22: TileSet with unicode tile name roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tileset_unicode_name_roundtrip() {
    let ts = TileSet {
        tileset_id: 7777,
        name: "地図タイル・世界地図 🌍".to_string(),
        tiles: vec![make_tile(
            0,
            0,
            ZoomLevel::Z0,
            TileFormat::PNG,
            vec![0x01; 32],
        )],
        total_tiles: 1,
    };

    let encoded = encode_to_vec(&ts).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (TileSet, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");

    assert_eq!(ts, decoded);
    assert_eq!(decoded.name, "地図タイル・世界地図 🌍");
    assert_eq!(decoded.tileset_id, 7777);
}
