//! Advanced property-based roundtrip tests (set 22) using proptest.
//!
//! Tests verify encode → decode is a perfect roundtrip for various types,
//! including custom derived structs, enums, configs, fixed-int sizes,
//! nested collections, and distinctness of encoded bytes.

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PairU32String {
    first: u32,
    second: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Triple {
    a: u32,
    b: u64,
    c: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Ternary {
    Zero,
    One(u32),
    Two(u32, u32),
}

// 1. u32 identity roundtrip
proptest! {
    #[test]
    fn prop_u32_identity(value: u32) {
        let encoded = encode_to_vec(&value).expect("encode u32 failed");
        let (decoded, _consumed): (u32, usize) =
            decode_from_slice(&encoded).expect("decode u32 failed");
        prop_assert_eq!(decoded, value);
    }
}

// 2. i32 identity roundtrip
proptest! {
    #[test]
    fn prop_i32_identity(value: i32) {
        let encoded = encode_to_vec(&value).expect("encode i32 failed");
        let (decoded, _consumed): (i32, usize) =
            decode_from_slice(&encoded).expect("decode i32 failed");
        prop_assert_eq!(decoded, value);
    }
}

// 3. u64 identity roundtrip
proptest! {
    #[test]
    fn prop_u64_identity(value: u64) {
        let encoded = encode_to_vec(&value).expect("encode u64 failed");
        let (decoded, _consumed): (u64, usize) =
            decode_from_slice(&encoded).expect("decode u64 failed");
        prop_assert_eq!(decoded, value);
    }
}

// 4. String identity roundtrip
proptest! {
    #[test]
    fn prop_string_identity(value: String) {
        let encoded = encode_to_vec(&value).expect("encode String failed");
        let (decoded, _consumed): (String, usize) =
            decode_from_slice(&encoded).expect("decode String failed");
        prop_assert_eq!(decoded, value);
    }
}

// 5. Vec<u8> identity roundtrip
proptest! {
    #[test]
    fn prop_vec_u8_identity(v in proptest::collection::vec(any::<u8>(), 0..100)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u8> failed");
        let (decoded, _consumed): (Vec<u8>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<u8> failed");
        prop_assert_eq!(decoded, v);
    }
}

// 6. PairU32String identity roundtrip
proptest! {
    #[test]
    fn prop_pair_identity(first: u32, second: String) {
        let value = PairU32String { first, second };
        let encoded = encode_to_vec(&value).expect("encode PairU32String failed");
        let (decoded, _consumed): (PairU32String, usize) =
            decode_from_slice(&encoded).expect("decode PairU32String failed");
        prop_assert_eq!(decoded, value);
    }
}

// 7. Triple identity roundtrip
proptest! {
    #[test]
    fn prop_triple_identity(a: u32, b: u64, c: String) {
        let value = Triple { a, b, c };
        let encoded = encode_to_vec(&value).expect("encode Triple failed");
        let (decoded, _consumed): (Triple, usize) =
            decode_from_slice(&encoded).expect("decode Triple failed");
        prop_assert_eq!(decoded, value);
    }
}

// 8. Ternary::Zero roundtrip
proptest! {
    #[test]
    fn prop_ternary_zero(_unused: u8) {
        let value = Ternary::Zero;
        let encoded = encode_to_vec(&value).expect("encode Ternary::Zero failed");
        let (decoded, _consumed): (Ternary, usize) =
            decode_from_slice(&encoded).expect("decode Ternary::Zero failed");
        prop_assert_eq!(decoded, value);
    }
}

// 9. Ternary::One roundtrip
proptest! {
    #[test]
    fn prop_ternary_one(n: u32) {
        let value = Ternary::One(n);
        let encoded = encode_to_vec(&value).expect("encode Ternary::One failed");
        let (decoded, _consumed): (Ternary, usize) =
            decode_from_slice(&encoded).expect("decode Ternary::One failed");
        prop_assert_eq!(decoded, value);
    }
}

// 10. Ternary::Two roundtrip
proptest! {
    #[test]
    fn prop_ternary_two(x: u32, y: u32) {
        let value = Ternary::Two(x, y);
        let encoded = encode_to_vec(&value).expect("encode Ternary::Two failed");
        let (decoded, _consumed): (Ternary, usize) =
            decode_from_slice(&encoded).expect("decode Ternary::Two failed");
        prop_assert_eq!(decoded, value);
    }
}

// 11. Option<u64>: None/Some roundtrip
proptest! {
    #[test]
    fn prop_option_u64(opt in proptest::option::of(any::<u64>())) {
        let encoded = encode_to_vec(&opt).expect("encode Option<u64> failed");
        let (decoded, _consumed): (Option<u64>, usize) =
            decode_from_slice(&encoded).expect("decode Option<u64> failed");
        prop_assert_eq!(decoded, opt);
    }
}

// 12. consumed bytes for u32 equals encoded length
proptest! {
    #[test]
    fn prop_consumed_bytes_u32(value: u32) {
        let encoded = encode_to_vec(&value).expect("encode u32 for consumed check failed");
        let (_decoded, consumed): (u32, usize) =
            decode_from_slice(&encoded).expect("decode u32 for consumed check failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 13. consumed bytes for String equals encoded length
proptest! {
    #[test]
    fn prop_consumed_bytes_string(value: String) {
        let encoded = encode_to_vec(&value).expect("encode String for consumed check failed");
        let (_decoded, consumed): (String, usize) =
            decode_from_slice(&encoded).expect("decode String for consumed check failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 14. Different u32 values produce different encoded bytes
proptest! {
    #[test]
    fn prop_different_u32_different_bytes(a: u32, b: u32) {
        prop_assume!(a != b);
        let encoded_a = encode_to_vec(&a).expect("encode u32 a failed");
        let encoded_b = encode_to_vec(&b).expect("encode u32 b failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// 15. Vec<bool> roundtrip
proptest! {
    #[test]
    fn prop_vec_bool_roundtrip(v in proptest::collection::vec(any::<bool>(), 0..64)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<bool> failed");
        let (decoded, _consumed): (Vec<bool>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<bool> failed");
        prop_assert_eq!(decoded, v);
    }
}

// 16. u8 with fixed_int_encoding is always 1 byte
proptest! {
    #[test]
    fn prop_u8_fixed_int_size(value: u8) {
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded =
            encode_to_vec_with_config(&value, cfg).expect("encode u8 with fixed_int failed");
        let (decoded, consumed): (u8, usize) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode u8 with fixed_int failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
        prop_assert_eq!(encoded.len(), 1usize, "u8 with fixed_int_encoding must always be 1 byte");
    }
}

// 17. u32 with fixed_int_encoding is always 4 bytes
proptest! {
    #[test]
    fn prop_u32_fixed_int_4_bytes(value: u32) {
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded =
            encode_to_vec_with_config(&value, cfg).expect("encode u32 with fixed_int failed");
        let (decoded, consumed): (u32, usize) =
            decode_from_slice_with_config(&encoded, cfg)
                .expect("decode u32 with fixed_int failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
        prop_assert_eq!(
            encoded.len(),
            4usize,
            "u32 with fixed_int_encoding must always be 4 bytes"
        );
    }
}

// 18. u64 with fixed_int_encoding is always 8 bytes
proptest! {
    #[test]
    fn prop_u64_fixed_int_8_bytes(value: u64) {
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded =
            encode_to_vec_with_config(&value, cfg).expect("encode u64 with fixed_int failed");
        let (decoded, consumed): (u64, usize) =
            decode_from_slice_with_config(&encoded, cfg)
                .expect("decode u64 with fixed_int failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
        prop_assert_eq!(
            encoded.len(),
            8usize,
            "u64 with fixed_int_encoding must always be 8 bytes"
        );
    }
}

// 19. Vec<u32> identity roundtrip (0..50 elements)
proptest! {
    #[test]
    fn prop_vec_u32_identity(v in proptest::collection::vec(any::<u32>(), 0..50)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u32> failed");
        let (decoded, consumed): (Vec<u32>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<u32> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 20. Vec<String> identity roundtrip (0..20 elements, each 0..50 chars)
proptest! {
    #[test]
    fn prop_vec_string_identity(
        v in proptest::collection::vec(
            proptest::string::string_regex(".{0,50}").expect("regex compile failed"),
            0..20,
        )
    ) {
        let encoded = encode_to_vec(&v).expect("encode Vec<String> failed");
        let (decoded, consumed): (Vec<String>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<String> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 21. (u32, u32) tuple identity roundtrip
proptest! {
    #[test]
    fn prop_tuple_u32_u32_identity(a: u32, b: u32) {
        let value = (a, b);
        let encoded = encode_to_vec(&value).expect("encode (u32, u32) failed");
        let (decoded, consumed): ((u32, u32), usize) =
            decode_from_slice(&encoded).expect("decode (u32, u32) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 22. Vec<Vec<u8>> nested identity roundtrip (max outer 10, inner 0..20)
proptest! {
    #[test]
    fn prop_nested_vec_identity(
        v in proptest::collection::vec(
            proptest::collection::vec(any::<u8>(), 0..20),
            0..10,
        )
    ) {
        let encoded = encode_to_vec(&v).expect("encode Vec<Vec<u8>> failed");
        let (decoded, consumed): (Vec<Vec<u8>>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Vec<u8>> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}
