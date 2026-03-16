//! Advanced tests for NonZero integer types: configs, collections, Option, structs, wire format.

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
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// 1. NonZeroU8 min value (1) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u8_min_roundtrip() {
    let v = NonZeroU8::new(1).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroU8, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 2. NonZeroU8 max value (255) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u8_max_roundtrip() {
    let v = NonZeroU8::new(255).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroU8, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 3. NonZeroU16 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u16_roundtrip() {
    let v = NonZeroU16::new(12345).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroU16, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 4. NonZeroU32 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u32_roundtrip() {
    let v = NonZeroU32::new(42).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroU32, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 5. NonZeroU64 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u64_roundtrip() {
    let v = NonZeroU64::new(u64::MAX / 3 + 7).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroU64, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 6. NonZeroU128 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u128_roundtrip() {
    let v = NonZeroU128::new(u128::MAX - 99).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroU128, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 7. NonZeroUsize roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_usize_roundtrip() {
    let v = NonZeroUsize::new(4096).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroUsize, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 8. NonZeroI8 roundtrip (positive)
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_i8_positive_roundtrip() {
    let v = NonZeroI8::new(99).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroI8, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 9. NonZeroI8 roundtrip (negative)
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_i8_negative_roundtrip() {
    let v = NonZeroI8::new(-55).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroI8, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 10. NonZeroI16 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_i16_roundtrip() {
    let v = NonZeroI16::new(-30000).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroI16, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 11. NonZeroI32 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_i32_roundtrip() {
    let v = NonZeroI32::new(2_000_000_007).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroI32, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 12. NonZeroI64 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_i64_roundtrip() {
    let v = NonZeroI64::new(-1_000_000_000_000_i64).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroI64, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 13. NonZeroI128 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_i128_roundtrip() {
    let v = NonZeroI128::new(i128::MIN + 1).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroI128, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 14. NonZeroIsize roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_isize_roundtrip() {
    let v = NonZeroIsize::new(isize::MAX).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (NonZeroIsize, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 15. Vec<NonZeroU32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_vec_nonzero_u32_roundtrip() {
    let v: Vec<NonZeroU32> = [1u32, 10, 100, 1_000, 1_000_000]
        .iter()
        .map(|&n| NonZeroU32::new(n).expect("nonzero"))
        .collect();
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (Vec<NonZeroU32>, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 16. Option<NonZeroU32> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_option_nonzero_u32_some_roundtrip() {
    let v: Option<NonZeroU32> = Some(NonZeroU32::new(42).expect("nonzero"));
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (Option<NonZeroU32>, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 17. Option<NonZeroU32> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_option_nonzero_u32_none_roundtrip() {
    let v: Option<NonZeroU32> = None;
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (Option<NonZeroU32>, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 18. Struct with NonZero fields roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Adv2NonZeroStruct {
    id: NonZeroU32,
    priority: NonZeroU8,
    score: NonZeroI64,
}

#[test]
fn test_adv2_struct_nonzero_fields_roundtrip() {
    let original = Adv2NonZeroStruct {
        id: NonZeroU32::new(99999).expect("nonzero"),
        priority: NonZeroU8::new(7).expect("nonzero"),
        score: NonZeroI64::new(-500).expect("nonzero"),
    };
    let bytes = encode_to_vec(&original).expect("encode");
    let (decoded, _bytes): (Adv2NonZeroStruct, usize) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 19. NonZeroU32 fixed-int encoding (exactly 4 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u32_fixed_int_exactly_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let v = NonZeroU32::new(42).expect("nonzero");
    let bytes = encode_to_vec_with_config(&v, cfg).expect("encode");
    assert_eq!(
        bytes.len(),
        4,
        "NonZeroU32 fixed-int must be exactly 4 bytes"
    );
    let (decoded, _bytes): (NonZeroU32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 20. NonZeroU64 big-endian config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u64_big_endian_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let v = NonZeroU64::new(0xDEAD_BEEF_CAFE_F00D).expect("nonzero");
    let bytes = encode_to_vec_with_config(&v, cfg).expect("encode");
    let (decoded, _bytes): (NonZeroU64, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 21. NonZeroU32 wire matches raw u32
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_nonzero_u32_wire_matches_raw_u32() {
    let raw: u32 = 777;
    let nonzero = NonZeroU32::new(raw).expect("nonzero");

    let raw_bytes = encode_to_vec(&raw).expect("raw encode");
    let nz_bytes = encode_to_vec(&nonzero).expect("nonzero encode");

    assert_eq!(
        raw_bytes, nz_bytes,
        "NonZeroU32 wire format must match plain u32 wire format"
    );

    // Also verify we can decode the NonZeroU32 bytes as a plain u32 and vice versa.
    let (decoded_raw, _bytes): (u32, usize) = decode_from_slice(&nz_bytes).expect("decode u32");
    assert_eq!(decoded_raw, raw);

    let (decoded_nz, _bytes): (NonZeroU32, usize) =
        decode_from_slice(&raw_bytes).expect("decode nonzero");
    assert_eq!(decoded_nz, nonzero);
}

// ---------------------------------------------------------------------------
// 22. Vec<Option<NonZeroU8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_adv2_vec_option_nonzero_u8_roundtrip() {
    let v: Vec<Option<NonZeroU8>> = vec![
        Some(NonZeroU8::new(1).expect("nonzero")),
        None,
        Some(NonZeroU8::new(128).expect("nonzero")),
        None,
        Some(NonZeroU8::new(255).expect("nonzero")),
    ];
    let bytes = encode_to_vec(&v).expect("encode");
    let (decoded, _bytes): (Vec<Option<NonZeroU8>>, usize) =
        decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}
