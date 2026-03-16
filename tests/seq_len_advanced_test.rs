//! 22 advanced tests for the `#[oxicode(seq_len = "...")]` field attribute.
//!
//! Note: `seq_len` is only valid on `Vec<T>` fields — it controls the fixed-width
//! integer type used as the length prefix instead of the default varint encoding.
//!
//! Covers:
//!   1.  seq_len = "u8" on Vec<u8> with 10 elements — verify 1-byte prefix in data
//!   2.  seq_len = "u8" on Vec<u8> with 255 elements — max u8 boundary
//!   3.  seq_len = "u16" on Vec<u32> roundtrip
//!   4.  seq_len = "u32" on Vec<String> roundtrip
//!   5.  seq_len = "u8" on Vec<u8> with 5 elements — compact prefix value check
//!   6.  seq_len = "u16" on Vec<u8> with 100 elements — 2-byte length prefix
//!   7.  Struct with multiple seq_len fields (u8 + u16 + u32)
//!   8.  seq_len = "u8" vs default — encoded size comparison for short vecs
//!   9.  seq_len = "u8" on Vec<u64>
//!   10. seq_len = "u16" on Vec<bool>
//!   11. seq_len = "u8" on empty Vec<u8>
//!   12. seq_len = "u8" on Vec<i8>
//!   13. seq_len = "u32" on large Vec<u8> (1000 elements)
//!   14. Nested struct — each level with different seq_len
//!   15. seq_len = "u8" on Vec<Vec<u8>>
//!   16. seq_len = "u16" on Vec<Option<u32>>
//!   17. Struct with both seq_len and skip fields
//!   18. seq_len = "u8" on Vec<String>
//!   19. Compact struct — all sequence fields with seq_len = "u8"
//!   20. Verify encoded size is smaller with seq_len = "u8" vs default for length >= 128
//!   21. seq_len = "u8" with fixed-int (legacy) encoding config
//!   22. seq_len = "u16" with big-endian config

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ── Test 1: seq_len = "u8" on Vec<u8> with 10 elements — 1-byte prefix ────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test01 {
    #[oxicode(seq_len = "u8")]
    data: Vec<u8>,
}

#[test]
fn test_01_seq_len_u8_vec_u8_ten_elements_prefix_byte() {
    let s = Test01 {
        data: vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
    };
    let enc = encode_to_vec(&s).expect("encode");
    // First byte must be the raw u8 length = 10
    assert_eq!(enc[0], 10u8, "first byte must be u8 length prefix = 10");
    let (dec, n): (Test01, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 2: seq_len = "u8" on Vec<u8> with 255 elements — max u8 boundary ─────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test02 {
    #[oxicode(seq_len = "u8")]
    data: Vec<u8>,
}

#[test]
fn test_02_seq_len_u8_vec_u8_max_255_elements() {
    let s = Test02 {
        data: (0u8..=254).collect(),
    };
    let enc = encode_to_vec(&s).expect("encode");
    assert_eq!(enc[0], 255u8, "length prefix must be 255 for 255 elements");
    let (dec, n): (Test02, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 3: seq_len = "u16" on Vec<u32> roundtrip ─────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test03 {
    #[oxicode(seq_len = "u16")]
    numbers: Vec<u32>,
}

#[test]
fn test_03_seq_len_u16_vec_u32_roundtrip() {
    let s = Test03 {
        numbers: (0u32..500).collect(),
    };
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, n): (Test03, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 4: seq_len = "u32" on Vec<String> roundtrip ──────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test04 {
    #[oxicode(seq_len = "u32")]
    labels: Vec<String>,
}

#[test]
fn test_04_seq_len_u32_vec_string_roundtrip() {
    let s = Test04 {
        labels: vec![
            "alpha".into(),
            "beta".into(),
            "gamma".into(),
            "delta".into(),
        ],
    };
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, n): (Test04, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 5: seq_len = "u8" on Vec<u8> with 5 elements — compact prefix value ──

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test05 {
    #[oxicode(seq_len = "u8")]
    items: Vec<u8>,
}

#[test]
fn test_05_seq_len_u8_vec_five_elements_compact_prefix() {
    let s = Test05 {
        items: vec![11, 22, 33, 44, 55],
    };
    let enc = encode_to_vec(&s).expect("encode");
    // First byte must be the raw u8 length = 5; total = 1 + 5 = 6 bytes
    assert_eq!(enc[0], 5u8, "first byte must be u8 length prefix = 5");
    assert_eq!(
        enc.len(),
        6,
        "total encoded bytes = 1 (prefix) + 5 (elements)"
    );
    let (dec, n): (Test05, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 6: seq_len = "u16" on Vec<u8> with 100 elements — 2-byte prefix ──────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test06 {
    #[oxicode(seq_len = "u16")]
    payload: Vec<u8>,
}

#[test]
fn test_06_seq_len_u16_vec_u8_100_elements_roundtrip() {
    let s = Test06 {
        payload: (0u8..100).collect(),
    };
    let enc = encode_to_vec(&s).expect("encode");
    // u16 little-endian prefix for 100: [100, 0]
    assert_eq!(enc[0], 100u8, "low byte of u16 length prefix = 100");
    assert_eq!(enc[1], 0u8, "high byte of u16 length prefix = 0");
    let (dec, n): (Test06, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 7: Struct with multiple seq_len fields ───────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test07 {
    #[oxicode(seq_len = "u8")]
    tags: Vec<String>,
    #[oxicode(seq_len = "u16")]
    values: Vec<u32>,
    #[oxicode(seq_len = "u32")]
    payload: Vec<u8>,
}

#[test]
fn test_07_multiple_seq_len_fields_roundtrip() {
    let s = Test07 {
        tags: vec!["rust".into(), "fast".into(), "safe".into()],
        values: vec![1, 2, 3, 4, 5, 6, 7, 8],
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, n): (Test07, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 8: seq_len = "u8" produces smaller or equal encoding vs default ───────

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSeqLen08 {
    #[oxicode(seq_len = "u8")]
    items: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithoutSeqLen08 {
    items: Vec<u8>,
}

#[test]
fn test_08_seq_len_u8_smaller_or_equal_to_default_for_short_vecs() {
    let short_data: Vec<u8> = vec![1, 2, 3, 4, 5];

    let enc_compact = encode_to_vec(&WithSeqLen08 {
        items: short_data.clone(),
    })
    .expect("encode compact");
    let enc_default =
        encode_to_vec(&WithoutSeqLen08 { items: short_data }).expect("encode default");

    // u8 prefix = 1 byte; default varint for 5 = 1 byte — equal or compact wins.
    assert!(
        enc_compact.len() <= enc_default.len(),
        "compact seq_len=u8 ({} bytes) should be <= default ({} bytes)",
        enc_compact.len(),
        enc_default.len()
    );
    let (dec, _): (WithSeqLen08, _) = decode_from_slice(&enc_compact).expect("decode compact");
    assert_eq!(dec.items, vec![1u8, 2, 3, 4, 5]);
}

// ── Test 9: seq_len = "u8" on Vec<u64> ───────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test09 {
    #[oxicode(seq_len = "u8")]
    longs: Vec<u64>,
}

#[test]
fn test_09_seq_len_u8_vec_u64_roundtrip() {
    let s = Test09 {
        longs: vec![u64::MAX, u64::MIN, 42, 0xDEAD_BEEF_CAFE_BABE],
    };
    let enc = encode_to_vec(&s).expect("encode");
    assert_eq!(enc[0], 4u8, "length prefix must be 4");
    let (dec, n): (Test09, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 10: seq_len = "u16" on Vec<bool> ────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test10 {
    #[oxicode(seq_len = "u16")]
    flags: Vec<bool>,
}

#[test]
fn test_10_seq_len_u16_vec_bool_roundtrip() {
    let s = Test10 {
        flags: vec![true, false, true, true, false, false, true],
    };
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, n): (Test10, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 11: seq_len = "u8" on empty Vec<u8> ─────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test11 {
    #[oxicode(seq_len = "u8")]
    data: Vec<u8>,
}

#[test]
fn test_11_seq_len_u8_empty_vec_u8() {
    let s = Test11 { data: vec![] };
    let enc = encode_to_vec(&s).expect("encode");
    assert_eq!(enc[0], 0u8, "length prefix of empty vec must be 0");
    assert_eq!(enc.len(), 1, "empty vec with u8 prefix = exactly 1 byte");
    let (dec, n): (Test11, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 12: seq_len = "u8" on Vec<i8> ───────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test12 {
    #[oxicode(seq_len = "u8")]
    signed_bytes: Vec<i8>,
}

#[test]
fn test_12_seq_len_u8_vec_i8_roundtrip() {
    let s = Test12 {
        signed_bytes: vec![-128, -1, 0, 1, 127],
    };
    let enc = encode_to_vec(&s).expect("encode");
    assert_eq!(enc[0], 5u8, "u8 prefix must be 5 for 5 elements");
    let (dec, n): (Test12, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 13: seq_len = "u32" on large Vec<u8> (1000 elements) ─────────────────
//
// Note: the seq_len prefix is encoded via the standard encoder (varint for standard config).
// For 1000 (> 250), the varint encoding is [251 (U16_BYTE), lo, hi] = [251, 0xE8, 0x03]
// in little-endian form, and the total encoded size = 3 (prefix) + 1000 (data) = 1003 bytes.

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test13 {
    #[oxicode(seq_len = "u32")]
    payload: Vec<u8>,
}

#[test]
fn test_13_seq_len_u32_large_vec_1000_elements() {
    let s = Test13 {
        payload: (0u16..1000).map(|x| (x % 256) as u8).collect(),
    };
    let enc = encode_to_vec(&s).expect("encode");
    // With standard config (varint), u32 length 1000 encodes as [U16_BYTE=251, 0xE8, 0x03]
    // because 1000 > SINGLE_BYTE_MAX (250) and fits in u16.
    assert_eq!(
        enc[0], 251u8,
        "varint marker U16_BYTE = 251 for length 1000"
    );
    assert_eq!(enc[1], 0xE8u8, "low byte of 1000 in little-endian = 0xE8");
    assert_eq!(enc[2], 0x03u8, "high byte of 1000 in little-endian = 0x03");
    assert_eq!(
        enc.len(),
        3 + 1000,
        "total = 3 (varint prefix) + 1000 (data bytes)"
    );
    let (dec, n): (Test13, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 14: Nested struct — each level with different seq_len ─────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerNested14 {
    #[oxicode(seq_len = "u8")]
    bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OuterNested14 {
    #[oxicode(seq_len = "u16")]
    items: Vec<u32>,
    inner: InnerNested14,
    #[oxicode(seq_len = "u32")]
    tags: Vec<String>,
}

#[test]
fn test_14_nested_struct_each_level_different_seq_len() {
    let s = OuterNested14 {
        items: vec![100, 200, 300],
        inner: InnerNested14 {
            bytes: vec![0xAA, 0xBB, 0xCC],
        },
        tags: vec!["one".into(), "two".into()],
    };
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, n): (OuterNested14, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 15: seq_len = "u8" on Vec<Vec<u8>> ──────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test15 {
    #[oxicode(seq_len = "u8")]
    chunks: Vec<Vec<u8>>,
}

#[test]
fn test_15_seq_len_u8_vec_of_vec_u8() {
    let s = Test15 {
        chunks: vec![vec![1, 2, 3], vec![4, 5], vec![], vec![9]],
    };
    let enc = encode_to_vec(&s).expect("encode");
    // Outer length prefix = 4 (number of inner vecs)
    assert_eq!(
        enc[0], 4u8,
        "outer u8 prefix must equal number of inner vecs (4)"
    );
    let (dec, n): (Test15, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 16: seq_len = "u16" on Vec<Option<u32>> ─────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test16 {
    #[oxicode(seq_len = "u16")]
    optionals: Vec<Option<u32>>,
}

#[test]
fn test_16_seq_len_u16_vec_option_u32() {
    let s = Test16 {
        optionals: vec![Some(1), None, Some(42), None, Some(u32::MAX)],
    };
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, n): (Test16, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 17: Struct with both seq_len and skip fields ─────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test17 {
    id: u32,
    #[oxicode(skip)]
    cache: u64,
    #[oxicode(seq_len = "u8")]
    tags: Vec<String>,
    #[oxicode(skip)]
    temp: bool,
    #[oxicode(seq_len = "u16")]
    scores: Vec<i32>,
}

#[test]
fn test_17_seq_len_combined_with_skip_fields() {
    let original = Test17 {
        id: 99,
        cache: 0xDEAD_BEEF,
        tags: vec!["foo".into(), "bar".into()],
        temp: true,
        scores: vec![-10, 0, 10, 20],
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, n): (Test17, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.id, 99);
    assert_eq!(dec.cache, 0u64, "skipped cache must be Default (0)");
    assert_eq!(dec.tags, vec!["foo".to_string(), "bar".to_string()]);
    assert!(!dec.temp, "skipped temp must be Default (false)");
    assert_eq!(dec.scores, vec![-10, 0, 10, 20]);
    assert_eq!(n, enc.len());
}

// ── Test 18: seq_len = "u8" on Vec<String> ───────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test18 {
    #[oxicode(seq_len = "u8")]
    words: Vec<String>,
}

#[test]
fn test_18_seq_len_u8_vec_string_roundtrip() {
    let s = Test18 {
        words: vec!["hello".into(), "world".into(), "oxicode".into()],
    };
    let enc = encode_to_vec(&s).expect("encode");
    assert_eq!(enc[0], 3u8, "u8 prefix must equal element count = 3");
    let (dec, n): (Test18, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 19: Compact struct — all sequence fields with seq_len = "u8" ─────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct CompactAll19 {
    #[oxicode(seq_len = "u8")]
    ids: Vec<u8>,
    #[oxicode(seq_len = "u8")]
    names: Vec<String>,
    #[oxicode(seq_len = "u8")]
    flags: Vec<bool>,
    #[oxicode(seq_len = "u8")]
    scores: Vec<u32>,
}

#[test]
fn test_19_compact_struct_all_fields_seq_len_u8() {
    let s = CompactAll19 {
        ids: vec![1, 2, 3, 4, 5],
        names: vec!["alice".into(), "bob".into()],
        flags: vec![true, false, true],
        scores: vec![100, 200, 150],
    };
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, n): (CompactAll19, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── Test 20: Encoded size is smaller with seq_len = "u8" vs default for length > 250
//
// The varint threshold is SINGLE_BYTE_MAX = 250. Lengths 0-250 use 1 byte in both modes.
// For lengths > 250, the default varint uses 3 bytes [251, lo, hi], but seq_len="u8"
// uses exactly 1 byte — saving 2 bytes.

#[derive(Debug, PartialEq, Encode, Decode)]
struct SmallSeqLen20 {
    #[oxicode(seq_len = "u8")]
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DefaultLen20 {
    data: Vec<u8>,
}

#[test]
fn test_20_encoded_size_smaller_with_seq_len_u8_for_length_gt_250() {
    // Use exactly SINGLE_BYTE_MAX + 1 = 251 elements: the default varint will use 3 bytes
    // for the prefix ([251, 251, 0] as little-endian), while seq_len="u8" uses 1 byte.
    // However, u8 can only represent up to 255, so 251 is within range.
    let data: Vec<u8> = (0u8..=250).collect(); // 251 elements

    let enc_compact = encode_to_vec(&SmallSeqLen20 { data: data.clone() }).expect("encode compact");
    let enc_default = encode_to_vec(&DefaultLen20 { data }).expect("encode default");

    // compact: 1 byte prefix + 251 bytes = 252 bytes
    // default: 3 bytes prefix (varint for 251) + 251 bytes = 254 bytes
    assert!(
        enc_compact.len() < enc_default.len(),
        "compact ({} bytes) must be smaller than default ({} bytes) for length 251",
        enc_compact.len(),
        enc_default.len()
    );
    assert_eq!(
        enc_compact.len(),
        1 + 251,
        "seq_len=u8: 1 prefix byte + 251 data bytes"
    );
    assert_eq!(
        enc_default.len(),
        3 + 251,
        "default varint: 3 prefix bytes + 251 data bytes"
    );
    let (dec, n): (SmallSeqLen20, _) = decode_from_slice(&enc_compact).expect("decode");
    assert_eq!(dec.data.len(), 251);
    assert_eq!(n, enc_compact.len());
}

// ── Test 21: seq_len = "u8" with fixed-int (legacy) encoding config ───────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test21 {
    version: u32,
    #[oxicode(seq_len = "u8")]
    payload: Vec<u8>,
}

#[test]
fn test_21_seq_len_u8_with_legacy_config() {
    let s = Test21 {
        version: 1,
        payload: vec![10, 20, 30, 40, 50],
    };
    let cfg = config::legacy();
    let enc = encode_to_vec_with_config(&s, cfg).expect("encode");
    let (dec, n): (Test21, _) = decode_from_slice_with_config(&enc, cfg).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
    // With legacy config: version (u32) = 4 bytes; seq_len = "u8" prefix = 1 byte; 5 bytes payload
    assert_eq!(
        enc.len(),
        4 + 1 + 5,
        "legacy: 4 (u32 fixed) + 1 (u8 prefix) + 5 (payload)"
    );
}

// ── Test 22: seq_len = "u16" with big-endian config ──────────────────────────
//
// With standard config (varint), small values (≤ 250) are encoded as a single byte
// regardless of endianness. For 3 items, the varint encodes as [3] (1 byte).
// To exercise the 3-byte varint path with big-endian, use 251 items (> SINGLE_BYTE_MAX).
// Then the varint prefix is [U16_BYTE=251, hi, lo] in big-endian order.

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test22 {
    #[oxicode(seq_len = "u16")]
    items: Vec<u8>,
}

#[test]
fn test_22_seq_len_u16_with_big_endian_config() {
    // 251 items: varint of 251 in big-endian = [U16_BYTE=251, 0x00, 0xFB]
    let s = Test22 {
        items: (0u8..=250).collect(), // 251 items
    };
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&s, cfg).expect("encode");
    // varint for 251 with big-endian: [251 (U16_BYTE), 0x00, 0xFB]
    assert_eq!(enc[0], 251u8, "first byte is U16_BYTE marker = 251");
    assert_eq!(enc[1], 0x00u8, "big-endian high byte of 251 = 0x00");
    assert_eq!(enc[2], 0xFBu8, "big-endian low byte of 251 = 0xFB");
    let (dec, n): (Test22, _) = decode_from_slice_with_config(&enc, cfg).expect("decode");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}
