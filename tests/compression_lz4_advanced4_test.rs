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
fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Telemetry {
    device_id: u32,
    timestamp: u64,
    readings: Vec<f32>,
    labels: Vec<String>,
}

#[test]
fn test_u32_lz4_roundtrip() {
    let val: u32 = 987654321;
    let enc = encode_to_vec(&val).expect("encode u32");
    let compressed = compress_lz4(&enc).expect("compress u32");
    let decompressed = decompress_lz4(&compressed).expect("decompress u32");
    let (decoded, _): (u32, usize) = decode_from_slice(&decompressed).expect("decode u32");
    assert_eq!(val, decoded);
}

#[test]
fn test_string_lz4_roundtrip() {
    let val = String::from("Hello, LZ4 compression world!");
    let enc = encode_to_vec(&val).expect("encode string");
    let compressed = compress_lz4(&enc).expect("compress string");
    let decompressed = decompress_lz4(&compressed).expect("decompress string");
    let (decoded, _): (String, usize) = decode_from_slice(&decompressed).expect("decode string");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_u8_lz4_roundtrip() {
    let val: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let enc = encode_to_vec(&val).expect("encode vec u8");
    let compressed = compress_lz4(&enc).expect("compress vec u8");
    let decompressed = decompress_lz4(&compressed).expect("decompress vec u8");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&decompressed).expect("decode vec u8");
    assert_eq!(val, decoded);
}

#[test]
fn test_telemetry_struct_lz4_roundtrip() {
    let val = Telemetry {
        device_id: 42,
        timestamp: 1700000000000,
        readings: vec![1.1, 2.2, 3.3, 4.4, 5.5],
        labels: vec![
            String::from("temp"),
            String::from("pressure"),
            String::from("humidity"),
        ],
    };
    let enc = encode_to_vec(&val).expect("encode telemetry");
    let compressed = compress_lz4(&enc).expect("compress telemetry");
    let decompressed = decompress_lz4(&compressed).expect("decompress telemetry");
    let (decoded, _): (Telemetry, usize) =
        decode_from_slice(&decompressed).expect("decode telemetry");
    assert_eq!(val, decoded);
}

#[test]
fn test_compressed_differs_from_uncompressed_u32() {
    let val: u32 = 42;
    let enc = encode_to_vec(&val).expect("encode u32");
    let compressed = compress_lz4(&enc).expect("compress u32");
    assert_ne!(
        enc, compressed,
        "compressed bytes should differ from uncompressed"
    );
}

#[test]
fn test_compressed_differs_from_uncompressed_string() {
    let val = String::from("This is a test string for LZ4 compression difference check");
    let enc = encode_to_vec(&val).expect("encode string");
    let compressed = compress_lz4(&enc).expect("compress string");
    assert_ne!(
        enc, compressed,
        "compressed bytes should differ from uncompressed"
    );
}

#[test]
fn test_large_repetitive_vec_u8_compresses_smaller() {
    let val: Vec<u8> = vec![0xABu8; 5000];
    let enc = encode_to_vec(&val).expect("encode large repetitive vec u8");
    let compressed = compress_lz4(&enc).expect("compress large repetitive vec u8");
    assert!(
        compressed.len() < enc.len(),
        "compressed size {} should be smaller than uncompressed size {}",
        compressed.len(),
        enc.len()
    );
}

#[test]
fn test_large_repetitive_vec_u32_roundtrip() {
    let val: Vec<u32> = vec![0xDEADBEEFu32; 1000];
    let enc = encode_to_vec(&val).expect("encode large repetitive vec u32");
    let compressed = compress_lz4(&enc).expect("compress large repetitive vec u32");
    let decompressed = decompress_lz4(&compressed).expect("decompress large repetitive vec u32");
    let (decoded, _): (Vec<u32>, usize) =
        decode_from_slice(&decompressed).expect("decode large repetitive vec u32");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_string_100_identical_roundtrip() {
    let val: Vec<String> = (0..100)
        .map(|_| String::from("repeated_string_value"))
        .collect();
    let enc = encode_to_vec(&val).expect("encode vec of identical strings");
    let compressed = compress_lz4(&enc).expect("compress vec of identical strings");
    let decompressed = decompress_lz4(&compressed).expect("decompress vec of identical strings");
    let (decoded, _): (Vec<String>, usize) =
        decode_from_slice(&decompressed).expect("decode vec of identical strings");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_telemetry_roundtrip() {
    let val: Vec<Telemetry> = (0..10)
        .map(|i| Telemetry {
            device_id: i,
            timestamp: 1700000000000 + i as u64 * 1000,
            readings: vec![i as f32 * 0.1, i as f32 * 0.2, i as f32 * 0.3],
            labels: vec![format!("sensor_{}", i), format!("channel_{}", i)],
        })
        .collect();
    let enc = encode_to_vec(&val).expect("encode vec of telemetry");
    let compressed = compress_lz4(&enc).expect("compress vec of telemetry");
    let decompressed = decompress_lz4(&compressed).expect("decompress vec of telemetry");
    let (decoded, _): (Vec<Telemetry>, usize) =
        decode_from_slice(&decompressed).expect("decode vec of telemetry");
    assert_eq!(val, decoded);
}

#[test]
fn test_empty_vec_u8_lz4_roundtrip() {
    let val: Vec<u8> = Vec::new();
    let enc = encode_to_vec(&val).expect("encode empty vec u8");
    let compressed = compress_lz4(&enc).expect("compress empty vec u8");
    let decompressed = decompress_lz4(&compressed).expect("decompress empty vec u8");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode empty vec u8");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_some_lz4_roundtrip() {
    let val: Option<String> = Some(String::from("optional value present"));
    let enc = encode_to_vec(&val).expect("encode option string some");
    let compressed = compress_lz4(&enc).expect("compress option string some");
    let decompressed = decompress_lz4(&compressed).expect("decompress option string some");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&decompressed).expect("decode option string some");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_none_lz4_roundtrip() {
    let val: Option<String> = None;
    let enc = encode_to_vec(&val).expect("encode option string none");
    let compressed = compress_lz4(&enc).expect("compress option string none");
    let decompressed = decompress_lz4(&compressed).expect("decompress option string none");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&decompressed).expect("decode option string none");
    assert_eq!(val, decoded);
}

#[test]
fn test_large_random_vec_u64_lcg_roundtrip() {
    let mut state = 12345u64;
    let vals: Vec<u64> = (0..200)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            state
        })
        .collect();
    let enc = encode_to_vec(&vals).expect("encode lcg vec u64");
    let compressed = compress_lz4(&enc).expect("compress lcg vec u64");
    let decompressed = decompress_lz4(&compressed).expect("decompress lcg vec u64");
    let (decoded, _): (Vec<u64>, usize) =
        decode_from_slice(&decompressed).expect("decode lcg vec u64");
    assert_eq!(vals, decoded);
}

#[test]
fn test_compress_same_data_twice_identical_output() {
    let val: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let enc = encode_to_vec(&val).expect("encode for idempotency test");
    let compressed1 = compress_lz4(&enc).expect("compress first time");
    let compressed2 = compress_lz4(&enc).expect("compress second time");
    assert_eq!(
        compressed1, compressed2,
        "compressing same data twice should yield identical bytes"
    );
}

#[test]
fn test_decompress_wrong_bytes_returns_error() {
    let bad_data = b"not_lz4_data";
    let result = decompress_lz4(bad_data);
    assert!(
        result.is_err(),
        "decompress_lz4 on invalid data should return an error"
    );
}

#[test]
fn test_compressed_size_positive_for_small_inputs() {
    let val: u32 = 1;
    let enc = encode_to_vec(&val).expect("encode small u32");
    let compressed = compress_lz4(&enc).expect("compress small u32");
    assert!(
        compressed.len() > 0,
        "compressed output must not be empty even for small inputs"
    );
}

#[test]
fn test_bool_lz4_roundtrip() {
    let val: bool = true;
    let enc = encode_to_vec(&val).expect("encode bool");
    let compressed = compress_lz4(&enc).expect("compress bool");
    let decompressed = decompress_lz4(&compressed).expect("decompress bool");
    let (decoded, _): (bool, usize) = decode_from_slice(&decompressed).expect("decode bool");
    assert_eq!(val, decoded);
}

#[test]
fn test_u128_lz4_roundtrip() {
    let val: u128 = 340282366920938463463374607431768211455u128;
    let enc = encode_to_vec(&val).expect("encode u128");
    let compressed = compress_lz4(&enc).expect("compress u128");
    let decompressed = decompress_lz4(&compressed).expect("decompress u128");
    let (decoded, _): (u128, usize) = decode_from_slice(&decompressed).expect("decode u128");
    assert_eq!(val, decoded);
}

#[test]
fn test_nested_vec_vec_u8_lz4_roundtrip() {
    let val: Vec<Vec<u8>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6, 7],
        vec![],
        vec![8, 9],
        vec![10, 11, 12, 13, 14],
    ];
    let enc = encode_to_vec(&val).expect("encode nested vec vec u8");
    let compressed = compress_lz4(&enc).expect("compress nested vec vec u8");
    let decompressed = decompress_lz4(&compressed).expect("decompress nested vec vec u8");
    let (decoded, _): (Vec<Vec<u8>>, usize) =
        decode_from_slice(&decompressed).expect("decode nested vec vec u8");
    assert_eq!(val, decoded);
}

#[test]
fn test_telemetry_unicode_labels_lz4_roundtrip() {
    let val = Telemetry {
        device_id: 99,
        timestamp: 9999999999999,
        readings: vec![0.0, 1.0, 2.0],
        labels: vec![
            String::from("温度センサー"),
            String::from("気圧計"),
            String::from("湿度センサー"),
            String::from("Ångström"),
            String::from("Ñoño"),
        ],
    };
    let enc = encode_to_vec(&val).expect("encode telemetry with unicode labels");
    let compressed = compress_lz4(&enc).expect("compress telemetry with unicode labels");
    let decompressed =
        decompress_lz4(&compressed).expect("decompress telemetry with unicode labels");
    let (decoded, _): (Telemetry, usize) =
        decode_from_slice(&decompressed).expect("decode telemetry with unicode labels");
    assert_eq!(val, decoded);
}

#[test]
fn test_checksum_decompressed_matches_original_encoded_bytes() {
    let val = Telemetry {
        device_id: 7,
        timestamp: 1234567890,
        readings: vec![3.14, 2.71, 1.41],
        labels: vec![String::from("pi"), String::from("e"), String::from("sqrt2")],
    };
    let enc = encode_to_vec(&val).expect("encode for checksum test");
    let compressed = compress_lz4(&enc).expect("compress for checksum test");
    let decompressed = decompress_lz4(&compressed).expect("decompress for checksum test");
    assert_eq!(
        enc, decompressed,
        "decompressed bytes must exactly match original encoded bytes"
    );
}
