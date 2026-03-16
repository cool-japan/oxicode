//! Advanced property-based tests using proptest (set 13).
//!
//! Each test is a top-level #[test] function containing exactly one
//! proptest! macro block, verifying roundtrip and encoding invariants for
//! Matrix, Expr, and various collection types.

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
use std::collections::{BTreeMap, BTreeSet, HashSet, LinkedList, VecDeque};

use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Matrix {
    rows: u32,
    cols: u32,
    data: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Expr {
    Num(i64),
    Str(String),
    Bool(bool),
}

// ── Strategies ────────────────────────────────────────────────────────────────

fn arb_expr() -> impl Strategy<Value = Expr> {
    prop_oneof![
        any::<i64>().prop_map(Expr::Num),
        any::<String>().prop_map(Expr::Str),
        any::<bool>().prop_map(Expr::Bool),
    ]
}

fn arb_matrix() -> impl Strategy<Value = Matrix> {
    (0u32..10, 0u32..10).prop_flat_map(|(rows, cols)| {
        let size = (rows * cols) as usize;
        proptest::collection::vec(any::<f64>(), size..=size).prop_map(move |data| Matrix {
            rows,
            cols,
            data,
        })
    })
}

// ── Test 1: Matrix struct roundtrip (any rows, cols 0..10, data of size rows*cols) ──

#[test]
fn prop_matrix_roundtrip() {
    proptest!(|(m in arb_matrix())| {
        let enc = encode_to_vec(&m).expect("encode Matrix");
        let (dec, bytes_read): (Matrix, usize) = decode_from_slice(&enc).expect("decode Matrix");
        prop_assert_eq!(&m, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 2: Expr::Num roundtrip ───────────────────────────────────────────────

#[test]
fn prop_expr_num_roundtrip() {
    proptest!(|(n: i64)| {
        let e = Expr::Num(n);
        let enc = encode_to_vec(&e).expect("encode Expr::Num");
        let (dec, bytes_read): (Expr, usize) = decode_from_slice(&enc).expect("decode Expr::Num");
        prop_assert_eq!(&e, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 3: Expr::Str roundtrip ───────────────────────────────────────────────

#[test]
fn prop_expr_str_roundtrip() {
    proptest!(|(s: String)| {
        let e = Expr::Str(s.clone());
        let enc = encode_to_vec(&e).expect("encode Expr::Str");
        let (dec, bytes_read): (Expr, usize) = decode_from_slice(&enc).expect("decode Expr::Str");
        prop_assert_eq!(&e, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 4: Expr::Bool roundtrip ──────────────────────────────────────────────

#[test]
fn prop_expr_bool_roundtrip() {
    proptest!(|(b: bool)| {
        let e = Expr::Bool(b);
        let enc = encode_to_vec(&e).expect("encode Expr::Bool");
        let (dec, bytes_read): (Expr, usize) = decode_from_slice(&enc).expect("decode Expr::Bool");
        prop_assert_eq!(&e, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 5: Vec<Expr> roundtrip (max 20) ─────────────────────────────────────

#[test]
fn prop_vec_expr_roundtrip() {
    proptest!(|(exprs in proptest::collection::vec(arb_expr(), 0usize..=20))| {
        let enc = encode_to_vec(&exprs).expect("encode Vec<Expr>");
        let (dec, bytes_read): (Vec<Expr>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Expr>");
        prop_assert_eq!(&exprs, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 6: Matrix with zero rows or cols roundtrip ───────────────────────────

#[test]
fn prop_matrix_zero_dim_roundtrip() {
    proptest!(|(rows in 0u32..10, cols in 0u32..10)| {
        // Force at least one dimension to be zero
        let (actual_rows, actual_cols) = if rows % 2 == 0 { (0u32, cols) } else { (rows, 0u32) };
        let m = Matrix {
            rows: actual_rows,
            cols: actual_cols,
            data: Vec::new(),
        };
        let enc = encode_to_vec(&m).expect("encode Matrix zero-dim");
        let (dec, bytes_read): (Matrix, usize) =
            decode_from_slice(&enc).expect("decode Matrix zero-dim");
        prop_assert_eq!(&m, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 7: Matrix data len consistency: rows * cols == data.len() ────────────

#[test]
fn prop_matrix_data_len_consistency() {
    proptest!(|(rows in 0u32..5, cols in 0u32..5)| {
        let size = (rows * cols) as usize;
        let data = vec![0.0f64; size];
        let m = Matrix { rows, cols, data };
        let enc = encode_to_vec(&m).expect("encode Matrix");
        let (dec, _): (Matrix, usize) = decode_from_slice(&enc).expect("decode Matrix");
        prop_assert_eq!(m.rows, dec.rows);
        prop_assert_eq!(m.cols, dec.cols);
        prop_assert_eq!(m.data.len(), dec.data.len());
    });
}

// ── Test 8: BTreeMap<String, Expr> roundtrip (max 10) ────────────────────────

#[test]
fn prop_btreemap_string_expr_roundtrip() {
    proptest!(|(map in proptest::collection::btree_map(any::<String>(), arb_expr(), 0usize..=10))| {
        let enc = encode_to_vec(&map).expect("encode BTreeMap<String, Expr>");
        let (dec, bytes_read): (BTreeMap<String, Expr>, usize) =
            decode_from_slice(&enc).expect("decode BTreeMap<String, Expr>");
        prop_assert_eq!(&map, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 9: All f64 values roundtrip bit-exactly (including NaN/inf) ──────────

#[test]
fn prop_f64_bit_exact_roundtrip() {
    proptest!(|(bits: u64)| {
        let val = f64::from_bits(bits);
        let enc = encode_to_vec(&val).expect("encode f64");
        let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64");
        prop_assert_eq!(
            val.to_bits(),
            dec.to_bits(),
            "f64 bit-exact roundtrip failed for bits={:#018x}",
            bits
        );
    });
}

// ── Test 10: All f32 values roundtrip bit-exactly ─────────────────────────────

#[test]
fn prop_f32_bit_exact_roundtrip() {
    proptest!(|(bits: u32)| {
        let val = f32::from_bits(bits);
        let enc = encode_to_vec(&val).expect("encode f32");
        let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32");
        prop_assert_eq!(
            val.to_bits(),
            dec.to_bits(),
            "f32 bit-exact roundtrip failed for bits={:#010x}",
            bits
        );
    });
}

// ── Test 11: char roundtrip (any valid Unicode char) ─────────────────────────

#[test]
fn prop_char_roundtrip() {
    proptest!(|(c: char)| {
        let enc = encode_to_vec(&c).expect("encode char");
        let (dec, bytes_read): (char, usize) = decode_from_slice(&enc).expect("decode char");
        prop_assert_eq!(c, dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 12: (u32, u64, String) 3-tuple roundtrip ────────────────────────────

#[test]
fn prop_triple_tuple_roundtrip() {
    proptest!(|(a: u32, b: u64, s: String)| {
        let tup = (a, b, s.clone());
        let enc = encode_to_vec(&tup).expect("encode (u32, u64, String)");
        let (dec, bytes_read): ((u32, u64, String), usize) =
            decode_from_slice(&enc).expect("decode (u32, u64, String)");
        prop_assert_eq!(&tup, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 13: Option<Expr> roundtrip ──────────────────────────────────────────

#[test]
fn prop_option_expr_roundtrip() {
    proptest!(|(opt in proptest::option::of(arb_expr()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<Expr>");
        let (dec, bytes_read): (Option<Expr>, usize) =
            decode_from_slice(&enc).expect("decode Option<Expr>");
        prop_assert_eq!(&opt, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 14: Vec<Vec<u32>> roundtrip (max 5x5) ───────────────────────────────

#[test]
fn prop_nested_vec_u32_roundtrip() {
    proptest!(|(v in proptest::collection::vec(
        proptest::collection::vec(any::<u32>(), 0usize..=5),
        0usize..=5
    ))| {
        let enc = encode_to_vec(&v).expect("encode Vec<Vec<u32>>");
        let (dec, bytes_read): (Vec<Vec<u32>>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Vec<u32>>");
        prop_assert_eq!(&v, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 15: encode then re-encode invariant for Matrix ───────────────────────

#[test]
fn prop_matrix_reencode_invariant() {
    proptest!(|(m in arb_matrix())| {
        let enc1 = encode_to_vec(&m).expect("first encode Matrix");
        let (decoded, _): (Matrix, usize) =
            decode_from_slice(&enc1).expect("decode Matrix for re-encode");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Matrix");
        prop_assert_eq!(&enc1, &enc2, "re-encode must produce identical bytes");
    });
}

// ── Test 16: String encode: len(encode(s)) == 1 + len(s) for short strings (len <= 250) ──

#[test]
fn prop_short_string_encode_len() {
    proptest!(|(s in proptest::collection::vec(0x20u8..=0x7Eu8, 0usize..=250usize)
        .prop_map(|bytes| String::from_utf8(bytes).expect("valid ascii string")))| {
        let n = s.len();
        let enc = encode_to_vec(&s).expect("encode String");
        let expected = 1 + n; // varint(n) is 1 byte for n <= 250, then n bytes of UTF-8
        prop_assert_eq!(
            enc.len(),
            expected,
            "String of len {} should encode to {} bytes but got {}",
            n, expected, enc.len()
        );
    });
}

// ── Test 17: u32 values 0..=250 always encode as single byte ─────────────────

#[test]
fn prop_u32_small_single_byte() {
    proptest!(|(v in 0u32..=250u32)| {
        let enc = encode_to_vec(&v).expect("encode small u32");
        prop_assert_eq!(
            enc.len(),
            1,
            "u32 value {} should encode as 1 byte but got {} bytes",
            v, enc.len()
        );
    });
}

// ── Test 18: u64 values > u32::MAX encode as 9 bytes ─────────────────────────

#[test]
fn prop_u64_large_nine_bytes() {
    proptest!(|(extra in 0u64..u64::MAX - u32::MAX as u64)| {
        let v: u64 = u32::MAX as u64 + 1 + extra;
        let enc = encode_to_vec(&v).expect("encode large u64");
        prop_assert_eq!(
            enc.len(),
            9,
            "u64 value {} (> u32::MAX) should encode as 9 bytes but got {}",
            v, enc.len()
        );
    });
}

// ── Test 19: BTreeSet<u32> roundtrip (max 20 elements) ───────────────────────

#[test]
fn prop_btreeset_u32_roundtrip() {
    proptest!(|(set in proptest::collection::btree_set(any::<u32>(), 0usize..=20))| {
        let enc = encode_to_vec(&set).expect("encode BTreeSet<u32>");
        let (dec, bytes_read): (BTreeSet<u32>, usize) =
            decode_from_slice(&enc).expect("decode BTreeSet<u32>");
        prop_assert_eq!(&set, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 20: HashSet<u32> roundtrip (max 20 elements) ────────────────────────

#[test]
fn prop_hashset_u32_roundtrip() {
    proptest!(|(set in proptest::collection::hash_set(any::<u32>(), 0usize..=20))| {
        let enc = encode_to_vec(&set).expect("encode HashSet<u32>");
        let (dec, bytes_read): (HashSet<u32>, usize) =
            decode_from_slice(&enc).expect("decode HashSet<u32>");
        prop_assert_eq!(&set, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 21: VecDeque<u32> roundtrip (max 20) ────────────────────────────────

#[test]
fn prop_vecdeque_u32_roundtrip() {
    proptest!(|(v in proptest::collection::vec(any::<u32>(), 0usize..=20))| {
        let deque: VecDeque<u32> = v.into_iter().collect();
        let enc = encode_to_vec(&deque).expect("encode VecDeque<u32>");
        let (dec, bytes_read): (VecDeque<u32>, usize) =
            decode_from_slice(&enc).expect("decode VecDeque<u32>");
        prop_assert_eq!(&deque, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 22: LinkedList<u32> roundtrip (max 10) ───────────────────────────────

#[test]
fn prop_linkedlist_u32_roundtrip() {
    proptest!(|(v in proptest::collection::vec(any::<u32>(), 0usize..=10))| {
        let list: LinkedList<u32> = v.into_iter().collect();
        let enc = encode_to_vec(&list).expect("encode LinkedList<u32>");
        let (dec, bytes_read): (LinkedList<u32>, usize) =
            decode_from_slice(&enc).expect("decode LinkedList<u32>");
        prop_assert_eq!(&list, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}
