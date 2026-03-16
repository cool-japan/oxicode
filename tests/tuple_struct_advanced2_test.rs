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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Meters(f64);

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserId(u32);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Label(String);

#[derive(Debug, PartialEq, Encode, Decode)]
struct TwoFields(u32, u64);

#[derive(Debug, PartialEq, Encode, Decode)]
struct ThreeFields(u8, u16, u32);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Wrapper<T>(T);

#[test]
fn test_meters_roundtrip() {
    let val = Meters(3.14);
    let enc = encode_to_vec(&val).expect("encode Meters failed");
    let (decoded, _): (Meters, usize) = decode_from_slice(&enc).expect("decode Meters failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_userid_roundtrip() {
    let val = UserId(42);
    let enc = encode_to_vec(&val).expect("encode UserId failed");
    let (decoded, _): (UserId, usize) = decode_from_slice(&enc).expect("decode UserId failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_label_roundtrip() {
    let val = Label("hello".into());
    let enc = encode_to_vec(&val).expect("encode Label failed");
    let (decoded, _): (Label, usize) = decode_from_slice(&enc).expect("decode Label failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_two_fields_roundtrip() {
    let val = TwoFields(100, 200);
    let enc = encode_to_vec(&val).expect("encode TwoFields failed");
    let (decoded, _): (TwoFields, usize) =
        decode_from_slice(&enc).expect("decode TwoFields failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_three_fields_roundtrip() {
    let val = ThreeFields(1, 2, 3);
    let enc = encode_to_vec(&val).expect("encode ThreeFields failed");
    let (decoded, _): (ThreeFields, usize) =
        decode_from_slice(&enc).expect("decode ThreeFields failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_u32_roundtrip() {
    let val = Wrapper::<u32>(99);
    let enc = encode_to_vec(&val).expect("encode Wrapper<u32> failed");
    let (decoded, _): (Wrapper<u32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<u32> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_string_roundtrip() {
    let val = Wrapper::<String>("test".into());
    let enc = encode_to_vec(&val).expect("encode Wrapper<String> failed");
    let (decoded, _): (Wrapper<String>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<String> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_userid_same_bytes_as_raw_u32() {
    let raw: u32 = 77;
    let uid = UserId(77);
    let enc_raw = encode_to_vec(&raw).expect("encode raw u32 failed");
    let enc_uid = encode_to_vec(&uid).expect("encode UserId failed");
    assert_eq!(
        enc_raw, enc_uid,
        "UserId should encode identically to raw u32"
    );
}

#[test]
fn test_meters_same_bytes_as_raw_f64() {
    let raw: f64 = 2.718;
    let meters = Meters(2.718);
    let enc_raw = encode_to_vec(&raw).expect("encode raw f64 failed");
    let enc_meters = encode_to_vec(&meters).expect("encode Meters failed");
    assert_eq!(
        enc_raw, enc_meters,
        "Meters should encode identically to raw f64"
    );
}

#[test]
fn test_vec_userid_roundtrip() {
    let val = vec![UserId(1), UserId(2), UserId(3)];
    let enc = encode_to_vec(&val).expect("encode Vec<UserId> failed");
    let (decoded, _): (Vec<UserId>, usize) =
        decode_from_slice(&enc).expect("decode Vec<UserId> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_userid_some_roundtrip() {
    let val: Option<UserId> = Some(UserId(55));
    let enc = encode_to_vec(&val).expect("encode Option<UserId> Some failed");
    let (decoded, _): (Option<UserId>, usize) =
        decode_from_slice(&enc).expect("decode Option<UserId> Some failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_userid_none_roundtrip() {
    let val: Option<UserId> = None;
    let enc = encode_to_vec(&val).expect("encode Option<UserId> None failed");
    let (decoded, _): (Option<UserId>, usize) =
        decode_from_slice(&enc).expect("decode Option<UserId> None failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_tuple_u8_u16_u32_roundtrip() {
    let val: (u8, u16, u32) = (10, 300, 70000);
    let enc = encode_to_vec(&val).expect("encode (u8, u16, u32) failed");
    let (decoded, _): ((u8, u16, u32), usize) =
        decode_from_slice(&enc).expect("decode (u8, u16, u32) failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_tuple_bool_string_roundtrip() {
    let val: (bool, String) = (true, "oxicode".into());
    let enc = encode_to_vec(&val).expect("encode (bool, String) failed");
    let (decoded, _): ((bool, String), usize) =
        decode_from_slice(&enc).expect("decode (bool, String) failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_tuple_four_u32_roundtrip() {
    let val: (u32, u32, u32, u32) = (1, 2, 3, 4);
    let enc = encode_to_vec(&val).expect("encode (u32, u32, u32, u32) failed");
    let (decoded, _): ((u32, u32, u32, u32), usize) =
        decode_from_slice(&enc).expect("decode (u32, u32, u32, u32) failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_userid_roundtrip() {
    let val = Wrapper(UserId(123));
    let enc = encode_to_vec(&val).expect("encode Wrapper<UserId> failed");
    let (decoded, _): (Wrapper<UserId>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<UserId> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_two_fields_fixed_int_config() {
    let val = TwoFields(7, 8);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode TwoFields fixed-int failed");
    let (decoded, _): (TwoFields, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode TwoFields fixed-int failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_userid_consumed_bytes_eq_encoded_length() {
    let val = UserId(999);
    let enc = encode_to_vec(&val).expect("encode UserId failed");
    let enc_len = enc.len();
    let (_, consumed): (UserId, usize) = decode_from_slice(&enc).expect("decode UserId failed");
    assert_eq!(
        consumed, enc_len,
        "consumed bytes must equal encoded length"
    );
}

#[test]
fn test_two_fields_zero_roundtrip() {
    let val = TwoFields(0, 0);
    let enc = encode_to_vec(&val).expect("encode TwoFields zero failed");
    let (decoded, _): (TwoFields, usize) =
        decode_from_slice(&enc).expect("decode TwoFields zero failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_three_fields_max_values_roundtrip() {
    let val = ThreeFields(255, 65535, u32::MAX);
    let enc = encode_to_vec(&val).expect("encode ThreeFields max failed");
    let (decoded, _): (ThreeFields, usize) =
        decode_from_slice(&enc).expect("decode ThreeFields max failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_tuple_vec_string_bool_roundtrip() {
    let val: (Vec<u8>, String, bool) = (vec![1, 2, 3, 4], "rust".into(), false);
    let enc = encode_to_vec(&val).expect("encode (Vec<u8>, String, bool) failed");
    let (decoded, _): ((Vec<u8>, String, bool), usize) =
        decode_from_slice(&enc).expect("decode (Vec<u8>, String, bool) failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_two_fields_five_elements_roundtrip() {
    let val = vec![
        TwoFields(10, 20),
        TwoFields(30, 40),
        TwoFields(50, 60),
        TwoFields(70, 80),
        TwoFields(90, 100),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<TwoFields> failed");
    let (decoded, _): (Vec<TwoFields>, usize) =
        decode_from_slice(&enc).expect("decode Vec<TwoFields> failed");
    assert_eq!(val, decoded);
}
