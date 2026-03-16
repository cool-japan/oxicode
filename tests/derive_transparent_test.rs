//! Tests for `#[oxicode(transparent)]` container attribute.

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

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct UserId(u64);

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct Username {
    inner: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct Wrapper<T>(T);

#[test]
fn test_transparent_tuple_newtype() {
    let id = UserId(42);
    let enc = oxicode::encode_to_vec(&id).expect("encode");

    // Must be identical to encoding u64 directly
    let raw_enc = oxicode::encode_to_vec(&42u64).expect("encode raw");
    assert_eq!(enc, raw_enc, "transparent should produce identical bytes");

    let (dec, _): (UserId, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(id, dec);
}

#[test]
fn test_transparent_named_field() {
    let u = Username {
        inner: "alice".to_string(),
    };
    let enc = oxicode::encode_to_vec(&u).expect("encode");

    let raw_enc = oxicode::encode_to_vec(&"alice".to_string()).expect("encode raw");
    assert_eq!(enc, raw_enc, "transparent should produce identical bytes");

    let (dec, _): (Username, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(u, dec);
}

#[test]
fn test_transparent_generic() {
    let w = Wrapper(123u32);
    let enc = oxicode::encode_to_vec(&w).expect("encode");
    let raw_enc = oxicode::encode_to_vec(&123u32).expect("encode raw");
    assert_eq!(enc, raw_enc);
    let (dec, _): (Wrapper<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

#[test]
fn test_transparent_roundtrip_zero() {
    let id = UserId(0);
    let enc = oxicode::encode_to_vec(&id).expect("encode");
    let (dec, _): (UserId, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(id, dec);
}

#[test]
fn test_transparent_roundtrip_max() {
    let id = UserId(u64::MAX);
    let enc = oxicode::encode_to_vec(&id).expect("encode");
    let (dec, _): (UserId, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(id, dec);
}

#[test]
fn test_transparent_generic_string() {
    let w = Wrapper("hello world".to_string());
    let enc = oxicode::encode_to_vec(&w).expect("encode");
    let raw_enc = oxicode::encode_to_vec(&"hello world".to_string()).expect("encode raw");
    assert_eq!(enc, raw_enc, "transparent generic<String> should match raw");
    let (dec, _): (Wrapper<String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

// Additional types for extended test coverage
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct U32Wrapper(u32);

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct StringWrap(String);

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct VecWrapper(Vec<u8>);

#[test]
fn test_transparent_u32_same_bytes_as_raw() {
    let w = U32Wrapper(42u32);
    let enc_wrapper = oxicode::encode_to_vec(&w).expect("encode wrapper");
    let enc_raw = oxicode::encode_to_vec(&42u32).expect("encode raw");
    assert_eq!(
        enc_wrapper, enc_raw,
        "transparent wrapper should encode identically to inner type"
    );
}

#[test]
fn test_transparent_u32_roundtrip() {
    let w = U32Wrapper(12345);
    let enc = oxicode::encode_to_vec(&w).expect("encode");
    let (dec, _): (U32Wrapper, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

#[test]
fn test_transparent_string_same_bytes_as_raw() {
    let w = StringWrap("hello".to_string());
    let enc_wrapper = oxicode::encode_to_vec(&w).expect("encode wrapper");
    let enc_raw = oxicode::encode_to_vec(&"hello".to_string()).expect("encode raw");
    assert_eq!(enc_wrapper, enc_raw);
}

#[test]
fn test_transparent_string_roundtrip() {
    let w = StringWrap("oxicode transparent".to_string());
    let enc = oxicode::encode_to_vec(&w).expect("encode");
    let (dec, _): (StringWrap, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

#[test]
fn test_transparent_vec_roundtrip() {
    let w = VecWrapper(vec![1, 2, 3, 4, 5]);
    let enc = oxicode::encode_to_vec(&w).expect("encode");
    let (dec, _): (VecWrapper, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

#[test]
fn test_transparent_can_decode_from_raw_encode() {
    // Encode the raw inner type, decode as U32Wrapper — they should be compatible
    let raw: u32 = 999;
    let enc = oxicode::encode_to_vec(&raw).expect("encode raw");
    let (dec, _): (U32Wrapper, _) = oxicode::decode_from_slice(&enc).expect("decode as wrapper");
    assert_eq!(U32Wrapper(999), dec);
}

#[test]
fn test_transparent_vec_same_bytes_as_raw() {
    let w = VecWrapper(vec![10, 20, 30]);
    let enc_wrapper = oxicode::encode_to_vec(&w).expect("encode wrapper");
    let enc_raw = oxicode::encode_to_vec(&vec![10u8, 20, 30]).expect("encode raw");
    assert_eq!(
        enc_wrapper, enc_raw,
        "transparent vec wrapper should encode identically"
    );
}
