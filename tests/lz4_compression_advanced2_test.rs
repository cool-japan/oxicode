//! Advanced LZ4 compression tests for oxicode — 22 test functions.
//!
//! Each test is individually gated on `compression-lz4`.

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
#[cfg(feature = "compression-lz4")]
use oxicode::{Decode, Encode};

#[cfg(feature = "compression-lz4")]
#[derive(Debug, PartialEq, Encode, Decode)]
struct SimpleStruct {
    id: u32,
    label: String,
}

#[cfg(feature = "compression-lz4")]
#[derive(Debug, PartialEq, Encode, Decode)]
enum SimpleEnum {
    Alpha,
    Beta(u32),
    Gamma(String),
}

// ──────────────────────────────────────────────────────────────────────────────
// 1. Compress/decompress u32 via encode then compress
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_u32_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: u32 = 987_654_321;
    let encoded = oxicode::encode_to_vec(&value).expect("encode u32 failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress u32 failed");
    let decompressed = decompress(&compressed).expect("decompress u32 failed");
    let (decoded, _): (u32, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode u32 failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 2. Compress/decompress empty Vec<u8>
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_empty_vec_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: Vec<u8> = vec![];
    let encoded = oxicode::encode_to_vec(&value).expect("encode empty vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress empty vec failed");
    let decompressed = decompress(&compressed).expect("decompress empty vec failed");
    let (decoded, _): (Vec<u8>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode empty vec failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 3. Compress/decompress String "hello"
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_hello_string_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value = String::from("hello");
    let encoded = oxicode::encode_to_vec(&value).expect("encode hello failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress hello failed");
    let decompressed = decompress(&compressed).expect("decompress hello failed");
    let (decoded, _): (String, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode hello failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 4. Compressed data decompresses to original bytes
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_decompress_to_original_bytes() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = b"oxicode lz4 byte identity check";
    let compressed = compress(original.as_ref(), Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    assert_eq!(original.as_ref(), decompressed.as_slice());
}

// ──────────────────────────────────────────────────────────────────────────────
// 5. Repeated data compresses well (ratio < 1.0 means compressed < original)
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_repeated_data_compression_ratio() {
    use oxicode::compression::{compress, Compression};

    let data: Vec<u8> = vec![0xABu8; 50_000];
    let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "Expected ratio < 1.0 for repeated data, got {ratio}"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// 6. Random-ish data compresses (output is Vec<u8>)
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_pseudo_random_data_compresses() {
    use oxicode::compression::{compress, decompress, Compression};

    // Pseudo-random pattern via simple LCG, no external crate needed.
    let mut state: u64 = 0x6C62272E07BB0142;
    let data: Vec<u8> = (0..4096)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 56) as u8
        })
        .collect();

    let encoded = oxicode::encode_to_vec(&data).expect("encode pseudo-random failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress pseudo-random failed");
    // Just verify the output is a non-empty Vec<u8>
    assert!(
        !compressed.is_empty(),
        "Compressed output must not be empty"
    );

    let decompressed = decompress(&compressed).expect("decompress pseudo-random failed");
    let (decoded, _): (Vec<u8>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode pseudo-random failed");
    assert_eq!(data, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 7. Large Vec<u8> 10 000 bytes compress/decompress
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_large_vec_u8_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let data: Vec<u8> = (0u32..10_000).map(|i| (i % 256) as u8).collect();
    let encoded = oxicode::encode_to_vec(&data).expect("encode large vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large vec failed");
    let decompressed = decompress(&compressed).expect("decompress large vec failed");
    let (decoded, _): (Vec<u8>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode large vec failed");
    assert_eq!(data, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 8. Unicode String compress/decompress
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_unicode_string_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value = String::from("日本語テスト: 🦀 Rust は素晴らしい！ αβγδεζηθ");
    let encoded = oxicode::encode_to_vec(&value).expect("encode unicode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress unicode failed");
    let decompressed = decompress(&compressed).expect("decompress unicode failed");
    let (decoded, _): (String, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode unicode failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 9. Bool compress/decompress
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_bool_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    for flag in [true, false] {
        let encoded = oxicode::encode_to_vec(&flag).expect("encode bool failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress bool failed");
        let decompressed = decompress(&compressed).expect("decompress bool failed");
        let (decoded, _): (bool, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode bool failed");
        assert_eq!(flag, decoded);
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 10. Vec<u32> compress/decompress
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_vec_u32_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let data: Vec<u32> = (0u32..256).collect();
    let encoded = oxicode::encode_to_vec(&data).expect("encode Vec<u32> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Vec<u32> failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<u32> failed");
    let (decoded, _): (Vec<u32>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<u32> failed");
    assert_eq!(data, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 11. Compress twice gives same bytes (deterministic)
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_compress_twice_same_output() {
    use oxicode::compression::{compress, Compression};

    let data = b"deterministic compression test payload";
    let c1 = compress(data.as_ref(), Compression::Lz4).expect("first compress failed");
    let c2 = compress(data.as_ref(), Compression::Lz4).expect("second compress failed");
    assert_eq!(c1, c2, "LZ4 compression must be deterministic");
}

// ──────────────────────────────────────────────────────────────────────────────
// 12. Compressed bytes differ from original bytes
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_compressed_differs_from_original() {
    use oxicode::compression::{compress, Compression};

    let data = b"The compressed form must differ from the raw input bytes.";
    let compressed = compress(data.as_ref(), Compression::Lz4).expect("compress failed");
    assert_ne!(
        data.as_ref(),
        compressed.as_slice(),
        "Compressed data must not equal the original bytes"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// 13. Decompress wrong / corrupted data fails gracefully
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_decompress_invalid_data_returns_error() {
    use oxicode::compression::decompress;

    // Garbage bytes with no valid OXC header.
    let garbage = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
    let result = decompress(&garbage);
    assert!(
        result.is_err(),
        "Decompressing garbage must return an error"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// 14. Struct roundtrip via compression
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_struct_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value = SimpleStruct {
        id: 42,
        label: String::from("oxicode-struct-test"),
    };
    let encoded = oxicode::encode_to_vec(&value).expect("encode struct failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress struct failed");
    let decompressed = decompress(&compressed).expect("decompress struct failed");
    let (decoded, _): (SimpleStruct, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode struct failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 15. Enum roundtrip via compression
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_enum_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let variants: Vec<SimpleEnum> = vec![
        SimpleEnum::Alpha,
        SimpleEnum::Beta(99),
        SimpleEnum::Gamma(String::from("gamma-variant")),
    ];

    for variant in variants {
        let encoded = oxicode::encode_to_vec(&variant).expect("encode enum failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress enum failed");
        let decompressed = decompress(&compressed).expect("decompress enum failed");
        let (decoded, _): (SimpleEnum, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode enum failed");
        assert_eq!(variant, decoded);
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 16. Option Some via compression
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_option_some_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: Option<u64> = Some(123_456_789_012_u64);
    let encoded = oxicode::encode_to_vec(&value).expect("encode Option::Some failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Option::Some failed");
    let decompressed = decompress(&compressed).expect("decompress Option::Some failed");
    let (decoded, _): (Option<u64>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Option::Some failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 17. Option None via compression
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_option_none_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: Option<u64> = None;
    let encoded = oxicode::encode_to_vec(&value).expect("encode Option::None failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Option::None failed");
    let decompressed = decompress(&compressed).expect("decompress Option::None failed");
    let (decoded, _): (Option<u64>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Option::None failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 18. Tuple (u32, String) via compression
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_tuple_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: (u32, String) = (777, String::from("tuple-test-value"));
    let encoded = oxicode::encode_to_vec(&value).expect("encode tuple failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress tuple failed");
    let decompressed = decompress(&compressed).expect("decompress tuple failed");
    let (decoded, _): ((u32, String), usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode tuple failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 19. Large String 1000 chars compress/decompress
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_large_string_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: String = "X".repeat(1_000);
    let encoded = oxicode::encode_to_vec(&value).expect("encode large string failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large string failed");
    let decompressed = decompress(&compressed).expect("decompress large string failed");
    let (decoded, _): (String, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode large string failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 20. i64 negative via compression
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_negative_i64_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: i64 = -9_876_543_210_i64;
    let encoded = oxicode::encode_to_vec(&value).expect("encode negative i64 failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress negative i64 failed");
    let decompressed = decompress(&compressed).expect("decompress negative i64 failed");
    let (decoded, _): (i64, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode negative i64 failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 21. u128 via compression
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_u128_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455_u128; // u128::MAX
    let encoded = oxicode::encode_to_vec(&value).expect("encode u128 failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress u128 failed");
    let decompressed = decompress(&compressed).expect("decompress u128 failed");
    let (decoded, _): (u128, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode u128 failed");
    assert_eq!(value, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// 22. Vec<String> via compression
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_adv2_vec_string_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let value: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
        String::from("delta"),
        String::from("epsilon"),
    ];
    let encoded = oxicode::encode_to_vec(&value).expect("encode Vec<String> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Vec<String> failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<String> failed");
    let (decoded, _): (Vec<String>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<String> failed");
    assert_eq!(value, decoded);
}
