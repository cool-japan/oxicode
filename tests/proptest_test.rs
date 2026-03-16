//! Property-based roundtrip tests using proptest.
//!
//! These tests verify that for any valid value, encoding then decoding
//! produces an identical value. They catch edge cases that hand-written
//! tests might miss (integer boundaries, unicode edge cases, etc.).

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
use core::cmp::Ordering;
use core::ops::ControlFlow;
use std::collections::LinkedList;

use oxicode::{decode_from_slice, decode_iter_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

// Helper: encode then decode, assert roundtrip
fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(value: &T) {
    let encoded = encode_to_vec(value).expect("encode failed");
    let (decoded, bytes_read): (T, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(value, &decoded, "roundtrip failed");
    assert_eq!(bytes_read, encoded.len(), "bytes_read mismatch");
}

proptest! {
    #[test]
    fn prop_roundtrip_u8(v: u8) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_u16(v: u16) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_u32(v: u32) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_u64(v: u64) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_u128(v: u128) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i8(v: i8) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i16(v: i16) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i32(v: i32) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i64(v: i64) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i128(v: i128) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_f32(v: f32) {
        // Skip NaN because NaN != NaN
        prop_assume!(!v.is_nan());
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_f64(v: f64) {
        prop_assume!(!v.is_nan());
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_bool(v: bool) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_char(v: char) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_string(v: String) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_vec_u8(v: Vec<u8>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_vec_u64(v: Vec<u64>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_option_u32(v: Option<u32>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_option_string(v: Option<String>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_tuple_u32_string(a: u32, b: String) {
        roundtrip(&(a, b));
    }

    #[test]
    fn prop_roundtrip_tuple_i64_f64_bool(a: i64, b: f64, c: bool) {
        prop_assume!(!b.is_nan());
        roundtrip(&(a, b, c));
    }

    #[test]
    fn prop_roundtrip_nested_vec(v: Vec<Vec<u32>>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_vec_string(v: Vec<String>) {
        roundtrip(&v);
    }

    // Varint edge cases: test values around encoding boundaries
    // 0-250: single byte; 251=u16; 252=u32; 253=u64; 254=u128
    #[test]
    fn prop_roundtrip_u64_varint_boundary(
        v in prop_oneof![
            0u64..=250,        // single byte range
            251u64..=65535,    // u16 range (tag 251)
            65536u64..=4294967295,  // u32 range (tag 252)
            any::<u64>(),      // full range including u64 (tag 253)
        ]
    ) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i64_zigzag_boundary(
        v in prop_oneof![
            -125i64..=125,       // zigzag single byte
            126i64..=32767,      // zigzag u16
            any::<i64>(),        // full range
        ]
    ) {
        roundtrip(&v);
    }
}

// Derived struct roundtrip
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Record {
    id: u64,
    name: String,
    tags: Vec<String>,
    score: Option<f64>,
}

proptest! {
    #[test]
    fn prop_roundtrip_point(x: f32, y: f32) {
        prop_assume!(!x.is_nan() && !y.is_nan());
        roundtrip(&Point { x, y });
    }

    #[test]
    fn prop_roundtrip_record(
        id: u64,
        name: String,
        tags: Vec<String>,
        score in prop::option::of(prop::num::f64::NORMAL),
    ) {
        roundtrip(&Record { id, name, tags, score });
    }

    // Verify encoded_size matches actual encoded length
    #[test]
    fn prop_encoded_size_matches_encode_to_vec_u64(v: u64) {
        let size = oxicode::encoded_size(&v).expect("encoded_size failed");
        let bytes = encode_to_vec(&v).expect("encode_to_vec failed");
        prop_assert_eq!(size, bytes.len());
    }

    #[test]
    fn prop_encoded_size_matches_encode_to_vec_string(v: String) {
        let size = oxicode::encoded_size(&v).expect("encoded_size failed");
        let bytes = encode_to_vec(&v).expect("encode_to_vec failed");
        prop_assert_eq!(size, bytes.len());
    }

    #[test]
    fn prop_encoded_size_matches_encode_to_vec_vec_u32(v: Vec<u32>) {
        let size = oxicode::encoded_size(&v).expect("encoded_size failed");
        let bytes = encode_to_vec(&v).expect("encode_to_vec failed");
        prop_assert_eq!(size, bytes.len());
    }

    // Verify that decoding extra bytes doesn't corrupt things
    #[test]
    fn prop_extra_bytes_ignored(v: u64, extra: Vec<u8>) {
        let mut encoded = encode_to_vec(&v).expect("encode failed");
        let orig_len = encoded.len();
        encoded.extend_from_slice(&extra);
        let (decoded, bytes_read): (u64, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(bytes_read, orig_len);
    }
}

// ===== New types added in recent work =====

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct PropU64(u64);

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipProp {
    value: u32,
    #[oxicode(skip)]
    ignored: u64,
}

proptest! {
    // 1. core::cmp::Ordering roundtrip
    #[test]
    fn prop_roundtrip_ordering(val in prop::sample::select(vec![
        Ordering::Less, Ordering::Equal, Ordering::Greater
    ])) {
        let enc = encode_to_vec(&val).expect("encode");
        let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(val, dec);
    }

    // 2. core::ops::ControlFlow<String, u32> roundtrip
    #[test]
    fn prop_roundtrip_control_flow(is_continue: bool, u: u32, s in ".*") {
        let cf: ControlFlow<String, u32> = if is_continue {
            ControlFlow::Continue(u)
        } else {
            ControlFlow::Break(s)
        };
        let enc = encode_to_vec(&cf).expect("encode");
        let (dec, _): (ControlFlow<String, u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(cf, dec);
    }

    // 3. Box<str> roundtrip
    #[test]
    fn prop_roundtrip_box_str(s in ".*") {
        let original: Box<str> = s.into_boxed_str();
        let enc = encode_to_vec(&original).expect("encode");
        let (dec, _): (Box<str>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(&*original, &*dec);
    }

    // 4. Box<[u32]> roundtrip
    #[test]
    fn prop_roundtrip_box_slice_u32(v: Vec<u32>) {
        let original: Box<[u32]> = v.into_boxed_slice();
        let enc = encode_to_vec(&original).expect("encode");
        let (dec, _): (Box<[u32]>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(&*original, &*dec);
    }

    // 5. LinkedList<u32> roundtrip
    #[test]
    fn prop_roundtrip_linked_list_u32(v: Vec<u32>) {
        let original: LinkedList<u32> = v.into_iter().collect();
        let enc = encode_to_vec(&original).expect("encode");
        let (dec, _): (LinkedList<u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(original, dec);
    }

    // 6. #[oxicode(transparent)] newtype roundtrip
    #[test]
    fn prop_roundtrip_transparent(val: u64) {
        let wrapped = PropU64(val);
        let enc_wrapped = encode_to_vec(&wrapped).expect("encode wrapped");
        let enc_raw = encode_to_vec(&val).expect("encode raw");
        prop_assert_eq!(enc_wrapped.clone(), enc_raw, "transparent must match raw encoding");
        let (dec, _): (PropU64, _) = decode_from_slice(&enc_wrapped).expect("decode");
        prop_assert_eq!(dec.0, val);
    }

    // 7. #[oxicode(skip)] struct: encoding is identical regardless of skipped field value
    #[test]
    fn prop_skip_field_matches_without_ignored(value: u32) {
        let with_skip = SkipProp { value, ignored: u64::MAX };
        let without = SkipProp { value, ignored: 0 };
        let enc_skip = encode_to_vec(&with_skip).expect("encode with skip");
        let enc_no_skip = encode_to_vec(&without).expect("encode without");
        // Both should produce identical bytes (ignored field is never written)
        prop_assert_eq!(enc_skip, enc_no_skip, "skipped field must not affect encoding");
    }

    // 8. DecodeIter consistency: decode_iter_from_slice gives same items as decode_from_slice::<Vec<T>>
    #[test]
    fn prop_decode_iter_consistent_with_vec(items: Vec<u64>) {
        let enc = encode_to_vec(&items).expect("encode");
        let (vec_dec, _): (Vec<u64>, _) = decode_from_slice(&enc).expect("decode vec");
        let iter_dec: Vec<u64> = decode_iter_from_slice::<u64>(&enc)
            .expect("iter")
            .collect::<Result<Vec<_>, _>>()
            .expect("decode iter items");
        prop_assert_eq!(vec_dec, iter_dec);
    }
}

// ===== Additional roundtrip tests =====

// Range<u32>
proptest! {
    #[test]
    fn prop_roundtrip_range_u32(start in 0u32..1000u32, end in 0u32..1000u32) {
        let range = start..end;
        let enc = encode_to_vec(&range).expect("encode");
        let (dec, _): (core::ops::Range<u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(range, dec);
    }
}

// Bound<i32>
proptest! {
    #[test]
    fn prop_roundtrip_bound_i32(v in -1000i32..1000i32, kind in 0u8..3u8) {
        let bound: core::ops::Bound<i32> = match kind {
            0 => core::ops::Bound::Unbounded,
            1 => core::ops::Bound::Included(v),
            _ => core::ops::Bound::Excluded(v),
        };
        let enc = encode_to_vec(&bound).expect("encode");
        let (dec, _): (core::ops::Bound<i32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(bound, dec);
    }
}

// Duration
proptest! {
    #[test]
    fn prop_roundtrip_duration(secs in 0u64..1_000_000u64, nanos in 0u32..999_999_999u32) {
        let dur = std::time::Duration::new(secs, nanos);
        let enc = encode_to_vec(&dur).expect("encode");
        let (dec, _): (std::time::Duration, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(dur, dec);
    }
}

// Wrapping<u32>
proptest! {
    #[test]
    fn prop_roundtrip_wrapping_u32(v in u32::MIN..u32::MAX) {
        let w = core::num::Wrapping(v);
        let enc = encode_to_vec(&w).expect("encode");
        let (dec, _): (core::num::Wrapping<u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(w, dec);
    }
}

// Vec<(String, i32)> - test complex nested types
proptest! {
    #[test]
    fn prop_roundtrip_vec_tuple_string_i32(
        pairs in proptest::collection::vec(
            (proptest::string::string_regex("[a-z]{1,20}").unwrap(), -1000i32..1000i32),
            0..20
        )
    ) {
        let enc = encode_to_vec(&pairs).expect("encode");
        let (dec, _): (Vec<(String, i32)>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(pairs, dec);
    }
}

// encoded_size matches encode_to_vec length
proptest! {
    #[test]
    fn prop_encoded_size_matches_vec_len_u64(v in u64::MIN..u64::MAX) {
        let size = oxicode::encoded_size(&v).expect("size");
        let enc = encode_to_vec(&v).expect("encode");
        prop_assert_eq!(size, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_encoded_size_matches_vec_len_string(
        s in proptest::string::string_regex("[a-z]{0,100}").unwrap()
    ) {
        let size = oxicode::encoded_size(&s).expect("size");
        let enc = encode_to_vec(&s).expect("encode");
        prop_assert_eq!(size, enc.len());
    }
}

// --- Task 1: derive attribute property tests ---

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSkipDefault {
    name: String,
    #[oxicode(skip)]
    skipped: u32,
}

proptest! {
    #[test]
    fn prop_skip_field_default_roundtrip(
        name in proptest::string::string_regex("[a-z]{1,20}").unwrap(),
        value in 0u32..u32::MAX
    ) {
        let original = WithSkipDefault { name: name.clone(), skipped: value };
        let enc = encode_to_vec(&original).expect("encode");
        let (dec, _): (WithSkipDefault, _) = decode_from_slice(&enc).expect("decode");
        // name is preserved, skipped becomes 0 (Default)
        prop_assert_eq!(&dec.name, &name);
        prop_assert_eq!(dec.skipped, 0u32);
    }
}

proptest! {
    #[test]
    fn prop_encoded_size_vec_u32(data in proptest::collection::vec(0u32..u32::MAX, 0..100)) {
        let size = oxicode::encoded_size(&data).expect("size");
        let enc = encode_to_vec(&data).expect("encode");
        prop_assert_eq!(size, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_encode_decode_option_u32(v in proptest::option::of(0u32..u32::MAX)) {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

proptest! {
    #[test]
    fn prop_truncated_data_returns_error(
        data in proptest::collection::vec(0u32..1000u32, 1..20),
        truncate_at in 0usize..50usize
    ) {
        let enc = encode_to_vec(&data).expect("encode");
        if truncate_at < enc.len() {
            let truncated = &enc[..truncate_at];
            let result: Result<(Vec<u32>, _), _> = decode_from_slice(truncated);
            // Truncated data should return error, never panic
            prop_assert!(result.is_err());
        }
    }
}

// --- Task 2: HashMap/BTreeMap roundtrips ---

proptest! {
    #[test]
    fn prop_roundtrip_btreemap(
        pairs in proptest::collection::btree_map(
            proptest::string::string_regex("[a-z]{1,10}").unwrap(),
            0i32..1000i32,
            0..20
        )
    ) {
        let enc = encode_to_vec(&pairs).expect("encode");
        let (dec, _): (std::collections::BTreeMap<String, i32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(pairs, dec);
    }
}

// --- Task 2: HashMap<String, u64> roundtrip ---

proptest! {
    #[test]
    fn prop_roundtrip_hashmap_string_u64(
        pairs in proptest::collection::hash_map(
            proptest::string::string_regex("[a-z]{1,15}").unwrap(),
            0u64..u64::MAX,
            0..30
        )
    ) {
        let enc = encode_to_vec(&pairs).expect("encode");
        let (dec, _): (std::collections::HashMap<String, u64>, _) =
            decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(pairs, dec);
    }
}

// --- Task 3: Cow<str> roundtrip ---

proptest! {
    #[test]
    fn prop_roundtrip_cow_str(
        s in proptest::string::string_regex("[a-z0-9 ]{0,50}").unwrap()
    ) {
        use std::borrow::Cow;
        let cow: Cow<str> = Cow::Owned(s.clone());
        let enc = encode_to_vec(&cow).expect("encode");
        let (dec, _): (Cow<str>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(s, dec.as_ref());
    }
}

// Ipv4Addr
proptest! {
    #[test]
    fn prop_roundtrip_ipv4addr(
        a in 0u8..=255u8, b in 0u8..=255u8, c in 0u8..=255u8, d in 0u8..=255u8
    ) {
        use std::net::Ipv4Addr;
        let addr = Ipv4Addr::new(a, b, c, d);
        let enc = encode_to_vec(&addr).expect("encode");
        let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// Ipv6Addr
proptest! {
    #[test]
    fn prop_roundtrip_ipv6addr(
        a in 0u16..=65535u16, b in 0u16..=65535u16,
        c in 0u16..=65535u16, d in 0u16..=65535u16,
        e in 0u16..=65535u16, f in 0u16..=65535u16,
        g in 0u16..=65535u16, h in 0u16..=65535u16
    ) {
        use std::net::Ipv6Addr;
        let addr = Ipv6Addr::new(a, b, c, d, e, f, g, h);
        let enc = encode_to_vec(&addr).expect("encode");
        let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// SocketAddrV4
proptest! {
    #[test]
    fn prop_roundtrip_socketaddrv4(
        a in 0u8..=255u8, b in 0u8..=255u8, c in 0u8..=255u8, d in 0u8..=255u8,
        port in 0u16..=65535u16
    ) {
        use std::net::{Ipv4Addr, SocketAddrV4};
        let addr = SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port);
        let enc = encode_to_vec(&addr).expect("encode");
        let (dec, _): (SocketAddrV4, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// SocketAddrV6
proptest! {
    #[test]
    fn prop_roundtrip_socketaddrv6(
        a in 0u16..=65535u16, b in 0u16..=65535u16,
        c in 0u16..=65535u16, d in 0u16..=65535u16,
        e in 0u16..=65535u16, f in 0u16..=65535u16,
        g in 0u16..=65535u16, h in 0u16..=65535u16,
        port in 0u16..=65535u16
    ) {
        use std::net::{Ipv6Addr, SocketAddrV6};
        let addr = SocketAddrV6::new(Ipv6Addr::new(a, b, c, d, e, f, g, h), port, 0, 0);
        let enc = encode_to_vec(&addr).expect("encode");
        let (dec, _): (SocketAddrV6, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// NonZeroU32
proptest! {
    #[test]
    fn prop_roundtrip_nonzero_u32(v in 1u32..=u32::MAX) {
        use core::num::NonZeroU32;
        let nz = NonZeroU32::new(v).expect("nonzero");
        let enc = encode_to_vec(&nz).expect("encode");
        let (dec, _): (NonZeroU32, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(nz, dec);
    }
}

// Reverse<i32>
proptest! {
    #[test]
    fn prop_roundtrip_reverse_i32(v in i32::MIN..=i32::MAX) {
        use core::cmp::Reverse;
        let r = Reverse(v);
        let enc = encode_to_vec(&r).expect("encode");
        let (dec, _): (Reverse<i32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// Verify encode_to_vec and encode_to_fixed_array produce the same bytes for u32
proptest! {
    #[test]
    fn prop_fixed_array_matches_vec_u32(v in 0u32..u32::MAX) {
        let vec_bytes = encode_to_vec(&v).expect("vec encode");
        if vec_bytes.len() <= 10 {
            let (arr, n): ([u8; 10], usize) =
                oxicode::encode_to_fixed_array(&v).expect("fixed encode");
            prop_assert_eq!(vec_bytes.len(), n, "fixed array written length must match vec length");
            prop_assert_eq!(&vec_bytes[..], &arr[..n], "fixed array bytes must match vec bytes");
        }
    }
}

// Verify Option::None always encodes to exactly the same bytes, independent of
// the concrete u32 value that would have been stored in Some(v).
proptest! {
    #[test]
    fn prop_option_none_always_same_bytes(_x in 0u32..u32::MAX) {
        let none: Option<u32> = None;
        let enc1 = encode_to_vec(&none).expect("encode");
        let enc2 = encode_to_vec(&none).expect("encode2");
        prop_assert_eq!(enc1, enc2);
    }
}

// =============================================================================
// Task 1: Bincode byte-for-byte compatibility tests
//
// oxicode::config::standard() uses little-endian + variable-int encoding,
// which matches bincode v2's standard() config exactly.
// =============================================================================

proptest! {
    #[test]
    fn prop_bincode_compat_u32(v in 0u32..u32::MAX) {
        let oxicode_bytes = oxicode::encode_to_vec_with_config(&v, oxicode::config::standard())
            .expect("oxicode encode u32");
        let bincode_bytes = bincode::encode_to_vec(v, bincode::config::standard())
            .expect("bincode encode u32");
        prop_assert_eq!(oxicode_bytes, bincode_bytes, "Mismatch for u32 = {}", v);
    }
}

proptest! {
    #[test]
    fn prop_bincode_compat_i32(v in i32::MIN..i32::MAX) {
        let oxicode_bytes = oxicode::encode_to_vec_with_config(&v, oxicode::config::standard())
            .expect("oxicode encode i32");
        let bincode_bytes = bincode::encode_to_vec(v, bincode::config::standard())
            .expect("bincode encode i32");
        prop_assert_eq!(oxicode_bytes, bincode_bytes, "Mismatch for i32 = {}", v);
    }
}

proptest! {
    #[test]
    fn prop_bincode_compat_string(
        s in proptest::string::string_regex("[a-z0-9 ]{0,100}").unwrap()
    ) {
        let oxicode_bytes = oxicode::encode_to_vec_with_config(&s, oxicode::config::standard())
            .expect("oxicode encode string");
        let bincode_bytes = bincode::encode_to_vec(&s, bincode::config::standard())
            .expect("bincode encode string");
        prop_assert_eq!(oxicode_bytes, bincode_bytes, "Mismatch for string = {:?}", s);
    }
}

proptest! {
    #[test]
    fn prop_bincode_compat_bool(b: bool) {
        let oxicode_bytes = oxicode::encode_to_vec_with_config(&b, oxicode::config::standard())
            .expect("oxicode encode bool");
        let bincode_bytes = bincode::encode_to_vec(b, bincode::config::standard())
            .expect("bincode encode bool");
        prop_assert_eq!(oxicode_bytes, bincode_bytes);
    }
}

// =============================================================================
// Task 2: Varint size invariants
// =============================================================================

// All u64 values in range 0-250 should encode as exactly 1 byte
proptest! {
    #[test]
    fn prop_varint_small_values_1_byte(v in 0u64..=250) {
        let enc = encode_to_vec(&v).expect("encode u64 small");
        prop_assert_eq!(enc.len(), 1, "Value {} should be 1 byte", v);
    }
}

// Encoded size always matches actual encoded length for i32
proptest! {
    #[test]
    fn prop_encoded_size_matches_i32(v in i32::MIN..i32::MAX) {
        let size = oxicode::encoded_size(&v).expect("encoded_size i32");
        let enc = encode_to_vec(&v).expect("encode i32");
        prop_assert_eq!(size, enc.len());
    }
}

// Encoded size always matches actual encoded length for bool
proptest! {
    #[test]
    fn prop_encoded_size_matches_bool(b: bool) {
        let size = oxicode::encoded_size(&b).expect("encoded_size bool");
        let enc = encode_to_vec(&b).expect("encode bool");
        prop_assert_eq!(size, enc.len());
    }
}

// =============================================================================
// Task 3: encode_to_fixed_array vs encode_to_vec consistency for u8
// =============================================================================

// u8 always encodes as exactly 1 byte; verify fixed-array matches vec
proptest! {
    #[test]
    fn prop_fixed_array_matches_vec_u8(v in 0u8..=u8::MAX) {
        let vec_bytes = encode_to_vec(&v).expect("vec encode u8");
        // u8 always fits in 1 byte — encode into a [u8; 1] fixed array
        let (arr, n): ([u8; 1], usize) =
            oxicode::encode_to_fixed_array(&v).expect("fixed encode u8");
        prop_assert_eq!(n, 1, "u8 must write exactly 1 byte");
        prop_assert_eq!(vec_bytes.len(), 1, "vec must also be 1 byte for u8");
        prop_assert_eq!(vec_bytes[0], arr[0]);
    }
}

// =============================================================================
// Task 3: proptest coverage for encode_to_writer / decode_from_reader
// =============================================================================

proptest! {
    // encode_to_writer into Vec<u8> (which implements Write) produces same bytes as encode_to_vec
    #[test]
    fn prop_encode_to_writer_matches_encode_to_vec_u64(v: u64) {
        let mut buf = Vec::new();
        let n = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let expected = encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(n, expected.len(), "bytes written must match vec length");
        prop_assert_eq!(buf, expected, "encode_to_writer must produce same bytes as encode_to_vec");
    }

    #[test]
    fn prop_encode_to_writer_matches_encode_to_vec_string(
        s in proptest::string::string_regex("[a-z0-9]{0,50}").unwrap()
    ) {
        let mut buf = Vec::new();
        let n = oxicode::encode_to_writer(&s, &mut buf).expect("encode_to_writer");
        let expected = encode_to_vec(&s).expect("encode_to_vec");
        prop_assert_eq!(n, expected.len());
        prop_assert_eq!(buf, expected);
    }

    #[test]
    fn prop_encode_to_writer_matches_encode_to_vec_vec_u32(v: Vec<u32>) {
        let mut buf = Vec::new();
        let n = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let expected = encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(n, expected.len());
        prop_assert_eq!(buf, expected);
    }

    // decode_from_reader: encode with encode_to_vec, decode via Cursor<&[u8]>
    #[test]
    fn prop_decode_from_reader_roundtrip_u64(v: u64) {
        use std::io::Cursor;
        let bytes = encode_to_vec(&v).expect("encode");
        let cursor = Cursor::new(&bytes);
        let (dec, n): (u64, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n, bytes.len());
    }

    #[test]
    fn prop_decode_from_reader_roundtrip_string(
        s in proptest::string::string_regex("[a-z0-9 ]{0,50}").unwrap()
    ) {
        use std::io::Cursor;
        let bytes = encode_to_vec(&s).expect("encode");
        let cursor = Cursor::new(bytes.clone());
        let (dec, n): (String, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, s);
        prop_assert_eq!(n, bytes.len());
    }

    #[test]
    fn prop_decode_from_reader_roundtrip_vec_u8(v: Vec<u8>) {
        use std::io::Cursor;
        let bytes = encode_to_vec(&v).expect("encode");
        let cursor = Cursor::new(bytes.clone());
        let (dec, n): (Vec<u8>, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n, bytes.len());
    }

    // encode_to_writer + decode_from_reader are inverses of each other end-to-end
    #[test]
    fn prop_writer_reader_roundtrip_i32(v: i32) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        let n_written = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let cursor = Cursor::new(&buf);
        let (dec, n_read): (i32, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n_written, n_read);
    }

    #[test]
    fn prop_writer_reader_roundtrip_option_string(v: Option<String>) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        let n_written = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let cursor = Cursor::new(&buf);
        let (dec, n_read): (Option<String>, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n_written, n_read);
    }
}

proptest! {
    #[test]
    fn prop_hex_roundtrip_u32(v: u32) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (u32, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }

    #[test]
    fn prop_hex_roundtrip_string(s: String) {
        let hex = oxicode::encode_to_hex(&s).expect("encode_to_hex");
        let (dec, _): (String, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(s, dec);
    }

    #[test]
    fn prop_hex_roundtrip_vec_u8(v: Vec<u8>) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (Vec<u8>, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }
}

proptest! {
    // Range types
    #[test]
    fn prop_range_u32_roundtrip(start: u32, end: u32) {
        let r = start..end;
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (core::ops::Range<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }

    #[test]
    fn prop_range_from_u32_roundtrip(start: u32) {
        let r: core::ops::RangeFrom<u32> = start..;
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (core::ops::RangeFrom<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r.start, dec.start);
    }

    #[test]
    fn prop_range_to_u32_roundtrip(end: u32) {
        let r: core::ops::RangeTo<u32> = ..end;
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (core::ops::RangeTo<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r.end, dec.end);
    }

    // encoded_size matches actual len
    #[test]
    fn prop_encoded_size_matches_actual(v: Vec<u32>) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&v).expect("encode").len();
        prop_assert_eq!(size, actual);
    }

    #[test]
    fn prop_encoded_size_matches_actual_string(s: String) {
        let size = oxicode::encoded_size(&s).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&s).expect("encode").len();
        prop_assert_eq!(size, actual);
    }

    // encode_copy == encode_to_vec for Copy types
    #[test]
    fn prop_encode_copy_matches_ref_u64(v: u64) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }

    #[test]
    fn prop_encode_copy_matches_ref_i64(v: i64) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }

    // hex roundtrip for integers
    #[test]
    fn prop_hex_roundtrip_i64(v: i64) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (i64, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }

    #[test]
    fn prop_hex_roundtrip_bool(v: bool) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (bool, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }
}

proptest! {
    // Tuple types
    #[test]
    fn prop_tuple_2_roundtrip(a: u32, b: u64) {
        let t = (a, b);
        let enc = oxicode::encode_to_vec(&t).expect("encode");
        let (dec, _): ((u32, u64), _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(t, dec);
    }

    #[test]
    fn prop_tuple_3_roundtrip(a: u8, b: i32, c: bool) {
        let t = (a, b, c);
        let enc = oxicode::encode_to_vec(&t).expect("encode");
        let (dec, _): ((u8, i32, bool), _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(t, dec);
    }

    // Wrapping types
    #[test]
    fn prop_wrapping_i32_roundtrip(v: i32) {
        use std::num::Wrapping;
        let w = Wrapping(v);
        let enc = oxicode::encode_to_vec(&w).expect("encode");
        let (dec, _): (Wrapping<i32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(w, dec);
    }

    // Option wrapping
    #[test]
    fn prop_option_u64_roundtrip(v: Option<u64>) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<u64>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    // char type
    #[test]
    fn prop_char_roundtrip(c: char) {
        let enc = oxicode::encode_to_vec(&c).expect("encode");
        let (dec, _): (char, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(c, dec);
    }

    // bool
    #[test]
    fn prop_bool_roundtrip(b: bool) {
        let enc = oxicode::encode_to_vec(&b).expect("encode");
        let (dec, _): (bool, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(b, dec);
    }

    // f32 (skip NaN since NaN != NaN)
    #[test]
    fn prop_f32_non_nan_roundtrip(v in proptest::num::f32::NORMAL | proptest::num::f32::ZERO | proptest::num::f32::INFINITE | proptest::num::f32::NEGATIVE | proptest::num::f32::SUBNORMAL) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (f32, _) = oxicode::decode_from_slice(&enc).expect("decode");
        // Use bits comparison to handle -0.0 == +0.0 edge case
        prop_assert_eq!(v.to_bits(), dec.to_bits());
    }

    // encoded_size for bool, char, tuples
    #[test]
    fn prop_encoded_size_matches_bool_new(v: bool) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&v).expect("encode").len();
        prop_assert_eq!(size, actual);
    }

    #[test]
    fn prop_encoded_size_matches_char(c: char) {
        let size = oxicode::encoded_size(&c).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&c).expect("encode").len();
        prop_assert_eq!(size, actual);
    }

    // hex roundtrip for i32
    #[test]
    fn prop_hex_roundtrip_i32(v: i32) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (i32, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }
}

proptest! {
    // i128/u128 roundtrip
    #[test]
    fn prop_i128_roundtrip(v: i128) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (i128, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    #[test]
    fn prop_u128_roundtrip(v: u128) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (u128, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    // isize/usize roundtrip
    #[test]
    fn prop_usize_roundtrip(v: usize) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (usize, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    #[test]
    fn prop_isize_roundtrip(v: isize) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (isize, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    // BTreeMap
    #[test]
    fn prop_btreemap_string_u32_roundtrip(map: std::collections::BTreeMap<String, u32>) {
        let enc = oxicode::encode_to_vec(&map).expect("encode");
        let (dec, _): (std::collections::BTreeMap<String, u32>, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(map, dec);
    }

    // Vec<Vec<u8>> nested
    #[test]
    fn prop_vec_vec_u8_roundtrip(v: Vec<Vec<u8>>) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Vec<Vec<u8>>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    // encoded_size for u128
    #[test]
    fn prop_encoded_size_matches_u128(v: u128) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&v).expect("encode").len();
        prop_assert_eq!(size, actual);
    }
}

// =============================================================================
// 15 new proptest tests (appended)
// =============================================================================

// 1. prop_encoded_size_matches_vec_len: Vec<u8> up to 1000 elements
proptest! {
    #[test]
    fn prop_encoded_size_matches_vec_len(
        v in proptest::collection::vec(any::<u8>(), 0..=1000)
    ) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len());
    }
}

// 2. prop_encoded_size_u32_matches
proptest! {
    #[test]
    fn prop_encoded_size_u32_matches(v: u32) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len());
    }
}

// 3. prop_encoded_size_string_matches
proptest! {
    #[test]
    fn prop_encoded_size_string_matches(v: String) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len());
    }
}

// 4. prop_encode_copy_matches_encode_to_vec_u64
proptest! {
    #[test]
    fn prop_encode_copy_matches_encode_to_vec_u64(v: u64) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }
}

// 5. prop_encode_copy_matches_encode_to_vec_i32
proptest! {
    #[test]
    fn prop_encode_copy_matches_encode_to_vec_i32(v: i32) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }
}

// 6. prop_hex_roundtrip_u64
proptest! {
    #[test]
    fn prop_hex_roundtrip_u64(v: u64) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (u64, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }
}

// 7. prop_hex_roundtrip_string_ascii: String limited to ASCII chars, max 100 chars
proptest! {
    #[test]
    fn prop_hex_roundtrip_string_ascii(
        s in proptest::string::string_regex("[\\x20-\\x7e]{0,100}").unwrap()
    ) {
        let hex = oxicode::encode_to_hex(&s).expect("encode_to_hex");
        let (dec, _): (String, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(s, dec);
    }
}

// 8. prop_encode_copy_i64
proptest! {
    #[test]
    fn prop_encode_copy_i64(v: i64) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }
}

// 9. prop_encoded_size_btreemap: BTreeMap<u32, u32>
proptest! {
    #[test]
    fn prop_encoded_size_btreemap(
        map in proptest::collection::btree_map(any::<u32>(), any::<u32>(), 0..50)
    ) {
        let size = oxicode::encoded_size(&map).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&map).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len());
    }
}

// 10. prop_writer_reader_u32_roundtrip
proptest! {
    #[test]
    fn prop_writer_reader_u32_roundtrip(v: u32) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        let n_written = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let cursor = Cursor::new(&buf);
        let (dec, n_read): (u32, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n_written, n_read);
    }
}

// 11. prop_writer_reader_string_roundtrip
proptest! {
    #[test]
    fn prop_writer_reader_string_roundtrip(
        s in proptest::string::string_regex("[a-z0-9 ]{0,80}").unwrap()
    ) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        let n_written = oxicode::encode_to_writer(&s, &mut buf).expect("encode_to_writer");
        let cursor = Cursor::new(&buf);
        let (dec, n_read): (String, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, s);
        prop_assert_eq!(n_written, n_read);
    }
}

// 12. prop_btreeset_roundtrip: BTreeSet<u32> limited to 200 elements
proptest! {
    #[test]
    fn prop_btreeset_roundtrip(
        set in proptest::collection::btree_set(any::<u32>(), 0..=200)
    ) {
        let enc = oxicode::encode_to_vec(&set).expect("encode");
        let (dec, _): (std::collections::BTreeSet<u32>, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(set, dec);
    }
}

// 13. prop_hashset_roundtrip_len_preserved: HashSet<u8> roundtrip preserves length
proptest! {
    #[test]
    fn prop_hashset_roundtrip_len_preserved(
        set in proptest::collection::hash_set(any::<u8>(), 0..=256)
    ) {
        let original_len = set.len();
        let enc = oxicode::encode_to_vec(&set).expect("encode");
        let (dec, _): (std::collections::HashSet<u8>, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(original_len, dec.len());
        prop_assert_eq!(set, dec);
    }
}

// 14. prop_duration_roundtrip: Duration::from_secs + from_nanos
proptest! {
    #[test]
    fn prop_duration_roundtrip(
        s in 0u64..1_000_000u64,
        n in 0u32..999_999_999u32
    ) {
        let dur = std::time::Duration::from_secs(s) + std::time::Duration::from_nanos(n as u64);
        let enc = oxicode::encode_to_vec(&dur).expect("encode");
        let (dec, _): (std::time::Duration, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(dur, dec);
    }
}

// 15. prop_option_roundtrip_bool: Option<bool>
proptest! {
    #[test]
    fn prop_option_roundtrip_bool(v: Option<bool>) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<bool>, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// =============================================================================
// 12 additional proptest tests (appended)
// =============================================================================

use std::collections::*;

// 1. prop_vecdeque_roundtrip - VecDeque<u32> up to 500 elements
proptest! {
    #[test]
    fn prop_vecdeque_roundtrip(
        v in proptest::collection::vec(any::<u32>(), 0..=500)
    ) {
        let deque: VecDeque<u32> = v.into_iter().collect();
        let enc = oxicode::encode_to_vec(&deque).expect("encode");
        let (dec, _): (VecDeque<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(deque, dec);
    }
}

// 2. prop_linkedlist_roundtrip - LinkedList<u32> up to 200 elements
proptest! {
    #[test]
    fn prop_linkedlist_roundtrip(
        v in proptest::collection::vec(any::<u32>(), 0..=200)
    ) {
        let list: LinkedList<u32> = v.into_iter().collect();
        let enc = oxicode::encode_to_vec(&list).expect("encode");
        let (dec, _): (LinkedList<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(list, dec);
    }
}

// 3. prop_hashmap_roundtrip - HashMap<u32, u32> up to 100 entries
proptest! {
    #[test]
    fn prop_hashmap_roundtrip(
        map in proptest::collection::hash_map(any::<u32>(), any::<u32>(), 0..=100)
    ) {
        let enc = oxicode::encode_to_vec(&map).expect("encode");
        let (dec, _): (HashMap<u32, u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(map, dec);
    }
}

// 4. prop_btreemap_roundtrip - BTreeMap<String, u64> up to 100 entries, strings max 50 chars
proptest! {
    #[test]
    fn prop_btreemap_roundtrip(
        map in proptest::collection::btree_map(
            proptest::string::string_regex("[a-zA-Z0-9]{1,50}").unwrap(),
            any::<u64>(),
            0..=100
        )
    ) {
        let enc = oxicode::encode_to_vec(&map).expect("encode");
        let (dec, _): (BTreeMap<String, u64>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(map, dec);
    }
}

// 5. prop_hashset_roundtrip - HashSet<u32> up to 200 elements
proptest! {
    #[test]
    fn prop_hashset_roundtrip(
        set in proptest::collection::hash_set(any::<u32>(), 0..=200)
    ) {
        let enc = oxicode::encode_to_vec(&set).expect("encode");
        let (dec, _): (HashSet<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(set, dec);
    }
}

// 6. prop_btreeset_i32_roundtrip - BTreeSet<i32> up to 200 elements
proptest! {
    #[test]
    fn prop_btreeset_i32_roundtrip(
        set in proptest::collection::btree_set(any::<i32>(), 0..=200)
    ) {
        let enc = oxicode::encode_to_vec(&set).expect("encode");
        let (dec, _): (BTreeSet<i32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(set, dec);
    }
}

// 7. prop_nested_vec_string_roundtrip - Vec<Vec<String>> outer max 10, inner max 5, string max 20
proptest! {
    #[test]
    fn prop_nested_vec_string_roundtrip(
        outer in proptest::collection::vec(
            proptest::collection::vec(
                proptest::string::string_regex("[a-z]{1,20}").unwrap(),
                0..=5
            ),
            0..=10
        )
    ) {
        let enc = oxicode::encode_to_vec(&outer).expect("encode");
        let (dec, _): (Vec<Vec<String>>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(outer, dec);
    }
}

// 8. prop_option_string_roundtrip - Option<String>
proptest! {
    #[test]
    fn prop_option_string_roundtrip(v: Option<String>) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 9. prop_option_vec_u8_roundtrip - Option<Vec<u8>> up to 1000 bytes
proptest! {
    #[test]
    fn prop_option_vec_u8_roundtrip(
        v in proptest::option::of(proptest::collection::vec(any::<u8>(), 0..=1000))
    ) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<Vec<u8>>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 10. prop_result_ok_u64_roundtrip - Result<u64, String> where Ok
proptest! {
    #[test]
    fn prop_result_ok_u64_roundtrip(val: u64) {
        let r: Result<u64, String> = Ok(val);
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (Result<u64, String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// 11. prop_result_err_string_roundtrip - Result<u32, String> where Err
proptest! {
    #[test]
    fn prop_result_err_string_roundtrip(
        msg in proptest::string::string_regex("[a-zA-Z0-9 ]{0,80}").unwrap()
    ) {
        let r: Result<u32, String> = Err(msg);
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (Result<u32, String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// 12. prop_large_btreemap_size - BTreeMap<u32, u32> with 500 entries, verify encoded_size matches encode_to_vec length
proptest! {
    #[test]
    fn prop_large_btreemap_size(
        map in proptest::collection::btree_map(any::<u32>(), any::<u32>(), 0..=500)
    ) {
        let size = oxicode::encoded_size(&map).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&map).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len(),
            "encoded_size must match encode_to_vec length for BTreeMap with {} entries",
            map.len()
        );
    }
}

// New batch: 15 additional proptest tests

// 1. prop_range_inclusive_u32_roundtrip - RangeInclusive<u32>
proptest! {
    #[test]
    fn prop_range_inclusive_u32_roundtrip(start: u32, end: u32) {
        let r = start..=end;
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (std::ops::RangeInclusive<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// 2. prop_bound_i32_exhaustive_roundtrip - std::ops::Bound<i32>
proptest! {
    #[test]
    fn prop_bound_i32_exhaustive_roundtrip(v: i32, kind in 0u8..3u8) {
        use std::ops::Bound;
        let b: Bound<i32> = match kind {
            0 => Bound::Included(v),
            1 => Bound::Excluded(v),
            _ => Bound::Unbounded,
        };
        let enc = oxicode::encode_to_vec(&b).expect("encode");
        let (dec, _): (Bound<i32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(b, dec);
    }
}

// 3. prop_cell_u32_roundtrip - std::cell::Cell<u32>
proptest! {
    #[test]
    fn prop_cell_u32_roundtrip(v: u32) {
        use std::cell::Cell;
        let c = Cell::new(v);
        let enc = oxicode::encode_to_vec(&c).expect("encode");
        let (dec, _): (Cell<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(c.get(), dec.get());
    }
}

// 4. prop_wrapping_i64_roundtrip - Wrapping<i64>
proptest! {
    #[test]
    fn prop_wrapping_i64_roundtrip(v: i64) {
        use std::num::Wrapping;
        let w = Wrapping(v);
        let enc = oxicode::encode_to_vec(&w).expect("encode");
        let (dec, _): (Wrapping<i64>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(w, dec);
    }
}

// 5. prop_nonzero_u32_new_roundtrip - NonZeroU32
proptest! {
    #[test]
    fn prop_nonzero_u32_new_roundtrip(v in 1u32..=u32::MAX) {
        use std::num::NonZeroU32;
        let nz = NonZeroU32::new(v).expect("nonzero");
        let enc = oxicode::encode_to_vec(&nz).expect("encode");
        let (dec, _): (NonZeroU32, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(nz, dec);
    }
}

// 6. prop_char_from_u32_roundtrip - char from valid unicode scalar values
proptest! {
    #[test]
    fn prop_char_from_u32_roundtrip(
        raw in (0u32..=0x10FFFFu32).prop_filter("not surrogate", |x| !(*x >= 0xD800 && *x <= 0xDFFF))
    ) {
        let c = char::from_u32(raw).expect("valid char");
        let enc = oxicode::encode_to_vec(&c).expect("encode");
        let (dec, _): (char, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(c, dec);
    }
}

// 7. prop_i128_new_roundtrip - i128
proptest! {
    #[test]
    fn prop_i128_new_roundtrip(v in i128::MIN..=i128::MAX) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (i128, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 8. prop_u128_new_roundtrip - u128
proptest! {
    #[test]
    fn prop_u128_new_roundtrip(v in u128::MIN..=u128::MAX) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (u128, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 9. prop_f32_finite_roundtrip - f32 finite values
proptest! {
    #[test]
    fn prop_f32_finite_roundtrip(
        v in proptest::num::f32::ANY.prop_filter("finite", |x| x.is_finite())
    ) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (f32, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v.to_bits(), dec.to_bits());
    }
}

// 10. prop_f64_finite_roundtrip - f64 finite values
proptest! {
    #[test]
    fn prop_f64_finite_roundtrip(
        v in proptest::num::f64::ANY.prop_filter("finite", |x| x.is_finite())
    ) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (f64, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v.to_bits(), dec.to_bits());
    }
}

// 11. prop_ipv4_from_octets_roundtrip - Ipv4Addr from 4 u8 values
proptest! {
    #[test]
    fn prop_ipv4_from_octets_roundtrip(a: u8, b: u8, c: u8, d: u8) {
        use std::net::Ipv4Addr;
        let addr = Ipv4Addr::new(a, b, c, d);
        let enc = oxicode::encode_to_vec(&addr).expect("encode");
        let (dec, _): (Ipv4Addr, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// 12. prop_ipv6_from_segments_roundtrip - Ipv6Addr from 8 u16 segments
proptest! {
    #[test]
    fn prop_ipv6_from_segments_roundtrip(
        a: u16, b: u16, c: u16, d: u16,
        e: u16, f: u16, g: u16, h: u16
    ) {
        use std::net::Ipv6Addr;
        let addr = Ipv6Addr::new(a, b, c, d, e, f, g, h);
        let enc = oxicode::encode_to_vec(&addr).expect("encode");
        let (dec, _): (Ipv6Addr, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// 13. prop_box_str_ascii_roundtrip - Box<str> from ASCII string
proptest! {
    #[test]
    fn prop_box_str_ascii_roundtrip(
        s in proptest::string::string_regex("[a-zA-Z0-9 _.-]{0,100}").unwrap()
    ) {
        let b: Box<str> = s.clone().into_boxed_str();
        let enc = oxicode::encode_to_vec(&b).expect("encode");
        let (dec, _): (Box<str>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(b, dec);
    }
}

// 14. prop_result_ok_err_u32_string_roundtrip - Result<u32, String>
proptest! {
    #[test]
    fn prop_result_ok_err_u32_string_roundtrip(v: u32, s: String, is_ok: bool) {
        let r: Result<u32, String> = if is_ok { Ok(v) } else { Err(s) };
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (Result<u32, String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// 15. prop_path_ascii_roundtrip - PathBuf from ASCII string (unix only)
proptest! {
    #[test]
    fn prop_path_ascii_roundtrip(
        s in proptest::string::string_regex("[a-zA-Z0-9_/.-]{1,50}").unwrap()
    ) {
        #[cfg(unix)]
        {
            use std::path::PathBuf;
            let p = PathBuf::from(&s);
            let enc = oxicode::encode_to_vec(&p).expect("encode");
            let (dec, _): (PathBuf, _) = oxicode::decode_from_slice(&enc).expect("decode");
            prop_assert_eq!(p, dec);
        }
        #[cfg(not(unix))]
        {
            let _ = s;
        }
    }
}
