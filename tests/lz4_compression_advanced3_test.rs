//! LZ4 compression advanced tests — series 3.
//!
//! 22 top-level `#[test]` functions covering new scenarios not exercised by
//! the existing suites.  All tests are gated on `compression-lz4`.

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
use oxicode::{Decode, Encode};

// ─────────────────────────────────────────────────────────────────────────────
// Shared test types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiField {
    id: u64,
    name: String,
    tags: Vec<String>,
    score: f64,
    active: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 – u32 basic roundtrip via compress/decompress
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_u32_basic_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: u32 = 0xDEAD_BEEF;
    let encoded = oxicode::encode_to_vec(&original).expect("encode u32 failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress u32 failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress u32 failed");
    let (decoded, _): (u32, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode u32 failed");
    assert_eq!(original, decoded, "u32 value must survive LZ4 roundtrip");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 – String basic roundtrip via compress/decompress
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_string_basic_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = String::from("OxiCode LZ4 advanced3 string test payload");
    let encoded = oxicode::encode_to_vec(&original).expect("encode String failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress String failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress String failed");
    let (decoded, _): (String, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode String failed");
    assert_eq!(original, decoded, "String must survive LZ4 roundtrip");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 – Vec<u8> basic roundtrip via compress/decompress
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_vec_u8_basic_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<u8> = (0u8..200).collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<u8> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<u8> failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress Vec<u8> failed");
    let (decoded, _): (Vec<u8>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<u8> failed");
    assert_eq!(original, decoded, "Vec<u8> must survive LZ4 roundtrip");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 – struct with multiple fields basic roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_struct_basic_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = MultiField {
        id: 42_000_000_u64,
        name: String::from("test-node-alpha"),
        tags: vec![
            String::from("rust"),
            String::from("oxicode"),
            String::from("lz4"),
        ],
        score: 98.765_f64,
        active: true,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode MultiField failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress MultiField failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress MultiField failed");
    let (decoded, _): (MultiField, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode MultiField failed");
    assert_eq!(original, decoded, "MultiField must survive LZ4 roundtrip");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 – compressed u32 bytes differ from uncompressed bytes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_compressed_u32_differs_from_uncompressed() {
    use oxicode::compression::{compress, Compression};

    let original: u32 = 12345678;
    let uncompressed = oxicode::encode_to_vec(&original).expect("encode u32 for diff test failed");
    let compressed =
        compress(&uncompressed, Compression::Lz4).expect("lz4 compress for diff test failed");
    assert_ne!(
        uncompressed, compressed,
        "compressed bytes must differ from uncompressed bytes"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 – compressed String bytes differ from uncompressed bytes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_compressed_string_differs_from_uncompressed() {
    use oxicode::compression::{compress, Compression};

    let value = String::from("compressed-differs-from-raw-oxicode");
    let uncompressed = oxicode::encode_to_vec(&value).expect("encode String for diff test failed");
    let compressed = compress(&uncompressed, Compression::Lz4)
        .expect("lz4 compress String for diff test failed");
    assert_ne!(
        uncompressed, compressed,
        "LZ4-compressed String bytes must differ from raw encoded bytes"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7 – compressed Vec<u8> bytes differ from uncompressed bytes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_compressed_vec_u8_differs_from_uncompressed() {
    use oxicode::compression::{compress, Compression};

    let value: Vec<u8> = vec![0xAA; 128];
    let uncompressed = oxicode::encode_to_vec(&value).expect("encode Vec<u8> for diff test failed");
    let compressed = compress(&uncompressed, Compression::Lz4)
        .expect("lz4 compress Vec<u8> for diff test failed");
    assert_ne!(
        uncompressed, compressed,
        "LZ4-compressed Vec<u8> bytes must differ from raw encoded bytes"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8 – compressed struct bytes differ from uncompressed bytes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_compressed_struct_differs_from_uncompressed() {
    use oxicode::compression::{compress, Compression};

    let value = MultiField {
        id: 1,
        name: String::from("differ-test"),
        tags: vec![String::from("a"), String::from("b")],
        score: 1.0_f64,
        active: false,
    };
    let uncompressed = oxicode::encode_to_vec(&value).expect("encode struct for diff test failed");
    let compressed = compress(&uncompressed, Compression::Lz4)
        .expect("lz4 compress struct for diff test failed");
    assert_ne!(
        uncompressed, compressed,
        "LZ4-compressed struct bytes must differ from raw encoded bytes"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9 – large highly-compressible data (1000+ bytes) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_large_compressible_data_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // 5000 repetitions of a fixed byte — extremely compressible
    let original: Vec<u8> = vec![0x42u8; 5_000];
    let encoded = oxicode::encode_to_vec(&original).expect("encode large compressible failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress large compressible failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress large compressible failed");
    let (decoded, _): (Vec<u8>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode large compressible failed");
    assert_eq!(
        original, decoded,
        "large compressible data must survive LZ4 roundtrip"
    );
    assert!(
        compressed.len() < encoded.len(),
        "compressed size ({}) must be smaller than encoded size ({}) for highly compressible data",
        compressed.len(),
        encoded.len()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10 – large highly-compressible Vec<String> (1000+ bytes) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_large_vec_string_compressible_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // 100 copies of the same string — highly compressible
    let original: Vec<String> = (0..100)
        .map(|_| String::from("oxicode-lz4-repeated-string-payload"))
        .collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode large Vec<String> failed");
    assert!(
        encoded.len() > 1_000,
        "precondition: encoded length must be > 1000 bytes, got {}",
        encoded.len()
    );
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress large Vec<String> failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress large Vec<String> failed");
    let (decoded, _): (Vec<String>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode large Vec<String> failed");
    assert_eq!(
        original, decoded,
        "large Vec<String> must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11 – large highly-compressible Vec<u64> (1000+ bytes) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_large_vec_u64_zeros_compressible_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // Use a mix of large values to guarantee the encoded payload exceeds 1000 bytes.
    // Each u64 with fixed encoding = 8 bytes; 200 × 8 = 1600 bytes.
    let original: Vec<u64> = (0u64..200).map(|i| u64::MAX - i).collect();
    let encoded = oxicode::encode_to_vec_with_config(
        &original,
        oxicode::config::standard().with_fixed_int_encoding(),
    )
    .expect("encode large Vec<u64> failed");
    assert!(
        encoded.len() > 1_000,
        "precondition: encoded length must be > 1000 bytes, got {}",
        encoded.len()
    );
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress large Vec<u64> failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress large Vec<u64> failed");
    let (decoded, _): (Vec<u64>, usize) = oxicode::decode_from_slice_with_config(
        &decompressed,
        oxicode::config::standard().with_fixed_int_encoding(),
    )
    .expect("decode large Vec<u64> failed");
    assert_eq!(
        original, decoded,
        "large Vec<u64> must survive LZ4 roundtrip"
    );
    assert!(
        compressed.len() < encoded.len(),
        "compressed size ({}) must be < encoded size ({}) for large Vec<u64>",
        compressed.len(),
        encoded.len()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12 – large "random-looking" LCG data roundtrip (1000+ bytes)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_large_lcg_random_data_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let mut state: u64 = 0x1234_5678_9ABC_DEF0;
    let original: Vec<u8> = (0..2_048)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 56) as u8
        })
        .collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode LCG random data failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress LCG random data failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress LCG random data failed");
    let (decoded, _): (Vec<u8>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode LCG random data failed");
    assert_eq!(
        original, decoded,
        "LCG random data must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13 – large random-ish Vec<u32> (1000+ bytes) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_large_pseudo_random_vec_u32_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // Simple xorshift to generate pseudo-random u32 values
    let mut x: u32 = 0xBAD_C0FFE;
    let original: Vec<u32> = (0..512)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            x
        })
        .collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode pseudo-random Vec<u32> failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress pseudo-random Vec<u32> failed");
    let decompressed =
        decompress(&compressed).expect("lz4 decompress pseudo-random Vec<u32> failed");
    let (decoded, _): (Vec<u32>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode pseudo-random Vec<u32> failed");
    assert_eq!(
        original, decoded,
        "pseudo-random Vec<u32> must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14 – large random-ish Vec<f64> (1000+ bytes) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_large_pseudo_random_vec_f64_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // Linear progression with fractional steps — not compressible, but valid
    let original: Vec<f64> = (0u32..256)
        .map(|i| (i as f64) * std::f64::consts::PI / 256.0)
        .collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<f64> random-ish failed");
    assert!(
        encoded.len() > 1_000,
        "precondition: encoded len must be > 1000, got {}",
        encoded.len()
    );
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<f64> random-ish failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress Vec<f64> random-ish failed");
    let (decoded, _): (Vec<f64>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<f64> random-ish failed");
    assert_eq!(
        original.len(),
        decoded.len(),
        "decoded Vec<f64> length must match original"
    );
    for (idx, (a, b)) in original.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "Vec<f64>[{idx}]: bit pattern must survive LZ4 roundtrip"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15 – Option<String> Some roundtrip with compression
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_option_string_some_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Option<String> = Some(String::from("optional-string-value-lz4-test"));
    let encoded = oxicode::encode_to_vec(&original).expect("encode Option<String>::Some failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress Option<String>::Some failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress Option<String>::Some failed");
    let (decoded, _): (Option<String>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Option<String>::Some failed");
    assert_eq!(
        original, decoded,
        "Option<String>::Some must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16 – Option<String> None roundtrip with compression
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_option_string_none_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Option<String> = None;
    let encoded = oxicode::encode_to_vec(&original).expect("encode Option<String>::None failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress Option<String>::None failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress Option<String>::None failed");
    let (decoded, _): (Option<String>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Option<String>::None failed");
    assert_eq!(
        original, decoded,
        "Option<String>::None must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17 – Option<Vec<u32>> Some roundtrip with compression
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_option_vec_u32_some_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Option<Vec<u32>> = Some((0u32..50).collect());
    let encoded = oxicode::encode_to_vec(&original).expect("encode Option<Vec<u32>>::Some failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress Option<Vec<u32>>::Some failed");
    let decompressed =
        decompress(&compressed).expect("lz4 decompress Option<Vec<u32>>::Some failed");
    let (decoded, _): (Option<Vec<u32>>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Option<Vec<u32>>::Some failed");
    assert_eq!(
        original, decoded,
        "Option<Vec<u32>>::Some must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18 – Vec<String> with many short strings roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_vec_string_many_short_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<String> = (0u32..50).map(|i| format!("item-{:04}", i)).collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<String> short items failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<String> short items failed");
    let decompressed =
        decompress(&compressed).expect("lz4 decompress Vec<String> short items failed");
    let (decoded, _): (Vec<String>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<String> short items failed");
    assert_eq!(
        original, decoded,
        "Vec<String> with short items must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19 – Vec<String> with long strings roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_vec_string_long_entries_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // Each string is 50+ chars, 20 strings total
    let original: Vec<String> = (0u32..20)
        .map(|i| {
            format!(
                "long-entry-{:04}-padded-with-oxicode-lz4-test-data-{}",
                i,
                i * 7
            )
        })
        .collect();
    let encoded =
        oxicode::encode_to_vec(&original).expect("encode Vec<String> long entries failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<String> long entries failed");
    let decompressed =
        decompress(&compressed).expect("lz4 decompress Vec<String> long entries failed");
    let (decoded, _): (Vec<String>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<String> long entries failed");
    assert_eq!(
        original, decoded,
        "Vec<String> with long entries must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20 – Vec<String> single-element roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_vec_string_single_element_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<String> = vec![String::from("sole-element-in-vec")];
    let encoded =
        oxicode::encode_to_vec(&original).expect("encode single-element Vec<String> failed");
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("lz4 compress single-element Vec<String> failed");
    let decompressed =
        decompress(&compressed).expect("lz4 decompress single-element Vec<String> failed");
    let (decoded, _): (Vec<String>, usize) = oxicode::decode_from_slice(&decompressed)
        .expect("decode single-element Vec<String> failed");
    assert_eq!(
        original, decoded,
        "single-element Vec<String> must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 21 – struct with multiple fields compressed and decompressed
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_multifield_struct_full_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = MultiField {
        id: u64::MAX,
        name: String::from("max-id-struct-lz4-advanced3-test"),
        tags: vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
            String::from("delta"),
            String::from("epsilon"),
            String::from("zeta"),
        ],
        score: -3.14159265358979_f64,
        active: false,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode MultiField full struct failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress MultiField full struct failed");
    let decompressed =
        decompress(&compressed).expect("lz4 decompress MultiField full struct failed");
    let (decoded, _): (MultiField, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode MultiField full struct failed");

    assert_eq!(
        original.id, decoded.id,
        "id field must survive LZ4 roundtrip"
    );
    assert_eq!(
        original.name, decoded.name,
        "name field must survive LZ4 roundtrip"
    );
    assert_eq!(
        original.tags, decoded.tags,
        "tags field must survive LZ4 roundtrip"
    );
    assert_eq!(
        original.score.to_bits(),
        decoded.score.to_bits(),
        "score f64 bit pattern must survive LZ4 roundtrip"
    );
    assert_eq!(
        original.active, decoded.active,
        "active field must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 22 – compressed + checksum combined (compression-lz4 + checksum features)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_lz4_adv3_compression_then_checksum_combined_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // Encode a value, compress it with LZ4, wrap with checksum if available,
    // then verify the original data is faithfully recovered.
    let original: Vec<u32> = (100u32..200).collect();
    let encoded =
        oxicode::encode_to_vec(&original).expect("encode Vec<u32> for combined test failed");

    // Apply LZ4 compression layer
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("lz4 compress for combined checksum test failed");

    // When `checksum` feature is enabled, wrap with CRC32 and verify; otherwise
    // fall back to a plain decompress + decode to exercise the compression path.
    #[cfg(feature = "checksum")]
    {
        use oxicode::checksum::{verify_checksum, wrap_with_checksum};

        let wrapped = wrap_with_checksum(&compressed);
        let payload = verify_checksum(&wrapped)
            .expect("checksum verify failed for combined lz4+checksum test");
        let decompressed =
            decompress(payload).expect("lz4 decompress after checksum verify failed");
        let (decoded, _): (Vec<u32>, usize) = oxicode::decode_from_slice(&decompressed)
            .expect("decode after combined lz4+checksum roundtrip failed");
        assert_eq!(
            original, decoded,
            "Vec<u32> must survive LZ4+checksum combined roundtrip"
        );
    }

    #[cfg(not(feature = "checksum"))]
    {
        // Without checksum feature: verify the compression layer alone is intact
        let decompressed = decompress(&compressed).expect("lz4 decompress in combined test failed");
        let (decoded, _): (Vec<u32>, usize) = oxicode::decode_from_slice(&decompressed)
            .expect("decode in combined test (no checksum feature) failed");
        assert_eq!(
            original, decoded,
            "Vec<u32> must survive LZ4 roundtrip (checksum feature absent)"
        );
    }
}
