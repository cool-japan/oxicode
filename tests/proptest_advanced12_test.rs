//! Advanced property-based tests using proptest (set 12).
//!
//! Each test is a top-level #[test] function containing exactly one
//! proptest! macro block, verifying non-trivial invariants for Record,
//! Status, collections, smart pointers, and encoding properties.

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
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Record {
    id: u64,
    name: String,
    value: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Pending(String),
}

// ── Strategies ────────────────────────────────────────────────────────────────

fn arb_record() -> impl Strategy<Value = Record> {
    (any::<u64>(), any::<String>(), any::<f64>()).prop_map(|(id, name, value)| Record {
        id,
        name,
        value,
    })
}

fn arb_status() -> impl Strategy<Value = Status> {
    prop_oneof![
        Just(Status::Active),
        Just(Status::Inactive),
        any::<String>().prop_map(Status::Pending),
    ]
}

// ── Test 1: Record struct roundtrip ──────────────────────────────────────────

#[test]
fn prop_record_roundtrip() {
    proptest!(|(r in arb_record())| {
        let encoded = encode_to_vec(&r).expect("encode Record");
        let (decoded, bytes_read): (Record, usize) =
            decode_from_slice(&encoded).expect("decode Record");
        prop_assert_eq!(&r, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 2: Status::Active roundtrip ─────────────────────────────────────────

#[test]
fn prop_status_active_roundtrip() {
    proptest!(|(_x: u8)| {
        let s = Status::Active;
        let encoded = encode_to_vec(&s).expect("encode Status::Active");
        let (decoded, bytes_read): (Status, usize) =
            decode_from_slice(&encoded).expect("decode Status::Active");
        prop_assert_eq!(&s, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 3: Status::Inactive roundtrip ───────────────────────────────────────

#[test]
fn prop_status_inactive_roundtrip() {
    proptest!(|(_x: u8)| {
        let s = Status::Inactive;
        let encoded = encode_to_vec(&s).expect("encode Status::Inactive");
        let (decoded, bytes_read): (Status, usize) =
            decode_from_slice(&encoded).expect("decode Status::Inactive");
        prop_assert_eq!(&s, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 4: Status::Pending(String) roundtrip ────────────────────────────────

#[test]
fn prop_status_pending_roundtrip() {
    proptest!(|(msg: String)| {
        let s = Status::Pending(msg.clone());
        let encoded = encode_to_vec(&s).expect("encode Status::Pending");
        let (decoded, bytes_read): (Status, usize) =
            decode_from_slice(&encoded).expect("decode Status::Pending");
        prop_assert_eq!(&s, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 5: Vec<Record> roundtrip (max 20 elements) ──────────────────────────

#[test]
fn prop_vec_record_roundtrip() {
    proptest!(|(records in proptest::collection::vec(arb_record(), 0usize..=20))| {
        let encoded = encode_to_vec(&records).expect("encode Vec<Record>");
        let (decoded, bytes_read): (Vec<Record>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Record>");
        prop_assert_eq!(&records, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 6: Vec<Status> roundtrip (max 50 elements) ──────────────────────────

#[test]
fn prop_vec_status_roundtrip() {
    proptest!(|(statuses in proptest::collection::vec(arb_status(), 0usize..=50))| {
        let encoded = encode_to_vec(&statuses).expect("encode Vec<Status>");
        let (decoded, bytes_read): (Vec<Status>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Status>");
        prop_assert_eq!(&statuses, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 7: Option<Record> roundtrip ─────────────────────────────────────────

#[test]
fn prop_option_record_roundtrip() {
    proptest!(|(opt in proptest::option::of(arb_record()))| {
        let encoded = encode_to_vec(&opt).expect("encode Option<Record>");
        let (decoded, bytes_read): (Option<Record>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Record>");
        prop_assert_eq!(&opt, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 8: BTreeMap<u64, Record> roundtrip (max 10 entries) ─────────────────

#[test]
fn prop_btreemap_u64_record_roundtrip() {
    proptest!(|(entries in proptest::collection::btree_map(any::<u64>(), arb_record(), 0usize..=10))| {
        let encoded = encode_to_vec(&entries).expect("encode BTreeMap<u64, Record>");
        let (decoded, bytes_read): (BTreeMap<u64, Record>, usize) =
            decode_from_slice(&encoded).expect("decode BTreeMap<u64, Record>");
        prop_assert_eq!(&entries, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 9: HashMap<String, Status> roundtrip (max 10 entries) ───────────────

#[test]
fn prop_hashmap_string_status_roundtrip() {
    proptest!(|(entries in proptest::collection::hash_map(any::<String>(), arb_status(), 0usize..=10))| {
        let encoded = encode_to_vec(&entries).expect("encode HashMap<String, Status>");
        let (decoded, bytes_read): (HashMap<String, Status>, usize) =
            decode_from_slice(&encoded).expect("decode HashMap<String, Status>");
        prop_assert_eq!(&entries, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 10: encode(decode(encode(x))) == encode(x) for Record ───────────────

#[test]
fn prop_record_reencode_invariant() {
    proptest!(|(r in arb_record())| {
        let enc1 = encode_to_vec(&r).expect("first encode Record");
        let (decoded, _): (Record, usize) =
            decode_from_slice(&enc1).expect("decode Record for re-encode");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Record");
        prop_assert_eq!(&enc1, &enc2, "re-encode must produce identical bytes");
    });
}

// ── Test 11: Encode size >= 1 for any Record ─────────────────────────────────

#[test]
fn prop_record_encode_size_at_least_one() {
    proptest!(|(r in arb_record())| {
        let encoded = encode_to_vec(&r).expect("encode Record for size check");
        prop_assert!(
            encoded.len() >= 1,
            "encoded Record must be at least 1 byte, got 0"
        );
    });
}

// ── Test 12: Two different Records may produce different encodings ────────────

#[test]
fn prop_different_records_different_encodings() {
    proptest!(|(r1 in arb_record(), r2 in arb_record())| {
        prop_assume!(r1 != r2);
        let enc1 = encode_to_vec(&r1).expect("encode Record r1");
        let enc2 = encode_to_vec(&r2).expect("encode Record r2");
        prop_assert_ne!(enc1, enc2, "different Records should produce different encodings");
    });
}

// ── Test 13: Record with id=0 roundtrip ──────────────────────────────────────

#[test]
fn prop_record_id_zero_roundtrip() {
    proptest!(|(name: String, value: f64)| {
        let r = Record { id: 0, name: name.clone(), value };
        let encoded = encode_to_vec(&r).expect("encode Record id=0");
        let (decoded, bytes_read): (Record, usize) =
            decode_from_slice(&encoded).expect("decode Record id=0");
        prop_assert_eq!(&r, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 14: Record with empty name roundtrip ─────────────────────────────────

#[test]
fn prop_record_empty_name_roundtrip() {
    proptest!(|(id: u64, value: f64)| {
        let r = Record { id, name: String::new(), value };
        let encoded = encode_to_vec(&r).expect("encode Record empty name");
        let (decoded, bytes_read): (Record, usize) =
            decode_from_slice(&encoded).expect("decode Record empty name");
        prop_assert_eq!(&r, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 15: f64 bit-exact roundtrip (NaN safe) ──────────────────────────────

#[test]
fn prop_f64_bit_exact_roundtrip() {
    proptest!(|(bits: u64)| {
        let val = f64::from_bits(bits);
        let enc = encode_to_vec(&val).expect("encode f64");
        let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64");
        prop_assert_eq!(val.to_bits(), dec.to_bits(), "f64 bit roundtrip");
    });
}

// ── Test 16: Vec<Vec<Status>> roundtrip ──────────────────────────────────────

#[test]
fn prop_nested_vec_status_roundtrip() {
    proptest!(|(nested in proptest::collection::vec(
        proptest::collection::vec(arb_status(), 0usize..10),
        0usize..10
    ))| {
        let encoded = encode_to_vec(&nested).expect("encode Vec<Vec<Status>>");
        let (decoded, bytes_read): (Vec<Vec<Status>>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Vec<Status>>");
        prop_assert_eq!(&nested, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 17: Tuple (Record, Status) roundtrip ────────────────────────────────

#[test]
fn prop_tuple_record_status_roundtrip() {
    proptest!(|(r in arb_record(), s in arb_status())| {
        let tup = (r.clone(), s.clone());
        let encoded = encode_to_vec(&tup).expect("encode (Record, Status)");
        let (decoded, bytes_read): ((Record, Status), usize) =
            decode_from_slice(&encoded).expect("decode (Record, Status)");
        prop_assert_eq!(&tup, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 18: Box<Record> roundtrip ───────────────────────────────────────────

#[test]
fn prop_box_record_roundtrip() {
    proptest!(|(r in arb_record())| {
        let boxed = Box::new(r.clone());
        let encoded = encode_to_vec(&boxed).expect("encode Box<Record>");
        let (decoded, bytes_read): (Box<Record>, usize) =
            decode_from_slice(&encoded).expect("decode Box<Record>");
        prop_assert_eq!(&boxed, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 19: Arc<Record> roundtrip ───────────────────────────────────────────

#[test]
fn prop_arc_record_roundtrip() {
    proptest!(|(r in arb_record())| {
        let arc = Arc::new(r.clone());
        let encoded = encode_to_vec(&arc).expect("encode Arc<Record>");
        let (decoded, bytes_read): (Arc<Record>, usize) =
            decode_from_slice(&encoded).expect("decode Arc<Record>");
        prop_assert_eq!(arc.as_ref(), decoded.as_ref());
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 20: Option<Vec<Status>> roundtrip ───────────────────────────────────

#[test]
fn prop_option_vec_status_roundtrip() {
    proptest!(|(opt in proptest::option::of(proptest::collection::vec(arb_status(), 0usize..20)))| {
        let encoded = encode_to_vec(&opt).expect("encode Option<Vec<Status>>");
        let (decoded, bytes_read): (Option<Vec<Status>>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Vec<Status>>");
        prop_assert_eq!(&opt, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 21: Sequential encode 5 Records, decode all back in order ────────────

#[test]
fn prop_sequential_encode_5_records() {
    proptest!(|(r0 in arb_record(), r1 in arb_record(), r2 in arb_record(), r3 in arb_record(), r4 in arb_record())| {
        let originals = [r0.clone(), r1.clone(), r2.clone(), r3.clone(), r4.clone()];
        let mut buf: Vec<u8> = Vec::new();
        for r in &originals {
            let enc = encode_to_vec(r).expect("encode Record sequentially");
            buf.extend_from_slice(&enc);
        }
        let mut offset = 0usize;
        for (i, orig) in originals.iter().enumerate() {
            let (decoded, consumed): (Record, usize) =
                decode_from_slice(&buf[offset..]).expect("decode Record sequentially");
            prop_assert_eq!(orig, &decoded, "record {} mismatch", i);
            offset += consumed;
        }
        prop_assert_eq!(offset, buf.len(), "all bytes must be consumed");
    });
}

// ── Test 22: encode_to_vec then decode_from_slice consumed == encode_to_vec length

#[test]
fn prop_encode_length_equals_consumed() {
    proptest!(|(r in arb_record())| {
        let encoded = encode_to_vec(&r).expect("encode Record for length check");
        let enc_len = encoded.len();
        let (_decoded, consumed): (Record, usize) =
            decode_from_slice(&encoded).expect("decode Record for length check");
        prop_assert_eq!(
            consumed, enc_len,
            "decode_from_slice consumed ({}) must equal encode_to_vec length ({})",
            consumed, enc_len
        );
    });
}
