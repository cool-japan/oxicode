//! Advanced PhantomData tests (set 2): Typed<T>, PhantomWrapper<T>, composite types.
//!
//! PhantomData<T> is a zero-sized type and encodes to exactly 0 bytes.
//! These tests verify ZST semantics across struct wrappers, tuples, Vecs,
//! Options, re-encoding, and multi-element collections.

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
use std::marker::PhantomData;

use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ── Shared type definitions ───────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Typed<T> {
    value: u32,
    _marker: PhantomData<T>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PhantomWrapper<T>(u64, PhantomData<T>);

// Helper struct used to compare encoding of Typed without PhantomData overhead.
#[derive(Debug, PartialEq, Encode, Decode)]
struct PlainU32 {
    value: u32,
}

// Struct with multiple PhantomData fields for test 18.
#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiMarker<A, B> {
    value: u32,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

// ── Test 1: PhantomData<u32> encodes to exactly 0 bytes ──────────────────────

#[test]
fn test_phantom_u32_zero_bytes() {
    let pd: PhantomData<u32> = PhantomData;
    let enc = encode_to_vec(&pd).expect("encode PhantomData::<u32>");
    assert_eq!(
        enc.len(),
        0,
        "PhantomData<u32> must encode to exactly 0 bytes, got {}",
        enc.len()
    );
}

// ── Test 2: PhantomData<String> encodes to exactly 0 bytes ───────────────────

#[test]
fn test_phantom_string_zero_bytes() {
    let pd: PhantomData<String> = PhantomData;
    let enc = encode_to_vec(&pd).expect("encode PhantomData::<String>");
    assert_eq!(
        enc.len(),
        0,
        "PhantomData<String> must encode to exactly 0 bytes, got {}",
        enc.len()
    );
}

// ── Test 3: PhantomData<Vec<u8>> encodes to exactly 0 bytes ──────────────────

#[test]
fn test_phantom_vec_u8_zero_bytes() {
    let pd: PhantomData<Vec<u8>> = PhantomData;
    let enc = encode_to_vec(&pd).expect("encode PhantomData::<Vec<u8>>");
    assert_eq!(
        enc.len(),
        0,
        "PhantomData<Vec<u8>> must encode to exactly 0 bytes, got {}",
        enc.len()
    );
}

// ── Test 4: PhantomData<()> roundtrip ────────────────────────────────────────

#[test]
fn test_phantom_unit_roundtrip() {
    let pd: PhantomData<()> = PhantomData;
    let enc = encode_to_vec(&pd).expect("encode PhantomData::<()>");
    let (decoded, _): (PhantomData<()>, usize) =
        decode_from_slice(&enc).expect("decode PhantomData::<()>");
    assert_eq!(pd, decoded);
}

// ── Test 5: PhantomData<u32> decoded value equals PhantomData ────────────────

#[test]
fn test_phantom_u32_decoded_equals_phantom() {
    let pd: PhantomData<u32> = PhantomData;
    let enc = encode_to_vec(&pd).expect("encode PhantomData::<u32>");
    let (decoded, _): (PhantomData<u32>, usize) =
        decode_from_slice(&enc).expect("decode PhantomData::<u32>");
    assert_eq!(
        decoded, PhantomData::<u32>,
        "Decoded PhantomData must equal PhantomData"
    );
}

// ── Test 6: Typed::<u32> { value: 42, _marker: PhantomData } roundtrip ───────

#[test]
fn test_typed_u32_value_42_roundtrip() {
    let original: Typed<u32> = Typed {
        value: 42,
        _marker: PhantomData,
    };
    let enc = encode_to_vec(&original).expect("encode Typed::<u32> value=42");
    let (decoded, _): (Typed<u32>, usize) =
        decode_from_slice(&enc).expect("decode Typed::<u32> value=42");
    assert_eq!(original, decoded);
}

// ── Test 7: Typed::<String> { value: 0, _marker: PhantomData } roundtrip ─────

#[test]
fn test_typed_string_value_0_roundtrip() {
    let original: Typed<String> = Typed {
        value: 0,
        _marker: PhantomData,
    };
    let enc = encode_to_vec(&original).expect("encode Typed::<String> value=0");
    let (decoded, _): (Typed<String>, usize) =
        decode_from_slice(&enc).expect("decode Typed::<String> value=0");
    assert_eq!(original, decoded);
}

// ── Test 8: Typed<u32> encodes same bytes as PlainU32 (phantom adds nothing) ─

#[test]
fn test_typed_u32_same_bytes_as_plain_u32() {
    let typed: Typed<u32> = Typed {
        value: 77,
        _marker: PhantomData,
    };
    let plain = PlainU32 { value: 77 };

    let enc_typed = encode_to_vec(&typed).expect("encode Typed::<u32>");
    let enc_plain = encode_to_vec(&plain).expect("encode PlainU32");

    assert_eq!(
        enc_typed, enc_plain,
        "Typed<u32> must encode identically to a plain struct with only the value field"
    );
}

// ── Test 9: PhantomWrapper::<u32>(99, PhantomData) roundtrip ─────────────────

#[test]
fn test_phantom_wrapper_u32_99_roundtrip() {
    let original: PhantomWrapper<u32> = PhantomWrapper(99, PhantomData);
    let enc = encode_to_vec(&original).expect("encode PhantomWrapper::<u32>(99)");
    let (decoded, _): (PhantomWrapper<u32>, usize) =
        decode_from_slice(&enc).expect("decode PhantomWrapper::<u32>(99)");
    assert_eq!(original, decoded);
}

// ── Test 10: PhantomWrapper::<String>(0, PhantomData) roundtrip ──────────────

#[test]
fn test_phantom_wrapper_string_0_roundtrip() {
    let original: PhantomWrapper<String> = PhantomWrapper(0, PhantomData);
    let enc = encode_to_vec(&original).expect("encode PhantomWrapper::<String>(0)");
    let (decoded, _): (PhantomWrapper<String>, usize) =
        decode_from_slice(&enc).expect("decode PhantomWrapper::<String>(0)");
    assert_eq!(original, decoded);
}

// ── Test 11: PhantomWrapper<T> encodes same bytes as raw u64 ─────────────────

#[test]
fn test_phantom_wrapper_same_bytes_as_raw_u64() {
    let wrapper: PhantomWrapper<u32> = PhantomWrapper(12345_u64, PhantomData);
    let raw: u64 = 12345_u64;

    let enc_wrapper = encode_to_vec(&wrapper).expect("encode PhantomWrapper");
    let enc_raw = encode_to_vec(&raw).expect("encode u64");

    assert_eq!(
        enc_wrapper, enc_raw,
        "PhantomWrapper<T> must encode identically to the raw u64 (PhantomData contributes 0 bytes)"
    );
}

// ── Test 12: Vec<PhantomData<u32>> with 3 elements — 1 byte (varint length) ──

#[test]
fn test_vec_phantom_3_elements_is_1_byte() {
    let v: Vec<PhantomData<u32>> = vec![PhantomData; 3];
    let enc = encode_to_vec(&v).expect("encode Vec<PhantomData<u32>> len=3");
    assert_eq!(
        enc.len(),
        1,
        "Vec of 3 PhantomData elements must encode to exactly 1 byte (varint(3)), got {}",
        enc.len()
    );
    assert_eq!(enc[0], 3u8, "length varint for 3 elements must be 0x03");
}

// ── Test 13: Option<PhantomData<u32>> Some roundtrip ─────────────────────────

#[test]
fn test_option_phantom_some_roundtrip() {
    let val: Option<PhantomData<u32>> = Some(PhantomData);
    let enc = encode_to_vec(&val).expect("encode Some(PhantomData::<u32>)");
    let (decoded, _): (Option<PhantomData<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Some(PhantomData::<u32>)");
    assert_eq!(val, decoded);
    // Some discriminant (1 byte) + PhantomData (0 bytes) = 1 byte total.
    assert_eq!(
        enc.len(),
        1,
        "Option Some(PhantomData) must be exactly 1 byte"
    );
}

// ── Test 14: Option<PhantomData<u32>> None roundtrip ─────────────────────────

#[test]
fn test_option_phantom_none_roundtrip() {
    let val: Option<PhantomData<u32>> = None;
    let enc = encode_to_vec(&val).expect("encode None::<PhantomData<u32>>");
    let (decoded, _): (Option<PhantomData<u32>>, usize) =
        decode_from_slice(&enc).expect("decode None::<PhantomData<u32>>");
    assert_eq!(val, decoded);
    assert_eq!(enc.len(), 1, "Option None must be exactly 1 byte");
}

// ── Test 15: (PhantomData::<u32>, u64) tuple — encodes as just the u64 ────────

#[test]
fn test_tuple_phantom_first_encodes_as_u64() {
    let tup: (PhantomData<u32>, u64) = (PhantomData, 888_u64);
    let raw: u64 = 888_u64;

    let enc_tup = encode_to_vec(&tup).expect("encode (PhantomData, u64)");
    let enc_raw = encode_to_vec(&raw).expect("encode u64");

    assert_eq!(
        enc_tup, enc_raw,
        "(PhantomData::<u32>, u64) must encode identically to bare u64"
    );
}

// ── Test 16: (u64, PhantomData::<u32>) tuple — encodes as just the u64 ────────

#[test]
fn test_tuple_phantom_second_encodes_as_u64() {
    let tup: (u64, PhantomData<u32>) = (555_u64, PhantomData);
    let raw: u64 = 555_u64;

    let enc_tup = encode_to_vec(&tup).expect("encode (u64, PhantomData)");
    let enc_raw = encode_to_vec(&raw).expect("encode u64");

    assert_eq!(
        enc_tup, enc_raw,
        "(u64, PhantomData::<u32>) must encode identically to bare u64"
    );
}

// ── Test 17: Two PhantomData types produce identical 0-byte encoding ──────────

#[test]
fn test_different_phantom_types_identical_encoding() {
    let pd_u32: PhantomData<u32> = PhantomData;
    let pd_str: PhantomData<String> = PhantomData;

    let enc_u32 = encode_to_vec(&pd_u32).expect("encode PhantomData::<u32>");
    let enc_str = encode_to_vec(&pd_str).expect("encode PhantomData::<String>");

    assert_eq!(
        enc_u32, enc_str,
        "PhantomData<u32> and PhantomData<String> must produce identical (empty) encodings"
    );
    assert!(enc_u32.is_empty(), "Both must be 0 bytes");
}

// ── Test 18: Struct with multiple PhantomData fields roundtrip ────────────────

#[test]
fn test_multi_marker_struct_roundtrip() {
    let original: MultiMarker<u32, String> = MultiMarker {
        value: 123,
        _a: PhantomData,
        _b: PhantomData,
    };
    let enc = encode_to_vec(&original).expect("encode MultiMarker");
    let (decoded, _): (MultiMarker<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode MultiMarker");
    assert_eq!(original, decoded);

    // Wire size must equal that of the bare u32 value (multiple PhantomData fields = 0 bytes).
    let enc_plain = encode_to_vec(&original.value).expect("encode plain u32");
    assert_eq!(
        enc.len(),
        enc_plain.len(),
        "MultiMarker encoding must equal bare u32 size"
    );
}

// ── Test 19: Typed<u32> consumed bytes == encoded size of bare u32(0) ─────────

#[test]
fn test_typed_u32_value_0_consumed_bytes_matches_bare_u32() {
    let typed: Typed<u32> = Typed {
        value: 0,
        _marker: PhantomData,
    };
    let enc = encode_to_vec(&typed).expect("encode Typed::<u32> value=0");
    let (_, consumed): (Typed<u32>, usize) =
        decode_from_slice(&enc).expect("decode Typed::<u32> value=0");

    let bare_enc = encode_to_vec(&0u32).expect("encode 0u32");
    assert_eq!(
        consumed,
        bare_enc.len(),
        "Typed<u32> value=0 must consume exactly as many bytes as bare varint(0)"
    );
}

// ── Test 20: Re-encoding decoded Typed<u32> gives same bytes ─────────────────

#[test]
fn test_typed_u32_reencode_gives_same_bytes() {
    let original: Typed<u32> = Typed {
        value: 255,
        _marker: PhantomData,
    };
    let enc_first = encode_to_vec(&original).expect("first encode Typed::<u32>");
    let (decoded, _): (Typed<u32>, usize) =
        decode_from_slice(&enc_first).expect("decode Typed::<u32>");
    let enc_second = encode_to_vec(&decoded).expect("re-encode decoded Typed::<u32>");

    assert_eq!(
        enc_first, enc_second,
        "Re-encoding a decoded Typed<u32> must yield identical bytes"
    );
}

// ── Test 21: Vec<Typed<u32>> with 3 elements roundtrip ───────────────────────

#[test]
fn test_vec_typed_u32_3_elements_roundtrip() {
    let original: Vec<Typed<u32>> = vec![
        Typed {
            value: 1,
            _marker: PhantomData,
        },
        Typed {
            value: 2,
            _marker: PhantomData,
        },
        Typed {
            value: 3,
            _marker: PhantomData,
        },
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Typed<u32>>");
    let (decoded, _): (Vec<Typed<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Typed<u32>>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 3, "Decoded Vec must have 3 elements");
}

// ── Test 22: Option<Typed<String>> Some roundtrip ────────────────────────────

#[test]
fn test_option_typed_string_some_roundtrip() {
    let original: Option<Typed<String>> = Some(Typed {
        value: 999,
        _marker: PhantomData,
    });
    let enc = encode_to_vec(&original).expect("encode Option<Typed<String>> Some");
    let (decoded, _): (Option<Typed<String>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Typed<String>> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some(), "Decoded Option must be Some");
}
