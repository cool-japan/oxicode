//! Advanced property-based tests using proptest (set 11).
//!
//! Each test is a top-level #[test] function containing exactly one
//! proptest! macro block, verifying non-trivial invariants for structs,
//! enums, configs, and encoding properties.

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

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};
use proptest::prelude::*;

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// ── Strategies ────────────────────────────────────────────────────────────────

fn arb_point() -> impl Strategy<Value = Point> {
    (any::<i32>(), any::<i32>()).prop_map(|(x, y)| Point { x, y })
}

fn arb_color() -> impl Strategy<Value = Color> {
    prop_oneof![
        Just(Color::Red),
        Just(Color::Green),
        Just(Color::Blue),
        (any::<u8>(), any::<u8>(), any::<u8>()).prop_map(|(r, g, b)| Color::Custom(r, g, b)),
    ]
}

// ── Test 1: Point struct roundtrip ────────────────────────────────────────────

#[test]
fn prop_point_roundtrip() {
    proptest!(|(p in arb_point())| {
        let encoded = encode_to_vec(&p).expect("encode Point");
        let (decoded, bytes_read): (Point, usize) = decode_from_slice(&encoded).expect("decode Point");
        prop_assert_eq!(&p, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 2: Color enum roundtrip – all variants ───────────────────────────────

#[test]
fn prop_color_roundtrip() {
    proptest!(|(c in arb_color())| {
        let encoded = encode_to_vec(&c).expect("encode Color");
        let (decoded, bytes_read): (Color, usize) = decode_from_slice(&encoded).expect("decode Color");
        prop_assert_eq!(&c, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 3: Vec<Point> roundtrip ──────────────────────────────────────────────

#[test]
fn prop_vec_point_roundtrip() {
    proptest!(|(pts in proptest::collection::vec(arb_point(), 0usize..20))| {
        let encoded = encode_to_vec(&pts).expect("encode Vec<Point>");
        let (decoded, bytes_read): (Vec<Point>, usize) = decode_from_slice(&encoded).expect("decode Vec<Point>");
        prop_assert_eq!(&pts, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 4: Vec<Color> roundtrip ──────────────────────────────────────────────

#[test]
fn prop_vec_color_roundtrip() {
    proptest!(|(colors in proptest::collection::vec(arb_color(), 0usize..20))| {
        let encoded = encode_to_vec(&colors).expect("encode Vec<Color>");
        let (decoded, bytes_read): (Vec<Color>, usize) = decode_from_slice(&encoded).expect("decode Vec<Color>");
        prop_assert_eq!(&colors, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 5: Option<Point> roundtrip ──────────────────────────────────────────

#[test]
fn prop_option_point_roundtrip() {
    proptest!(|(opt in proptest::option::of(arb_point()))| {
        let encoded = encode_to_vec(&opt).expect("encode Option<Point>");
        let (decoded, bytes_read): (Option<Point>, usize) = decode_from_slice(&encoded).expect("decode Option<Point>");
        prop_assert_eq!(&opt, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 6: BTreeMap<u32, Point> roundtrip ───────────────────────────────────

#[test]
fn prop_btreemap_u32_point_roundtrip() {
    proptest!(|(entries in proptest::collection::btree_map(any::<u32>(), arb_point(), 0usize..10))| {
        let encoded = encode_to_vec(&entries).expect("encode BTreeMap<u32, Point>");
        let (decoded, bytes_read): (BTreeMap<u32, Point>, usize) =
            decode_from_slice(&encoded).expect("decode BTreeMap<u32, Point>");
        prop_assert_eq!(&entries, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 7: Fixed-int config: encode size == field_size * field_count ────────
// Point has 2 × i32 fields; with fixed-int encoding each i32 is exactly 4 bytes,
// so the total must be 8 bytes regardless of the value.

#[test]
fn prop_fixed_int_point_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    proptest!(|(p in arb_point())| {
        let encoded = encode_to_vec_with_config(&p, cfg).expect("encode Point fixed-int");
        // 2 fields × 4 bytes each = 8 bytes
        prop_assert_eq!(encoded.len(), 8,
            "fixed-int Point must be 8 bytes, got {}", encoded.len());
    });
}

// ── Test 8: Big-endian config: roundtrip for u32 ─────────────────────────────

#[test]
fn prop_big_endian_u32_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    proptest!(|(v: u32)| {
        let encoded = encode_to_vec_with_config(&v, cfg).expect("encode u32 big-endian");
        let (decoded, bytes_read): (u32, usize) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode u32 big-endian");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 9: Legacy config: roundtrip for u64 ─────────────────────────────────

#[test]
fn prop_legacy_u64_roundtrip() {
    proptest!(|(v: u64)| {
        let encoded = encode_to_vec_with_config(&v, config::legacy()).expect("encode u64 legacy");
        let (decoded, bytes_read): (u64, usize) =
            decode_from_slice_with_config(&encoded, config::legacy()).expect("decode u64 legacy");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 10: consumed < total_bytes is possible with trailing data ────────────

#[test]
fn prop_trailing_bytes_not_consumed() {
    proptest!(|(p in arb_point(), trailing in proptest::collection::vec(any::<u8>(), 1usize..16))| {
        let mut encoded = encode_to_vec(&p).expect("encode Point");
        let original_len = encoded.len();
        encoded.extend_from_slice(&trailing);

        let (decoded, bytes_read): (Point, usize) = decode_from_slice(&encoded).expect("decode Point with trailing");
        prop_assert_eq!(&p, &decoded);
        prop_assert_eq!(bytes_read, original_len, "consumed bytes should equal original encoded length");
        prop_assert!(bytes_read < encoded.len(), "trailing bytes should not be consumed");
    });
}

// ── Test 11: Concatenation: encode(p1) + encode(p2) == encode((p1, p2)) ──────

#[test]
fn prop_point_concat_encoding() {
    proptest!(|(p1 in arb_point(), p2 in arb_point())| {
        let enc_pair = encode_to_vec(&(p1.clone(), p2.clone())).expect("encode (Point, Point)");
        let enc1 = encode_to_vec(&p1).expect("encode Point 1");
        let enc2 = encode_to_vec(&p2).expect("encode Point 2");
        let concat: Vec<u8> = enc1.iter().chain(enc2.iter()).copied().collect();
        prop_assert_eq!(enc_pair, concat);
    });
}

// ── Test 12: Vec<u8> length: len(encode(v)) > len(v) for non-empty v ─────────

#[test]
fn prop_vec_u8_encoded_longer_than_raw() {
    proptest!(|(v in proptest::collection::vec(any::<u8>(), 1usize..100))| {
        let encoded = encode_to_vec(&v).expect("encode Vec<u8>");
        prop_assert!(
            encoded.len() > v.len(),
            "encoded Vec<u8> ({} bytes) should be longer than raw data ({} bytes) due to length prefix",
            encoded.len(), v.len()
        );
    });
}

// ── Test 13: i32 zigzag: abs(x) determines encoding size ─────────────────────
// Positive and negative values with the same absolute value produce the same
// varint encoding length (zigzag maps i → 2i for positive, i → -2i-1 for negative,
// but both |x| and -(|x|) map to zigzag values of similar magnitude).

#[test]
fn prop_i32_zigzag_symmetry() {
    proptest!(|(x in 1i32..=i32::MAX)| {
        let pos_enc = encode_to_vec(&x).expect("encode positive i32");
        // negate: same absolute value, different sign
        let neg = -x;
        let neg_enc = encode_to_vec(&neg).expect("encode negative i32");
        prop_assert_eq!(
            pos_enc.len(), neg_enc.len(),
            "abs({}) == abs({}) so encoding lengths should match: {} vs {}",
            x, neg, pos_enc.len(), neg_enc.len()
        );
    });
}

// ── Test 14: u64 size: small values (< 251) encode as 1 byte ─────────────────

#[test]
fn prop_u64_varint_small_is_one_byte() {
    proptest!(|(v in 0u64..=250)| {
        let encoded = encode_to_vec(&v).expect("encode small u64");
        prop_assert_eq!(
            encoded.len(), 1,
            "u64 value {} should encode as 1 byte (varint), got {}",
            v, encoded.len()
        );
        let large: u64 = 251;
        let enc_large = encode_to_vec(&large).expect("encode u64=251");
        prop_assert!(
            enc_large.len() > 1,
            "u64=251 should encode as more than 1 byte, got {}",
            enc_large.len()
        );
    });
}

// ── Test 15: String UTF-8: all Rust strings roundtrip perfectly ───────────────

#[test]
fn prop_string_utf8_roundtrip() {
    proptest!(|(s: String)| {
        // Rust strings are always valid UTF-8; encoding/decoding must preserve them exactly
        let encoded = encode_to_vec(&s).expect("encode String");
        let (decoded, bytes_read): (String, usize) = decode_from_slice(&encoded).expect("decode String");
        prop_assert_eq!(&s, &decoded, "String roundtrip must be lossless");
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 16: Option Some size always > None size for same type ────────────────

#[test]
fn prop_option_some_larger_than_none_point() {
    proptest!(|(p in arb_point())| {
        let none_val: Option<Point> = None;
        let some_val: Option<Point> = Some(p.clone());
        let enc_none = encode_to_vec(&none_val).expect("encode None<Point>");
        let enc_some = encode_to_vec(&some_val).expect("encode Some(Point)");
        prop_assert!(
            enc_none.len() < enc_some.len(),
            "None<Point> ({} bytes) must be smaller than Some(Point) ({} bytes)",
            enc_none.len(), enc_some.len()
        );
    });
}

// ── Test 17: Two equal values produce equal encodings ────────────────────────

#[test]
fn prop_equal_values_equal_encodings() {
    proptest!(|(p in arb_point())| {
        let enc1 = encode_to_vec(&p).expect("encode Point first");
        let enc2 = encode_to_vec(&p).expect("encode Point second");
        prop_assert_eq!(enc1, enc2, "same Point must produce identical encodings");
    });
}

// ── Test 18: Different u32 values produce different encodings ─────────────────

#[test]
fn prop_different_u32_different_encodings() {
    proptest!(|(a: u32, b: u32)| {
        prop_assume!(a != b);
        let enc_a = encode_to_vec(&a).expect("encode u32 a");
        let enc_b = encode_to_vec(&b).expect("encode u32 b");
        prop_assert_ne!(enc_a, enc_b,
            "different u32 values ({} vs {}) should produce different encodings", a, b);
    });
}

// ── Test 19: Vec roundtrip with any length 0..=200 ────────────────────────────

#[test]
fn prop_vec_any_length_roundtrip() {
    proptest!(|(v in proptest::collection::vec(any::<u32>(), 0usize..=200))| {
        let encoded = encode_to_vec(&v).expect("encode Vec<u32>");
        let (decoded, bytes_read): (Vec<u32>, usize) = decode_from_slice(&encoded).expect("decode Vec<u32>");
        prop_assert_eq!(&v, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 20: Color::Custom roundtrip (tuple variant) ─────────────────────────

#[test]
fn prop_color_custom_roundtrip() {
    proptest!(|(r: u8, g: u8, b: u8)| {
        let c = Color::Custom(r, g, b);
        let encoded = encode_to_vec(&c).expect("encode Color::Custom");
        let (decoded, bytes_read): (Color, usize) = decode_from_slice(&encoded).expect("decode Color::Custom");
        prop_assert_eq!(&c, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 21: Struct with all field types roundtrip ────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct AllFields {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: i16,
    g: i32,
    h: i64,
    s: String,
    flag: bool,
}

#[test]
fn prop_all_fields_struct_roundtrip() {
    proptest!(|(a: u8, b: u16, c: u32, d: u64, e: i8, f: i16, g: i32, h: i64, s: String, flag: bool)| {
        let val = AllFields { a, b, c, d, e, f, g, h, s: s.clone(), flag };
        let encoded = encode_to_vec(&val).expect("encode AllFields");
        let (decoded, bytes_read): (AllFields, usize) = decode_from_slice(&encoded).expect("decode AllFields");
        prop_assert_eq!(&val, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 22: Nested Option<Option<u32>> roundtrip ─────────────────────────────

#[test]
fn prop_nested_option_roundtrip() {
    proptest!(|(inner in proptest::option::of(any::<u32>()))| {
        let outer: Option<Option<u32>> = Some(inner);
        let encoded = encode_to_vec(&outer).expect("encode Option<Option<u32>>");
        let (decoded, bytes_read): (Option<Option<u32>>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Option<u32>>");
        prop_assert_eq!(&outer, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());

        let none_outer: Option<Option<u32>> = None;
        let enc_none = encode_to_vec(&none_outer).expect("encode None<Option<u32>>");
        let (dec_none, _): (Option<Option<u32>>, usize) =
            decode_from_slice(&enc_none).expect("decode None<Option<u32>>");
        prop_assert_eq!(&none_outer, &dec_none);
    });
}
