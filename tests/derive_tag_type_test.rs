//! Tests for the `#[oxicode(tag_type = "...")]` container attribute.
//! Verifies that enum discriminants are encoded with the specified width.
//!
//! Size assertions use the `legacy()` config (fixed-int encoding) so the
//! discriminant bytes are predictable. Roundtrip tests use the default
//! standard (varint) config to verify correctness with the common config.

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
use oxicode::{Decode, Encode};

/// Encode using the legacy fixed-int config and return the raw bytes.
/// In fixed-int mode every integer is written at its natural width:
///   u8 → 1 byte, u16 → 2 bytes, u32 → 4 bytes, u64 → 8 bytes.
fn encode_fixed<T: Encode>(val: &T) -> Vec<u8> {
    oxicode::encode_to_vec_with_config(val, oxicode::config::legacy()).expect("encode_fixed")
}

/// Encode using the standard (varint) config.
fn encode_bytes<T: Encode>(val: &T) -> Vec<u8> {
    oxicode::encode_to_vec(val).expect("encode")
}

/// Decode (standard config).
fn decode_val<T: Decode>(bytes: &[u8]) -> T {
    let (val, _) = oxicode::decode_from_slice(bytes).expect("decode");
    val
}

/// Decode with legacy (fixed-int) config.
fn decode_fixed<T: Decode>(bytes: &[u8]) -> T {
    let (val, _) = oxicode::decode_from_slice_with_config(bytes, oxicode::config::legacy())
        .expect("decode_fixed");
    val
}

// ---------------------------------------------------------------------------
// u8 tag type
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8Enum {
    A,
    B,
    C,
}

#[test]
fn test_tag_type_u8_discriminant_size() {
    // Fixed-int encoding: u8 discriminant → 1 byte for a unit variant.
    let bytes = encode_fixed(&U8Enum::A);
    assert_eq!(
        bytes.len(),
        1,
        "u8 tag_type: discriminant should be 1 byte (fixed), got: {:?}",
        bytes
    );
    assert_eq!(bytes[0], 0u8);

    let bytes_b = encode_fixed(&U8Enum::B);
    assert_eq!(bytes_b.len(), 1);
    assert_eq!(bytes_b[0], 1u8);

    let bytes_c = encode_fixed(&U8Enum::C);
    assert_eq!(bytes_c.len(), 1);
    assert_eq!(bytes_c[0], 2u8);
}

#[test]
fn test_tag_type_u8_roundtrip() {
    for val in [U8Enum::A, U8Enum::B, U8Enum::C] {
        let bytes = encode_bytes(&val);
        let decoded: U8Enum = decode_val(&bytes);
        assert_eq!(val, decoded);
    }
}

#[test]
fn test_tag_type_u8_roundtrip_fixed() {
    for val in [U8Enum::A, U8Enum::B, U8Enum::C] {
        let bytes = encode_fixed(&val);
        let decoded: U8Enum = decode_fixed(&bytes);
        assert_eq!(val, decoded);
    }
}

// ---------------------------------------------------------------------------
// u16 tag type
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum U16Enum {
    First,
    Second { x: u32, y: u32 },
    Third(String),
}

#[test]
fn test_tag_type_u16_discriminant_size() {
    // Fixed-int encoding: u16 discriminant → 2 bytes for a unit variant.
    let bytes = encode_fixed(&U16Enum::First);
    assert_eq!(
        bytes.len(),
        2,
        "u16 tag_type: discriminant should be 2 bytes (fixed), got: {:?}",
        bytes
    );
}

#[test]
fn test_tag_type_u16_roundtrip() {
    let vals = vec![
        U16Enum::First,
        U16Enum::Second { x: 42, y: 99 },
        U16Enum::Third("hello".into()),
    ];
    for val in vals {
        let bytes = encode_bytes(&val);
        let decoded: U16Enum = decode_val(&bytes);
        assert_eq!(val, decoded);
    }
}

#[test]
fn test_tag_type_u16_roundtrip_fixed() {
    let vals = vec![
        U16Enum::First,
        U16Enum::Second { x: 42, y: 99 },
        U16Enum::Third("hello".into()),
    ];
    for val in vals {
        let bytes = encode_fixed(&val);
        let decoded: U16Enum = decode_fixed(&bytes);
        assert_eq!(val, decoded);
    }
}

// ---------------------------------------------------------------------------
// u32 tag type (explicit)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u32")]
enum U32Enum {
    Only,
}

#[test]
fn test_tag_type_u32_discriminant_size() {
    // Fixed-int encoding: u32 discriminant → 4 bytes for a unit variant.
    let bytes = encode_fixed(&U32Enum::Only);
    assert_eq!(
        bytes.len(),
        4,
        "u32 tag_type: discriminant should be 4 bytes (fixed), got: {:?}",
        bytes
    );
}

#[test]
fn test_tag_type_u32_roundtrip() {
    let val = U32Enum::Only;
    let bytes = encode_bytes(&val);
    let decoded: U32Enum = decode_val(&bytes);
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// u64 tag type
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u64")]
enum U64Enum {
    Alpha,
    Beta(i64),
}

#[test]
fn test_tag_type_u64_discriminant_size() {
    // Fixed-int encoding: u64 discriminant → 8 bytes for a unit variant.
    let bytes = encode_fixed(&U64Enum::Alpha);
    assert_eq!(
        bytes.len(),
        8,
        "u64 tag_type: discriminant should be 8 bytes (fixed), got: {:?}",
        bytes
    );
}

#[test]
fn test_tag_type_u64_roundtrip() {
    let vals = vec![U64Enum::Alpha, U64Enum::Beta(-1234)];
    for val in vals {
        let bytes = encode_bytes(&val);
        let decoded: U64Enum = decode_val(&bytes);
        assert_eq!(val, decoded);
    }
}

#[test]
fn test_tag_type_u64_roundtrip_fixed() {
    let vals = vec![U64Enum::Alpha, U64Enum::Beta(-1234)];
    for val in vals {
        let bytes = encode_fixed(&val);
        let decoded: U64Enum = decode_fixed(&bytes);
        assert_eq!(val, decoded);
    }
}

// ---------------------------------------------------------------------------
// Size comparison (fixed-int encoding to get predictable widths)
// ---------------------------------------------------------------------------

#[test]
fn test_tag_type_size_comparison() {
    // In fixed-int mode: u8 → 1 byte, u32 → 4 bytes, u64 → 8 bytes for the discriminant.
    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u8")]
    enum SmallTag {
        X,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u32")]
    enum DefaultTag {
        X,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u64")]
    enum LargeTag {
        X,
    }

    let small = encode_fixed(&SmallTag::X);
    let default_size = encode_fixed(&DefaultTag::X);
    let large = encode_fixed(&LargeTag::X);

    assert_eq!(small.len(), 1);
    assert_eq!(default_size.len(), 4);
    assert_eq!(large.len(), 8);
    assert!(small.len() < default_size.len());
    assert!(default_size.len() < large.len());
}

// ---------------------------------------------------------------------------
// Combined with custom variant tags (#[oxicode(variant = N)])
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum CustomTagU8 {
    #[oxicode(variant = 10)]
    First,
    #[oxicode(variant = 20)]
    Second,
}

#[test]
fn test_tag_type_u8_with_custom_variant_discriminant() {
    let bytes = encode_fixed(&CustomTagU8::First);
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0], 10u8);

    let bytes_s = encode_fixed(&CustomTagU8::Second);
    assert_eq!(bytes_s.len(), 1);
    assert_eq!(bytes_s[0], 20u8);
}

#[test]
fn test_tag_type_u8_custom_variant_roundtrip() {
    let vals = [CustomTagU8::First, CustomTagU8::Second];
    for val in vals {
        let bytes = encode_bytes(&val);
        let decoded: CustomTagU8 = decode_val(&bytes);
        assert_eq!(val, decoded);
    }
}

// ---------------------------------------------------------------------------
// Enum with data fields and u16 tag
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum U16WithData {
    Empty,
    Named { a: u64, b: String },
    Tuple(u32, bool),
}

#[test]
fn test_tag_type_u16_with_data_roundtrip() {
    let vals = vec![
        U16WithData::Empty,
        U16WithData::Named {
            a: 999,
            b: "oxicode".into(),
        },
        U16WithData::Tuple(42, true),
    ];
    for val in vals {
        let bytes = encode_bytes(&val);
        let decoded: U16WithData = decode_val(&bytes);
        assert_eq!(val, decoded);
    }
}

// ---------------------------------------------------------------------------
// Default (no tag_type attr) still uses u32 — 4 bytes in fixed-int encoding
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum DefaultTagType {
    Only,
}

#[test]
fn test_default_tag_type_is_u32() {
    // In fixed-int encoding the default u32 discriminant must be exactly 4 bytes.
    let bytes = encode_fixed(&DefaultTagType::Only);
    assert_eq!(
        bytes.len(),
        4,
        "default tag_type should be 4 bytes (u32 fixed), got: {:?}",
        bytes
    );
}

// ---------------------------------------------------------------------------
// Verify varint roundtrip also works for all tag types
// ---------------------------------------------------------------------------

#[test]
fn test_tag_type_varint_roundtrip_all() {
    // All tag types should roundtrip correctly with the default varint config.
    let a_bytes = encode_bytes(&U8Enum::A);
    assert_eq!(decode_val::<U8Enum>(&a_bytes), U8Enum::A);

    let first_bytes = encode_bytes(&U16Enum::First);
    assert_eq!(decode_val::<U16Enum>(&first_bytes), U16Enum::First);

    let only_bytes = encode_bytes(&U32Enum::Only);
    assert_eq!(decode_val::<U32Enum>(&only_bytes), U32Enum::Only);

    let alpha_bytes = encode_bytes(&U64Enum::Alpha);
    assert_eq!(decode_val::<U64Enum>(&alpha_bytes), U64Enum::Alpha);
}
