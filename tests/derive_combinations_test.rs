//! Tests for complex combinations of derive attributes.
//!
//! Verifies that multiple `#[oxicode(...)]` attributes interact correctly when
//! applied together on the same struct/enum, including:
//! - `rename_all` + `skip` + `default_value`
//! - `tag_type` + `variant` together on enums
//! - `encode_with` + `decode_with` on multiple fields
//! - `bound` with generic structs
//! - `seq_len` with different width specifiers on multiple fields

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Test 1: rename_all + skip + default_value together
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct RenameWithSkip {
    user_name: String, // would be "userName" in serde, no-op on wire
    #[oxicode(skip, default_value = "99u32")]
    user_id: u32, // skipped on encode; restored as 99 on decode
}

#[test]
fn test_rename_all_with_skip() {
    let original = RenameWithSkip {
        user_name: "Alice".into(),
        user_id: 42,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (RenameWithSkip, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.user_name, "Alice");
    // user_id was skipped; default_value = "99u32" is used on decode
    assert_eq!(dec.user_id, 99);
}

#[test]
fn test_rename_all_with_skip_bytes_size() {
    // Encoding should only contain user_name (String), not user_id (u32).
    let with_skip = RenameWithSkip {
        user_name: "x".into(),
        user_id: u32::MAX,
    };

    #[derive(Encode)]
    struct NoSkip {
        user_name: String,
        user_id: u32,
    }

    let no_skip = NoSkip {
        user_name: "x".into(),
        user_id: u32::MAX,
    };

    let skip_len = encode_to_vec(&with_skip).expect("encode skip").len();
    let no_skip_len = encode_to_vec(&no_skip).expect("encode no_skip").len();
    assert!(
        skip_len < no_skip_len,
        "skipped user_id should produce a smaller encoding: {skip_len} vs {no_skip_len}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: tag_type + variant together on enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum MultiAttrEnum {
    #[oxicode(variant = 10)]
    Alpha,
    #[oxicode(variant = 20)]
    Beta(u32),
    Gamma {
        x: i32,
    },
}

#[test]
fn test_enum_tag_type_with_custom_variants_roundtrip() {
    let cases = [
        MultiAttrEnum::Alpha,
        MultiAttrEnum::Beta(42),
        MultiAttrEnum::Gamma { x: -5 },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, _): (MultiAttrEnum, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, &dec);
    }
}

#[test]
fn test_enum_custom_variant_uses_u8_discriminant() {
    // With legacy (fixed-int) config: u8 discriminant = 1 byte; FirstVariant = 10.
    let enc = oxicode::encode_to_vec_with_config(&MultiAttrEnum::Alpha, oxicode::config::legacy())
        .expect("encode");
    assert_eq!(enc.len(), 1, "u8 discriminant should be exactly 1 byte");
    assert_eq!(enc[0], 10u8, "Alpha discriminant value should be 10");
}

#[test]
fn test_enum_second_variant_u8_discriminant() {
    // SecondVariant (variant = 20) with a u32 payload.
    // Fixed-int: 1 byte discriminant + 4 bytes u32 payload = 5 bytes.
    let enc =
        oxicode::encode_to_vec_with_config(&MultiAttrEnum::Beta(0), oxicode::config::legacy())
            .expect("encode");
    assert_eq!(enc.len(), 5, "1 byte tag + 4 bytes u32 = 5 bytes");
    assert_eq!(enc[0], 20u8, "Beta discriminant value should be 20");
}

// ---------------------------------------------------------------------------
// Test 3: encode_with + decode_with on multiple fields
// ---------------------------------------------------------------------------

mod transforms {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };

    #[allow(clippy::ptr_arg)]
    pub fn encode_upper<E: Encoder>(s: &String, encoder: &mut E) -> Result<(), Error> {
        s.to_uppercase().encode(encoder)
    }

    pub fn decode_lower<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<String, Error> {
        Ok(String::decode(decoder)?.to_lowercase())
    }

    pub fn encode_doubled<E: Encoder>(n: &u32, encoder: &mut E) -> Result<(), Error> {
        (n * 2).encode(encoder)
    }

    pub fn decode_halved<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
        Ok(u32::decode(decoder)? / 2)
    }
}

#[derive(Debug, Encode, Decode)]
struct MultiTransform {
    #[oxicode(
        encode_with = "transforms::encode_upper",
        decode_with = "transforms::decode_lower"
    )]
    name: String,
    #[oxicode(
        encode_with = "transforms::encode_doubled",
        decode_with = "transforms::decode_halved"
    )]
    value: u32,
}

#[test]
fn test_multi_field_transforms() {
    let original = MultiTransform {
        name: "Hello".into(),
        value: 21,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (MultiTransform, _) = decode_from_slice(&enc).expect("decode");
    // "Hello" was encoded as "HELLO" (uppercase), decoded back as "hello" (lowercase)
    assert_eq!(dec.name, "hello");
    // 21 was encoded as 42 (doubled), decoded back as 21 (halved)
    assert_eq!(dec.value, 21);
}

#[test]
fn test_multi_field_transforms_wire_values() {
    // The string "test" uppercased → "TEST"; we can verify via a plain decode
    // of the raw bytes to confirm what was actually written on the wire.
    let original = MultiTransform {
        name: "abc".into(),
        value: 10,
    };
    let enc = encode_to_vec(&original).expect("encode");

    // Decode the name field directly as String (first field on wire is "ABC")
    // and the value as u32 (on wire = 20).
    // We use decode_from_slice to get a temporary struct that mirrors the wire.
    #[derive(Debug, Decode)]
    struct WireRepr {
        name: String,
        value: u32,
    }
    let (wire, _): (WireRepr, _) = decode_from_slice(&enc).expect("decode wire");
    assert_eq!(wire.name, "ABC", "wire should have uppercased name");
    assert_eq!(wire.value, 20, "wire should have doubled value");
}

// ---------------------------------------------------------------------------
// Test 4: generic struct with bound attribute
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode + Clone")]
struct BoundedGeneric<T> {
    value: T,
    backup: T,
}

#[test]
fn test_custom_bound_attribute_u32() {
    let g = BoundedGeneric {
        value: 42u32,
        backup: 99u32,
    };
    let enc = encode_to_vec(&g).expect("encode");
    let (dec, _): (BoundedGeneric<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(g, dec);
}

#[test]
fn test_custom_bound_attribute_string() {
    let g = BoundedGeneric {
        value: "hello".to_string(),
        backup: "world".to_string(),
    };
    let enc = encode_to_vec(&g).expect("encode");
    let (dec, _): (BoundedGeneric<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(g, dec);
}

#[test]
fn test_custom_bound_attribute_vec() {
    let g = BoundedGeneric {
        value: vec![1u8, 2, 3],
        backup: vec![4u8, 5, 6],
    };
    let enc = encode_to_vec(&g).expect("encode");
    let (dec, _): (BoundedGeneric<Vec<u8>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(g, dec);
}

// ---------------------------------------------------------------------------
// Test 5: seq_len with different widths on multiple fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiSeqLen {
    #[oxicode(seq_len = "u8")]
    small: Vec<u32>,
    #[oxicode(seq_len = "u16")]
    medium: Vec<u32>,
    #[oxicode(seq_len = "u32")]
    large: Vec<u32>,
}

#[test]
fn test_multi_seq_len_roundtrip() {
    let data = MultiSeqLen {
        small: vec![1, 2, 3],
        medium: vec![4, 5, 6],
        large: vec![7, 8, 9],
    };
    let enc = encode_to_vec(&data).expect("encode");
    let (dec, _): (MultiSeqLen, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(data, dec);
}

#[test]
fn test_multi_seq_len_byte_sizes_with_legacy_config() {
    let data = MultiSeqLen {
        small: vec![1, 2, 3],
        medium: vec![4, 5, 6],
        large: vec![7, 8, 9],
    };
    // legacy config = fixed-int encoding, so integer widths are exact:
    //   small:  1 byte (u8 len=3) + 3 * 4 bytes (u32 elements) = 13
    //   medium: 2 bytes (u16 len=3) + 3 * 4 bytes              = 14
    //   large:  4 bytes (u32 len=3) + 3 * 4 bytes              = 16
    //   total = 43 bytes
    let enc = oxicode::encode_to_vec_with_config(&data, oxicode::config::legacy()).expect("encode");
    assert_eq!(
        enc.len(),
        43,
        "Expected 43 bytes with legacy config, got {}",
        enc.len()
    );
}

#[test]
fn test_multi_seq_len_empty_vecs() {
    let data = MultiSeqLen {
        small: vec![],
        medium: vec![],
        large: vec![],
    };
    let enc = encode_to_vec(&data).expect("encode");
    let (dec, _): (MultiSeqLen, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(data, dec);
}

#[test]
fn test_multi_seq_len_different_element_counts() {
    let data = MultiSeqLen {
        small: (0u32..10).collect(),
        medium: (100u32..200).collect(),
        large: (1000u32..1050).collect(),
    };
    let enc = encode_to_vec(&data).expect("encode");
    let (dec, _): (MultiSeqLen, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// Test 6: Combined rename_all + tag_type + variant
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16", rename_all = "SCREAMING_SNAKE_CASE")]
enum ComplexEnum {
    #[oxicode(variant = 100)]
    Ping,
    #[oxicode(variant = 200)]
    Push(String),
    Pull {
        count: u32,
    },
}

#[test]
fn test_complex_enum_roundtrip() {
    let cases = [
        ComplexEnum::Ping,
        ComplexEnum::Push("hello".into()),
        ComplexEnum::Pull { count: 42 },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, _): (ComplexEnum, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, &dec);
    }
}

#[test]
fn test_complex_enum_u16_discriminant_size() {
    // Fixed-int: u16 discriminant = 2 bytes; FirstItem has no payload.
    let enc = oxicode::encode_to_vec_with_config(&ComplexEnum::Ping, oxicode::config::legacy())
        .expect("encode");
    assert_eq!(
        enc.len(),
        2,
        "u16 discriminant should be 2 bytes in legacy mode"
    );
    // Little-endian: value 100 = [100, 0]
    assert_eq!(
        u16::from_le_bytes([enc[0], enc[1]]),
        100u16,
        "FirstItem discriminant should be 100"
    );
}

// ---------------------------------------------------------------------------
// Test 7: skip + default_value on multiple fields simultaneously
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiSkipDefault {
    id: u32,
    #[oxicode(skip, default_value = "255u8")]
    flags: u8,
    name: String,
    #[oxicode(skip, default_value = r#""anonymous".to_string()"#)]
    author: String,
    score: f32,
}

#[test]
fn test_multi_skip_default_roundtrip() {
    let original = MultiSkipDefault {
        id: 7,
        flags: 0, // will be overwritten by default_value on decode
        name: "doc".into(),
        author: "alice".into(), // will be overwritten by default_value on decode
        score: 9.5,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (MultiSkipDefault, _) = decode_from_slice(&enc).expect("decode");

    assert_eq!(dec.id, 7);
    assert_eq!(dec.flags, 255u8); // default_value = "255u8"
    assert_eq!(dec.name, "doc");
    assert_eq!(dec.author, "anonymous"); // default_value = r#""anonymous".to_string()"#
    assert!((dec.score - 9.5f32).abs() < f32::EPSILON);
}
