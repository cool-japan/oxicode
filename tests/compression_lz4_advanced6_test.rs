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

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Inventory {
    sku: String,
    count: u32,
    price: f64,
    categories: Vec<String>,
}

// Test 1: Inventory roundtrip via LZ4
#[test]
fn test_inventory_lz4_roundtrip() {
    let item = Inventory {
        sku: String::from("SKU-001"),
        count: 42,
        price: 19.99,
        categories: vec![String::from("electronics"), String::from("gadgets")],
    };
    let encoded = encode_to_vec(&item).expect("Failed to encode Inventory");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Inventory");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress Inventory");
    let (decoded, _): (Inventory, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Inventory");
    assert_eq!(item, decoded);
}

// Test 2: u64 LZ4 roundtrip
#[test]
fn test_u64_lz4_roundtrip() {
    let val: u64 = 18_446_744_073_709_551_615u64;
    let encoded = encode_to_vec(&val).expect("Failed to encode u64");
    let compressed = compress_lz4(&encoded).expect("Failed to compress u64");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress u64");
    let (decoded, _): (u64, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode u64");
    assert_eq!(val, decoded);
}

// Test 3: i32 negative LZ4 roundtrip
#[test]
fn test_i32_negative_lz4_roundtrip() {
    let val: i32 = -2_147_483_648i32;
    let encoded = encode_to_vec(&val).expect("Failed to encode i32");
    let compressed = compress_lz4(&encoded).expect("Failed to compress i32");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress i32");
    let (decoded, _): (i32, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode i32");
    assert_eq!(val, decoded);
}

// Test 4: f64 LZ4 roundtrip
#[test]
fn test_f64_lz4_roundtrip() {
    let val: f64 = std::f64::consts::PI;
    let encoded = encode_to_vec(&val).expect("Failed to encode f64");
    let compressed = compress_lz4(&encoded).expect("Failed to compress f64");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress f64");
    let (decoded, _): (f64, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode f64");
    assert_eq!(val, decoded);
}

// Test 5: String with repeated pattern LZ4 roundtrip
#[test]
fn test_repeated_pattern_string_lz4_roundtrip() {
    let val: String = "abcabc".repeat(500);
    let encoded = encode_to_vec(&val).expect("Failed to encode repeated String");
    let compressed = compress_lz4(&encoded).expect("Failed to compress repeated String");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress repeated String");
    let (decoded, _): (String, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode repeated String");
    assert_eq!(val, decoded);
}

// Test 6: Vec<Inventory> LZ4 roundtrip (5 items)
#[test]
fn test_vec_inventory_lz4_roundtrip() {
    let items: Vec<Inventory> = (0..5)
        .map(|i| Inventory {
            sku: format!("SKU-{:04}", i),
            count: i as u32 * 10,
            price: (i as f64) * 3.14,
            categories: vec![format!("cat-{}", i), format!("sub-{}", i * 2)],
        })
        .collect();
    let encoded = encode_to_vec(&items).expect("Failed to encode Vec<Inventory>");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Vec<Inventory>");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress Vec<Inventory>");
    let (decoded, _): (Vec<Inventory>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Vec<Inventory>");
    assert_eq!(items, decoded);
}

// Test 7: Large repetitive bytes (10000 of 0xCC) compress smaller
#[test]
fn test_large_repetitive_bytes_compress_smaller() {
    let data: Vec<u8> = vec![0xCCu8; 10_000];
    let encoded = encode_to_vec(&data).expect("Failed to encode large repetitive bytes");
    let compressed = compress_lz4(&encoded).expect("Failed to compress large repetitive bytes");
    assert!(
        compressed.len() < encoded.len(),
        "Compressed size {} should be less than original size {}",
        compressed.len(),
        encoded.len()
    );
}

// Test 8: Large repetitive u32s (2000 of 0xBEEF) roundtrip
#[test]
fn test_large_repetitive_u32_lz4_roundtrip() {
    let items: Vec<u32> = vec![0xBEEFu32; 2_000];
    let encoded = encode_to_vec(&items).expect("Failed to encode repetitive u32s");
    let compressed = compress_lz4(&encoded).expect("Failed to compress repetitive u32s");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress repetitive u32s");
    let (decoded, _): (Vec<u32>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode repetitive u32s");
    assert_eq!(items, decoded);
}

// Test 9: Empty Vec<u8> LZ4 roundtrip
#[test]
fn test_empty_vec_u8_lz4_roundtrip() {
    let val: Vec<u8> = Vec::new();
    let encoded = encode_to_vec(&val).expect("Failed to encode empty Vec<u8>");
    let compressed = compress_lz4(&encoded).expect("Failed to compress empty Vec<u8>");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress empty Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode empty Vec<u8>");
    assert_eq!(val, decoded);
}

// Test 10: Vec<u8> with all 256 byte values roundtrip
#[test]
fn test_all_256_bytes_lz4_roundtrip() {
    let val: Vec<u8> = (0u8..=255u8).collect();
    let encoded = encode_to_vec(&val).expect("Failed to encode all-256-byte Vec");
    let compressed = compress_lz4(&encoded).expect("Failed to compress all-256-byte Vec");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress all-256-byte Vec");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode all-256-byte Vec");
    assert_eq!(val, decoded);
}

// Test 11: bool true LZ4 roundtrip
#[test]
fn test_bool_true_lz4_roundtrip() {
    let val: bool = true;
    let encoded = encode_to_vec(&val).expect("Failed to encode bool true");
    let compressed = compress_lz4(&encoded).expect("Failed to compress bool true");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress bool true");
    let (decoded, _): (bool, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode bool true");
    assert_eq!(val, decoded);
}

// Test 12: bool false LZ4 roundtrip
#[test]
fn test_bool_false_lz4_roundtrip() {
    let val: bool = false;
    let encoded = encode_to_vec(&val).expect("Failed to encode bool false");
    let compressed = compress_lz4(&encoded).expect("Failed to compress bool false");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress bool false");
    let (decoded, _): (bool, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode bool false");
    assert_eq!(val, decoded);
}

// Test 13: Option<Inventory> Some LZ4 roundtrip
#[test]
fn test_option_inventory_some_lz4_roundtrip() {
    let val: Option<Inventory> = Some(Inventory {
        sku: String::from("OPT-SKU-999"),
        count: 7,
        price: 99.99,
        categories: vec![String::from("optional"), String::from("rare")],
    });
    let encoded = encode_to_vec(&val).expect("Failed to encode Option<Inventory> Some");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Option<Inventory> Some");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress Option<Inventory> Some");
    let (decoded, _): (Option<Inventory>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Option<Inventory> Some");
    assert_eq!(val, decoded);
}

// Test 14: Option<Inventory> None LZ4 roundtrip
#[test]
fn test_option_inventory_none_lz4_roundtrip() {
    let val: Option<Inventory> = None;
    let encoded = encode_to_vec(&val).expect("Failed to encode Option<Inventory> None");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Option<Inventory> None");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress Option<Inventory> None");
    let (decoded, _): (Option<Inventory>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Option<Inventory> None");
    assert_eq!(val, decoded);
}

// Test 15: u128 LZ4 roundtrip
#[test]
fn test_u128_lz4_roundtrip() {
    let val: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455u128;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128");
    let compressed = compress_lz4(&encoded).expect("Failed to compress u128");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress u128");
    let (decoded, _): (u128, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode u128");
    assert_eq!(val, decoded);
}

// Test 16: String with unicode LZ4 roundtrip
#[test]
fn test_unicode_string_lz4_roundtrip() {
    let val: String =
        String::from("日本語テスト: \u{1F600}\u{1F4E6}\u{2764}\u{FE0F} Unicode LZ4 test!");
    let encoded = encode_to_vec(&val).expect("Failed to encode unicode String");
    let compressed = compress_lz4(&encoded).expect("Failed to compress unicode String");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress unicode String");
    let (decoded, _): (String, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode unicode String");
    assert_eq!(val, decoded);
}

// Test 17: Compress same data 3 times - all identical
#[test]
fn test_compress_same_data_three_times_identical() {
    let item = Inventory {
        sku: String::from("REPEAT-SKU"),
        count: 100,
        price: 5.55,
        categories: vec![String::from("repeated")],
    };
    let encoded = encode_to_vec(&item).expect("Failed to encode for repeat compression");
    let c1 = compress_lz4(&encoded).expect("Failed to compress first time");
    let c2 = compress_lz4(&encoded).expect("Failed to compress second time");
    let c3 = compress_lz4(&encoded).expect("Failed to compress third time");
    assert_eq!(c1, c2, "First and second compressions should be identical");
    assert_eq!(c2, c3, "Second and third compressions should be identical");
}

// Test 18: Decompress bad data returns error
#[test]
fn test_decompress_bad_data_returns_error() {
    let bad_data: Vec<u8> = vec![0xDEu8, 0xADu8, 0xBEu8, 0xEFu8, 0x00u8, 0xFF, 0x42u8];
    let result = decompress_lz4(&bad_data);
    assert!(
        result.is_err(),
        "Decompressing invalid LZ4 data should return an error"
    );
}

// Test 19: Compressed output is non-empty even for 1 byte
#[test]
fn test_compressed_output_nonempty_for_single_byte() {
    let data: Vec<u8> = vec![0x42u8];
    let encoded = encode_to_vec(&data).expect("Failed to encode single byte");
    let compressed = compress_lz4(&encoded).expect("Failed to compress single byte");
    assert!(
        !compressed.is_empty(),
        "Compressed output should be non-empty even for a single byte"
    );
}

// Test 20: Nested Vec<Vec<String>> LZ4 roundtrip
#[test]
fn test_nested_vec_vec_string_lz4_roundtrip() {
    let val: Vec<Vec<String>> = vec![
        vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ],
        vec![String::from("delta"), String::from("epsilon")],
        vec![
            String::from("zeta"),
            String::from("eta"),
            String::from("theta"),
            String::from("iota"),
        ],
        vec![],
        vec![String::from("kappa")],
    ];
    let encoded = encode_to_vec(&val).expect("Failed to encode Vec<Vec<String>>");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Vec<Vec<String>>");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress Vec<Vec<String>>");
    let (decoded, _): (Vec<Vec<String>>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Vec<Vec<String>>");
    assert_eq!(val, decoded);
}

// Test 21: LCG random Vec<u64> (200 items) roundtrip
#[test]
fn test_lcg_random_vec_u64_lz4_roundtrip() {
    // Linear congruential generator: x_{n+1} = (a * x_n + c) mod m
    let a: u64 = 6_364_136_223_846_793_005u64;
    let c: u64 = 1_442_695_040_888_963_407u64;
    let mut state: u64 = 0xDEAD_BEEF_CAFE_BABEu64;
    let items: Vec<u64> = (0..200)
        .map(|_| {
            state = state.wrapping_mul(a).wrapping_add(c);
            state
        })
        .collect();
    let encoded = encode_to_vec(&items).expect("Failed to encode LCG Vec<u64>");
    let compressed = compress_lz4(&encoded).expect("Failed to compress LCG Vec<u64>");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress LCG Vec<u64>");
    let (decoded, _): (Vec<u64>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode LCG Vec<u64>");
    assert_eq!(items, decoded);
}

// Test 22: Decompressed bytes exactly match original encoded bytes
#[test]
fn test_decompressed_bytes_exactly_match_encoded() {
    let item = Inventory {
        sku: String::from("EXACT-MATCH-SKU"),
        count: 255,
        price: 1234.5678,
        categories: vec![
            String::from("exact"),
            String::from("match"),
            String::from("verification"),
        ],
    };
    let encoded = encode_to_vec(&item).expect("Failed to encode for exact match test");
    let compressed = compress_lz4(&encoded).expect("Failed to compress for exact match test");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress for exact match test");
    assert_eq!(
        encoded, decompressed,
        "Decompressed bytes must exactly equal original encoded bytes"
    );
}
