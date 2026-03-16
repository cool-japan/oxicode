//! Advanced property-based roundtrip tests using proptest.
//!
//! These tests complement `proptest_test.rs` with additional coverage
//! for primitive types, compound types, byte-length consistency, and
//! fixed-int encoding variants.

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
use proptest::collection::vec;
use proptest::option;
use proptest::prelude::*;

/// Encode then decode, assert roundtrip identity and bytes-consumed count.
fn roundtrip_adv<T: Encode + Decode + PartialEq + std::fmt::Debug>(value: &T) {
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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    // 1. u8 any value roundtrip
    #[test]
    fn prop_adv_u8_roundtrip(v in any::<u8>()) {
        roundtrip_adv(&v);
    }

    // 2. u16 any value roundtrip
    #[test]
    fn prop_adv_u16_roundtrip(v in any::<u16>()) {
        roundtrip_adv(&v);
    }

    // 3. u32 any value roundtrip
    #[test]
    fn prop_adv_u32_roundtrip(v in any::<u32>()) {
        roundtrip_adv(&v);
    }

    // 4. u64 any value roundtrip
    #[test]
    fn prop_adv_u64_roundtrip(v in any::<u64>()) {
        roundtrip_adv(&v);
    }

    // 5. i8 any value roundtrip
    #[test]
    fn prop_adv_i8_roundtrip(v in any::<i8>()) {
        roundtrip_adv(&v);
    }

    // 6. i16 any value roundtrip
    #[test]
    fn prop_adv_i16_roundtrip(v in any::<i16>()) {
        roundtrip_adv(&v);
    }

    // 7. i32 any value roundtrip
    #[test]
    fn prop_adv_i32_roundtrip(v in any::<i32>()) {
        roundtrip_adv(&v);
    }

    // 8. i64 any value roundtrip
    #[test]
    fn prop_adv_i64_roundtrip(v in any::<i64>()) {
        roundtrip_adv(&v);
    }

    // 9. bool any value roundtrip
    #[test]
    fn prop_adv_bool_roundtrip(v in any::<bool>()) {
        roundtrip_adv(&v);
    }

    // 10. char any valid Unicode scalar roundtrip
    #[test]
    fn prop_adv_char_roundtrip(v in any::<char>()) {
        roundtrip_adv(&v);
    }

    // 11. any string roundtrip (limited size via regex)
    #[test]
    fn prop_adv_string_roundtrip(
        s in ".*".prop_filter("short string", |x| x.len() <= 200)
    ) {
        roundtrip_adv(&s);
    }

    // 12. Vec<u8> 0..100 elements roundtrip
    #[test]
    fn prop_adv_vec_u8_roundtrip(v in vec(any::<u8>(), 0..100usize)) {
        roundtrip_adv(&v);
    }

    // 13. Vec<u32> 0..50 elements roundtrip
    #[test]
    fn prop_adv_vec_u32_roundtrip(v in vec(any::<u32>(), 0..50usize)) {
        roundtrip_adv(&v);
    }

    // 14. Option<u64> roundtrip
    #[test]
    fn prop_adv_option_u64_roundtrip(v in option::of(any::<u64>())) {
        roundtrip_adv(&v);
    }

    // 15. (u32, u64, bool) 3-tuple roundtrip
    #[test]
    fn prop_adv_tuple3_roundtrip(v in (any::<u32>(), any::<u64>(), any::<bool>())) {
        roundtrip_adv(&v);
    }

    // 16. Vec<bool> 0..100 elements roundtrip
    #[test]
    fn prop_adv_vec_bool_roundtrip(v in vec(any::<bool>(), 0..100usize)) {
        roundtrip_adv(&v);
    }

    // 17. u128 any value roundtrip
    #[test]
    fn prop_adv_u128_roundtrip(v in any::<u128>()) {
        roundtrip_adv(&v);
    }

    // 18. i128 any value roundtrip
    #[test]
    fn prop_adv_i128_roundtrip(v in any::<i128>()) {
        roundtrip_adv(&v);
    }

    // 19. i64 with fixed-int encoding roundtrip
    #[test]
    fn prop_adv_i64_fixedint_roundtrip(v in any::<i64>()) {
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded = encode_to_vec_with_config(&v, cfg).expect("encode_to_vec_with_config failed");
        let (decoded, bytes_read): (i64, usize) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode_from_slice_with_config failed");
        prop_assert_eq!(v, decoded, "fixed-int i64 roundtrip value mismatch");
        prop_assert_eq!(bytes_read, encoded.len(), "bytes_read should equal encoded length");
        // Fixed int encoding always uses exactly 8 bytes for i64
        prop_assert_eq!(encoded.len(), 8usize, "fixed-int i64 must be 8 bytes");
    }

    // 20. Vec<u8> 0..50: decoded bytes_consumed equals encoded length
    #[test]
    fn prop_adv_vec_u8_consumed_eq_encoded_len(v in vec(any::<u8>(), 0..50usize)) {
        let encoded = encode_to_vec(&v).expect("encode_to_vec failed");
        let (_decoded, bytes_read): (Vec<u8>, usize) =
            decode_from_slice(&encoded).expect("decode_from_slice failed");
        prop_assert_eq!(bytes_read, encoded.len(), "consumed bytes must equal encoded length");
    }

    // 21. u64: encoding the same value twice produces a buffer twice the size
    #[test]
    fn prop_adv_u64_double_encode_size(v in any::<u64>()) {
        let single = encode_to_vec(&v).expect("encode_to_vec single failed");
        let mut double = single.clone();
        double.extend_from_slice(&single);
        prop_assert_eq!(
            double.len(),
            single.len() * 2,
            "concatenation of two encodings must have double the size"
        );
        // Verify both copies decode correctly from the concatenated buffer
        let (first, consumed1): (u64, usize) =
            decode_from_slice(&double).expect("decode first copy failed");
        prop_assert_eq!(first, v);
        let (second, consumed2): (u64, usize) =
            decode_from_slice(&double[consumed1..]).expect("decode second copy failed");
        prop_assert_eq!(second, v);
        prop_assert_eq!(consumed1 + consumed2, double.len());
    }

    // 22. (String, u32) pair roundtrip
    #[test]
    fn prop_adv_string_u32_pair_roundtrip(
        v in (".*".prop_filter("short string", |x| x.len() <= 200), any::<u32>())
    ) {
        roundtrip_adv(&v);
    }
}
