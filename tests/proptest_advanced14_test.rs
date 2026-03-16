//! Advanced property-based tests using proptest (set 14).
//!
//! Each test is a top-level #[test] function containing exactly one
//! proptest! macro block, verifying unique properties and invariants
//! for a variety of Rust standard-library types.

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
use std::collections::{BTreeMap, BTreeSet, HashMap, LinkedList};
use std::num::{NonZeroI32, NonZeroU32, Saturating, Wrapping};
use std::rc::Rc;
use std::sync::Arc;

use oxicode::{decode_from_slice, encode_to_vec};
use proptest::prelude::*;

// ── Test 1: NonZeroU32 roundtrip ──────────────────────────────────────────────

#[test]
fn prop_nonzero_u32_roundtrip() {
    proptest!(|(v in 1u32..=u32::MAX)| {
        let nz = std::num::NonZeroU32::new(v).expect("nonzero");
        let encoded = encode_to_vec(&nz).expect("encode NonZeroU32");
        let (decoded, bytes_read): (NonZeroU32, usize) =
            decode_from_slice(&encoded).expect("decode NonZeroU32");
        prop_assert_eq!(nz, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 2: NonZeroI32 roundtrip ──────────────────────────────────────────────

#[test]
fn prop_nonzero_i32_roundtrip() {
    proptest!(|(v in prop_oneof![
        (1i32..=i32::MAX),
        (i32::MIN..=-1i32),
    ])| {
        let nz = std::num::NonZeroI32::new(v).expect("nonzero i32");
        let encoded = encode_to_vec(&nz).expect("encode NonZeroI32");
        let (decoded, bytes_read): (NonZeroI32, usize) =
            decode_from_slice(&encoded).expect("decode NonZeroI32");
        prop_assert_eq!(nz, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 3: Duration roundtrip ────────────────────────────────────────────────

#[test]
fn prop_duration_roundtrip() {
    proptest!(|(secs: u64, nanos in 0u32..1_000_000_000)| {
        let dur = std::time::Duration::new(secs, nanos);
        let encoded = encode_to_vec(&dur).expect("encode Duration");
        let (decoded, bytes_read): (std::time::Duration, usize) =
            decode_from_slice(&encoded).expect("decode Duration");
        prop_assert_eq!(dur, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 4: IpAddr::V4 roundtrip ─────────────────────────────────────────────

#[test]
fn prop_ipaddr_v4_roundtrip() {
    proptest!(|(octets: [u8; 4])| {
        let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::from(octets));
        let encoded = encode_to_vec(&ip).expect("encode IpAddr::V4");
        let (decoded, bytes_read): (std::net::IpAddr, usize) =
            decode_from_slice(&encoded).expect("decode IpAddr::V4");
        prop_assert_eq!(ip, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 5: IpAddr::V6 roundtrip ─────────────────────────────────────────────

#[test]
fn prop_ipaddr_v6_roundtrip() {
    proptest!(|(segments: [u16; 8])| {
        let ip = std::net::IpAddr::V6(std::net::Ipv6Addr::from(segments));
        let encoded = encode_to_vec(&ip).expect("encode IpAddr::V6");
        let (decoded, bytes_read): (std::net::IpAddr, usize) =
            decode_from_slice(&encoded).expect("decode IpAddr::V6");
        prop_assert_eq!(ip, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 6: Ipv4Addr encode is always exactly 4 bytes ────────────────────────

#[test]
fn prop_ipv4addr_encode_exactly_4_bytes() {
    proptest!(|(octets: [u8; 4])| {
        let ip = std::net::Ipv4Addr::from(octets);
        let encoded = encode_to_vec(&ip).expect("encode Ipv4Addr");
        prop_assert_eq!(
            encoded.len(),
            4,
            "Ipv4Addr must always encode to exactly 4 bytes, got {}",
            encoded.len()
        );
    });
}

// ── Test 7: char always roundtrips ───────────────────────────────────────────

#[test]
fn prop_char_roundtrip() {
    proptest!(|(c: char)| {
        let encoded = encode_to_vec(&c).expect("encode char");
        let (decoded, bytes_read): (char, usize) =
            decode_from_slice(&encoded).expect("decode char");
        prop_assert_eq!(c, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 8: Range<u32> roundtrip (start <= end) ───────────────────────────────

#[test]
fn prop_range_u32_roundtrip() {
    proptest!(|(a: u32, b: u32)| {
        let (start, end) = if a <= b { (a, b) } else { (b, a) };
        let range = start..end;
        let encoded = encode_to_vec(&range).expect("encode Range<u32>");
        let (decoded, bytes_read): (std::ops::Range<u32>, usize) =
            decode_from_slice(&encoded).expect("decode Range<u32>");
        prop_assert_eq!(range.start, decoded.start);
        prop_assert_eq!(range.end, decoded.end);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 9: RangeInclusive<u32> roundtrip ────────────────────────────────────

#[test]
fn prop_range_inclusive_u32_roundtrip() {
    proptest!(|(a: u32, b: u32)| {
        let (start, end) = if a <= b { (a, b) } else { (b, a) };
        let range = start..=end;
        let encoded = encode_to_vec(&range).expect("encode RangeInclusive<u32>");
        let (decoded, bytes_read): (std::ops::RangeInclusive<u32>, usize) =
            decode_from_slice(&encoded).expect("decode RangeInclusive<u32>");
        prop_assert_eq!(*range.start(), *decoded.start());
        prop_assert_eq!(*range.end(), *decoded.end());
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 10: Ordering roundtrip (all 3 values) ───────────────────────────────

#[test]
fn prop_ordering_roundtrip() {
    proptest!(|(x in 0u8..3u8)| {
        let ordering = match x {
            0 => std::cmp::Ordering::Less,
            1 => std::cmp::Ordering::Equal,
            _ => std::cmp::Ordering::Greater,
        };
        let encoded = encode_to_vec(&ordering).expect("encode Ordering");
        let (decoded, bytes_read): (std::cmp::Ordering, usize) =
            decode_from_slice(&encoded).expect("decode Ordering");
        prop_assert_eq!(ordering, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 11: BTreeMap<u32, u32> roundtrip ────────────────────────────────────

#[test]
fn prop_btreemap_u32_u32_roundtrip() {
    proptest!(|(map in proptest::collection::btree_map(any::<u32>(), any::<u32>(), 0usize..=30))| {
        let encoded = encode_to_vec(&map).expect("encode BTreeMap<u32,u32>");
        let (decoded, bytes_read): (BTreeMap<u32, u32>, usize) =
            decode_from_slice(&encoded).expect("decode BTreeMap<u32,u32>");
        prop_assert_eq!(&map, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 12: BTreeSet<u32> roundtrip ─────────────────────────────────────────

#[test]
fn prop_btreeset_u32_roundtrip() {
    proptest!(|(set in proptest::collection::btree_set(any::<u32>(), 0usize..=30))| {
        let encoded = encode_to_vec(&set).expect("encode BTreeSet<u32>");
        let (decoded, bytes_read): (BTreeSet<u32>, usize) =
            decode_from_slice(&encoded).expect("decode BTreeSet<u32>");
        prop_assert_eq!(&set, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 13: Wrapping<u32> roundtrip ─────────────────────────────────────────

#[test]
fn prop_wrapping_u32_roundtrip() {
    proptest!(|(v: u32)| {
        let w = Wrapping(v);
        let encoded = encode_to_vec(&w).expect("encode Wrapping<u32>");
        let (decoded, bytes_read): (Wrapping<u32>, usize) =
            decode_from_slice(&encoded).expect("decode Wrapping<u32>");
        prop_assert_eq!(w, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 14: Saturating<u32> roundtrip ───────────────────────────────────────

#[test]
fn prop_saturating_u32_roundtrip() {
    proptest!(|(v: u32)| {
        let s = Saturating(v);
        let encoded = encode_to_vec(&s).expect("encode Saturating<u32>");
        let (decoded, bytes_read): (Saturating<u32>, usize) =
            decode_from_slice(&encoded).expect("decode Saturating<u32>");
        prop_assert_eq!(s, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 15: Option<NonZeroU32> roundtrip ────────────────────────────────────

#[test]
fn prop_option_nonzero_u32_roundtrip() {
    proptest!(|(v in proptest::option::of(1u32..=u32::MAX))| {
        let opt: Option<NonZeroU32> = v.map(|n| NonZeroU32::new(n).expect("nonzero for option"));
        let encoded = encode_to_vec(&opt).expect("encode Option<NonZeroU32>");
        let (decoded, bytes_read): (Option<NonZeroU32>, usize) =
            decode_from_slice(&encoded).expect("decode Option<NonZeroU32>");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 16: Vec<char> roundtrip (max 50) ────────────────────────────────────

#[test]
fn prop_vec_char_roundtrip() {
    proptest!(|(chars in proptest::collection::vec(any::<char>(), 0usize..=50))| {
        let encoded = encode_to_vec(&chars).expect("encode Vec<char>");
        let (decoded, bytes_read): (Vec<char>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<char>");
        prop_assert_eq!(&chars, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 17: (u8, u16, u32, u64) 4-tuple roundtrip ───────────────────────────

#[test]
fn prop_4tuple_u8_u16_u32_u64_roundtrip() {
    proptest!(|(a: u8, b: u16, c: u32, d: u64)| {
        let tup = (a, b, c, d);
        let encoded = encode_to_vec(&tup).expect("encode (u8,u16,u32,u64)");
        let (decoded, bytes_read): ((u8, u16, u32, u64), usize) =
            decode_from_slice(&encoded).expect("decode (u8,u16,u32,u64)");
        prop_assert_eq!(tup, decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 18: Box<u32> encode size == raw u32 encode size ─────────────────────

#[test]
fn prop_box_u32_size_equals_raw_u32_size() {
    proptest!(|(v: u32)| {
        let boxed = Box::new(v);
        let enc_boxed = encode_to_vec(&boxed).expect("encode Box<u32>");
        let enc_raw = encode_to_vec(&v).expect("encode u32");
        prop_assert_eq!(
            enc_boxed.len(),
            enc_raw.len(),
            "Box<u32> encode size must equal raw u32 encode size"
        );
        prop_assert_eq!(enc_boxed, enc_raw, "Box<u32> bytes must equal raw u32 bytes");
    });
}

// ── Test 19: Arc<String> roundtrip ───────────────────────────────────────────

#[test]
fn prop_arc_string_roundtrip() {
    proptest!(|(s: String)| {
        let arc = Arc::new(s.clone());
        let encoded = encode_to_vec(&arc).expect("encode Arc<String>");
        let (decoded, bytes_read): (Arc<String>, usize) =
            decode_from_slice(&encoded).expect("decode Arc<String>");
        prop_assert_eq!(arc.as_ref(), decoded.as_ref());
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 20: Rc<Vec<u8>> roundtrip ───────────────────────────────────────────

#[test]
fn prop_rc_vec_u8_roundtrip() {
    proptest!(|(bytes in proptest::collection::vec(any::<u8>(), 0usize..=64))| {
        let rc = Rc::new(bytes.clone());
        let encoded = encode_to_vec(&rc).expect("encode Rc<Vec<u8>>");
        let (decoded, bytes_read): (Rc<Vec<u8>>, usize) =
            decode_from_slice(&encoded).expect("decode Rc<Vec<u8>>");
        prop_assert_eq!(rc.as_ref(), decoded.as_ref());
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 21: HashMap<u32, u32> roundtrip (max 30 entries) ────────────────────

#[test]
fn prop_hashmap_u32_u32_roundtrip() {
    proptest!(|(map in proptest::collection::hash_map(any::<u32>(), any::<u32>(), 0usize..=30))| {
        let encoded = encode_to_vec(&map).expect("encode HashMap<u32,u32>");
        let (decoded, bytes_read): (HashMap<u32, u32>, usize) =
            decode_from_slice(&encoded).expect("decode HashMap<u32,u32>");
        prop_assert_eq!(&map, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}

// ── Test 22: LinkedList<u32> roundtrip (max 20) ───────────────────────────────

#[test]
fn prop_linked_list_u32_roundtrip() {
    proptest!(|(items in proptest::collection::vec(any::<u32>(), 0usize..=20))| {
        let list: LinkedList<u32> = items.iter().copied().collect();
        let encoded = encode_to_vec(&list).expect("encode LinkedList<u32>");
        let (decoded, bytes_read): (LinkedList<u32>, usize) =
            decode_from_slice(&encoded).expect("decode LinkedList<u32>");
        prop_assert_eq!(&list, &decoded);
        prop_assert_eq!(bytes_read, encoded.len());
    });
}
