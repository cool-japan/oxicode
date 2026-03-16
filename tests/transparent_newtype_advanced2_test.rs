//! Advanced tests for newtype wrappers and transparent encoding in OxiCode.

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

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct UserId(u64);

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Email(String);

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Score(f32);

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Bytes(Vec<u8>);

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Pair<A, B>(A, B);

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Triple<A, B, C>(A, B, C);

#[test]
fn test_user_id_roundtrip() {
    let original = UserId(42);
    let enc = encode_to_vec(&original).expect("encode UserId(42)");
    let (val, _): (UserId, usize) = decode_from_slice(&enc).expect("decode UserId(42)");
    assert_eq!(original, val);
}

#[test]
fn test_user_id_zero_roundtrip() {
    let original = UserId(0);
    let enc = encode_to_vec(&original).expect("encode UserId(0)");
    let (val, _): (UserId, usize) = decode_from_slice(&enc).expect("decode UserId(0)");
    assert_eq!(original, val);
}

#[test]
fn test_user_id_max_roundtrip() {
    let original = UserId(u64::MAX);
    let enc = encode_to_vec(&original).expect("encode UserId(u64::MAX)");
    let (val, _): (UserId, usize) = decode_from_slice(&enc).expect("decode UserId(u64::MAX)");
    assert_eq!(original, val);
}

#[test]
fn test_email_roundtrip() {
    let original = Email("test@example.com".to_string());
    let enc = encode_to_vec(&original).expect("encode Email");
    let (val, _): (Email, usize) = decode_from_slice(&enc).expect("decode Email");
    assert_eq!(original, val);
}

#[test]
fn test_email_empty_roundtrip() {
    let original = Email(String::new());
    let enc = encode_to_vec(&original).expect("encode Email(empty)");
    let (val, _): (Email, usize) = decode_from_slice(&enc).expect("decode Email(empty)");
    assert_eq!(original, val);
}

#[test]
fn test_score_roundtrip() {
    let score_val = 9.5f32;
    let original = Score(score_val);
    let enc = encode_to_vec(&original).expect("encode Score(9.5)");
    let (val, _): (Score, usize) = decode_from_slice(&enc).expect("decode Score(9.5)");
    assert_eq!(original, val);
}

#[test]
fn test_score_zero_roundtrip() {
    let score_val = 0.0f32;
    let original = Score(score_val);
    let enc = encode_to_vec(&original).expect("encode Score(0.0)");
    let (val, _): (Score, usize) = decode_from_slice(&enc).expect("decode Score(0.0)");
    assert_eq!(original, val);
}

#[test]
fn test_bytes_roundtrip() {
    let original = Bytes(vec![1u8, 2, 3]);
    let enc = encode_to_vec(&original).expect("encode Bytes([1,2,3])");
    let (val, _): (Bytes, usize) = decode_from_slice(&enc).expect("decode Bytes([1,2,3])");
    assert_eq!(original, val);
}

#[test]
fn test_bytes_empty_roundtrip() {
    let original = Bytes(vec![]);
    let enc = encode_to_vec(&original).expect("encode Bytes(empty)");
    let (val, _): (Bytes, usize) = decode_from_slice(&enc).expect("decode Bytes(empty)");
    assert_eq!(original, val);
}

#[test]
fn test_user_id_same_size_as_u64() {
    let uid = UserId(42u64);
    let raw: u64 = 42u64;
    let enc_uid = encode_to_vec(&uid).expect("encode UserId(42)");
    let enc_raw = encode_to_vec(&raw).expect("encode 42u64");
    assert_eq!(
        enc_uid.len(),
        enc_raw.len(),
        "UserId newtype should encode with same size as inner u64"
    );
}

#[test]
fn test_email_same_size_as_string() {
    let s = "hello@oxicode.rs".to_string();
    let email = Email(s.clone());
    let enc_email = encode_to_vec(&email).expect("encode Email");
    let enc_str = encode_to_vec(&s).expect("encode String");
    assert_eq!(
        enc_email.len(),
        enc_str.len(),
        "Email newtype should encode with same size as inner String"
    );
}

#[test]
fn test_pair_u32_string_roundtrip() {
    let original = Pair(1u32, "hello".to_string());
    let enc = encode_to_vec(&original).expect("encode Pair(u32, String)");
    let (val, _): (Pair<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Pair(u32, String)");
    assert_eq!(original, val);
}

#[test]
fn test_triple_u8_u16_u32_roundtrip() {
    let original = Triple(1u8, 2u16, 3u32);
    let enc = encode_to_vec(&original).expect("encode Triple(u8, u16, u32)");
    let (val, _): (Triple<u8, u16, u32>, usize) =
        decode_from_slice(&enc).expect("decode Triple(u8, u16, u32)");
    assert_eq!(original, val);
}

#[test]
fn test_vec_user_id_roundtrip() {
    let original = vec![UserId(10), UserId(20), UserId(30)];
    let enc = encode_to_vec(&original).expect("encode Vec<UserId>");
    let (val, _): (Vec<UserId>, usize) = decode_from_slice(&enc).expect("decode Vec<UserId>");
    assert_eq!(original, val);
}

#[test]
fn test_option_user_id_some_roundtrip() {
    let original: Option<UserId> = Some(UserId(99));
    let enc = encode_to_vec(&original).expect("encode Option<UserId>::Some");
    let (val, _): (Option<UserId>, usize) =
        decode_from_slice(&enc).expect("decode Option<UserId>::Some");
    assert_eq!(original, val);
}

#[test]
fn test_option_user_id_none_roundtrip() {
    let original: Option<UserId> = None;
    let enc = encode_to_vec(&original).expect("encode Option<UserId>::None");
    let (val, _): (Option<UserId>, usize) =
        decode_from_slice(&enc).expect("decode Option<UserId>::None");
    assert_eq!(original, val);
}

#[test]
fn test_user_id_consumed_equals_len() {
    let original = UserId(12345);
    let enc = encode_to_vec(&original).expect("encode UserId for consumed check");
    let (_, consumed): (UserId, usize) =
        decode_from_slice(&enc).expect("decode UserId for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes should equal total encoded length"
    );
}

#[test]
fn test_email_consumed_equals_len() {
    let original = Email("consumed@test.com".to_string());
    let enc = encode_to_vec(&original).expect("encode Email for consumed check");
    let (_, consumed): (Email, usize) =
        decode_from_slice(&enc).expect("decode Email for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes should equal total encoded length"
    );
}

#[test]
fn test_user_id_fixed_int_config() {
    let cfg = config::legacy();
    let original = UserId(777);
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode UserId with legacy config");
    let (val, _): (UserId, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode UserId with legacy config");
    assert_eq!(original, val);
}

#[test]
fn test_email_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let original = Email("big-endian@oxicode.rs".to_string());
    let enc =
        encode_to_vec_with_config(&original, cfg).expect("encode Email with big_endian config");
    let (val, _): (Email, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Email with big_endian config");
    assert_eq!(original, val);
}

#[test]
fn test_nested_user_id_pair() {
    let original = Pair(UserId(1), UserId(2));
    let enc = encode_to_vec(&original).expect("encode Pair(UserId, UserId)");
    let (val, _): (Pair<UserId, UserId>, usize) =
        decode_from_slice(&enc).expect("decode Pair(UserId, UserId)");
    assert_eq!(original, val);
}

#[test]
fn test_bytes_large_roundtrip() {
    let data: Vec<u8> = (0u8..=255).cycle().take(500).collect();
    let original = Bytes(data);
    let enc = encode_to_vec(&original).expect("encode Bytes(500 bytes)");
    let (val, _): (Bytes, usize) = decode_from_slice(&enc).expect("decode Bytes(500 bytes)");
    assert_eq!(original, val);
}
