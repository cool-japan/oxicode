//! Advanced tests for unit struct and zero-sized type serialization in OxiCode.
//!
//! Unit structs encode as 0 bytes. ZSTs wrapping `()` also encode as 0 bytes.
//! Structs that contain unit struct fields treat those fields as contributing nothing
//! to the encoded size.

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
    encode_to_vec_with_config, encoded_size, Decode, Encode,
};

// ── Type definitions ──────────────────────────────────────────────────────────

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct UnitA;

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct UnitB;

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct EmptyStruct {}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct ZstWrapper(());

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct StructWithUnit {
    marker: UnitA,
    value: u32,
}

// Additional types for tests 21 and 22
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct ThreeUnits {
    a: UnitA,
    b: UnitB,
    c: EmptyStruct,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct OuterWithUnit {
    inner: StructWithUnit,
    tag: UnitB,
}

// ── Test 1: UnitA roundtrip ───────────────────────────────────────────────────

#[test]
fn test_unit_a_roundtrip() {
    let original = UnitA;
    let encoded = encode_to_vec(&original).expect("encode UnitA");
    let (decoded, _): (UnitA, _) = decode_from_slice(&encoded).expect("decode UnitA");
    assert_eq!(original, decoded);
}

// ── Test 2: UnitA encoded size is 0 ──────────────────────────────────────────

#[test]
fn test_unit_a_encoded_size_is_zero() {
    let size = encoded_size(&UnitA).expect("encoded_size UnitA");
    assert_eq!(size, 0, "UnitA must encode to 0 bytes");
}

// ── Test 3: UnitA consumed == 0 ──────────────────────────────────────────────

#[test]
fn test_unit_a_decode_consumes_zero_bytes() {
    let encoded = encode_to_vec(&UnitA).expect("encode UnitA");
    let (_decoded, consumed): (UnitA, _) = decode_from_slice(&encoded).expect("decode UnitA");
    assert_eq!(consumed, 0, "UnitA decode must consume 0 bytes");
}

// ── Test 4: UnitB roundtrip ───────────────────────────────────────────────────

#[test]
fn test_unit_b_roundtrip() {
    let original = UnitB;
    let encoded = encode_to_vec(&original).expect("encode UnitB");
    let (decoded, _): (UnitB, _) = decode_from_slice(&encoded).expect("decode UnitB");
    assert_eq!(original, decoded);
}

// ── Test 5: EmptyStruct {} roundtrip ─────────────────────────────────────────

#[test]
fn test_empty_struct_roundtrip() {
    let original = EmptyStruct {};
    let encoded = encode_to_vec(&original).expect("encode EmptyStruct");
    let (decoded, _): (EmptyStruct, _) = decode_from_slice(&encoded).expect("decode EmptyStruct");
    assert_eq!(original, decoded);
}

// ── Test 6: EmptyStruct encoded size is 0 ────────────────────────────────────

#[test]
fn test_empty_struct_encoded_size_is_zero() {
    let size = encoded_size(&EmptyStruct {}).expect("encoded_size EmptyStruct");
    assert_eq!(size, 0, "EmptyStruct must encode to 0 bytes");
}

// ── Test 7: Vec<UnitA> with 3 elements — length prefix only ──────────────────

#[test]
fn test_vec_unit_a_three_elements_length_prefix_only() {
    let original: Vec<UnitA> = vec![UnitA, UnitA, UnitA];
    let encoded = encode_to_vec(&original).expect("encode Vec<UnitA> (3 elements)");
    // varint(3) = 1 byte; each UnitA = 0 bytes → total = 1 byte
    assert_eq!(
        encoded.len(),
        1,
        "Vec<UnitA> with 3 elements must be 1 byte (length prefix only)"
    );
    assert_eq!(encoded[0], 3u8, "varint for 3 elements must be 0x03");
}

// ── Test 8: Option<UnitA> Some ────────────────────────────────────────────────

#[test]
fn test_option_unit_a_some_roundtrip() {
    let original: Option<UnitA> = Some(UnitA);
    let encoded = encode_to_vec(&original).expect("encode Some(UnitA)");
    // Option discriminant: 1 byte; UnitA payload: 0 bytes → 1 byte total
    assert_eq!(encoded.len(), 1, "Some(UnitA) must encode to 1 byte");
    let (decoded, _): (Option<UnitA>, _) = decode_from_slice(&encoded).expect("decode Some(UnitA)");
    assert_eq!(original, decoded);
}

// ── Test 9: Option<UnitA> None ────────────────────────────────────────────────

#[test]
fn test_option_unit_a_none_roundtrip() {
    let original: Option<UnitA> = None;
    let encoded = encode_to_vec(&original).expect("encode None::<UnitA>");
    assert_eq!(encoded.len(), 1, "None::<UnitA> must encode to 1 byte");
    assert_eq!(encoded[0], 0u8, "None discriminant must be 0");
    let (decoded, _): (Option<UnitA>, _) =
        decode_from_slice(&encoded).expect("decode None::<UnitA>");
    assert_eq!(original, decoded);
}

// ── Test 10: ZstWrapper(()) roundtrip ────────────────────────────────────────

#[test]
fn test_zst_wrapper_roundtrip() {
    let original = ZstWrapper(());
    let encoded = encode_to_vec(&original).expect("encode ZstWrapper(())");
    assert!(encoded.is_empty(), "ZstWrapper(()) must encode to 0 bytes");
    let (decoded, _): (ZstWrapper, _) = decode_from_slice(&encoded).expect("decode ZstWrapper(())");
    assert_eq!(original, decoded);
}

// ── Test 11: Vec<ZstWrapper> roundtrip ───────────────────────────────────────

#[test]
fn test_vec_zst_wrapper_roundtrip() {
    let original: Vec<ZstWrapper> = vec![ZstWrapper(()), ZstWrapper(())];
    let encoded = encode_to_vec(&original).expect("encode Vec<ZstWrapper>");
    // varint(2) = 1 byte; each ZstWrapper = 0 bytes → total = 1 byte
    assert_eq!(
        encoded.len(),
        1,
        "Vec<ZstWrapper> with 2 elements must be 1 byte"
    );
    let (decoded, _): (Vec<ZstWrapper>, _) =
        decode_from_slice(&encoded).expect("decode Vec<ZstWrapper>");
    assert_eq!(original, decoded);
}

// ── Test 12: StructWithUnit roundtrip ────────────────────────────────────────

#[test]
fn test_struct_with_unit_roundtrip() {
    let original = StructWithUnit {
        marker: UnitA,
        value: 99,
    };
    let encoded = encode_to_vec(&original).expect("encode StructWithUnit");
    let (decoded, _): (StructWithUnit, _) =
        decode_from_slice(&encoded).expect("decode StructWithUnit");
    assert_eq!(original, decoded);
}

// ── Test 13: StructWithUnit — encoded size equals u32 size ───────────────────

#[test]
fn test_struct_with_unit_encoded_size_equals_u32() {
    let original = StructWithUnit {
        marker: UnitA,
        value: 99,
    };
    let struct_size = encoded_size(&original).expect("encoded_size StructWithUnit");
    let u32_size = encoded_size(&99u32).expect("encoded_size u32");
    assert_eq!(
        struct_size, u32_size,
        "unit field contributes nothing; StructWithUnit size must equal its u32 field size"
    );
}

// ── Test 14: Fixed-int config with UnitA ─────────────────────────────────────

#[test]
fn test_unit_a_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&UnitA, cfg).expect("encode UnitA fixed-int");
    assert!(
        encoded.is_empty(),
        "UnitA must encode to 0 bytes even with fixed-int config"
    );
    let (decoded, _): (UnitA, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode UnitA fixed-int");
    assert_eq!(UnitA, decoded);
}

// ── Test 15: Big-endian config with UnitA ────────────────────────────────────

#[test]
fn test_unit_a_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&UnitA, cfg).expect("encode UnitA big-endian");
    assert!(
        encoded.is_empty(),
        "UnitA must encode to 0 bytes even with big-endian config"
    );
    let (decoded, _): (UnitA, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode UnitA big-endian");
    assert_eq!(UnitA, decoded);
}

// ── Test 16: (UnitA, UnitA) tuple of unit structs ────────────────────────────

#[test]
fn test_tuple_unit_a_unit_a_encodes_zero_bytes() {
    let original = (UnitA, UnitA);
    let encoded = encode_to_vec(&original).expect("encode (UnitA, UnitA)");
    assert!(encoded.is_empty(), "(UnitA, UnitA) must encode to 0 bytes");
    let (decoded, _): ((UnitA, UnitA), _) =
        decode_from_slice(&encoded).expect("decode (UnitA, UnitA)");
    assert_eq!(original, decoded);
}

// ── Test 17: [UnitA; 5] array of unit structs ─────────────────────────────────

#[test]
fn test_fixed_array_unit_a_five_encodes_zero_bytes() {
    // Fixed-size arrays have no length prefix; each UnitA = 0 bytes → total = 0
    let original: [UnitA; 5] = [UnitA, UnitA, UnitA, UnitA, UnitA];
    let encoded = encode_to_vec(&original).expect("encode [UnitA; 5]");
    assert!(
        encoded.is_empty(),
        "[UnitA; 5] must encode to 0 bytes (no length prefix, each element is 0 bytes)"
    );
    let (decoded, _): ([UnitA; 5], _) = decode_from_slice(&encoded).expect("decode [UnitA; 5]");
    assert_eq!(original, decoded);
}

// ── Test 18: UnitA consumed + UnitB consumed = total for (UnitA, UnitB) ──────

#[test]
fn test_tuple_unit_a_unit_b_consumed_is_sum_of_parts() {
    let encoded_a = encode_to_vec(&UnitA).expect("encode UnitA");
    let encoded_b = encode_to_vec(&UnitB).expect("encode UnitB");
    let encoded_tuple = encode_to_vec(&(UnitA, UnitB)).expect("encode (UnitA, UnitB)");

    let (_, consumed_a): (UnitA, _) = decode_from_slice(&encoded_a).expect("decode UnitA");
    let (_, consumed_b): (UnitB, _) = decode_from_slice(&encoded_b).expect("decode UnitB");
    let (_, consumed_tuple): ((UnitA, UnitB), _) =
        decode_from_slice(&encoded_tuple).expect("decode (UnitA, UnitB)");

    assert_eq!(
        consumed_a + consumed_b,
        consumed_tuple,
        "sum of individual consumed bytes must equal tuple consumed bytes"
    );
}

// ── Test 19: Vec<UnitA> with 100 elements — only length prefix ───────────────

#[test]
fn test_vec_unit_a_hundred_elements_length_prefix_only() {
    let original: Vec<UnitA> = vec![UnitA; 100];
    let encoded = encode_to_vec(&original).expect("encode Vec<UnitA> (100 elements)");
    // varint(100) = 1 byte (100 < 128); each UnitA = 0 bytes → total = 1 byte
    assert_eq!(
        encoded.len(),
        1,
        "Vec<UnitA> with 100 elements must be 1 byte (varint length prefix only)"
    );
    assert_eq!(encoded[0], 100u8, "varint for 100 elements must be 0x64");
}

// ── Test 20: Option<EmptyStruct> Some ────────────────────────────────────────

#[test]
fn test_option_empty_struct_some_roundtrip() {
    let original: Option<EmptyStruct> = Some(EmptyStruct {});
    let encoded = encode_to_vec(&original).expect("encode Some(EmptyStruct)");
    // Option discriminant: 1 byte; EmptyStruct payload: 0 bytes → 1 byte total
    assert_eq!(encoded.len(), 1, "Some(EmptyStruct) must encode to 1 byte");
    let (decoded, _): (Option<EmptyStruct>, _) =
        decode_from_slice(&encoded).expect("decode Some(EmptyStruct)");
    assert_eq!(original, decoded);
}

// ── Test 21: Struct with 3 unit fields ───────────────────────────────────────

#[test]
fn test_three_unit_fields_encodes_zero_bytes() {
    let original = ThreeUnits {
        a: UnitA,
        b: UnitB,
        c: EmptyStruct {},
    };
    let encoded = encode_to_vec(&original).expect("encode ThreeUnits");
    assert!(
        encoded.is_empty(),
        "ThreeUnits (all unit fields) must encode to 0 bytes"
    );
    let (decoded, _): (ThreeUnits, _) = decode_from_slice(&encoded).expect("decode ThreeUnits");
    assert_eq!(original, decoded);
}

// ── Test 22: Nested struct with UnitA field ───────────────────────────────────

#[test]
fn test_nested_struct_with_unit_field_roundtrip() {
    let original = OuterWithUnit {
        inner: StructWithUnit {
            marker: UnitA,
            value: 42,
        },
        tag: UnitB,
    };
    let encoded = encode_to_vec(&original).expect("encode OuterWithUnit");
    // Only the u32 field (value: 42) contributes bytes; all unit fields contribute nothing
    let u32_size = encoded_size(&42u32).expect("encoded_size u32");
    assert_eq!(
        encoded.len(),
        u32_size,
        "OuterWithUnit encoded size must equal its single u32 field"
    );
    let (decoded, _): (OuterWithUnit, _) =
        decode_from_slice(&encoded).expect("decode OuterWithUnit");
    assert_eq!(original, decoded);
}
