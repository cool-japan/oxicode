//! Property-based roundtrip tests for struct and enum types using proptest.
//!
//! Tests verify that for any valid value produced by arbitrary strategies,
//! encoding then decoding yields an identical value, and that `encoded_size`
//! matches the actual byte length produced by `encode_to_vec`.

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

use oxicode::{decode_from_slice, encode_to_vec, encoded_size, Decode, Encode};
use proptest::prelude::*;

// ---------------------------------------------------------------------------
// Custom struct / enum definitions (names are unique – no clash with
// proptest_test.rs which defines Point, Record, PropU64, SkipProp, etc.)
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct StructPrimA {
    a: u32,
    b: u64,
    c: bool,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct StructStrBytes {
    s: String,
    v: Vec<u8>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct StructInner {
    x: i32,
    y: i32,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct StructOuter {
    inner: StructInner,
    tag: u8,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
enum EnumVariants {
    Unit,
    Single(u32),
    Pair(i64, bool),
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct StructOptStr {
    name: Option<String>,
    value: u32,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn roundtrip_val<T: Encode + Decode + PartialEq + std::fmt::Debug>(
    value: T,
) -> Result<(), TestCaseError> {
    let encoded = encode_to_vec(&value).map_err(|e| TestCaseError::fail(format!("encode: {e}")))?;
    let (decoded, bytes_read): (T, usize) =
        decode_from_slice(&encoded).map_err(|e| TestCaseError::fail(format!("decode: {e}")))?;
    prop_assert_eq!(&value, &decoded, "roundtrip value mismatch");
    prop_assert_eq!(bytes_read, encoded.len(), "bytes_read != encoded.len()");
    Ok(())
}

// ---------------------------------------------------------------------------
// Property tests
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    // 1. Arbitrary u8 roundtrip
    #[test]
    fn prop_struct_u8_roundtrip(v: u8) {
        roundtrip_val(v)?;
    }

    // 2. Arbitrary i8 roundtrip
    #[test]
    fn prop_struct_i8_roundtrip(v: i8) {
        roundtrip_val(v)?;
    }

    // 3. Arbitrary u16 roundtrip
    #[test]
    fn prop_struct_u16_roundtrip(v: u16) {
        roundtrip_val(v)?;
    }

    // 4. Arbitrary i16 roundtrip
    #[test]
    fn prop_struct_i16_roundtrip(v: i16) {
        roundtrip_val(v)?;
    }

    // 5. Arbitrary u128 roundtrip
    #[test]
    fn prop_struct_u128_roundtrip(v: u128) {
        roundtrip_val(v)?;
    }

    // 6. Arbitrary i128 roundtrip
    #[test]
    fn prop_struct_i128_roundtrip(v: i128) {
        roundtrip_val(v)?;
    }

    // 7. Arbitrary bool roundtrip
    #[test]
    fn prop_struct_bool_roundtrip(v: bool) {
        roundtrip_val(v)?;
    }

    // 8. Struct with arbitrary u32, u64, bool roundtrip
    #[test]
    fn prop_struct_prim_a_roundtrip(a: u32, b: u64, c: bool) {
        let s = StructPrimA { a, b, c };
        roundtrip_val(s)?;
    }

    // 9. Struct with arbitrary String, Vec<u8> roundtrip
    #[test]
    fn prop_struct_str_bytes_roundtrip(s: String, v: Vec<u8>) {
        let val = StructStrBytes { s, v };
        roundtrip_val(val)?;
    }

    // 10. Nested struct arbitrary roundtrip
    #[test]
    fn prop_struct_outer_roundtrip(x: i32, y: i32, tag: u8) {
        let val = StructOuter { inner: StructInner { x, y }, tag };
        roundtrip_val(val)?;
    }

    // 11. Enum with arbitrary variants roundtrip
    #[test]
    fn prop_enum_variants_roundtrip(kind in 0u8..3u8, n: u32, m: i64, flag: bool) {
        let val = match kind {
            0 => EnumVariants::Unit,
            1 => EnumVariants::Single(n),
            _ => EnumVariants::Pair(m, flag),
        };
        roundtrip_val(val)?;
    }

    // 12. Vec<(u32, String)> roundtrip
    #[test]
    fn prop_struct_vec_tuple_u32_string_roundtrip(
        items in proptest::collection::vec((0u32..u32::MAX, ".*"), 0..32)
    ) {
        roundtrip_val(items)?;
    }

    // 13. BTreeMap<u32, u32> roundtrip (deterministic ordering)
    #[test]
    fn prop_struct_btreemap_u32_u32_roundtrip(
        pairs in proptest::collection::btree_map(0u32..u32::MAX, 0u32..u32::MAX, 0..32)
    ) {
        let map: BTreeMap<u32, u32> = pairs;
        roundtrip_val(map)?;
    }

    // 14. Arbitrary tuples (u8, u16, u32, u64) roundtrip
    #[test]
    fn prop_struct_tuple4_roundtrip(a: u8, b: u16, c: u32, d: u64) {
        roundtrip_val((a, b, c, d))?;
    }

    // 15. encoded_size matches encode_to_vec len for u64
    #[test]
    fn prop_struct_encoded_size_u64(v: u64) {
        let size = encoded_size(&v).map_err(|e| TestCaseError::fail(format!("encoded_size: {e}")))?;
        let bytes = encode_to_vec(&v).map_err(|e| TestCaseError::fail(format!("encode_to_vec: {e}")))?;
        prop_assert_eq!(size, bytes.len());
    }

    // 16. encoded_size matches encode_to_vec len for String
    #[test]
    fn prop_struct_encoded_size_string(v: String) {
        let size = encoded_size(&v).map_err(|e| TestCaseError::fail(format!("encoded_size: {e}")))?;
        let bytes = encode_to_vec(&v).map_err(|e| TestCaseError::fail(format!("encode_to_vec: {e}")))?;
        prop_assert_eq!(size, bytes.len());
    }

    // 17. Vec<u64> encoded_size matches encode_to_vec len
    #[test]
    fn prop_struct_encoded_size_vec_u64(
        v in proptest::collection::vec(0u64..u64::MAX, 0..64)
    ) {
        let size = encoded_size(&v).map_err(|e| TestCaseError::fail(format!("encoded_size: {e}")))?;
        let bytes = encode_to_vec(&v).map_err(|e| TestCaseError::fail(format!("encode_to_vec: {e}")))?;
        prop_assert_eq!(size, bytes.len());
    }

    // 18. Option<u32> roundtrip
    #[test]
    fn prop_struct_option_u32_roundtrip(v: Option<u32>) {
        roundtrip_val(v)?;
    }

    // 19. Struct with Option<String> roundtrip
    #[test]
    fn prop_struct_opt_str_roundtrip(name: Option<String>, value: u32) {
        let s = StructOptStr { name, value };
        roundtrip_val(s)?;
    }

    // 20. Vec<Option<u32>> roundtrip
    #[test]
    fn prop_struct_vec_option_u32_roundtrip(
        v in proptest::collection::vec(proptest::option::of(0u32..u32::MAX), 0..64)
    ) {
        roundtrip_val(v)?;
    }
}
