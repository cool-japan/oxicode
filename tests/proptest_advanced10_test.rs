//! Advanced property-based tests using proptest (set 10).
//!
//! Each test is a top-level #[test] function containing exactly one
//! proptest! macro block, verifying non-trivial invariants beyond basic roundtrips.

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
use proptest::prelude::*;

// ── Test 20 / 21 helper types ─────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct PropStruct {
    id: u32,
    name: String,
    flag: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PropEnum {
    Unit,
    Newtype(u32),
    Pair(u32, String),
}

// ── 1. Re-encode invariant for u32 ───────────────────────────────────────────

#[test]
fn prop_reencode_u32() {
    proptest!(|(val: u32)| {
        let enc1 = encode_to_vec(&val).expect("encode u32 first");
        let (dec, _): (u32, usize) = decode_from_slice(&enc1).expect("decode u32");
        let enc2 = encode_to_vec(&dec).expect("encode u32 second");
        prop_assert_eq!(enc1, enc2);
    });
}

// ── 2. Re-encode invariant for String ────────────────────────────────────────

#[test]
fn prop_reencode_string() {
    proptest!(|(val: String)| {
        let enc1 = encode_to_vec(&val).expect("encode String first");
        let (dec, _): (String, usize) = decode_from_slice(&enc1).expect("decode String");
        let enc2 = encode_to_vec(&dec).expect("encode String second");
        prop_assert_eq!(enc1, enc2);
    });
}

// ── 3. Size monotonicity: longer strings produce longer encodings ─────────────

#[test]
fn prop_string_size_monotonicity() {
    proptest!(|(prefix in "[a-z]{1,20}", suffix in "[a-z]{1,20}")| {
        let short = prefix.clone();
        let long = format!("{}{}", prefix, suffix);
        let enc_short = encode_to_vec(&short).expect("encode short");
        let enc_long = encode_to_vec(&long).expect("encode long");
        prop_assert!(enc_short.len() <= enc_long.len(),
            "shorter string {:?} encoded to {} bytes but longer {:?} encoded to {} bytes",
            short, enc_short.len(), long, enc_long.len());
    });
}

// ── 4. Concatenation: encode((a, b)) == [encode(a), encode(b)].concat() ──────

#[test]
fn prop_tuple_concat_encoding() {
    proptest!(|(a: u32, b: u32)| {
        let enc_pair = encode_to_vec(&(a, b)).expect("encode pair");
        let enc_a = encode_to_vec(&a).expect("encode a");
        let enc_b = encode_to_vec(&b).expect("encode b");
        let concat: Vec<u8> = enc_a.iter().chain(enc_b.iter()).copied().collect();
        prop_assert_eq!(enc_pair, concat);
    });
}

// ── 5. Option None is smaller than Some(x) for all x: u32 ───────────────────

#[test]
fn prop_option_none_smaller() {
    proptest!(|(x: u32)| {
        let none_val: Option<u32> = None;
        let some_val: Option<u32> = Some(x);
        let enc_none = encode_to_vec(&none_val).expect("encode None");
        let enc_some = encode_to_vec(&some_val).expect("encode Some");
        prop_assert!(enc_none.len() < enc_some.len(),
            "None ({} bytes) should be smaller than Some({}) ({} bytes)",
            enc_none.len(), x, enc_some.len());
    });
}

// ── 6. Vec length prefix: first bytes encode the length ──────────────────────

#[test]
fn prop_vec_length_prefix() {
    proptest!(|(elems in proptest::collection::vec(any::<u32>(), 0usize..10))| {
        let encoded = encode_to_vec(&elems).expect("encode Vec<u32>");
        // With varint encoding, the first byte for lengths < 128 is just the length itself
        let len = elems.len() as u8;
        if elems.len() < 128 {
            prop_assert!(!encoded.is_empty(), "encoded should not be empty");
            prop_assert_eq!(encoded[0], len, "first byte should be the length varint for short vecs");
        }
    });
}

// ── 7. Bool encoding: encode(true) != encode(false) ──────────────────────────

#[test]
fn prop_bool_encoding_differs() {
    proptest!(|(_dummy: u8)| {
        let enc_true = encode_to_vec(&true).expect("encode true");
        let enc_false = encode_to_vec(&false).expect("encode false");
        prop_assert_ne!(enc_true, enc_false, "true and false must encode differently");
    });
}

// ── 8. Fixed-int config: u32 always 4 bytes ──────────────────────────────────

#[test]
fn prop_fixed_int_u32_size() {
    proptest!(|(val: u32)| {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode u32 fixed");
        prop_assert_eq!(enc.len(), 4, "u32 with fixed-int encoding must be exactly 4 bytes");
    });
}

// ── 9. Fixed-int config: u64 always 8 bytes ──────────────────────────────────

#[test]
fn prop_fixed_int_u64_size() {
    proptest!(|(val: u64)| {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode u64 fixed");
        prop_assert_eq!(enc.len(), 8, "u64 with fixed-int encoding must be exactly 8 bytes");
    });
}

// ── 10. Endianness: big- and little-endian differ for values >255 ─────────────

#[test]
fn prop_endianness_differs() {
    proptest!(|(val in 256u32..=u32::MAX)| {
        let cfg_le = config::standard().with_fixed_int_encoding().with_little_endian();
        let cfg_be = config::standard().with_fixed_int_encoding().with_big_endian();
        let enc_le = encode_to_vec_with_config(&val, cfg_le).expect("encode LE");
        let enc_be = encode_to_vec_with_config(&val, cfg_be).expect("encode BE");
        prop_assert_ne!(enc_le, enc_be,
            "LE and BE encodings of {} must differ", val);
    });
}

// ── 11. Config roundtrip: encode with cfg A, decode with cfg A succeeds ───────

#[test]
fn prop_config_roundtrip() {
    proptest!(|(val: u64)| {
        let cfg = config::standard().with_fixed_int_encoding().with_big_endian();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode with config");
        let (dec, _): (u64, usize) = decode_from_slice_with_config(&enc, cfg).expect("decode with same config");
        prop_assert_eq!(val, dec);
    });
}

// ── 12. Cross-config failure: fixed-int encode, default decode → wrong value ──

#[test]
fn prop_cross_config_wrong_value() {
    proptest!(|(val in 256u32..=u32::MAX)| {
        let fixed_cfg = config::standard().with_fixed_int_encoding();
        let enc_fixed = encode_to_vec_with_config(&val, fixed_cfg).expect("encode fixed");
        // Decode with default (varint) config — it may succeed but the value should be different
        // or it may fail. Either way it should not produce the same original value.
        match decode_from_slice::<u32>(&enc_fixed) {
            Ok((decoded, _)) => {
                // If it decoded successfully, the value should be wrong
                prop_assert_ne!(decoded, val,
                    "cross-config decode of {} should not yield the same value", val);
            }
            Err(_) => {
                // Decoding failed — that's also acceptable (formats are incompatible)
            }
        }
    });
}

// ── 13. Non-empty Vec has more bytes than empty Vec ───────────────────────────

#[test]
fn prop_nonempty_vec_larger() {
    proptest!(|(elem: u32)| {
        let empty: Vec<u32> = vec![];
        let nonempty: Vec<u32> = vec![elem];
        let enc_empty = encode_to_vec(&empty).expect("encode empty vec");
        let enc_nonempty = encode_to_vec(&nonempty).expect("encode nonempty vec");
        prop_assert!(enc_empty.len() < enc_nonempty.len(),
            "empty vec ({} bytes) should be smaller than nonempty ({} bytes)",
            enc_empty.len(), enc_nonempty.len());
    });
}

// ── 14. u8 always encodes as exactly 1 byte ───────────────────────────────────

#[test]
fn prop_u8_exactly_one_byte() {
    proptest!(|(val: u8)| {
        let enc = encode_to_vec(&val).expect("encode u8");
        prop_assert_eq!(enc.len(), 1, "u8 must always encode to exactly 1 byte, got {}", enc.len());
    });
}

// ── 15. Consumed invariant: consumed == encode_to_vec length ─────────────────

#[test]
fn prop_consumed_equals_len() {
    proptest!(|(val: u64)| {
        let enc = encode_to_vec(&val).expect("encode u64");
        let (_, consumed) = decode_from_slice::<u64>(&enc).expect("decode u64");
        prop_assert_eq!(consumed, enc.len(),
            "consumed bytes {} must equal encoded length {}", consumed, enc.len());
    });
}

// ── 16. i32 zigzag: small negatives encode efficiently (<9 bytes) ─────────────

#[test]
fn prop_i32_small_negatives_efficient() {
    proptest!(|(val in -1000i32..=-1i32)| {
        let enc = encode_to_vec(&val).expect("encode negative i32");
        prop_assert!(enc.len() < 9,
            "negative i32 {} encoded to {} bytes (expected < 9)", val, enc.len());
    });
}

// ── 17. HashMap roundtrip ─────────────────────────────────────────────────────

#[test]
fn prop_hashmap_roundtrip() {
    proptest!(|(map in proptest::collection::hash_map(any::<u32>(), any::<u32>(), 0..20))| {
        use std::collections::HashMap;
        let enc = encode_to_vec(&map).expect("encode HashMap");
        let (dec, _): (HashMap<u32, u32>, usize) = decode_from_slice(&enc).expect("decode HashMap");
        prop_assert_eq!(map, dec);
    });
}

// ── 18. BTreeMap roundtrip ────────────────────────────────────────────────────

#[test]
fn prop_btreemap_roundtrip() {
    proptest!(|(map in proptest::collection::btree_map(any::<u32>(), any::<String>(), 0..20))| {
        use std::collections::BTreeMap;
        let enc = encode_to_vec(&map).expect("encode BTreeMap");
        let (dec, _): (BTreeMap<u32, String>, usize) = decode_from_slice(&enc).expect("decode BTreeMap");
        prop_assert_eq!(map, dec);
    });
}

// ── 19. Nested Vec roundtrip ──────────────────────────────────────────────────

#[test]
fn prop_nested_vec_roundtrip() {
    proptest!(|(outer in proptest::collection::vec(
        proptest::collection::vec(any::<u8>(), 0..10),
        0..10
    ))| {
        let enc = encode_to_vec(&outer).expect("encode Vec<Vec<u8>>");
        let (dec, _): (Vec<Vec<u8>>, usize) = decode_from_slice(&enc).expect("decode Vec<Vec<u8>>");
        prop_assert_eq!(outer, dec);
    });
}

// ── 20. Struct roundtrip ──────────────────────────────────────────────────────

#[test]
fn prop_struct_roundtrip() {
    proptest!(|(id: u32, name: String, flag: bool)| {
        let val = PropStruct { id, name, flag };
        let enc = encode_to_vec(&val).expect("encode PropStruct");
        let (dec, _): (PropStruct, usize) = decode_from_slice(&enc).expect("decode PropStruct");
        prop_assert_eq!(val, dec);
    });
}

// ── 21. Enum roundtrip ────────────────────────────────────────────────────────

#[test]
fn prop_enum_roundtrip() {
    // Use manual strategy to cover all variants
    proptest!(|(variant_tag in 0u8..3, num: u32, text: String)| {
        let val = match variant_tag % 3 {
            0 => PropEnum::Unit,
            1 => PropEnum::Newtype(num),
            _ => PropEnum::Pair(num, text),
        };
        let enc = encode_to_vec(&val).expect("encode PropEnum");
        let (dec, _): (PropEnum, usize) = decode_from_slice(&enc).expect("decode PropEnum");
        prop_assert_eq!(val, dec);
    });
}

// ── 22. Option consistency: Some prefix is same for all x: u32 ───────────────

#[test]
fn prop_option_some_prefix_consistent() {
    proptest!(|(a: u32, b: u32)| {
        let enc_a = encode_to_vec(&Some(a)).expect("encode Some(a)");
        let enc_b = encode_to_vec(&Some(b)).expect("encode Some(b)");
        // Both Some variants must start with the same discriminant byte
        prop_assert!(!enc_a.is_empty(), "Some(a) encoded should not be empty");
        prop_assert!(!enc_b.is_empty(), "Some(b) encoded should not be empty");
        prop_assert_eq!(enc_a[0], enc_b[0],
            "Some({}) and Some({}) must share the same leading discriminant byte", a, b);
    });
}
