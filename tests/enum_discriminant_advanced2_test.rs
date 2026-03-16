//! Advanced tests for enum discriminant encoding in OxiCode — set 2.
//!
//! 22 top-level #[test] functions covering unit enums, C-style enums with
//! explicit discriminants, tuple/struct/mixed variants, large discriminants,
//! payload size differences, container types, nested enums, error handling,
//! tag_type attribute, and config interactions.

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

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

/// Simple 4-variant unit enum — used in tests 1, 2, 8, 10, 11, 12.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

/// C-style enum with explicit discriminants — used in test 3.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Proto {
    #[oxicode(variant = 0)]
    Tcp,
    #[oxicode(variant = 10)]
    Udp,
    #[oxicode(variant = 20)]
    Icmp,
}

/// Enum with a single tuple variant — used in tests 4, 9.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Wrapper {
    Empty,
    Int(i64),
}

/// Enum with a single struct variant — used in test 5.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Positioned {
    Origin,
    At { x: i32, y: i32 },
}

/// Mixed enum (unit + tuple + struct) — used in tests 6, 21.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Mixed {
    Unit,
    Pair(u32, u32),
    Named { label: String, value: u64 },
}

/// Enum with variant index 100 (large discriminant) — used in test 7.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Sparse {
    #[oxicode(variant = 0)]
    First,
    #[oxicode(variant = 100)]
    Hundredth,
}

/// Enum with a String payload — used in test 13.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Named {
    Empty,
    WithStr(String),
}

/// Enum with a Vec<u8> payload — used in test 14.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Buffered {
    None,
    Data(Vec<u8>),
}

/// Inner enum used for nesting — used in test 15.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Polarity {
    Positive,
    Negative,
}

/// Enum with nested enum payload — used in test 15.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Signal {
    Off,
    On(Polarity),
}

/// Enum with u32 tag_type — used in test 17.
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u32")]
enum TaggedU32 {
    Alpha,
    Beta,
    Gamma,
}

/// 2-variant unit enum — used in test 18.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Binary {
    Off,
    On,
}

/// Enum with tuple variant containing multiple fields — used in test 19.
#[derive(Debug, PartialEq, Encode, Decode)]
enum MultiTuple {
    None,
    Triple(u8, u16, u32),
}

/// Enum with struct variant containing multiple fields — used in test 20.
#[derive(Debug, PartialEq, Encode, Decode)]
enum MultiStruct {
    Empty,
    Record { id: u64, name: String, active: bool },
}

// ---------------------------------------------------------------------------
// Test 1: Simple unit enum (4 variants) — all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_01_unit_enum_all_variants_roundtrip() {
    let variants = [
        Cardinal::North,
        Cardinal::South,
        Cardinal::East,
        Cardinal::West,
    ];
    for (idx, variant) in variants.iter().enumerate() {
        let encoded = encode_to_vec(variant).expect("encode Cardinal failed");
        let (decoded, consumed): (Cardinal, usize) =
            decode_from_slice(&encoded).expect("decode Cardinal failed");
        assert_eq!(
            &decoded, variant,
            "Cardinal variant {idx} roundtrip mismatch"
        );
        assert_eq!(
            consumed,
            encoded.len(),
            "Cardinal variant {idx} consumed bytes mismatch"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 2: Unit enum discriminant 0 encodes as varint 0
// ---------------------------------------------------------------------------

#[test]
fn test_02_unit_enum_discriminant_zero_is_varint_zero() {
    let encoded = encode_to_vec(&Cardinal::North).expect("encode Cardinal::North failed");
    // varint(0) = single byte 0x00
    assert_eq!(
        encoded.len(),
        1,
        "Cardinal::North must encode to exactly 1 byte"
    );
    assert_eq!(
        encoded[0], 0x00u8,
        "Cardinal::North discriminant must be 0x00"
    );

    let (decoded, consumed): (Cardinal, usize) =
        decode_from_slice(&encoded).expect("decode Cardinal::North failed");
    assert_eq!(
        decoded,
        Cardinal::North,
        "Cardinal::North roundtrip mismatch"
    );
    assert_eq!(consumed, 1, "Cardinal::North must consume exactly 1 byte");
}

// ---------------------------------------------------------------------------
// Test 3: C-style enum with explicit discriminants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_03_c_style_enum_explicit_discriminants_roundtrip() {
    // Proto::Tcp has variant=0, Proto::Udp has variant=10, Proto::Icmp has variant=20.
    let cases = [(Proto::Tcp, 0u8), (Proto::Udp, 10u8), (Proto::Icmp, 20u8)];
    for (variant, expected_disc) in &cases {
        let encoded = encode_to_vec(variant).expect("encode Proto failed");
        // Discriminants 0, 10, 20 each fit in a single varint byte.
        assert_eq!(
            encoded[0], *expected_disc,
            "Proto::{variant:?} discriminant byte mismatch"
        );
        let (decoded, consumed): (Proto, usize) =
            decode_from_slice(&encoded).expect("decode Proto failed");
        assert_eq!(&decoded, variant, "Proto::{variant:?} roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "Proto::{variant:?} consumed bytes mismatch"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 4: Enum with tuple variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_04_enum_tuple_variant_roundtrip() {
    let empty = Wrapper::Empty;
    let with_int = Wrapper::Int(i64::MIN);

    for variant in [&empty, &with_int] {
        let encoded = encode_to_vec(variant).expect("encode Wrapper failed");
        let (decoded, consumed): (Wrapper, usize) =
            decode_from_slice(&encoded).expect("decode Wrapper failed");
        assert_eq!(&decoded, variant, "Wrapper roundtrip mismatch");
        assert_eq!(consumed, encoded.len(), "Wrapper consumed bytes mismatch");
    }

    // Discriminant check: Wrapper::Empty → 0, Wrapper::Int → 1
    let enc_empty = encode_to_vec(&Wrapper::Empty).expect("encode Wrapper::Empty failed");
    let enc_int = encode_to_vec(&Wrapper::Int(0)).expect("encode Wrapper::Int failed");
    assert_eq!(enc_empty[0], 0u8, "Wrapper::Empty discriminant must be 0");
    assert_eq!(enc_int[0], 1u8, "Wrapper::Int discriminant must be 1");
}

// ---------------------------------------------------------------------------
// Test 5: Enum with struct variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_05_enum_struct_variant_roundtrip() {
    let origin = Positioned::Origin;
    let at = Positioned::At { x: -100, y: 200 };

    for variant in [&origin, &at] {
        let encoded = encode_to_vec(variant).expect("encode Positioned failed");
        let (decoded, consumed): (Positioned, usize) =
            decode_from_slice(&encoded).expect("decode Positioned failed");
        assert_eq!(&decoded, variant, "Positioned roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "Positioned consumed bytes mismatch"
        );
    }

    // Discriminant check: Origin → 0, At → 1
    let enc_origin = encode_to_vec(&Positioned::Origin).expect("encode Positioned::Origin failed");
    let enc_at =
        encode_to_vec(&Positioned::At { x: 0, y: 0 }).expect("encode Positioned::At failed");
    assert_eq!(
        enc_origin[0], 0u8,
        "Positioned::Origin discriminant must be 0"
    );
    assert_eq!(enc_at[0], 1u8, "Positioned::At discriminant must be 1");
}

// ---------------------------------------------------------------------------
// Test 6: Mixed enum (unit + tuple + struct) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_06_mixed_enum_all_variants_roundtrip() {
    let cases = [
        Mixed::Unit,
        Mixed::Pair(42, 99),
        Mixed::Named {
            label: "oxicode-mixed".to_string(),
            value: u64::MAX,
        },
    ];
    for (idx, variant) in cases.iter().enumerate() {
        let encoded = encode_to_vec(variant).expect("encode Mixed failed");
        let (decoded, consumed): (Mixed, usize) =
            decode_from_slice(&encoded).expect("decode Mixed failed");
        assert_eq!(&decoded, variant, "Mixed variant {idx} roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "Mixed variant {idx} consumed bytes mismatch"
        );
        assert_eq!(
            encoded[0], idx as u8,
            "Mixed variant {idx} discriminant byte mismatch"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 7: Enum with large discriminant (variant index 100) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_07_enum_large_discriminant_variant_roundtrip() {
    // Sparse::First has variant=0, Sparse::Hundredth has variant=100.
    // varint(100) < 251, so still a single byte.
    let first = Sparse::First;
    let hundredth = Sparse::Hundredth;

    let enc_first = encode_to_vec(&first).expect("encode Sparse::First failed");
    let enc_hundredth = encode_to_vec(&hundredth).expect("encode Sparse::Hundredth failed");

    assert_eq!(enc_first.len(), 1, "Sparse::First must encode to 1 byte");
    assert_eq!(enc_first[0], 0u8, "Sparse::First discriminant must be 0");

    assert_eq!(
        enc_hundredth.len(),
        1,
        "Sparse::Hundredth must encode to 1 byte (varint)"
    );
    assert_eq!(
        enc_hundredth[0], 100u8,
        "Sparse::Hundredth discriminant must be 100"
    );

    let (dec_first, consumed_first): (Sparse, usize) =
        decode_from_slice(&enc_first).expect("decode Sparse::First failed");
    assert_eq!(dec_first, first, "Sparse::First roundtrip mismatch");
    assert_eq!(
        consumed_first,
        enc_first.len(),
        "Sparse::First consumed bytes mismatch"
    );

    let (dec_hundredth, consumed_hundredth): (Sparse, usize) =
        decode_from_slice(&enc_hundredth).expect("decode Sparse::Hundredth failed");
    assert_eq!(
        dec_hundredth, hundredth,
        "Sparse::Hundredth roundtrip mismatch"
    );
    assert_eq!(
        consumed_hundredth,
        enc_hundredth.len(),
        "Sparse::Hundredth consumed bytes mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Enum discriminant consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_08_discriminant_consumed_equals_encoded_length() {
    // For unit variants the consumed byte count must equal the full encoded length.
    let variants = [
        Cardinal::North,
        Cardinal::South,
        Cardinal::East,
        Cardinal::West,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode Cardinal failed");
        let (_, consumed): (Cardinal, usize) =
            decode_from_slice(&encoded).expect("decode Cardinal failed");
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed ({consumed}) must equal encoded.len() ({}) for {variant:?}",
            encoded.len()
        );
    }
}

// ---------------------------------------------------------------------------
// Test 9: Enum with payload — different variants produce different sizes
// ---------------------------------------------------------------------------

#[test]
fn test_09_enum_payload_variants_produce_different_sizes() {
    // Wrapper::Empty is a unit variant (1 byte discriminant, no payload).
    // Wrapper::Int(i64) adds an 8-byte payload under fixed-int encoding.
    let enc_empty = encode_to_vec(&Wrapper::Empty).expect("encode Wrapper::Empty failed");
    let enc_int = encode_to_vec(&Wrapper::Int(1)).expect("encode Wrapper::Int(1) failed");

    // The int variant must be strictly larger than the unit variant.
    assert!(
        enc_int.len() > enc_empty.len(),
        "Wrapper::Int must produce more bytes than Wrapper::Empty: {} vs {}",
        enc_int.len(),
        enc_empty.len()
    );
}

// ---------------------------------------------------------------------------
// Test 10: Vec<SimpleEnum> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_10_vec_simple_enum_roundtrip() {
    let items: Vec<Cardinal> = vec![
        Cardinal::North,
        Cardinal::East,
        Cardinal::South,
        Cardinal::West,
        Cardinal::North,
        Cardinal::South,
    ];
    let encoded = encode_to_vec(&items).expect("encode Vec<Cardinal> failed");
    let (decoded, consumed): (Vec<Cardinal>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Cardinal> failed");
    assert_eq!(decoded, items, "Vec<Cardinal> roundtrip mismatch");
    assert_eq!(
        consumed,
        encoded.len(),
        "Vec<Cardinal> consumed bytes mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Option<SimpleEnum> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_11_option_simple_enum_some_roundtrip() {
    let some_north: Option<Cardinal> = Some(Cardinal::North);
    let some_west: Option<Cardinal> = Some(Cardinal::West);

    for variant in [&some_north, &some_west] {
        let encoded = encode_to_vec(variant).expect("encode Option<Cardinal> Some failed");
        let (decoded, consumed): (Option<Cardinal>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Cardinal> Some failed");
        assert_eq!(
            &decoded, variant,
            "Option<Cardinal> Some roundtrip mismatch"
        );
        assert_eq!(
            consumed,
            encoded.len(),
            "Option<Cardinal> Some consumed bytes mismatch"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 12: Option<SimpleEnum> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_12_option_simple_enum_none_roundtrip() {
    let none_val: Option<Cardinal> = None;
    let encoded = encode_to_vec(&none_val).expect("encode Option<Cardinal> None failed");
    let (decoded, consumed): (Option<Cardinal>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Cardinal> None failed");
    assert_eq!(
        decoded, none_val,
        "Option<Cardinal> None roundtrip mismatch"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "Option<Cardinal> None consumed bytes mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Enum with String payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_13_enum_string_payload_roundtrip() {
    let cases = [
        Named::Empty,
        Named::WithStr(String::new()),
        Named::WithStr("hello, oxicode".to_string()),
        Named::WithStr("こんにちは世界".to_string()),
        Named::WithStr("A".repeat(256)),
    ];
    for variant in &cases {
        let encoded = encode_to_vec(variant).expect("encode Named failed");
        let (decoded, consumed): (Named, usize) =
            decode_from_slice(&encoded).expect("decode Named failed");
        assert_eq!(&decoded, variant, "Named roundtrip mismatch");
        assert_eq!(consumed, encoded.len(), "Named consumed bytes mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 14: Enum with Vec<u8> payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_14_enum_vec_u8_payload_roundtrip() {
    let cases = [
        Buffered::None,
        Buffered::Data(vec![]),
        Buffered::Data(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        Buffered::Data((0u8..=255u8).collect()),
    ];
    for variant in &cases {
        let encoded = encode_to_vec(variant).expect("encode Buffered failed");
        let (decoded, consumed): (Buffered, usize) =
            decode_from_slice(&encoded).expect("decode Buffered failed");
        assert_eq!(&decoded, variant, "Buffered roundtrip mismatch");
        assert_eq!(consumed, encoded.len(), "Buffered consumed bytes mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 15: Enum with nested enum payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_15_enum_nested_enum_payload_roundtrip() {
    let cases = [
        Signal::Off,
        Signal::On(Polarity::Positive),
        Signal::On(Polarity::Negative),
    ];
    for variant in &cases {
        let encoded = encode_to_vec(variant).expect("encode Signal failed");
        let (decoded, consumed): (Signal, usize) =
            decode_from_slice(&encoded).expect("decode Signal failed");
        assert_eq!(&decoded, variant, "Signal roundtrip mismatch");
        assert_eq!(consumed, encoded.len(), "Signal consumed bytes mismatch");
    }

    // Wire layout: Signal::On(Polarity::Positive) → [1, 0]
    let enc_on_pos =
        encode_to_vec(&Signal::On(Polarity::Positive)).expect("encode Signal::On(Positive) failed");
    assert_eq!(
        enc_on_pos[0], 1u8,
        "Signal::On outer discriminant must be 1"
    );
    assert_eq!(
        enc_on_pos[1], 0u8,
        "Polarity::Positive inner discriminant must be 0"
    );

    // Wire layout: Signal::On(Polarity::Negative) → [1, 1]
    let enc_on_neg =
        encode_to_vec(&Signal::On(Polarity::Negative)).expect("encode Signal::On(Negative) failed");
    assert_eq!(
        enc_on_neg[0], 1u8,
        "Signal::On outer discriminant must be 1"
    );
    assert_eq!(
        enc_on_neg[1], 1u8,
        "Polarity::Negative inner discriminant must be 1"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Enum decode invalid discriminant fails
// ---------------------------------------------------------------------------

#[test]
fn test_16_enum_decode_invalid_discriminant_fails() {
    // Cardinal has 4 variants (indices 0..=3).
    // varint byte 0x0A (= 10) is a valid single-byte varint but refers to no variant.
    let invalid_bytes = [0x0Au8];
    let result: Result<(Cardinal, usize), _> = decode_from_slice(&invalid_bytes);
    assert!(
        result.is_err(),
        "Decoding Cardinal from discriminant 10 must fail, not succeed"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Unit enum with fixed-int config (tag_type u32 = 4 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_17_enum_tag_type_u32_fixed_int_config_4_bytes() {
    let cfg = config::legacy();

    // TaggedU32 uses #[oxicode(tag_type = "u32")] — in fixed-int config each
    // discriminant must be exactly 4 bytes.
    let enc_alpha =
        encode_to_vec_with_config(&TaggedU32::Alpha, cfg).expect("encode TaggedU32::Alpha failed");
    assert_eq!(
        enc_alpha.len(),
        4,
        "tag_type=u32 Alpha discriminant must be 4 bytes"
    );
    let disc_alpha = u32::from_le_bytes([enc_alpha[0], enc_alpha[1], enc_alpha[2], enc_alpha[3]]);
    assert_eq!(
        disc_alpha, 0u32,
        "TaggedU32::Alpha discriminant value must be 0"
    );

    let enc_beta =
        encode_to_vec_with_config(&TaggedU32::Beta, cfg).expect("encode TaggedU32::Beta failed");
    assert_eq!(
        enc_beta.len(),
        4,
        "tag_type=u32 Beta discriminant must be 4 bytes"
    );
    let disc_beta = u32::from_le_bytes([enc_beta[0], enc_beta[1], enc_beta[2], enc_beta[3]]);
    assert_eq!(
        disc_beta, 1u32,
        "TaggedU32::Beta discriminant value must be 1"
    );

    let enc_gamma =
        encode_to_vec_with_config(&TaggedU32::Gamma, cfg).expect("encode TaggedU32::Gamma failed");
    assert_eq!(
        enc_gamma.len(),
        4,
        "tag_type=u32 Gamma discriminant must be 4 bytes"
    );
    let disc_gamma = u32::from_le_bytes([enc_gamma[0], enc_gamma[1], enc_gamma[2], enc_gamma[3]]);
    assert_eq!(
        disc_gamma, 2u32,
        "TaggedU32::Gamma discriminant value must be 2"
    );

    // Roundtrip in fixed-int config.
    for variant in [TaggedU32::Alpha, TaggedU32::Beta, TaggedU32::Gamma] {
        let encoded =
            encode_to_vec_with_config(&variant, cfg).expect("encode TaggedU32 roundtrip failed");
        let (decoded, consumed): (TaggedU32, usize) = decode_from_slice_with_config(&encoded, cfg)
            .expect("decode TaggedU32 roundtrip failed");
        assert_eq!(decoded, variant, "TaggedU32 roundtrip mismatch");
        assert_eq!(consumed, encoded.len(), "TaggedU32 consumed bytes mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 18: Enum with 2 variants — first variant encodes to 1-byte discriminant
// ---------------------------------------------------------------------------

#[test]
fn test_18_two_variant_enum_first_variant_1_byte() {
    // Binary::Off is variant index 0 — varint(0) = 0x00, exactly 1 byte.
    let enc_off = encode_to_vec(&Binary::Off).expect("encode Binary::Off failed");
    assert_eq!(
        enc_off.len(),
        1,
        "Binary::Off must encode to exactly 1 byte"
    );
    assert_eq!(enc_off[0], 0x00u8, "Binary::Off discriminant must be 0x00");

    // Binary::On is variant index 1 — varint(1) = 0x01, exactly 1 byte.
    let enc_on = encode_to_vec(&Binary::On).expect("encode Binary::On failed");
    assert_eq!(enc_on.len(), 1, "Binary::On must encode to exactly 1 byte");
    assert_eq!(enc_on[0], 0x01u8, "Binary::On discriminant must be 0x01");

    // Both roundtrip correctly.
    for variant in [Binary::Off, Binary::On] {
        let encoded = encode_to_vec(&variant).expect("encode Binary failed");
        let (decoded, consumed): (Binary, usize) =
            decode_from_slice(&encoded).expect("decode Binary failed");
        assert_eq!(decoded, variant, "Binary roundtrip mismatch");
        assert_eq!(consumed, encoded.len(), "Binary consumed bytes mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 19: Enum with tuple variant containing multiple fields roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_19_enum_multi_field_tuple_variant_roundtrip() {
    let cases = [
        MultiTuple::None,
        MultiTuple::Triple(0, 0, 0),
        MultiTuple::Triple(u8::MAX, u16::MAX, u32::MAX),
        MultiTuple::Triple(1, 1000, 1_000_000),
    ];
    for variant in &cases {
        let encoded = encode_to_vec(variant).expect("encode MultiTuple failed");
        let (decoded, consumed): (MultiTuple, usize) =
            decode_from_slice(&encoded).expect("decode MultiTuple failed");
        assert_eq!(&decoded, variant, "MultiTuple roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "MultiTuple consumed bytes mismatch"
        );
    }

    // Discriminant check
    let enc_none = encode_to_vec(&MultiTuple::None).expect("encode MultiTuple::None failed");
    let enc_triple =
        encode_to_vec(&MultiTuple::Triple(0, 0, 0)).expect("encode MultiTuple::Triple failed");
    assert_eq!(enc_none[0], 0u8, "MultiTuple::None discriminant must be 0");
    assert_eq!(
        enc_triple[0], 1u8,
        "MultiTuple::Triple discriminant must be 1"
    );

    // Triple variant must be larger than None.
    assert!(
        enc_triple.len() > enc_none.len(),
        "MultiTuple::Triple must produce more bytes than MultiTuple::None"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Enum with struct variant containing multiple fields roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_20_enum_multi_field_struct_variant_roundtrip() {
    let cases = [
        MultiStruct::Empty,
        MultiStruct::Record {
            id: 0,
            name: String::new(),
            active: false,
        },
        MultiStruct::Record {
            id: u64::MAX,
            name: "full-record-oxicode".to_string(),
            active: true,
        },
        MultiStruct::Record {
            id: 42,
            name: "こんにちは".to_string(),
            active: false,
        },
    ];
    for variant in &cases {
        let encoded = encode_to_vec(variant).expect("encode MultiStruct failed");
        let (decoded, consumed): (MultiStruct, usize) =
            decode_from_slice(&encoded).expect("decode MultiStruct failed");
        assert_eq!(&decoded, variant, "MultiStruct roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "MultiStruct consumed bytes mismatch"
        );
    }

    // Discriminant check
    let enc_empty = encode_to_vec(&MultiStruct::Empty).expect("encode MultiStruct::Empty failed");
    assert_eq!(
        enc_empty[0], 0u8,
        "MultiStruct::Empty discriminant must be 0"
    );

    let enc_record = encode_to_vec(&MultiStruct::Record {
        id: 0,
        name: String::new(),
        active: false,
    })
    .expect("encode MultiStruct::Record failed");
    assert_eq!(
        enc_record[0], 1u8,
        "MultiStruct::Record discriminant must be 1"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Vec of mixed enum variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_21_vec_mixed_enum_variants_roundtrip() {
    let items: Vec<Mixed> = vec![
        Mixed::Unit,
        Mixed::Pair(1, 2),
        Mixed::Named {
            label: "first".to_string(),
            value: 100,
        },
        Mixed::Unit,
        Mixed::Pair(u32::MAX, 0),
        Mixed::Named {
            label: String::new(),
            value: u64::MAX,
        },
        Mixed::Pair(0, 0),
        Mixed::Unit,
    ];
    let encoded = encode_to_vec(&items).expect("encode Vec<Mixed> failed");
    let (decoded, consumed): (Vec<Mixed>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Mixed> failed");
    assert_eq!(decoded, items, "Vec<Mixed> roundtrip mismatch");
    assert_eq!(
        consumed,
        encoded.len(),
        "Vec<Mixed> consumed bytes mismatch"
    );

    // Empty vec also roundtrips.
    let empty: Vec<Mixed> = vec![];
    let enc_empty = encode_to_vec(&empty).expect("encode empty Vec<Mixed> failed");
    let (dec_empty, _): (Vec<Mixed>, usize) =
        decode_from_slice(&enc_empty).expect("decode empty Vec<Mixed> failed");
    assert!(
        dec_empty.is_empty(),
        "decoded empty Vec<Mixed> must be empty"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Enum roundtrip with big-endian config
// ---------------------------------------------------------------------------

#[test]
fn test_22_enum_roundtrip_big_endian_config() {
    let cfg = config::standard().with_big_endian();

    let cases = [
        Mixed::Unit,
        Mixed::Pair(0xDEAD, 0xBEEF),
        Mixed::Named {
            label: "big-endian-test".to_string(),
            value: 0xCAFE_BABE_0000_0001u64,
        },
    ];
    for variant in &cases {
        let encoded =
            encode_to_vec_with_config(variant, cfg).expect("encode Mixed big-endian failed");
        let (decoded, consumed): (Mixed, usize) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode Mixed big-endian failed");
        assert_eq!(&decoded, variant, "Mixed big-endian roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "Mixed big-endian consumed bytes mismatch"
        );
    }

    // Discriminant byte is not affected by endian config for the varint tag
    // (varint uses its own byte ordering independent of the endian config).
    let enc_unit =
        encode_to_vec_with_config(&Mixed::Unit, cfg).expect("encode Mixed::Unit big-endian failed");
    assert_eq!(
        enc_unit[0], 0x00u8,
        "Mixed::Unit discriminant byte must be 0x00 in big-endian config"
    );

    let enc_pair = encode_to_vec_with_config(&Mixed::Pair(0, 0), cfg)
        .expect("encode Mixed::Pair big-endian failed");
    assert_eq!(
        enc_pair[0], 0x01u8,
        "Mixed::Pair discriminant byte must be 0x01 in big-endian config"
    );
}
