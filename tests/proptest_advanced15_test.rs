//! Advanced property-based tests (set 15) using proptest.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers Invoice roundtrip, collections, numeric types, tuples, and more.

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
use std::collections::{BTreeMap, HashSet, LinkedList, VecDeque};
use std::num::Wrapping;

use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Invoice {
    id: u64,
    amount: u32,
    paid: bool,
    note: String,
}

// ── 1. Invoice roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_invoice_roundtrip() {
    proptest!(|(
        id: u64,
        amount in 0u32..1_000_000u32,
        paid: bool,
        note in "[a-zA-Z0-9 ]{0,50}",
    )| {
        let inv = Invoice { id, amount, paid, note };
        let enc = encode_to_vec(&inv).expect("encode Invoice failed");
        let (decoded, _): (Invoice, usize) = decode_from_slice(&enc).expect("decode Invoice failed");
        prop_assert_eq!(inv, decoded);
    });
}

// ── 2. Vec<Invoice> roundtrip ─────────────────────────────────────────────────

#[test]
fn test_vec_invoice_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (any::<u64>(), 0u32..1_000_000u32, any::<bool>(), "[a-zA-Z0-9 ]{0,50}").prop_map(
                |(id, amount, paid, note)| Invoice { id, amount, paid, note }
            ),
            0..10,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<Invoice> failed");
        let (decoded, _): (Vec<Invoice>, usize) = decode_from_slice(&enc).expect("decode Vec<Invoice> failed");
        prop_assert_eq!(items, decoded);
    });
}

// ── 3. BTreeMap<u64, String> roundtrip ───────────────────────────────────────

#[test]
fn test_btreemap_u64_string_roundtrip() {
    proptest!(|(
        map in prop::collection::btree_map(any::<u64>(), "[a-zA-Z0-9]{0,30}", 0..15usize),
    )| {
        let enc = encode_to_vec(&map).expect("encode BTreeMap failed");
        let (decoded, _): (BTreeMap<u64, String>, usize) =
            decode_from_slice(&enc).expect("decode BTreeMap failed");
        prop_assert_eq!(map, decoded);
    });
}

// ── 4. Vec<(u32, String)> tuples roundtrip ───────────────────────────────────

#[test]
fn test_vec_tuple_u32_string_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (any::<u32>(), "[a-zA-Z0-9]{0,20}"),
            0..20usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<(u32,String)> failed");
        let (decoded, _): (Vec<(u32, String)>, usize) =
            decode_from_slice(&enc).expect("decode Vec<(u32,String)> failed");
        prop_assert_eq!(items, decoded);
    });
}

// ── 5. i8 roundtrip ──────────────────────────────────────────────────────────

#[test]
fn test_i8_roundtrip() {
    proptest!(|(v: i8)| {
        let enc = encode_to_vec(&v).expect("encode i8 failed");
        let (decoded, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8 failed");
        prop_assert_eq!(v, decoded);
    });
}

// ── 6. i16 roundtrip ─────────────────────────────────────────────────────────

#[test]
fn test_i16_roundtrip() {
    proptest!(|(v: i16)| {
        let enc = encode_to_vec(&v).expect("encode i16 failed");
        let (decoded, _): (i16, usize) = decode_from_slice(&enc).expect("decode i16 failed");
        prop_assert_eq!(v, decoded);
    });
}

// ── 7. i32 zigzag: encode length ≤ encode of corresponding positive + 1 ──────

#[test]
fn test_i32_zigzag_length_bound() {
    proptest!(|(v: i32)| {
        let enc_signed = encode_to_vec(&v).expect("encode i32 failed");
        // The absolute value (as u32) is the positive counterpart in zigzag terms.
        // Zigzag maps i32 -> u32: positive n -> 2n, negative n -> 2|n|-1.
        // So a negative i32 always maps to a u32 that is at most 2*|i32::MIN| which
        // fits u32 (zigzag of i32::MIN is u32::MAX). The encoded size of the zigzag
        // value should never exceed the encoded size of the same magnitude as u32 + 1.
        let abs_val = (v as i64).unsigned_abs() as u32;
        let enc_pos = encode_to_vec(&abs_val).expect("encode u32 abs failed");
        prop_assert!(enc_signed.len() <= enc_pos.len() + 1);
    });
}

// ── 8. u128 roundtrip ────────────────────────────────────────────────────────

#[test]
fn test_u128_roundtrip() {
    proptest!(|(v: u128)| {
        let enc = encode_to_vec(&v).expect("encode u128 failed");
        let (decoded, _): (u128, usize) = decode_from_slice(&enc).expect("decode u128 failed");
        prop_assert_eq!(v, decoded);
    });
}

// ── 9. i128 roundtrip ────────────────────────────────────────────────────────

#[test]
fn test_i128_roundtrip() {
    proptest!(|(v: i128)| {
        let enc = encode_to_vec(&v).expect("encode i128 failed");
        let (decoded, _): (i128, usize) = decode_from_slice(&enc).expect("decode i128 failed");
        prop_assert_eq!(v, decoded);
    });
}

// ── 10. [u32; 4] fixed array roundtrip ───────────────────────────────────────

#[test]
fn test_fixed_array_u32_4_roundtrip() {
    proptest!(|(v: [u32; 4])| {
        let enc = encode_to_vec(&v).expect("encode [u32;4] failed");
        let (decoded, _): ([u32; 4], usize) =
            decode_from_slice(&enc).expect("decode [u32;4] failed");
        prop_assert_eq!(v, decoded);
    });
}

// ── 11. [u8; 16] fixed array roundtrip ───────────────────────────────────────

#[test]
fn test_fixed_array_u8_16_roundtrip() {
    proptest!(|(v: [u8; 16])| {
        let enc = encode_to_vec(&v).expect("encode [u8;16] failed");
        let (decoded, _): ([u8; 16], usize) =
            decode_from_slice(&enc).expect("decode [u8;16] failed");
        prop_assert_eq!(v, decoded);
    });
}

// ── 12. (u8, u16, u32, u64) 4-tuple roundtrip ────────────────────────────────

#[test]
fn test_4tuple_u8_u16_u32_u64_roundtrip() {
    proptest!(|(a: u8, b: u16, c: u32, d: u64)| {
        let tup = (a, b, c, d);
        let enc = encode_to_vec(&tup).expect("encode 4-tuple failed");
        let (decoded, _): ((u8, u16, u32, u64), usize) =
            decode_from_slice(&enc).expect("decode 4-tuple failed");
        prop_assert_eq!(tup, decoded);
    });
}

// ── 13. (bool, bool, bool) roundtrip ─────────────────────────────────────────

#[test]
fn test_triple_bool_roundtrip() {
    proptest!(|(a: bool, b: bool, c: bool)| {
        let tup = (a, b, c);
        let enc = encode_to_vec(&tup).expect("encode (bool,bool,bool) failed");
        let (decoded, _): ((bool, bool, bool), usize) =
            decode_from_slice(&enc).expect("decode (bool,bool,bool) failed");
        prop_assert_eq!(tup, decoded);
    });
}

// ── 14. Result<u32, String> Ok roundtrip ─────────────────────────────────────

#[test]
fn test_result_ok_roundtrip() {
    proptest!(|(v: u32)| {
        let res: Result<u32, String> = Ok(v);
        let enc = encode_to_vec(&res).expect("encode Result::Ok failed");
        let (decoded, _): (Result<u32, String>, usize) =
            decode_from_slice(&enc).expect("decode Result::Ok failed");
        prop_assert_eq!(res, decoded);
    });
}

// ── 15. Result<u32, String> Err roundtrip ────────────────────────────────────

#[test]
fn test_result_err_roundtrip() {
    proptest!(|(msg in "[a-zA-Z0-9 ]{0,40}")| {
        let res: Result<u32, String> = Err(msg);
        let enc = encode_to_vec(&res).expect("encode Result::Err failed");
        let (decoded, _): (Result<u32, String>, usize) =
            decode_from_slice(&enc).expect("decode Result::Err failed");
        prop_assert_eq!(res, decoded);
    });
}

// ── 16. Re-encoding decoded Invoice gives identical bytes ─────────────────────

#[test]
fn test_invoice_reencode_idempotent() {
    proptest!(|(
        id: u64,
        amount in 0u32..1_000_000u32,
        paid: bool,
        note in "[a-zA-Z0-9 ]{0,50}",
    )| {
        let inv = Invoice { id, amount, paid, note };
        let enc1 = encode_to_vec(&inv).expect("first encode failed");
        let (decoded, _): (Invoice, usize) =
            decode_from_slice(&enc1).expect("first decode failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode failed");
        prop_assert_eq!(enc1, enc2, "re-encoding should produce identical bytes");
    });
}

// ── 17. Option<Vec<u32>> roundtrip ───────────────────────────────────────────

#[test]
fn test_option_vec_u32_roundtrip() {
    proptest!(|(
        opt in prop::option::of(
            prop::collection::vec(any::<u32>(), 0..20usize)
        ),
    )| {
        let enc = encode_to_vec(&opt).expect("encode Option<Vec<u32>> failed");
        let (decoded, _): (Option<Vec<u32>>, usize) =
            decode_from_slice(&enc).expect("decode Option<Vec<u32>> failed");
        prop_assert_eq!(opt, decoded);
    });
}

// ── 18. Vec<Option<u32>> roundtrip ───────────────────────────────────────────

#[test]
fn test_vec_option_u32_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            prop::option::of(any::<u32>()),
            0..20usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<Option<u32>> failed");
        let (decoded, _): (Vec<Option<u32>>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Option<u32>> failed");
        prop_assert_eq!(items, decoded);
    });
}

// ── 19. HashSet<u32> roundtrip ────────────────────────────────────────────────

#[test]
fn test_hashset_u32_roundtrip() {
    proptest!(|(
        set in prop::collection::hash_set(any::<u32>(), 0..20usize),
    )| {
        let enc = encode_to_vec(&set).expect("encode HashSet<u32> failed");
        let (decoded, _): (HashSet<u32>, usize) =
            decode_from_slice(&enc).expect("decode HashSet<u32> failed");
        prop_assert_eq!(set, decoded);
    });
}

// ── 20. VecDeque<u32> roundtrip ──────────────────────────────────────────────

#[test]
fn test_vecdeque_u32_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(any::<u32>(), 0..20usize),
    )| {
        let deque: VecDeque<u32> = items.into_iter().collect();
        let enc = encode_to_vec(&deque).expect("encode VecDeque<u32> failed");
        let (decoded, _): (VecDeque<u32>, usize) =
            decode_from_slice(&enc).expect("decode VecDeque<u32> failed");
        prop_assert_eq!(deque, decoded);
    });
}

// ── 21. LinkedList<u32> roundtrip ────────────────────────────────────────────

#[test]
fn test_linked_list_u32_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(any::<u32>(), 0..10usize),
    )| {
        let list: LinkedList<u32> = items.into_iter().collect();
        let enc = encode_to_vec(&list).expect("encode LinkedList<u32> failed");
        let (decoded, _): (LinkedList<u32>, usize) =
            decode_from_slice(&enc).expect("decode LinkedList<u32> failed");
        prop_assert_eq!(list, decoded);
    });
}

// ── 22. Wrapping<u32> roundtrip ──────────────────────────────────────────────

#[test]
fn test_wrapping_u32_roundtrip() {
    proptest!(|(v: u32)| {
        let w = Wrapping(v);
        let enc = encode_to_vec(&w).expect("encode Wrapping<u32> failed");
        let (decoded, _): (Wrapping<u32>, usize) =
            decode_from_slice(&enc).expect("decode Wrapping<u32> failed");
        prop_assert_eq!(w, decoded);
    });
}
