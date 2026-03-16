//! Advanced tests for OxiCode serialization of enums with newtype (single-field) variants.
//!
//! Covers 22 distinct scenarios including roundtrips, wire-format properties,
//! config variants, collections, nesting, and float/integer variants.

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

// ---------------------------------------------------------------------------
// Shared enum definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Message {
    Text(String),
    Number(u64),
    Bytes(Vec<u8>),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Wrapper {
    A(u32),
    B(i32),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Opt {
    Some(u64),
    None,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Nested {
    Inner(Box<u32>),
    Other(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Tagged {
    Tagged(u32),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TupleInner {
    Pair(u32, String),
    Single(u64),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Either {
    Left(u32),
    Right(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Inner {
    Val(u32),
    Empty,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Outer {
    Wrapped(Inner),
    Raw(u64),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FloatVariant {
    F(f64),
    I(i64),
}

// ---------------------------------------------------------------------------
// Test 1: Message::Text roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_text_roundtrip() {
    let original = Message::Text(String::from("hello oxicode"));
    let encoded = encode_to_vec(&original).expect("encode Message::Text");
    let (decoded, _): (Message, usize) = decode_from_slice(&encoded).expect("decode Message::Text");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Message::Number roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_number_roundtrip() {
    let original = Message::Number(9_876_543_210_u64);
    let encoded = encode_to_vec(&original).expect("encode Message::Number");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Number");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Message::Bytes roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_bytes_roundtrip() {
    let original = Message::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let encoded = encode_to_vec(&original).expect("encode Message::Bytes");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Bytes");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Wrapper::A roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_wrapper_a_roundtrip() {
    let original = Wrapper::A(42_u32);
    let encoded = encode_to_vec(&original).expect("encode Wrapper::A");
    let (decoded, _): (Wrapper, usize) = decode_from_slice(&encoded).expect("decode Wrapper::A");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Wrapper::B roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_wrapper_b_roundtrip() {
    let original = Wrapper::B(-99_i32);
    let encoded = encode_to_vec(&original).expect("encode Wrapper::B");
    let (decoded, _): (Wrapper, usize) = decode_from_slice(&encoded).expect("decode Wrapper::B");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Opt::Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_opt_some_roundtrip() {
    let original = Opt::Some(1_000_000_u64);
    let encoded = encode_to_vec(&original).expect("encode Opt::Some");
    let (decoded, _): (Opt, usize) = decode_from_slice(&encoded).expect("decode Opt::Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Opt::None (unit variant) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_opt_none_roundtrip() {
    let original = Opt::None;
    let encoded = encode_to_vec(&original).expect("encode Opt::None");
    let (decoded, _): (Opt, usize) = decode_from_slice(&encoded).expect("decode Opt::None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Nested::Inner(Box<u32>) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nested_inner_box_roundtrip() {
    let original = Nested::Inner(Box::new(255_u32));
    let encoded = encode_to_vec(&original).expect("encode Nested::Inner");
    let (decoded, _): (Nested, usize) = decode_from_slice(&encoded).expect("decode Nested::Inner");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Large payload in newtype Vec<u8> variant
// ---------------------------------------------------------------------------

#[test]
fn test_large_bytes_payload_roundtrip() {
    let payload: Vec<u8> = (0_u16..1024).map(|i| (i % 256) as u8).collect();
    let original = Message::Bytes(payload);
    let encoded = encode_to_vec(&original).expect("encode large Message::Bytes");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("decode large Message::Bytes");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Tagged single-variant newtype roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_single_variant_roundtrip() {
    let original = Tagged::Tagged(7_u32);
    let encoded = encode_to_vec(&original).expect("encode Tagged::Tagged");
    let (decoded, _): (Tagged, usize) = decode_from_slice(&encoded).expect("decode Tagged::Tagged");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Tuple inner type (u32, String) as newtype-like multi-field variant
// ---------------------------------------------------------------------------

#[test]
fn test_tuple_inner_pair_roundtrip() {
    // TupleInner::Pair holds two fields — tests a tuple variant (not strictly
    // a newtype, but exercising the same path with compound inner data).
    let original = TupleInner::Pair(100_u32, String::from("tuple_inner"));
    let encoded = encode_to_vec(&original).expect("encode TupleInner::Pair");
    let (decoded, _): (TupleInner, usize) =
        decode_from_slice(&encoded).expect("decode TupleInner::Pair");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Either::Left roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_either_left_roundtrip() {
    let original = Either::Left(999_u32);
    let encoded = encode_to_vec(&original).expect("encode Either::Left");
    let (decoded, _): (Either, usize) = decode_from_slice(&encoded).expect("decode Either::Left");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Either::Right roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_either_right_roundtrip() {
    let original = Either::Right(String::from("right side"));
    let encoded = encode_to_vec(&original).expect("encode Either::Right");
    let (decoded, _): (Either, usize) = decode_from_slice(&encoded).expect("decode Either::Right");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Wire bytes differ between Left and Right discriminants
// ---------------------------------------------------------------------------

#[test]
fn test_either_left_right_wire_differ() {
    // Left is discriminant 0, Right is discriminant 1 — first byte must differ.
    let left_bytes = encode_to_vec(&Either::Left(0_u32)).expect("encode Left");
    let right_bytes = encode_to_vec(&Either::Right(String::new())).expect("encode Right");
    assert_ne!(
        left_bytes[0], right_bytes[0],
        "Left and Right must have different discriminant bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Consumed == encoded.len() for newtype enum
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_equals_encoded_len() {
    let original = Wrapper::A(123_u32);
    let encoded = encode_to_vec(&original).expect("encode Wrapper::A");
    let (_, consumed): (Wrapper, usize) = decode_from_slice(&encoded).expect("decode Wrapper::A");
    assert_eq!(
        consumed,
        encoded.len(),
        "all encoded bytes must be consumed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Fixed-int (legacy) config with newtype enum
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_newtype_enum() {
    let original = Wrapper::B(-1_i32);
    let cfg = config::legacy();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("legacy encode Wrapper::B");
    let (decoded, consumed): (Wrapper, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("legacy decode Wrapper::B");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Big-endian config with newtype enum
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_config_newtype_enum() {
    let original = Wrapper::A(256_u32);
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("big-endian encode Wrapper::A");
    let (decoded, consumed): (Wrapper, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("big-endian decode Wrapper::A");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: Vec<enum> newtype variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_enum_newtype_roundtrip() {
    let original: Vec<Either> = vec![
        Either::Left(1),
        Either::Right(String::from("two")),
        Either::Left(3),
        Either::Right(String::from("four")),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Either>");
    let (decoded, _): (Vec<Either>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Either>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Option<enum> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_enum_some_roundtrip() {
    let original: Option<Wrapper> = Some(Wrapper::A(77_u32));
    let encoded = encode_to_vec(&original).expect("encode Option<Wrapper> Some");
    let (decoded, _): (Option<Wrapper>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Wrapper> Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Option<enum> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_enum_none_roundtrip() {
    let original: Option<Wrapper> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Wrapper> None");
    let (decoded, _): (Option<Wrapper>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Wrapper> None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Nested enum — outer contains inner enum as newtype
// ---------------------------------------------------------------------------

#[test]
fn test_nested_enum_outer_contains_inner_roundtrip() {
    let original = Outer::Wrapped(Inner::Val(42_u32));
    let encoded = encode_to_vec(&original).expect("encode Outer::Wrapped(Inner::Val)");
    let (decoded, consumed): (Outer, usize) =
        decode_from_slice(&encoded).expect("decode Outer::Wrapped(Inner::Val)");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: FloatVariant::F(f64) and FloatVariant::I(i64) roundtrip both
// ---------------------------------------------------------------------------

#[test]
fn test_float_variant_roundtrip_both() {
    let float_val = FloatVariant::F(std::f64::consts::E);
    let int_val = FloatVariant::I(-9_223_372_036_854_775_807_i64);

    let encoded_f = encode_to_vec(&float_val).expect("encode FloatVariant::F");
    let (decoded_f, _): (FloatVariant, usize) =
        decode_from_slice(&encoded_f).expect("decode FloatVariant::F");
    assert_eq!(float_val, decoded_f);

    let encoded_i = encode_to_vec(&int_val).expect("encode FloatVariant::I");
    let (decoded_i, _): (FloatVariant, usize) =
        decode_from_slice(&encoded_i).expect("decode FloatVariant::I");
    assert_eq!(int_val, decoded_i);
}
