//! Comprehensive tests for all integer types.

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
use oxicode::{decode_from_slice, encode_to_vec};

macro_rules! test_integer_roundtrip {
    ($ty:ty, $min:expr, $max:expr, $name:ident) => {
        #[test]
        fn $name() {
            for v in [$min, $max, 0 as $ty, 1 as $ty] {
                let enc = encode_to_vec(&v).expect("encode");
                let (dec, _): ($ty, _) = decode_from_slice(&enc).expect("decode");
                assert_eq!(v, dec);
            }
        }
    };
}

test_integer_roundtrip!(u8, u8::MIN, u8::MAX, test_u8_boundaries);
test_integer_roundtrip!(u16, u16::MIN, u16::MAX, test_u16_boundaries);
test_integer_roundtrip!(u32, u32::MIN, u32::MAX, test_u32_boundaries);
test_integer_roundtrip!(u64, u64::MIN, u64::MAX, test_u64_boundaries);
test_integer_roundtrip!(u128, u128::MIN, u128::MAX, test_u128_boundaries);
test_integer_roundtrip!(i8, i8::MIN, i8::MAX, test_i8_boundaries);
test_integer_roundtrip!(i16, i16::MIN, i16::MAX, test_i16_boundaries);
test_integer_roundtrip!(i32, i32::MIN, i32::MAX, test_i32_boundaries);
test_integer_roundtrip!(i64, i64::MIN, i64::MAX, test_i64_boundaries);
test_integer_roundtrip!(i128, i128::MIN, i128::MAX, test_i128_boundaries);
test_integer_roundtrip!(usize, usize::MIN, usize::MAX, test_usize_boundaries);
test_integer_roundtrip!(isize, isize::MIN, isize::MAX, test_isize_boundaries);

#[test]
fn test_varint_encoding_increases_monotonically() {
    // Values in the same varint range should encode to same number of bytes
    let v0_enc = encode_to_vec(&0u64).expect("encode");
    let v1_enc = encode_to_vec(&1u64).expect("encode");
    let v127_enc = encode_to_vec(&127u64).expect("encode");

    assert_eq!(
        v0_enc.len(),
        v1_enc.len(),
        "0 and 1 should use same byte count"
    );
    assert_eq!(
        v0_enc.len(),
        v127_enc.len(),
        "0..=127 should use same byte count"
    );
}

#[test]
fn test_signed_negative_encoding() {
    // i64::MIN and i64::MAX should both encode and decode correctly
    let values = [
        i64::MIN,
        i64::MIN / 2,
        -1i64,
        0i64,
        1i64,
        i64::MAX / 2,
        i64::MAX,
    ];
    for v in values {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (i64, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(v, dec, "failed for {}", v);
    }
}

#[test]
fn test_i128_extremes_roundtrip() {
    let values = [i128::MIN, -1i128, 0i128, 1i128, i128::MAX];
    for v in values {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (i128, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(v, dec);
    }
}
