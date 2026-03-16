//! Advanced property-based tests (set 8) for OxiCode.
//!
//! Each test function contains exactly one proptest! block.
//! All tests are top-level — no #[cfg(test)] module wrapper.

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
use std::collections::BTreeMap;

use oxicode::{config, decode_from_slice, encode_to_vec, encoded_size};
use proptest::prelude::*;

// ── 1. u8 roundtrip ──────────────────────────────────────────────────────────

#[test]
fn test_u8_roundtrip() {
    proptest!(|(val: u8)| {
        let enc = encode_to_vec(&val).expect("encode u8");
        let (dec, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8");
        prop_assert_eq!(val, dec);
    });
}

// ── 2. i8 roundtrip ──────────────────────────────────────────────────────────

#[test]
fn test_i8_roundtrip() {
    proptest!(|(val: i8)| {
        let enc = encode_to_vec(&val).expect("encode i8");
        let (dec, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8");
        prop_assert_eq!(val, dec);
    });
}

// ── 3. u16 roundtrip ─────────────────────────────────────────────────────────

#[test]
fn test_u16_roundtrip() {
    proptest!(|(val: u16)| {
        let enc = encode_to_vec(&val).expect("encode u16");
        let (dec, _): (u16, usize) = decode_from_slice(&enc).expect("decode u16");
        prop_assert_eq!(val, dec);
    });
}

// ── 4. i16 roundtrip ─────────────────────────────────────────────────────────

#[test]
fn test_i16_roundtrip() {
    proptest!(|(val: i16)| {
        let enc = encode_to_vec(&val).expect("encode i16");
        let (dec, _): (i16, usize) = decode_from_slice(&enc).expect("decode i16");
        prop_assert_eq!(val, dec);
    });
}

// ── 5. f32 roundtrip via bit patterns ────────────────────────────────────────

#[test]
fn test_f32_bits_roundtrip() {
    proptest!(|(bits: u32)| {
        let enc = encode_to_vec(&bits).expect("encode f32 bits");
        let (dec, _): (u32, usize) = decode_from_slice(&enc).expect("decode f32 bits");
        // verify bit-level identity (covers NaN, ±Inf, subnormals, etc.)
        prop_assert_eq!(bits, dec);
        // verify f32 round-trips through its bit representation
        let f_orig = f32::from_bits(bits);
        let f_dec  = f32::from_bits(dec);
        prop_assert_eq!(f_orig.to_bits(), f_dec.to_bits());
    });
}

// ── 6. f64 roundtrip via bit patterns ────────────────────────────────────────

#[test]
fn test_f64_bits_roundtrip() {
    proptest!(|(bits: u64)| {
        let enc = encode_to_vec(&bits).expect("encode f64 bits");
        let (dec, _): (u64, usize) = decode_from_slice(&enc).expect("decode f64 bits");
        prop_assert_eq!(bits, dec);
        let f_orig = f64::from_bits(bits);
        let f_dec  = f64::from_bits(dec);
        prop_assert_eq!(f_orig.to_bits(), f_dec.to_bits());
    });
}

// ── 7. char roundtrip ────────────────────────────────────────────────────────

#[test]
fn test_char_roundtrip() {
    proptest!(|(val in prop::char::any())| {
        let enc = encode_to_vec(&val).expect("encode char");
        let (dec, _): (char, usize) = decode_from_slice(&enc).expect("decode char");
        prop_assert_eq!(val, dec);
    });
}

// ── 8. Vec<u8> roundtrip (0..100 elements) ───────────────────────────────────

#[test]
fn test_vec_u8_roundtrip() {
    proptest!(|(val in prop::collection::vec(any::<u8>(), 0..100))| {
        let enc = encode_to_vec(&val).expect("encode Vec<u8>");
        let (dec, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8>");
        prop_assert_eq!(val, dec);
    });
}

// ── 9. Vec<i32> roundtrip ────────────────────────────────────────────────────

#[test]
fn test_vec_i32_roundtrip() {
    proptest!(|(val in prop::collection::vec(any::<i32>(), 0..50))| {
        let enc = encode_to_vec(&val).expect("encode Vec<i32>");
        let (dec, _): (Vec<i32>, usize) = decode_from_slice(&enc).expect("decode Vec<i32>");
        prop_assert_eq!(val, dec);
    });
}

// ── 10. String roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_string_roundtrip() {
    proptest!(|(val in "[a-zA-Z0-9 ]{0,50}")| {
        let enc = encode_to_vec(&val).expect("encode String");
        let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode String");
        prop_assert_eq!(val, dec);
    });
}

// ── 11. Option<u32> roundtrip ────────────────────────────────────────────────

#[test]
fn test_option_u32_roundtrip() {
    proptest!(|(val: Option<u32>)| {
        let enc = encode_to_vec(&val).expect("encode Option<u32>");
        let (dec, _): (Option<u32>, usize) = decode_from_slice(&enc).expect("decode Option<u32>");
        prop_assert_eq!(val, dec);
    });
}

// ── 12. (u32, u32) tuple roundtrip ───────────────────────────────────────────

#[test]
fn test_tuple_u32_u32_roundtrip() {
    proptest!(|(a: u32, b: u32)| {
        let val = (a, b);
        let enc = encode_to_vec(&val).expect("encode (u32, u32)");
        let (dec, _): ((u32, u32), usize) = decode_from_slice(&enc).expect("decode (u32, u32)");
        prop_assert_eq!(val, dec);
    });
}

// ── 13. (u8, u16, u32) triple roundtrip ──────────────────────────────────────

#[test]
fn test_triple_u8_u16_u32_roundtrip() {
    proptest!(|(a: u8, b: u16, c: u32)| {
        let val = (a, b, c);
        let enc = encode_to_vec(&val).expect("encode (u8, u16, u32)");
        let (dec, _): ((u8, u16, u32), usize) =
            decode_from_slice(&enc).expect("decode (u8, u16, u32)");
        prop_assert_eq!(val, dec);
    });
}

// ── 14. BTreeMap<u32, u32> roundtrip (0..10 entries) ─────────────────────────

#[test]
fn test_btreemap_u32_u32_roundtrip() {
    proptest!(|(val in prop::collection::btree_map(any::<u32>(), any::<u32>(), 0..10))| {
        let enc = encode_to_vec(&val).expect("encode BTreeMap<u32, u32>");
        let (dec, _): (BTreeMap<u32, u32>, usize) =
            decode_from_slice(&enc).expect("decode BTreeMap<u32, u32>");
        prop_assert_eq!(val, dec);
    });
}

// ── 15. Vec<Option<u32>> roundtrip ───────────────────────────────────────────

#[test]
fn test_vec_option_u32_roundtrip() {
    proptest!(|(val in prop::collection::vec(any::<Option<u32>>(), 0..30))| {
        let enc = encode_to_vec(&val).expect("encode Vec<Option<u32>>");
        let (dec, _): (Vec<Option<u32>>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Option<u32>>");
        prop_assert_eq!(val, dec);
    });
}

// ── 16. encoded_size matches encode_to_vec length for u64 ────────────────────

#[test]
fn test_encoded_size_matches_vec_len_u64() {
    proptest!(|(val: u64)| {
        let enc = encode_to_vec(&val).expect("encode u64 for size check");
        let size = encoded_size(&val).expect("encoded_size u64");
        prop_assert_eq!(enc.len(), size);
    });
}

// ── 17. encoded_size matches encode_to_vec length for Vec<u32> ───────────────

#[test]
fn test_encoded_size_matches_vec_len_vec_u32() {
    proptest!(|(val in prop::collection::vec(any::<u32>(), 0..20))| {
        let enc = encode_to_vec(&val).expect("encode Vec<u32> for size check");
        let size = encoded_size(&val).expect("encoded_size Vec<u32>");
        prop_assert_eq!(enc.len(), size);
    });
}

// ── 18. fixed-int encoding u32 always 4 bytes ────────────────────────────────

#[test]
fn test_fixed_int_u32_always_4_bytes() {
    proptest!(|(val: u32)| {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = oxicode::encode_to_vec_with_config(&val, cfg).expect("encode fixed u32");
        prop_assert_eq!(enc.len(), 4usize, "fixed-int u32 must be exactly 4 bytes");
    });
}

// ── 19. big-endian encoding roundtrip u32 ────────────────────────────────────

#[test]
fn test_big_endian_u32_roundtrip() {
    proptest!(|(val: u32)| {
        let cfg = config::standard().with_big_endian().with_fixed_int_encoding();
        let enc = oxicode::encode_to_vec_with_config(&val, cfg).expect("encode big-endian u32");
        let (dec, _): (u32, usize) =
            oxicode::decode_from_slice_with_config(&enc, cfg).expect("decode big-endian u32");
        prop_assert_eq!(val, dec);
    });
}

// ── 20. Encode→decode→encode idempotent for u64 ──────────────────────────────

#[test]
fn test_idempotent_encode_decode_encode_u64() {
    proptest!(|(val: u64)| {
        let enc1 = encode_to_vec(&val).expect("first encode u64");
        let (mid, _): (u64, usize) = decode_from_slice(&enc1).expect("decode u64");
        let enc2 = encode_to_vec(&mid).expect("second encode u64");
        prop_assert_eq!(enc1, enc2, "encode→decode→encode must be idempotent for u64");
    });
}

// ── 21. Encode→decode→encode idempotent for String ───────────────────────────

#[test]
fn test_idempotent_encode_decode_encode_string() {
    proptest!(|(val in "[a-zA-Z0-9 ]{0,50}")| {
        let enc1 = encode_to_vec(&val).expect("first encode String");
        let (mid, _): (String, usize) = decode_from_slice(&enc1).expect("decode String");
        let enc2 = encode_to_vec(&mid).expect("second encode String");
        prop_assert_eq!(enc1, enc2, "encode→decode→encode must be idempotent for String");
    });
}

// ── 22. Encode→decode→encode idempotent for Vec<u8> ──────────────────────────

#[test]
fn test_idempotent_encode_decode_encode_vec_u8() {
    proptest!(|(val in prop::collection::vec(any::<u8>(), 0..100))| {
        let enc1 = encode_to_vec(&val).expect("first encode Vec<u8>");
        let (mid, _): (Vec<u8>, usize) = decode_from_slice(&enc1).expect("decode Vec<u8>");
        let enc2 = encode_to_vec(&mid).expect("second encode Vec<u8>");
        prop_assert_eq!(enc1, enc2, "encode→decode→encode must be idempotent for Vec<u8>");
    });
}
