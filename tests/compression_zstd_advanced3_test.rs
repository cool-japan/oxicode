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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

fn compress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Zstd).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LogRecord {
    id: u64,
    level: u8,
    message: String,
    tags: Vec<String>,
}

#[test]
fn test_zstd_u32_roundtrip() {
    let val: u32 = 987654321u32;
    let encoded = encode_to_vec(&val).expect("encode u32 failed");
    let compressed = compress_zstd(&encoded).expect("compress u32 failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress u32 failed");
    let (decoded, _): (u32, usize) = decode_from_slice(&decompressed).expect("decode u32 failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_string_roundtrip() {
    let val = String::from("Hello, OxiCode zstd advanced3!");
    let encoded = encode_to_vec(&val).expect("encode String failed");
    let compressed = compress_zstd(&encoded).expect("compress String failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress String failed");
    let (decoded, _): (String, usize) =
        decode_from_slice(&decompressed).expect("decode String failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_vec_u8_roundtrip() {
    let val: Vec<u8> = (0u8..=255u8).collect();
    let encoded = encode_to_vec(&val).expect("encode Vec<u8> failed");
    let compressed = compress_zstd(&encoded).expect("compress Vec<u8> failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<u8> failed");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<u8> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_log_record_roundtrip() {
    let val = LogRecord {
        id: 42u64,
        level: 3u8,
        message: String::from("test log message"),
        tags: vec![
            String::from("info"),
            String::from("production"),
            String::from("service-a"),
        ],
    };
    let encoded = encode_to_vec(&val).expect("encode LogRecord failed");
    let compressed = compress_zstd(&encoded).expect("compress LogRecord failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress LogRecord failed");
    let (decoded, _): (LogRecord, usize) =
        decode_from_slice(&decompressed).expect("decode LogRecord failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_bool_true_roundtrip() {
    let val: bool = true;
    let encoded = encode_to_vec(&val).expect("encode bool true failed");
    let compressed = compress_zstd(&encoded).expect("compress bool true failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress bool true failed");
    let (decoded, _): (bool, usize) =
        decode_from_slice(&decompressed).expect("decode bool true failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_bool_false_roundtrip() {
    let val: bool = false;
    let encoded = encode_to_vec(&val).expect("encode bool false failed");
    let compressed = compress_zstd(&encoded).expect("compress bool false failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress bool false failed");
    let (decoded, _): (bool, usize) =
        decode_from_slice(&decompressed).expect("decode bool false failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_u128_roundtrip() {
    let val: u128 = 340282366920938463463374607431768211455u128;
    let encoded = encode_to_vec(&val).expect("encode u128 failed");
    let compressed = compress_zstd(&encoded).expect("compress u128 failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress u128 failed");
    let (decoded, _): (u128, usize) = decode_from_slice(&decompressed).expect("decode u128 failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_large_repetitive_bytes_compress_smaller() {
    let val: Vec<u8> = vec![0xABu8; 5000];
    let encoded = encode_to_vec(&val).expect("encode repetitive Vec<u8> failed");
    let compressed = compress_zstd(&encoded).expect("compress repetitive Vec<u8> failed");
    assert!(
        compressed.len() < encoded.len(),
        "expected compressed size {} < original size {}",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_zstd_large_repetitive_vec_u32_roundtrip() {
    let val: Vec<u32> = vec![0xDEADBEEFu32; 1000];
    let encoded = encode_to_vec(&val).expect("encode repetitive Vec<u32> failed");
    let compressed = compress_zstd(&encoded).expect("compress repetitive Vec<u32> failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress repetitive Vec<u32> failed");
    let (decoded, _): (Vec<u32>, usize) =
        decode_from_slice(&decompressed).expect("decode repetitive Vec<u32> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_vec_of_identical_strings_roundtrip() {
    let val: Vec<String> = (0..100)
        .map(|_| String::from("identical_string_value"))
        .collect();
    let encoded = encode_to_vec(&val).expect("encode Vec<String> failed");
    let compressed = compress_zstd(&encoded).expect("compress Vec<String> failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<String> failed");
    let (decoded, _): (Vec<String>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<String> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_vec_log_records_roundtrip() {
    let val: Vec<LogRecord> = (0..10)
        .map(|i| LogRecord {
            id: i as u64,
            level: (i % 5) as u8,
            message: format!("log message number {}", i),
            tags: vec![format!("tag-{}", i), String::from("common-tag")],
        })
        .collect();
    let encoded = encode_to_vec(&val).expect("encode Vec<LogRecord> failed");
    let compressed = compress_zstd(&encoded).expect("compress Vec<LogRecord> failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<LogRecord> failed");
    let (decoded, _): (Vec<LogRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<LogRecord> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_empty_vec_u8_roundtrip() {
    let val: Vec<u8> = Vec::new();
    let encoded = encode_to_vec(&val).expect("encode empty Vec<u8> failed");
    let compressed = compress_zstd(&encoded).expect("compress empty Vec<u8> failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress empty Vec<u8> failed");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode empty Vec<u8> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_option_string_some_roundtrip() {
    let val: Option<String> = Some(String::from("optional string content"));
    let encoded = encode_to_vec(&val).expect("encode Option<String> Some failed");
    let compressed = compress_zstd(&encoded).expect("compress Option<String> Some failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress Option<String> Some failed");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<String> Some failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_option_string_none_roundtrip() {
    let val: Option<String> = None;
    let encoded = encode_to_vec(&val).expect("encode Option<String> None failed");
    let compressed = compress_zstd(&encoded).expect("compress Option<String> None failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress Option<String> None failed");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<String> None failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_compressed_differs_from_uncompressed() {
    let val = String::from("This string will be encoded and then compressed with zstd.");
    let encoded = encode_to_vec(&val).expect("encode String failed");
    let compressed = compress_zstd(&encoded).expect("compress String failed");
    assert_ne!(
        encoded, compressed,
        "compressed bytes must differ from encoded bytes"
    );
}

#[test]
fn test_zstd_large_random_vec_u64_roundtrip() {
    // LCG pseudorandom sequence for deterministic "random" data
    let mut state: u64 = 6364136223846793005u64;
    let val: Vec<u64> = (0..512)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005u64)
                .wrapping_add(1442695040888963407u64);
            state
        })
        .collect();
    let encoded = encode_to_vec(&val).expect("encode random Vec<u64> failed");
    let compressed = compress_zstd(&encoded).expect("compress random Vec<u64> failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress random Vec<u64> failed");
    let (decoded, _): (Vec<u64>, usize) =
        decode_from_slice(&decompressed).expect("decode random Vec<u64> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_compress_same_data_twice_identical() {
    let val: Vec<u8> = vec![0x55u8; 256];
    let encoded = encode_to_vec(&val).expect("encode Vec<u8> failed");
    let compressed_a = compress_zstd(&encoded).expect("first compress failed");
    let compressed_b = compress_zstd(&encoded).expect("second compress failed");
    assert_eq!(
        compressed_a, compressed_b,
        "compressing same data twice must yield identical output"
    );
}

#[test]
fn test_zstd_decompress_bad_data_returns_error() {
    let bad_data: Vec<u8> = vec![0xFFu8, 0xFEu8, 0xFDu8, 0xFCu8, 0x00u8, 0x01u8, 0x02u8];
    let result = decompress_zstd(&bad_data);
    assert!(
        result.is_err(),
        "decompressing bad data must return an error"
    );
}

#[test]
fn test_zstd_compressed_size_positive_for_small_input() {
    let val: u32 = 1u32;
    let encoded = encode_to_vec(&val).expect("encode small u32 failed");
    let compressed = compress_zstd(&encoded).expect("compress small u32 failed");
    assert!(
        !compressed.is_empty(),
        "compressed output must have positive size"
    );
}

#[test]
fn test_zstd_nested_vec_vec_u8_roundtrip() {
    let val: Vec<Vec<u8>> = vec![
        vec![1u8, 2u8, 3u8],
        vec![],
        vec![255u8, 128u8, 64u8, 32u8],
        (0u8..=127u8).collect(),
    ];
    let encoded = encode_to_vec(&val).expect("encode Vec<Vec<u8>> failed");
    let compressed = compress_zstd(&encoded).expect("compress Vec<Vec<u8>> failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<Vec<u8>> failed");
    let (decoded, _): (Vec<Vec<u8>>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<Vec<u8>> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_u64_roundtrip() {
    let val: u64 = 18446744073709551615u64;
    let encoded = encode_to_vec(&val).expect("encode u64 failed");
    let compressed = compress_zstd(&encoded).expect("compress u64 failed");
    let decompressed = decompress_zstd(&compressed).expect("decompress u64 failed");
    let (decoded, _): (u64, usize) = decode_from_slice(&decompressed).expect("decode u64 failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_zstd_decompressed_bytes_match_encoded_checksum() {
    let val = LogRecord {
        id: 9999u64,
        level: 1u8,
        message: String::from("checksum verification message"),
        tags: vec![String::from("checksum"), String::from("verify")],
    };
    let encoded = encode_to_vec(&val).expect("encode LogRecord for checksum failed");
    let compressed = compress_zstd(&encoded).expect("compress LogRecord for checksum failed");
    let decompressed =
        decompress_zstd(&compressed).expect("decompress LogRecord for checksum failed");
    assert_eq!(
        encoded, decompressed,
        "decompressed bytes must exactly match original encoded bytes"
    );
    let checksum_original: u64 = encoded.iter().enumerate().fold(0u64, |acc, (i, &b)| {
        acc.wrapping_add((b as u64).wrapping_mul(i as u64 + 1))
    });
    let checksum_roundtrip: u64 = decompressed.iter().enumerate().fold(0u64, |acc, (i, &b)| {
        acc.wrapping_add((b as u64).wrapping_mul(i as u64 + 1))
    });
    assert_eq!(
        checksum_original, checksum_roundtrip,
        "checksums of original and decompressed bytes must match"
    );
}
