//! Miscellaneous type tests covering NonZero integers, collections,
//! Option, BTreeMap, arrays, tuples, and error cases.

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
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

mod misc_types_tests {
    use super::*;

    // ===== Test 1: NonZeroU8(1) roundtrip =====

    #[test]
    fn test_misc_nonzero_u8_one_roundtrip() {
        let original = NonZeroU8::new(1).expect("nonzero u8 one");
        let bytes = encode_to_vec(&original).expect("encode NonZeroU8(1)");
        let (decoded, _): (NonZeroU8, _) = decode_from_slice(&bytes).expect("decode NonZeroU8(1)");
        assert_eq!(original, decoded);
    }

    // ===== Test 2: NonZeroU8(255) roundtrip (max value) =====

    #[test]
    fn test_misc_nonzero_u8_max_roundtrip() {
        let original = NonZeroU8::new(255).expect("nonzero u8 max");
        let bytes = encode_to_vec(&original).expect("encode NonZeroU8(255)");
        let (decoded, _): (NonZeroU8, _) =
            decode_from_slice(&bytes).expect("decode NonZeroU8(255)");
        assert_eq!(original, decoded);
    }

    // ===== Test 3: NonZeroU16(1000) roundtrip =====

    #[test]
    fn test_misc_nonzero_u16_1000_roundtrip() {
        let original = NonZeroU16::new(1000).expect("nonzero u16 1000");
        let bytes = encode_to_vec(&original).expect("encode NonZeroU16(1000)");
        let (decoded, _): (NonZeroU16, _) =
            decode_from_slice(&bytes).expect("decode NonZeroU16(1000)");
        assert_eq!(original, decoded);
    }

    // ===== Test 4: NonZeroU32(u32::MAX) roundtrip =====

    #[test]
    fn test_misc_nonzero_u32_max_roundtrip() {
        let original = NonZeroU32::new(u32::MAX).expect("nonzero u32::MAX");
        let bytes = encode_to_vec(&original).expect("encode NonZeroU32(u32::MAX)");
        let (decoded, _): (NonZeroU32, _) =
            decode_from_slice(&bytes).expect("decode NonZeroU32(u32::MAX)");
        assert_eq!(original, decoded);
    }

    // ===== Test 5: NonZeroU64(u64::MAX) roundtrip =====

    #[test]
    fn test_misc_nonzero_u64_max_roundtrip() {
        let original = NonZeroU64::new(u64::MAX).expect("nonzero u64::MAX");
        let bytes = encode_to_vec(&original).expect("encode NonZeroU64(u64::MAX)");
        let (decoded, _): (NonZeroU64, _) =
            decode_from_slice(&bytes).expect("decode NonZeroU64(u64::MAX)");
        assert_eq!(original, decoded);
    }

    // ===== Test 6: NonZeroUsize(usize::MAX) roundtrip =====

    #[test]
    fn test_misc_nonzero_usize_max_roundtrip() {
        let original = NonZeroUsize::new(usize::MAX).expect("nonzero usize::MAX");
        let bytes = encode_to_vec(&original).expect("encode NonZeroUsize(usize::MAX)");
        let (decoded, _): (NonZeroUsize, _) =
            decode_from_slice(&bytes).expect("decode NonZeroUsize(usize::MAX)");
        assert_eq!(original, decoded);
    }

    // ===== Test 7: NonZeroI8(1) roundtrip =====

    #[test]
    fn test_misc_nonzero_i8_one_roundtrip() {
        let original = NonZeroI8::new(1).expect("nonzero i8 one");
        let bytes = encode_to_vec(&original).expect("encode NonZeroI8(1)");
        let (decoded, _): (NonZeroI8, _) = decode_from_slice(&bytes).expect("decode NonZeroI8(1)");
        assert_eq!(original, decoded);
    }

    // ===== Test 8: NonZeroI32(-1) roundtrip =====

    #[test]
    fn test_misc_nonzero_i32_neg_one_roundtrip() {
        let original = NonZeroI32::new(-1).expect("nonzero i32 neg one");
        let bytes = encode_to_vec(&original).expect("encode NonZeroI32(-1)");
        let (decoded, _): (NonZeroI32, _) =
            decode_from_slice(&bytes).expect("decode NonZeroI32(-1)");
        assert_eq!(original, decoded);
    }

    // ===== Test 9: NonZeroI64(i64::MIN) roundtrip =====

    #[test]
    fn test_misc_nonzero_i64_min_roundtrip() {
        let original = NonZeroI64::new(i64::MIN).expect("nonzero i64::MIN");
        let bytes = encode_to_vec(&original).expect("encode NonZeroI64(i64::MIN)");
        let (decoded, _): (NonZeroI64, _) =
            decode_from_slice(&bytes).expect("decode NonZeroI64(i64::MIN)");
        assert_eq!(original, decoded);
    }

    // ===== Test 10: Struct with NonZero field derive roundtrip =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MiscNonZeroStruct {
        id: NonZeroU32,
        value: NonZeroI64,
        label: u64,
    }

    #[test]
    fn test_misc_struct_nonzero_field_derive_roundtrip() {
        let original = MiscNonZeroStruct {
            id: NonZeroU32::new(42).expect("nonzero u32 42"),
            value: NonZeroI64::new(-9_876_543_210_i64).expect("nonzero i64"),
            label: 0xDEAD_BEEF_CAFE_u64,
        };
        let bytes = encode_to_vec(&original).expect("encode MiscNonZeroStruct");
        let (decoded, _): (MiscNonZeroStruct, _) =
            decode_from_slice(&bytes).expect("decode MiscNonZeroStruct");
        assert_eq!(original, decoded);
    }

    // ===== Test 11: Vec<NonZeroU32> roundtrip =====

    #[test]
    fn test_misc_vec_nonzero_u32_roundtrip() {
        let original: Vec<NonZeroU32> = [1u32, 7, 42, 999, u32::MAX - 3]
            .iter()
            .map(|&n| NonZeroU32::new(n).expect("nonzero u32 element"))
            .collect();
        let bytes = encode_to_vec(&original).expect("encode Vec<NonZeroU32>");
        let (decoded, _): (Vec<NonZeroU32>, _) =
            decode_from_slice(&bytes).expect("decode Vec<NonZeroU32>");
        assert_eq!(original, decoded);
    }

    // ===== Test 12: Option<NonZeroU32> Some roundtrip =====

    #[test]
    fn test_misc_option_nonzero_u32_some_roundtrip() {
        let original: Option<NonZeroU32> =
            Some(NonZeroU32::new(123_456).expect("nonzero u32 123456"));
        let bytes = encode_to_vec(&original).expect("encode Option<NonZeroU32> Some");
        let (decoded, _): (Option<NonZeroU32>, _) =
            decode_from_slice(&bytes).expect("decode Option<NonZeroU32> Some");
        assert_eq!(original, decoded);
    }

    // ===== Test 13: Option<NonZeroU32> None roundtrip =====

    #[test]
    fn test_misc_option_nonzero_u32_none_roundtrip() {
        let original: Option<NonZeroU32> = None;
        let bytes = encode_to_vec(&original).expect("encode Option<NonZeroU32> None");
        let (decoded, _): (Option<NonZeroU32>, _) =
            decode_from_slice(&bytes).expect("decode Option<NonZeroU32> None");
        assert_eq!(original, decoded);
    }

    // ===== Test 14: BTreeMap<u32, NonZeroU32> roundtrip =====

    #[test]
    fn test_misc_btreemap_u32_nonzero_u32_roundtrip() {
        let mut original: BTreeMap<u32, NonZeroU32> = BTreeMap::new();
        original.insert(1, NonZeroU32::new(100).expect("nonzero 100"));
        original.insert(2, NonZeroU32::new(200).expect("nonzero 200"));
        original.insert(3, NonZeroU32::new(u32::MAX).expect("nonzero u32::MAX"));
        let bytes = encode_to_vec(&original).expect("encode BTreeMap<u32, NonZeroU32>");
        let (decoded, _): (BTreeMap<u32, NonZeroU32>, _) =
            decode_from_slice(&bytes).expect("decode BTreeMap<u32, NonZeroU32>");
        assert_eq!(original, decoded);
    }

    // ===== Test 15: Binary heap contents encoded as Vec<NonZeroU32> =====

    #[test]
    fn test_misc_binary_heap_nonzero_u32_via_vec_roundtrip() {
        use std::collections::BinaryHeap;
        let heap: BinaryHeap<NonZeroU32> = [5u32, 3, 9, 1, 7]
            .iter()
            .map(|&n| NonZeroU32::new(n).expect("nonzero heap element"))
            .collect();
        // Encode as Vec to test the encoded form; heap ordering is non-deterministic
        let heap_vec: Vec<NonZeroU32> = heap.into_sorted_vec();
        let bytes = encode_to_vec(&heap_vec).expect("encode sorted heap as Vec<NonZeroU32>");
        let (decoded, _): (Vec<NonZeroU32>, _) =
            decode_from_slice(&bytes).expect("decode sorted heap Vec<NonZeroU32>");
        assert_eq!(heap_vec, decoded);
    }

    // ===== Test 16: NonZeroU128 roundtrip =====

    #[test]
    fn test_misc_nonzero_u128_roundtrip() {
        let original = NonZeroU128::new(u128::MAX).expect("nonzero u128::MAX");
        let bytes = encode_to_vec(&original).expect("encode NonZeroU128(u128::MAX)");
        let (decoded, _): (NonZeroU128, _) =
            decode_from_slice(&bytes).expect("decode NonZeroU128(u128::MAX)");
        assert_eq!(original, decoded);
    }

    // ===== Test 17: NonZeroI128 roundtrip =====

    #[test]
    fn test_misc_nonzero_i128_roundtrip() {
        let original = NonZeroI128::new(i128::MIN).expect("nonzero i128::MIN");
        let bytes = encode_to_vec(&original).expect("encode NonZeroI128(i128::MIN)");
        let (decoded, _): (NonZeroI128, _) =
            decode_from_slice(&bytes).expect("decode NonZeroI128(i128::MIN)");
        assert_eq!(original, decoded);
    }

    // ===== Test 18: NonZeroU32(0) encodes as 0 and decodes as error =====

    #[test]
    fn test_misc_nonzero_u32_zero_decode_is_error() {
        // Encode a plain u32 value of 0, then attempt to decode it as NonZeroU32.
        // This must fail because NonZeroU32 cannot represent zero.
        let zero_bytes = encode_to_vec(&0u32).expect("encode u32 zero");
        let result: Result<(NonZeroU32, usize), _> = decode_from_slice(&zero_bytes);
        assert!(
            result.is_err(),
            "decoding zero u32 bytes as NonZeroU32 must produce an error"
        );
    }

    // ===== Test 19: NonZeroI16 positive and negative values =====

    #[test]
    fn test_misc_nonzero_i16_positive_and_negative_roundtrip() {
        let pos = NonZeroI16::new(i16::MAX).expect("nonzero i16::MAX");
        let bytes_pos = encode_to_vec(&pos).expect("encode NonZeroI16(i16::MAX)");
        let (decoded_pos, _): (NonZeroI16, _) =
            decode_from_slice(&bytes_pos).expect("decode NonZeroI16(i16::MAX)");
        assert_eq!(pos, decoded_pos);

        let neg = NonZeroI16::new(i16::MIN).expect("nonzero i16::MIN");
        let bytes_neg = encode_to_vec(&neg).expect("encode NonZeroI16(i16::MIN)");
        let (decoded_neg, _): (NonZeroI16, _) =
            decode_from_slice(&bytes_neg).expect("decode NonZeroI16(i16::MIN)");
        assert_eq!(neg, decoded_neg);
    }

    // ===== Test 20: NonZeroU32 in two different value contexts =====

    #[test]
    fn test_misc_nonzero_u32_different_contexts_roundtrip() {
        // Verify that small and large NonZeroU32 values both roundtrip independently.
        let small = NonZeroU32::new(1).expect("nonzero u32 small");
        let large = NonZeroU32::new(u32::MAX / 2).expect("nonzero u32 large");

        let bytes_small = encode_to_vec(&small).expect("encode small NonZeroU32");
        let (decoded_small, _): (NonZeroU32, _) =
            decode_from_slice(&bytes_small).expect("decode small NonZeroU32");
        assert_eq!(small, decoded_small);

        let bytes_large = encode_to_vec(&large).expect("encode large NonZeroU32");
        let (decoded_large, _): (NonZeroU32, _) =
            decode_from_slice(&bytes_large).expect("decode large NonZeroU32");
        assert_eq!(large, decoded_large);
    }

    // ===== Test 21: Array [NonZeroU32; 4] roundtrip =====

    #[test]
    fn test_misc_array_nonzero_u32_roundtrip() {
        let original: [NonZeroU32; 4] = [
            NonZeroU32::new(10).expect("nonzero 10"),
            NonZeroU32::new(20).expect("nonzero 20"),
            NonZeroU32::new(30).expect("nonzero 30"),
            NonZeroU32::new(u32::MAX).expect("nonzero u32::MAX"),
        ];
        let bytes = encode_to_vec(&original).expect("encode [NonZeroU32; 4]");
        let (decoded, _): ([NonZeroU32; 4], _) =
            decode_from_slice(&bytes).expect("decode [NonZeroU32; 4]");
        assert_eq!(original, decoded);
    }

    // ===== Test 22: Tuple (NonZeroU32, NonZeroU64) roundtrip =====

    #[test]
    fn test_misc_tuple_nonzero_u32_u64_roundtrip() {
        let original: (NonZeroU32, NonZeroU64) = (
            NonZeroU32::new(999_999).expect("nonzero u32 tuple"),
            NonZeroU64::new(u64::MAX / 3).expect("nonzero u64 tuple"),
        );
        let bytes = encode_to_vec(&original).expect("encode (NonZeroU32, NonZeroU64)");
        let (decoded, _): ((NonZeroU32, NonZeroU64), _) =
            decode_from_slice(&bytes).expect("decode (NonZeroU32, NonZeroU64)");
        assert_eq!(original, decoded);
    }
}
