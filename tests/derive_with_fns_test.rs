//! Tests for `#[oxicode(encode_with = "fn")]` and `#[oxicode(decode_with = "fn")]`
//! field-level attributes for asymmetric custom encode/decode functions.

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

// ---------------------------------------------------------------------------
// Custom encode/decode helpers
// ---------------------------------------------------------------------------

/// Encodes a `u32` as a `u64` on the wire; decodes a `u64` back to `u32`.
/// In fixed-int encoding this produces a 4-byte difference (u64 = 8B, u32 = 4B).
mod encode_u32_as_u64 {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };

    pub fn encode<E: Encoder>(value: &u32, encoder: &mut E) -> Result<(), Error> {
        (*value as u64).encode(encoder)
    }

    pub fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
        let v = u64::decode(decoder)?;
        Ok(v as u32)
    }
}

/// Encodes a `String` as its byte-length (u32); decodes a `u32` into a string
/// of that many `'x'` characters. The roundtrip is intentionally lossy to
/// demonstrate that encode_with and decode_with can be genuinely asymmetric.
mod string_as_len_marker {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };

    #[allow(clippy::ptr_arg)]
    pub fn encode<E: Encoder>(value: &String, encoder: &mut E) -> Result<(), Error> {
        (value.len() as u32).encode(encoder)
    }

    pub fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<String, Error> {
        let len = u32::decode(decoder)?;
        Ok("x".repeat(len as usize))
    }
}

// ---------------------------------------------------------------------------
// Named struct with encode_with + decode_with
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithU32AsU64 {
    #[oxicode(
        encode_with = "encode_u32_as_u64::encode",
        decode_with = "encode_u32_as_u64::decode"
    )]
    value: u32,
    name: String,
}

#[test]
fn test_encode_with_decode_with_named_struct_roundtrip() {
    let v = WithU32AsU64 {
        value: 42,
        name: "hello".into(),
    };
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (WithU32AsU64, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

#[test]
fn test_encode_with_produces_wider_wire_format() {
    // Use fixed-int encoding so integer widths are predictable:
    //   u32 → 4 bytes, u64 → 8 bytes (4-byte difference).
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PlainU32 {
        value: u32,
        name: String,
    }

    let config = oxicode::config::legacy();

    let custom = WithU32AsU64 {
        value: 1,
        name: "".into(),
    };
    let plain = PlainU32 {
        value: 1,
        name: "".into(),
    };

    let custom_bytes = oxicode::encode_to_vec_with_config(&custom, config).expect("encode custom");
    let plain_bytes = oxicode::encode_to_vec_with_config(&plain, config).expect("encode plain");

    // u64 is 4 bytes wider than u32 in fixed-int encoding.
    assert_eq!(
        custom_bytes.len(),
        plain_bytes.len() + 4,
        "encode_with as u64 should be 4 bytes longer than plain u32 (fixed-int): \
         custom={:?} plain={:?}",
        custom_bytes,
        plain_bytes
    );
}

// ---------------------------------------------------------------------------
// Named struct with asymmetric encode_with / decode_with (lossy)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithStringLenMarker {
    #[oxicode(
        encode_with = "string_as_len_marker::encode",
        decode_with = "string_as_len_marker::decode"
    )]
    label: String,
    id: u64,
}

#[test]
fn test_asymmetric_encode_decode_with() {
    let v = WithStringLenMarker {
        label: "hello".into(), // len=5, encoded as u32(5), decoded back as "xxxxx"
        id: 99,
    };
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (WithStringLenMarker, _) =
        oxicode::decode_from_slice(&bytes).expect("decode");
    // Label decoded as 5 'x' chars matching the length of "hello"
    assert_eq!(decoded.label, "xxxxx");
    assert_eq!(decoded.id, 99);
}

// ---------------------------------------------------------------------------
// Tuple struct with encode_with + decode_with
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct TupleWithCustom(
    #[oxicode(
        encode_with = "encode_u32_as_u64::encode",
        decode_with = "encode_u32_as_u64::decode"
    )]
    u32,
    String,
);

#[test]
fn test_encode_with_decode_with_tuple_struct_roundtrip() {
    let v = TupleWithCustom(7, "world".into());
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (TupleWithCustom, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// Enum: named variant fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum EventNamed {
    Created {
        #[oxicode(
            encode_with = "encode_u32_as_u64::encode",
            decode_with = "encode_u32_as_u64::decode"
        )]
        timestamp: u32,
        label: String,
    },
    Deleted,
}

#[test]
fn test_encode_with_decode_with_enum_named_field_roundtrip() {
    let vals = vec![
        EventNamed::Created {
            timestamp: 1234,
            label: "created".into(),
        },
        EventNamed::Deleted,
    ];
    for v in vals {
        let bytes = oxicode::encode_to_vec(&v).expect("encode");
        let (decoded, _): (EventNamed, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Enum: unnamed (tuple) variant fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum EventUnnamed {
    Value(
        #[oxicode(
            encode_with = "encode_u32_as_u64::encode",
            decode_with = "encode_u32_as_u64::decode"
        )]
        u32,
    ),
    Empty,
}

#[test]
fn test_encode_with_decode_with_enum_unnamed_field_roundtrip() {
    let vals = vec![EventUnnamed::Value(9999), EventUnnamed::Empty];
    for v in vals {
        let bytes = oxicode::encode_to_vec(&v).expect("encode");
        let (decoded, _): (EventUnnamed, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Multiple encode_with/decode_with fields in a single struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiCustom {
    #[oxicode(
        encode_with = "encode_u32_as_u64::encode",
        decode_with = "encode_u32_as_u64::decode"
    )]
    a: u32,
    normal: String,
    #[oxicode(
        encode_with = "encode_u32_as_u64::encode",
        decode_with = "encode_u32_as_u64::decode"
    )]
    b: u32,
}

#[test]
fn test_multiple_encode_decode_with_fields_roundtrip() {
    let v = MultiCustom {
        a: 1,
        normal: "test".into(),
        b: 2,
    };
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (MultiCustom, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

#[test]
fn test_multiple_encode_with_fields_wider_wire_format() {
    // Use fixed-int encoding so integer widths are predictable.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PlainMulti {
        a: u32,
        normal: String,
        b: u32,
    }

    let config = oxicode::config::legacy();

    let custom = MultiCustom {
        a: 1,
        normal: "".into(),
        b: 2,
    };
    let plain = PlainMulti {
        a: 1,
        normal: "".into(),
        b: 2,
    };

    let custom_bytes = oxicode::encode_to_vec_with_config(&custom, config).expect("encode custom");
    let plain_bytes = oxicode::encode_to_vec_with_config(&plain, config).expect("encode plain");

    // Two u32→u64 fields each add 4 bytes → total 8 bytes wider.
    assert_eq!(
        custom_bytes.len(),
        plain_bytes.len() + 8,
        "two encode_with u32→u64 fields should add 8 bytes total (fixed-int)"
    );
}
