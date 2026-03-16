#![cfg(feature = "compression-zstd")]
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
fn compress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Zstd).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct SensorRecord {
    sensor_id: u32,
    unix_time: u64,
    values: Vec<f64>,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GeoPoint {
    latitude: f64,
    longitude: f64,
    altitude: f32,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedPayload {
    name: String,
    points: Vec<GeoPoint>,
    metadata: Vec<u8>,
}

#[test]
fn test_u32_zstd_roundtrip() {
    let val: u32 = 987654321;
    let enc = encode_to_vec(&val).expect("encode u32");
    let compressed = compress_zstd(&enc).expect("compress u32");
    let decompressed = decompress_zstd(&compressed).expect("decompress u32");
    let (decoded, _): (u32, usize) = decode_from_slice(&decompressed).expect("decode u32");
    assert_eq!(val, decoded);
}

#[test]
fn test_string_zstd_roundtrip() {
    let val = String::from("Hello, Zstd compression world!");
    let enc = encode_to_vec(&val).expect("encode string");
    let compressed = compress_zstd(&enc).expect("compress string");
    let decompressed = decompress_zstd(&compressed).expect("decompress string");
    let (decoded, _): (String, usize) = decode_from_slice(&decompressed).expect("decode string");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_u8_zstd_roundtrip() {
    let val: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let enc = encode_to_vec(&val).expect("encode vec u8");
    let compressed = compress_zstd(&enc).expect("compress vec u8");
    let decompressed = decompress_zstd(&compressed).expect("decompress vec u8");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&decompressed).expect("decode vec u8");
    assert_eq!(val, decoded);
}

#[test]
fn test_large_repetitive_vec_u8_compresses_smaller() {
    let val: Vec<u8> = vec![0xABu8; 5000];
    let enc = encode_to_vec(&val).expect("encode large repetitive vec u8");
    let compressed = compress_zstd(&enc).expect("compress large repetitive vec u8");
    assert!(
        compressed.len() < enc.len(),
        "zstd compressed size {} should be smaller than uncompressed size {}",
        compressed.len(),
        enc.len()
    );
}

#[test]
fn test_struct_multiple_fields_zstd_roundtrip() {
    let val = SensorRecord {
        sensor_id: 42,
        unix_time: 1700000000000,
        values: vec![1.1, 2.2, 3.3, 4.4, 5.5],
        tags: vec![
            String::from("temperature"),
            String::from("pressure"),
            String::from("humidity"),
        ],
    };
    let enc = encode_to_vec(&val).expect("encode sensor record");
    let compressed = compress_zstd(&enc).expect("compress sensor record");
    let decompressed = decompress_zstd(&compressed).expect("decompress sensor record");
    let (decoded, _): (SensorRecord, usize) =
        decode_from_slice(&decompressed).expect("decode sensor record");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_string_zstd_roundtrip() {
    let val: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
        String::from("delta"),
        String::from("epsilon"),
    ];
    let enc = encode_to_vec(&val).expect("encode vec string");
    let compressed = compress_zstd(&enc).expect("compress vec string");
    let decompressed = decompress_zstd(&compressed).expect("decompress vec string");
    let (decoded, _): (Vec<String>, usize) =
        decode_from_slice(&decompressed).expect("decode vec string");
    assert_eq!(val, decoded);
}

#[test]
fn test_empty_vec_u8_zstd_roundtrip() {
    let val: Vec<u8> = Vec::new();
    let enc = encode_to_vec(&val).expect("encode empty vec u8");
    let compressed = compress_zstd(&enc).expect("compress empty vec u8");
    let decompressed = decompress_zstd(&compressed).expect("decompress empty vec u8");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode empty vec u8");
    assert_eq!(val, decoded);
}

#[test]
fn test_bool_zstd_roundtrip() {
    let val: bool = true;
    let enc = encode_to_vec(&val).expect("encode bool");
    let compressed = compress_zstd(&enc).expect("compress bool");
    let decompressed = decompress_zstd(&compressed).expect("decompress bool");
    let (decoded, _): (bool, usize) = decode_from_slice(&decompressed).expect("decode bool");
    assert_eq!(val, decoded);
}

#[test]
fn test_u128_zstd_roundtrip() {
    let val: u128 = 340282366920938463463374607431768211455u128;
    let enc = encode_to_vec(&val).expect("encode u128");
    let compressed = compress_zstd(&enc).expect("compress u128");
    let decompressed = decompress_zstd(&compressed).expect("decompress u128");
    let (decoded, _): (u128, usize) = decode_from_slice(&decompressed).expect("decode u128");
    assert_eq!(val, decoded);
}

#[test]
fn test_nested_struct_zstd_roundtrip() {
    let val = NestedPayload {
        name: String::from("mission_alpha"),
        points: vec![
            GeoPoint {
                latitude: 35.6762,
                longitude: 139.6503,
                altitude: 40.0,
                label: String::from("Tokyo"),
            },
            GeoPoint {
                latitude: 48.8566,
                longitude: 2.3522,
                altitude: 35.0,
                label: String::from("Paris"),
            },
        ],
        metadata: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let enc = encode_to_vec(&val).expect("encode nested payload");
    let compressed = compress_zstd(&enc).expect("compress nested payload");
    let decompressed = decompress_zstd(&compressed).expect("decompress nested payload");
    let (decoded, _): (NestedPayload, usize) =
        decode_from_slice(&decompressed).expect("decode nested payload");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_some_zstd_roundtrip() {
    let val: Option<String> = Some(String::from("optional value present"));
    let enc = encode_to_vec(&val).expect("encode option string some");
    let compressed = compress_zstd(&enc).expect("compress option string some");
    let decompressed = decompress_zstd(&compressed).expect("decompress option string some");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&decompressed).expect("decode option string some");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_none_zstd_roundtrip() {
    let val: Option<String> = None;
    let enc = encode_to_vec(&val).expect("encode option string none");
    let compressed = compress_zstd(&enc).expect("compress option string none");
    let decompressed = decompress_zstd(&compressed).expect("decompress option string none");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&decompressed).expect("decode option string none");
    assert_eq!(val, decoded);
}

#[test]
fn test_large_vec_u32_zstd_roundtrip() {
    let val: Vec<u32> = (0u32..1000).collect();
    let enc = encode_to_vec(&val).expect("encode large vec u32");
    let compressed = compress_zstd(&enc).expect("compress large vec u32");
    let decompressed = decompress_zstd(&compressed).expect("decompress large vec u32");
    let (decoded, _): (Vec<u32>, usize) =
        decode_from_slice(&decompressed).expect("decode large vec u32");
    assert_eq!(val, decoded);
}

#[test]
fn test_compressed_bytes_differ_from_uncompressed() {
    let val = String::from("This is a test string for Zstd compression difference check");
    let enc = encode_to_vec(&val).expect("encode string for diff check");
    let compressed = compress_zstd(&enc).expect("compress string for diff check");
    assert_ne!(
        enc, compressed,
        "compressed bytes should differ from uncompressed bytes"
    );
}

#[test]
fn test_decompress_twice_gives_same_result() {
    let val: Vec<u32> = vec![111, 222, 333, 444, 555, 666, 777, 888, 999];
    let enc = encode_to_vec(&val).expect("encode for idempotency check");
    let compressed = compress_zstd(&enc).expect("compress for idempotency check");
    let decompressed1 = decompress_zstd(&compressed).expect("decompress first time");
    let decompressed2 = decompress_zstd(&compressed).expect("decompress second time");
    assert_eq!(
        decompressed1, decompressed2,
        "decompressing the same bytes twice must yield identical results"
    );
}

#[test]
fn test_invalid_bytes_fail_to_decompress() {
    let bad_data = b"this_is_not_valid_zstd_compressed_data_at_all";
    let result = decompress_zstd(bad_data);
    assert!(
        result.is_err(),
        "decompress_zstd on invalid data should return an error"
    );
}

#[test]
fn test_unicode_string_zstd_roundtrip() {
    let val = String::from("日本語テスト: 圧縮と展開 — Ångström Ñoño");
    let enc = encode_to_vec(&val).expect("encode unicode string");
    let compressed = compress_zstd(&enc).expect("compress unicode string");
    let decompressed = decompress_zstd(&compressed).expect("decompress unicode string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&decompressed).expect("decode unicode string");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_vec_u8_zstd_roundtrip() {
    let val: Vec<Vec<u8>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6, 7],
        vec![],
        vec![8, 9],
        vec![10, 11, 12, 13, 14],
    ];
    let enc = encode_to_vec(&val).expect("encode vec vec u8");
    let compressed = compress_zstd(&enc).expect("compress vec vec u8");
    let decompressed = decompress_zstd(&compressed).expect("decompress vec vec u8");
    let (decoded, _): (Vec<Vec<u8>>, usize) =
        decode_from_slice(&decompressed).expect("decode vec vec u8");
    assert_eq!(val, decoded);
}

#[test]
fn test_large_repetitive_vec_u32_compresses_smaller() {
    let val: Vec<u32> = vec![0xDEADBEEFu32; 1000];
    let enc = encode_to_vec(&val).expect("encode large repetitive vec u32");
    let compressed = compress_zstd(&enc).expect("compress large repetitive vec u32");
    assert!(
        compressed.len() < enc.len(),
        "zstd compressed size {} should be smaller than uncompressed size {} for repetitive u32",
        compressed.len(),
        enc.len()
    );
}

#[test]
fn test_decompressed_matches_original_encoded_bytes() {
    let val = SensorRecord {
        sensor_id: 7,
        unix_time: 1234567890,
        values: vec![3.14159, 2.71828, 1.41421],
        tags: vec![
            String::from("pi"),
            String::from("euler"),
            String::from("sqrt2"),
        ],
    };
    let enc = encode_to_vec(&val).expect("encode for content check");
    let compressed = compress_zstd(&enc).expect("compress for content check");
    let decompressed = decompress_zstd(&compressed).expect("decompress for content check");
    assert_eq!(
        enc, decompressed,
        "decompressed bytes must exactly match the original encoded bytes"
    );
}

#[test]
fn test_compress_same_data_twice_deterministic() {
    let val: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let enc = encode_to_vec(&val).expect("encode for determinism test");
    let compressed1 = compress_zstd(&enc).expect("compress first time");
    let compressed2 = compress_zstd(&enc).expect("compress second time");
    assert_eq!(
        compressed1, compressed2,
        "compressing the same data twice with zstd should yield identical bytes"
    );
}

#[test]
fn test_vec_100_identical_strings_compresses_smaller() {
    let val: Vec<String> = (0..100)
        .map(|_| String::from("repeated_zstd_string_value"))
        .collect();
    let enc = encode_to_vec(&val).expect("encode vec of 100 identical strings");
    let compressed = compress_zstd(&enc).expect("compress vec of 100 identical strings");
    assert!(
        compressed.len() < enc.len(),
        "zstd compressed size {} should be much smaller than uncompressed size {} for 100 identical strings",
        compressed.len(),
        enc.len()
    );
}
