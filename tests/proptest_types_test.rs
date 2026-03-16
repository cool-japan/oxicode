//! Property-based tests covering additional types not covered by proptest_test.rs
//! or proptest_advanced_test.rs.
//!
//! Tests cover: HashMap, BTreeSet, tuples-of-tuples, nested vecs, Option<Vec>,
//! Result variants, Range, RangeInclusive, Cow<str>, zigzag symmetry, varint
//! single-byte invariant, length-prefix invariants, and derived structs.

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
use std::borrow::Cow;
use std::ops::{Range, RangeInclusive};

use oxicode::{decode_from_slice, encode_to_vec, encoded_size, Decode, Encode};
use proptest::prelude::*;

// ===== Derive struct for test 20 =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct SimpleStruct {
    id: u32,
    label: String,
    active: bool,
}

// ===== Helpers =====

fn rt<T: Encode + Decode + PartialEq + std::fmt::Debug>(value: &T) {
    let encoded = encode_to_vec(value).expect("encode_to_vec failed");
    let (decoded, bytes_read): (T, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(value, &decoded, "roundtrip value mismatch");
    assert_eq!(
        bytes_read,
        encoded.len(),
        "bytes_read should equal encoded length"
    );
}

// ===== Property tests =====

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    // 1. HashMap<u32, String> roundtrip, max 10 entries
    #[test]
    fn prop_hashmap_u32_string_roundtrip(
        m in proptest::collection::hash_map(any::<u32>(), any::<String>(), 0..=10usize)
    ) {
        rt(&m);
    }

    // 2. BTreeSet<u64> roundtrip
    #[test]
    fn prop_btreeset_u64_roundtrip(
        s in proptest::collection::btree_set(any::<u64>(), 0..=20usize)
    ) {
        rt(&s);
    }

    // 3. Vec<(u32, String)> max 10 items roundtrip
    #[test]
    fn prop_vec_tuple_roundtrip(
        v in proptest::collection::vec((any::<u32>(), any::<String>()), 0..=10usize)
    ) {
        rt(&v);
    }

    // 4. Vec<Vec<u8>> max 5 x 5 items roundtrip
    #[test]
    fn prop_nested_vec_roundtrip(
        v in proptest::collection::vec(
            proptest::collection::vec(any::<u8>(), 0..=5usize),
            0..=5usize,
        )
    ) {
        rt(&v);
    }

    // 5. Option<Vec<u32>> roundtrip
    #[test]
    fn prop_option_vec_roundtrip(
        v in prop::option::of(proptest::collection::vec(any::<u32>(), 0..=10usize))
    ) {
        rt(&v);
    }

    // 6. Result<u32, String> Ok variant roundtrip
    #[test]
    fn prop_result_ok_roundtrip(ok_val: u32) {
        let v: Result<u32, String> = Ok(ok_val);
        rt(&v);
    }

    // 7. Result<u32, String> Err variant roundtrip
    #[test]
    fn prop_result_err_roundtrip(err_val: String) {
        let v: Result<u32, String> = Err(err_val);
        rt(&v);
    }

    // 8. Range<u32> roundtrip (start <= end to satisfy ordering requirement)
    #[test]
    fn prop_range_u32_roundtrip(a: u32, b: u32) {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let range: Range<u32> = lo..hi;
        rt(&range);
    }

    // 9. RangeInclusive<u32> roundtrip
    #[test]
    fn prop_range_inclusive_roundtrip(a: u32, b: u32) {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let range: RangeInclusive<u32> = lo..=hi;
        rt(&range);
    }

    // 10. Cow<str> (backed by owned String) roundtrip
    #[test]
    fn prop_cow_str_roundtrip(s: String) {
        let cow: Cow<str> = Cow::Owned(s.clone());
        let encoded = encode_to_vec(&cow).expect("encode_to_vec failed for Cow<str>");
        let (decoded, bytes_read): (Cow<str>, usize) =
            decode_from_slice(&encoded).expect("decode_from_slice failed for Cow<str>");
        prop_assert_eq!(s.as_str(), decoded.as_ref(), "Cow<str> roundtrip value mismatch");
        prop_assert_eq!(bytes_read, encoded.len(), "bytes_read mismatch for Cow<str>");
    }

    // 11. i32 zigzag symmetry: encode then decode returns original value
    #[test]
    fn prop_i32_zigzag_symmetry(v: i32) {
        let encoded = encode_to_vec(&v).expect("encode_to_vec failed for i32");
        let (decoded, _): (i32, usize) =
            decode_from_slice(&encoded).expect("decode_from_slice failed for i32");
        prop_assert_eq!(v, decoded, "i32 zigzag symmetry failed");
    }

    // 12. u64 values 0..=250 encode to exactly 1 byte (single-byte varint range)
    #[test]
    fn prop_u64_varint_size(v in 0u64..=250u64) {
        let encoded = encode_to_vec(&v).expect("encode_to_vec failed for small u64");
        prop_assert_eq!(
            encoded.len(),
            1,
            "u64 value {} should encode to 1 byte but got {}",
            v,
            encoded.len()
        );
    }

    // 13. String of length N encodes as varint(N) + N bytes
    #[test]
    fn prop_string_len_prefix(
        s in proptest::collection::vec(0x20u8..=0x7Eu8, 0..50usize)
            .prop_map(|bytes| String::from_utf8(bytes).expect("valid ascii"))
    ) {
        let n = s.len();
        let encoded = encode_to_vec(&s).expect("encode_to_vec failed for String");
        // varint for n (0..=250) occupies 1 byte; string bytes follow
        let expected_len = 1 + n; // varint tag is 1 byte for n <= 250
        prop_assert_eq!(
            encoded.len(),
            expected_len,
            "String of len {} should encode to {} bytes but got {}",
            n,
            expected_len,
            encoded.len()
        );
    }

    // 14. Vec<u8> of length N encodes as varint(N) + N bytes
    #[test]
    fn prop_vec_len_prefix(
        v in proptest::collection::vec(any::<u8>(), 0..50usize)
    ) {
        let n = v.len();
        let encoded = encode_to_vec(&v).expect("encode_to_vec failed for Vec<u8>");
        let expected_len = 1 + n; // varint 1 byte for n <= 250
        prop_assert_eq!(
            encoded.len(),
            expected_len,
            "Vec<u8> of len {} should encode to {} bytes but got {}",
            n,
            expected_len,
            encoded.len()
        );
    }

    // 15. Tuple (a, b) bytes == concat of encode(a) + encode(b)
    #[test]
    fn prop_tuple_concat_bytes(a: u32, b: u64) {
        let tuple_bytes = encode_to_vec(&(a, b)).expect("encode tuple failed");
        let a_bytes = encode_to_vec(&a).expect("encode a failed");
        let b_bytes = encode_to_vec(&b).expect("encode b failed");
        let mut concat = a_bytes;
        concat.extend_from_slice(&b_bytes);
        prop_assert_eq!(
            tuple_bytes,
            concat,
            "tuple encoding should equal concatenation of field encodings"
        );
    }

    // 16. Option prefix: Some(v) starts with byte [1], None encodes as [0]
    #[test]
    fn prop_option_some_prefix(v: u32) {
        let some_encoded = encode_to_vec(&Some(v)).expect("encode Some failed");
        let none_encoded = encode_to_vec(&(None::<u32>)).expect("encode None failed");
        prop_assert_eq!(
            some_encoded[0],
            1u8,
            "Some variant should start with tag byte 1"
        );
        prop_assert_eq!(
            none_encoded,
            vec![0u8],
            "None should encode to exactly [0]"
        );
    }

    // 17. encoded_size<u32> matches encode_to_vec(u32).len()
    #[test]
    fn prop_encoded_size_u32_matches_vec(v: u32) {
        let encoded = encode_to_vec(&v).expect("encode_to_vec failed for u32");
        let size = encoded_size(&v).expect("encoded_size failed for u32");
        prop_assert_eq!(
            size,
            encoded.len(),
            "encoded_size should match actual encoded length for u32 {}",
            v
        );
    }

    // 18. bool always encodes to exactly 1 byte
    #[test]
    fn prop_bool_is_one_byte(v: bool) {
        let encoded = encode_to_vec(&v).expect("encode_to_vec failed for bool");
        prop_assert_eq!(
            encoded.len(),
            1,
            "bool should always encode to exactly 1 byte"
        );
    }

    // 19. [u8; 4] encodes as exactly 4 bytes (no length prefix for fixed-size arrays)
    #[test]
    fn prop_array_no_length_prefix(a: u8, b: u8, c: u8, d: u8) {
        let arr: [u8; 4] = [a, b, c, d];
        let encoded = encode_to_vec(&arr).expect("encode_to_vec failed for [u8;4]");
        prop_assert_eq!(
            encoded.len(),
            4,
            "[u8; 4] should encode as exactly 4 bytes without length prefix"
        );
        prop_assert_eq!(
            encoded.as_slice(),
            &[a, b, c, d],
            "[u8; 4] bytes should equal the raw array contents"
        );
    }

    // 20. Derived struct with u32 + String + bool fields roundtrip
    #[test]
    fn prop_derive_struct_roundtrip(id: u32, label: String, active: bool) {
        let original = SimpleStruct { id, label, active };
        let encoded = encode_to_vec(&original).expect("encode_to_vec failed for SimpleStruct");
        let (decoded, bytes_read): (SimpleStruct, usize) =
            decode_from_slice(&encoded).expect("decode_from_slice failed for SimpleStruct");
        prop_assert_eq!(
            original.id,
            decoded.id,
            "SimpleStruct.id roundtrip failed"
        );
        prop_assert_eq!(
            original.label,
            decoded.label,
            "SimpleStruct.label roundtrip failed"
        );
        prop_assert_eq!(
            original.active,
            decoded.active,
            "SimpleStruct.active roundtrip failed"
        );
        prop_assert_eq!(
            bytes_read,
            encoded.len(),
            "bytes_read should equal encoded length for SimpleStruct"
        );
    }
}
