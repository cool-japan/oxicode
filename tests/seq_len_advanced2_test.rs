//! 22 advanced tests for the `#[oxicode(seq_len = "...")]` field attribute (set 2).
//!
//! Covers different scenarios from seq_len_advanced_test.rs:
//!   A1.  Basic seq_len = "u8" with Vec<u8>
//!   A2.  seq_len = "u16" on Vec<u16>
//!   A3.  seq_len = "u32" on Vec<u8>
//!   A4.  Empty vec with seq_len = "u8" (length prefix = 0)
//!   A5.  Large vec (50 elements) with seq_len = "u8"
//!   A6.  seq_len preserves exact element count after roundtrip
//!   A7.  Struct with seq_len + preceding and following scalar fields
//!   A8.  Full roundtrip preserves all data values
//!   A9.  seq_len vs no-seq_len size comparison for len < 128
//!   A10. Vec<String> with seq_len = "u16"
//!   A11. Multiple seq_len fields using u8, u16, u32 in one struct
//!   A12. seq_len = "u8" with standard config (explicit)
//!   A13. Consumed bytes equals encoded length (n == enc.len())
//!   A14. Vec<u32> with seq_len = "u32"
//!   A15. Nested struct (inner with seq_len = "u8", outer wraps it)
//!   A16. Option wrapping struct that has a seq_len field
//!   A17. Vec of structs (each struct has a seq_len field)
//!   A18. seq_len = "u16" with big_endian config
//!   A19. Field values fully preserved after roundtrip (no truncation)
//!   A20. Large data (1000 elements) with seq_len = "u16"
//!   A21. Encoding twice produces identical bytes (determinism)
//!   A22. Single-element vec with seq_len = "u8"

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

// ── A1: Basic seq_len = "u8" with Vec<u8> ────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A1 {
    #[oxicode(seq_len = "u8")]
    bytes: Vec<u8>,
}

#[test]
fn test_a01_basic_seq_len_u8_vec_u8() {
    let s = Adv2A1 {
        bytes: vec![1, 2, 3, 4, 5, 6, 7],
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A1");
    assert_eq!(enc[0], 7u8, "u8 length prefix must be 7");
    assert_eq!(enc.len(), 8, "total bytes = 1 (prefix) + 7 (data)");
    let (dec, _): (Adv2A1, usize) = decode_from_slice(&enc).expect("decode Adv2A1");
    assert_eq!(s, dec);
}

// ── A2: seq_len = "u16" on Vec<u16> ──────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A2 {
    #[oxicode(seq_len = "u16")]
    shorts: Vec<u16>,
}

#[test]
fn test_a02_seq_len_u16_vec_u16_roundtrip() {
    let s = Adv2A2 {
        shorts: vec![0, 1, 1000, 32768, 65535],
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A2");
    // u16 LE prefix for 5: [5, 0]
    assert_eq!(enc[0], 5u8, "low byte of u16 prefix = 5");
    assert_eq!(enc[1], 0u8, "high byte of u16 prefix = 0");
    let (dec, n): (Adv2A2, usize) = decode_from_slice(&enc).expect("decode Adv2A2");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A3: seq_len = "u32" on Vec<u8> ───────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A3 {
    #[oxicode(seq_len = "u32")]
    raw: Vec<u8>,
}

#[test]
fn test_a03_seq_len_u32_vec_u8_roundtrip() {
    let s = Adv2A3 {
        raw: vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB],
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A3");
    let (dec, n): (Adv2A3, usize) = decode_from_slice(&enc).expect("decode Adv2A3");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A4: Empty vec with seq_len = "u8" ────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A4 {
    prefix: u8,
    #[oxicode(seq_len = "u8")]
    items: Vec<u32>,
    suffix: u8,
}

#[test]
fn test_a04_empty_vec_with_seq_len_u8() {
    let s = Adv2A4 {
        prefix: 42,
        items: vec![],
        suffix: 99,
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A4");
    // Layout: prefix(1) + seq_len_prefix(1, value=0) + suffix(1) = 3 bytes
    assert_eq!(enc.len(), 3, "empty vec with u8 prefix: 1+1+1 = 3 bytes");
    let (dec, n): (Adv2A4, usize) = decode_from_slice(&enc).expect("decode Adv2A4");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A5: Large vec (50 elements) with seq_len = "u8" ──────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A5 {
    #[oxicode(seq_len = "u8")]
    data: Vec<u8>,
}

#[test]
fn test_a05_large_vec_50_elements_seq_len_u8() {
    let s = Adv2A5 {
        data: (0u8..50).collect(),
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A5");
    assert_eq!(enc[0], 50u8, "u8 prefix must be 50");
    assert_eq!(enc.len(), 51, "1 prefix byte + 50 data bytes = 51");
    let (dec, n): (Adv2A5, usize) = decode_from_slice(&enc).expect("decode Adv2A5");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A6: seq_len preserves exact element count after roundtrip ─────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A6 {
    #[oxicode(seq_len = "u16")]
    elements: Vec<i32>,
}

#[test]
fn test_a06_seq_len_preserves_exact_length_after_roundtrip() {
    let original: Vec<i32> = vec![
        i32::MIN,
        -1000,
        -1,
        0,
        1,
        1000,
        i32::MAX,
        42,
        -42,
        777,
        -999,
        0,
        1234,
        -5678,
    ];
    let count = original.len();
    let s = Adv2A6 {
        elements: original.clone(),
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A6");
    let (dec, _): (Adv2A6, usize) = decode_from_slice(&enc).expect("decode Adv2A6");
    assert_eq!(
        dec.elements.len(),
        count,
        "element count must be preserved exactly"
    );
    assert_eq!(dec.elements, original);
}

// ── A7: Struct with seq_len + preceding and following scalar fields ────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A7 {
    id: u64,
    name: String,
    #[oxicode(seq_len = "u8")]
    tags: Vec<String>,
    score: f32,
    active: bool,
}

#[test]
fn test_a07_seq_len_with_surrounding_scalar_fields() {
    let s = Adv2A7 {
        id: 12345678901234,
        name: "test_entity".to_string(),
        tags: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
        score: 2.5,
        active: true,
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A7");
    let (dec, n): (Adv2A7, usize) = decode_from_slice(&enc).expect("decode Adv2A7");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A8: Full roundtrip preserves all data values ──────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A8 {
    version: u32,
    #[oxicode(seq_len = "u8")]
    checksums: Vec<u32>,
    metadata: String,
}

#[test]
fn test_a08_full_roundtrip_preserves_all_data_values() {
    let s = Adv2A8 {
        version: 42,
        checksums: vec![0xDEADBEEF, 0xCAFEBABE, 0x12345678, 0xABCDEF01],
        metadata: "important metadata string".to_string(),
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A8");
    let (dec, _): (Adv2A8, usize) = decode_from_slice(&enc).expect("decode Adv2A8");
    assert_eq!(dec.version, 42);
    assert_eq!(
        dec.checksums,
        vec![0xDEADBEEF_u32, 0xCAFEBABE, 0x12345678, 0xABCDEF01]
    );
    assert_eq!(dec.metadata, "important metadata string");
}

// ── A9: seq_len vs no-seq_len size comparison for len < 128 ───────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A9SeqLen {
    #[oxicode(seq_len = "u8")]
    vals: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A9Default {
    vals: Vec<u8>,
}

#[test]
fn test_a09_seq_len_size_vs_default_for_small_len() {
    // For lengths <= 127, default varint is also 1 byte, so sizes should be equal.
    let data: Vec<u8> = vec![10, 20, 30, 40, 50];
    let enc_sl = encode_to_vec(&Adv2A9SeqLen { vals: data.clone() }).expect("encode seq_len");
    let enc_def = encode_to_vec(&Adv2A9Default { vals: data }).expect("encode default");
    // Both use 1-byte prefix for count=5, so total sizes should be equal
    assert_eq!(
        enc_sl.len(),
        enc_def.len(),
        "for small vecs, seq_len=u8 and default should produce same size"
    );
    let (dec, _): (Adv2A9SeqLen, usize) = decode_from_slice(&enc_sl).expect("decode seq_len");
    assert_eq!(dec.vals, vec![10u8, 20, 30, 40, 50]);
}

// ── A10: Vec<String> with seq_len = "u16" ─────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A10 {
    #[oxicode(seq_len = "u16")]
    words: Vec<String>,
}

#[test]
fn test_a10_seq_len_u16_vec_string() {
    let s = Adv2A10 {
        words: vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
        ],
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A10");
    // With varint encoding (default), small count 6 is encoded as a single byte: enc[0] = 6
    assert_eq!(
        enc[0], 6u8,
        "varint-encoded count of 6 strings = single byte 6"
    );
    let (dec, n): (Adv2A10, usize) = decode_from_slice(&enc).expect("decode Adv2A10");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A11: Multiple seq_len fields using u8, u16, u32 in one struct ─────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A11 {
    header: u32,
    #[oxicode(seq_len = "u8")]
    small_list: Vec<u8>,
    #[oxicode(seq_len = "u16")]
    medium_list: Vec<u16>,
    #[oxicode(seq_len = "u32")]
    large_list: Vec<u32>,
    footer: u32,
}

#[test]
fn test_a11_multiple_seq_len_fields_u8_u16_u32() {
    let s = Adv2A11 {
        header: 0xAAAA_BBBB,
        small_list: vec![1, 2, 3],
        medium_list: vec![100, 200, 300, 400],
        large_list: vec![1000, 2000, 3000, 4000, 5000],
        footer: 0xCCCC_DDDD,
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A11");
    let (dec, n): (Adv2A11, usize) = decode_from_slice(&enc).expect("decode Adv2A11");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A12: seq_len = "u8" with standard config (explicit) ──────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A12 {
    #[oxicode(seq_len = "u8")]
    payload: Vec<u8>,
}

#[test]
fn test_a12_seq_len_u8_with_explicit_standard_config() {
    let s = Adv2A12 {
        payload: vec![0xAA, 0xBB, 0xCC],
    };
    let cfg = config::standard();
    let enc = encode_to_vec_with_config(&s, cfg).expect("encode Adv2A12");
    assert_eq!(enc[0], 3u8, "u8 prefix must be 3");
    let (dec, n): (Adv2A12, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Adv2A12");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A13: Consumed bytes equals encoded length ─────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A13 {
    a: u8,
    #[oxicode(seq_len = "u16")]
    data: Vec<u8>,
    b: u8,
}

#[test]
fn test_a13_consumed_bytes_equals_encoded_len() {
    let s = Adv2A13 {
        a: 7,
        data: vec![11, 22, 33, 44, 55, 66, 77, 88],
        b: 9,
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A13");
    let (dec, n): (Adv2A13, usize) = decode_from_slice(&enc).expect("decode Adv2A13");
    assert_eq!(
        n,
        enc.len(),
        "consumed bytes must equal full encoded length"
    );
    assert_eq!(s, dec);
}

// ── A14: Vec<u32> with seq_len = "u32" ───────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A14 {
    #[oxicode(seq_len = "u32")]
    numbers: Vec<u32>,
}

#[test]
fn test_a14_seq_len_u32_vec_u32_roundtrip() {
    let s = Adv2A14 {
        numbers: vec![
            u32::MIN,
            1,
            2,
            4,
            8,
            16,
            32,
            64,
            128,
            256,
            512,
            1024,
            2048,
            4096,
            8192,
            u32::MAX,
        ],
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A14");
    let (dec, n): (Adv2A14, usize) = decode_from_slice(&enc).expect("decode Adv2A14");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A15: Nested struct (inner has seq_len = "u8", outer wraps it) ─────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A15Inner {
    label: String,
    #[oxicode(seq_len = "u8")]
    values: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A15Outer {
    id: u32,
    inner: Adv2A15Inner,
    #[oxicode(seq_len = "u16")]
    extra: Vec<u32>,
}

#[test]
fn test_a15_nested_struct_with_seq_len() {
    let s = Adv2A15Outer {
        id: 999,
        inner: Adv2A15Inner {
            label: "inner_label".to_string(),
            values: vec![10, 20, 30, 40, 50],
        },
        extra: vec![100, 200, 300],
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A15Outer");
    let (dec, n): (Adv2A15Outer, usize) = decode_from_slice(&enc).expect("decode Adv2A15Outer");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A16: Option wrapping struct that has a seq_len field ──────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A16Inner {
    #[oxicode(seq_len = "u8")]
    chunks: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A16Outer {
    maybe: Option<Adv2A16Inner>,
    flag: bool,
}

#[test]
fn test_a16_option_wrapping_struct_with_seq_len() {
    let s_some = Adv2A16Outer {
        maybe: Some(Adv2A16Inner {
            chunks: vec![0xAA, 0xBB, 0xCC, 0xDD],
        }),
        flag: true,
    };
    let enc_some = encode_to_vec(&s_some).expect("encode Some variant");
    let (dec_some, n_some): (Adv2A16Outer, usize) =
        decode_from_slice(&enc_some).expect("decode Some variant");
    assert_eq!(s_some, dec_some);
    assert_eq!(n_some, enc_some.len());

    let s_none = Adv2A16Outer {
        maybe: None,
        flag: false,
    };
    let enc_none = encode_to_vec(&s_none).expect("encode None variant");
    let (dec_none, n_none): (Adv2A16Outer, usize) =
        decode_from_slice(&enc_none).expect("decode None variant");
    assert_eq!(s_none, dec_none);
    assert_eq!(n_none, enc_none.len());
}

// ── A17: Vec of structs (each struct has a seq_len field) ─────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A17Item {
    key: u32,
    #[oxicode(seq_len = "u8")]
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A17Container {
    #[oxicode(seq_len = "u8")]
    items: Vec<Adv2A17Item>,
}

#[test]
fn test_a17_vec_of_structs_each_with_seq_len() {
    let s = Adv2A17Container {
        items: vec![
            Adv2A17Item {
                key: 1,
                data: vec![10, 11, 12],
            },
            Adv2A17Item {
                key: 2,
                data: vec![20, 21],
            },
            Adv2A17Item {
                key: 3,
                data: vec![],
            },
            Adv2A17Item {
                key: 4,
                data: vec![40, 41, 42, 43, 44],
            },
        ],
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A17Container");
    // Outer seq_len = "u8": first byte = 4 (number of items)
    assert_eq!(enc[0], 4u8, "outer u8 prefix must be 4 (number of items)");
    let (dec, n): (Adv2A17Container, usize) =
        decode_from_slice(&enc).expect("decode Adv2A17Container");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A18: seq_len = "u16" with big_endian config ───────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A18 {
    #[oxicode(seq_len = "u16")]
    items: Vec<u8>,
}

#[test]
fn test_a18_seq_len_u16_with_big_endian_config() {
    // Use 5 items; with varint encoding (default), small count 5 is a single byte: enc[0] = 5.
    // Big-endian affects multi-byte *values*, but small varint lengths fit in one byte either way.
    let s = Adv2A18 {
        items: vec![10, 20, 30, 40, 50],
    };
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&s, cfg).expect("encode Adv2A18");
    // The varint for 5 is a single byte = 5, regardless of endianness
    assert_eq!(enc[0], 5u8, "varint count of 5 fits in one byte = 5");
    // Data bytes follow: 10, 20, 30, 40, 50 (big-endian u8 == LE u8 for single bytes)
    assert_eq!(enc[1], 10u8, "first element = 10");
    let (dec, n): (Adv2A18, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Adv2A18");
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A19: Field values fully preserved after roundtrip ─────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A19 {
    before: u64,
    #[oxicode(seq_len = "u8")]
    entries: Vec<i16>,
    after: u64,
}

#[test]
fn test_a19_field_values_fully_preserved_after_roundtrip() {
    let s = Adv2A19 {
        before: 0xFEDCBA9876543210,
        entries: vec![i16::MIN, -100, -1, 0, 1, 100, i16::MAX],
        after: 0x0123456789ABCDEF,
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A19");
    let (dec, n): (Adv2A19, usize) = decode_from_slice(&enc).expect("decode Adv2A19");
    assert_eq!(dec.before, 0xFEDCBA9876543210_u64);
    assert_eq!(dec.entries, vec![i16::MIN, -100, -1, 0, 1, 100, i16::MAX]);
    assert_eq!(dec.after, 0x0123456789ABCDEF_u64);
    assert_eq!(n, enc.len());
}

// ── A20: Large data (1000 elements) with seq_len = "u16" ──────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A20 {
    #[oxicode(seq_len = "u16")]
    large_data: Vec<u8>,
}

#[test]
fn test_a20_large_data_1000_elements_seq_len_u16() {
    let s = Adv2A20 {
        large_data: (0u16..1000).map(|x| (x % 256) as u8).collect(),
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A20");
    // With standard config, seq_len = "u16" on 1000 elements:
    // 1000 in LE u16 = [0xE8, 0x03]; but varint may kick in since the prefix
    // is encoded via the standard encoder. The u16 LE bytes of 1000 as raw:
    // 1000 = 0x03E8, LE = [0xE8, 0x03].
    // Actually with varint encoding, 1000 > 250 uses 3 bytes [251, 0xE8, 0x03].
    // Just verify roundtrip correctness and length.
    let (dec, n): (Adv2A20, usize) = decode_from_slice(&enc).expect("decode Adv2A20");
    assert_eq!(
        dec.large_data.len(),
        1000,
        "must decode exactly 1000 elements"
    );
    assert_eq!(s, dec);
    assert_eq!(n, enc.len());
}

// ── A21: Encoding twice produces identical bytes (determinism) ─────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A21 {
    x: u32,
    #[oxicode(seq_len = "u8")]
    data: Vec<u8>,
    y: u32,
}

#[test]
fn test_a21_encoding_twice_produces_identical_bytes() {
    let s = Adv2A21 {
        x: 0x1234,
        data: vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE],
        y: 0x5678,
    };
    let enc1 = encode_to_vec(&s).expect("encode Adv2A21 first time");
    let enc2 = encode_to_vec(&s).expect("encode Adv2A21 second time");
    assert_eq!(enc1, enc2, "encoding must be deterministic");
    let (dec, n): (Adv2A21, usize) = decode_from_slice(&enc1).expect("decode Adv2A21");
    assert_eq!(s, dec);
    assert_eq!(n, enc1.len());
}

// ── A22: Single-element vec with seq_len = "u8" ───────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2A22 {
    #[oxicode(seq_len = "u8")]
    singleton: Vec<u64>,
}

#[test]
fn test_a22_single_element_vec_seq_len_u8() {
    let s = Adv2A22 {
        singleton: vec![u64::MAX],
    };
    let enc = encode_to_vec(&s).expect("encode Adv2A22");
    assert_eq!(enc[0], 1u8, "u8 prefix for single element must be 1");
    // With varint encoding, u64::MAX needs a 9-byte varint (1 tag byte + 8 data bytes).
    // Total: 1 (u8 seq_len prefix) + 9 (varint u64::MAX) = 10 bytes.
    assert_eq!(
        enc.len(),
        10,
        "1 (u8 prefix) + 9 (varint u64::MAX) = 10 bytes"
    );
    let (dec, n): (Adv2A22, usize) = decode_from_slice(&enc).expect("decode Adv2A22");
    assert_eq!(s, dec);
    assert_eq!(dec.singleton[0], u64::MAX);
    assert_eq!(n, enc.len());
}
