//! Advanced tests for `Infallible`, `ControlFlow`, and `Ordering` encoding in OxiCode.

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
use oxicode::{config, decode_from_slice, encode_to_vec};
use std::cmp::Ordering;
use std::ops::ControlFlow;

// ===== Ordering roundtrip tests =====

#[test]
fn test_ordering_less_roundtrip() {
    let enc = encode_to_vec(&Ordering::Less).expect("encode Ordering::Less");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode Ordering::Less");
    assert_eq!(Ordering::Less, dec);
}

#[test]
fn test_ordering_equal_roundtrip() {
    let enc = encode_to_vec(&Ordering::Equal).expect("encode Ordering::Equal");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode Ordering::Equal");
    assert_eq!(Ordering::Equal, dec);
}

#[test]
fn test_ordering_greater_roundtrip() {
    let enc = encode_to_vec(&Ordering::Greater).expect("encode Ordering::Greater");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode Ordering::Greater");
    assert_eq!(Ordering::Greater, dec);
}

#[test]
fn test_ordering_less_encodes_as_one_byte() {
    // Ordering::Less encodes as i8(-1) via varint; varint(-1 as i8) = 1 byte (0xFF in zigzag or raw)
    let enc = encode_to_vec(&Ordering::Less).expect("encode Ordering::Less");
    assert_eq!(
        1,
        enc.len(),
        "Ordering::Less should encode as exactly 1 byte, got {:?}",
        enc
    );
}

#[test]
fn test_all_ordering_variants_in_vec_roundtrip() {
    let variants = vec![Ordering::Less, Ordering::Equal, Ordering::Greater];
    let enc = encode_to_vec(&variants).expect("encode Vec<Ordering>");
    let (dec, _): (Vec<Ordering>, _) = decode_from_slice(&enc).expect("decode Vec<Ordering>");
    assert_eq!(variants, dec);
}

#[test]
fn test_option_ordering_some_roundtrip() {
    let val: Option<Ordering> = Some(Ordering::Equal);
    let enc = encode_to_vec(&val).expect("encode Option<Ordering>::Some");
    let (dec, _): (Option<Ordering>, _) =
        decode_from_slice(&enc).expect("decode Option<Ordering>::Some");
    assert_eq!(val, dec);
}

#[test]
fn test_option_ordering_none_roundtrip() {
    let val: Option<Ordering> = None;
    let enc = encode_to_vec(&val).expect("encode Option<Ordering>::None");
    let (dec, _): (Option<Ordering>, _) =
        decode_from_slice(&enc).expect("decode Option<Ordering>::None");
    assert_eq!(val, dec);
}

#[test]
fn test_ordering_with_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let enc =
        oxicode::encode_to_vec_with_config(&Ordering::Greater, cfg).expect("encode with fixed_int");
    let (dec, _): (Ordering, _) =
        oxicode::decode_from_slice_with_config(&enc, cfg).expect("decode with fixed_int");
    assert_eq!(Ordering::Greater, dec);
}

#[test]
fn test_ordering_with_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let enc =
        oxicode::encode_to_vec_with_config(&Ordering::Less, cfg).expect("encode with big_endian");
    let (dec, _): (Ordering, _) =
        oxicode::decode_from_slice_with_config(&enc, cfg).expect("decode with big_endian");
    assert_eq!(Ordering::Less, dec);
}

#[test]
fn test_ordering_tuple_with_u32_roundtrip() {
    let val: (Ordering, u32) = (Ordering::Greater, 99u32);
    let enc = encode_to_vec(&val).expect("encode (Ordering, u32)");
    let (dec, _): ((Ordering, u32), _) = decode_from_slice(&enc).expect("decode (Ordering, u32)");
    assert_eq!(val, dec);
}

#[test]
fn test_struct_with_ordering_field_roundtrip() {
    #[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
    struct Comparison {
        label: String,
        result: Ordering,
    }

    let val = Comparison {
        label: "compare".to_string(),
        result: Ordering::Equal,
    };
    let enc = encode_to_vec(&val).expect("encode Comparison struct");
    let (dec, _): (Comparison, _) = decode_from_slice(&enc).expect("decode Comparison struct");
    assert_eq!(val, dec);
}

// ===== ControlFlow tests =====

#[test]
fn test_controlflow_unit_continue_roundtrip() {
    let cf: ControlFlow<(), ()> = ControlFlow::Continue(());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(())");
    let (dec, _): (ControlFlow<(), ()>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(())");
    assert_eq!(cf, dec);
}

#[test]
fn test_controlflow_unit_break_roundtrip() {
    let cf: ControlFlow<(), ()> = ControlFlow::Break(());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Break(())");
    let (dec, _): (ControlFlow<(), ()>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Break(())");
    assert_eq!(cf, dec);
}

#[test]
fn test_controlflow_continue_string_roundtrip() {
    let cf: ControlFlow<u32, String> = ControlFlow::Continue("hello".to_string());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(String)");
    let (dec, _): (ControlFlow<u32, String>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(String)");
    assert_eq!(cf, dec);
}

#[test]
fn test_controlflow_break_u32_roundtrip() {
    let cf: ControlFlow<u32, String> = ControlFlow::Break(42u32);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Break(u32)");
    let (dec, _): (ControlFlow<u32, String>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Break(u32)");
    assert_eq!(cf, dec);
}

#[test]
fn test_controlflow_continue_vec_break_string_continue_roundtrip() {
    let cf: ControlFlow<Vec<u8>, String> = ControlFlow::Continue("go".to_string());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::<Vec<u8>,String>::Continue");
    let (dec, _): (ControlFlow<Vec<u8>, String>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::<Vec<u8>,String>::Continue");
    assert_eq!(cf, dec);
}

#[test]
fn test_controlflow_break_vec_roundtrip() {
    let cf: ControlFlow<Vec<u8>, String> = ControlFlow::Break(vec![1u8, 2, 3]);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Break(Vec<u8>)");
    let (dec, _): (ControlFlow<Vec<u8>, String>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Break(Vec<u8>)");
    assert_eq!(cf, dec);
}

#[test]
fn test_vec_of_controlflow_roundtrip() {
    let items: Vec<ControlFlow<u32, String>> = vec![
        ControlFlow::Continue("first".to_string()),
        ControlFlow::Break(1u32),
        ControlFlow::Continue("third".to_string()),
        ControlFlow::Break(99u32),
    ];
    let enc = encode_to_vec(&items).expect("encode Vec<ControlFlow<u32,String>>");
    let (dec, _): (Vec<ControlFlow<u32, String>>, _) =
        decode_from_slice(&enc).expect("decode Vec<ControlFlow<u32,String>>");
    assert_eq!(items, dec);
}

#[test]
fn test_option_controlflow_some_roundtrip() {
    let val: Option<ControlFlow<u32, String>> = Some(ControlFlow::Continue("running".to_string()));
    let enc = encode_to_vec(&val).expect("encode Option<ControlFlow>::Some");
    let (dec, _): (Option<ControlFlow<u32, String>>, _) =
        decode_from_slice(&enc).expect("decode Option<ControlFlow>::Some");
    assert_eq!(val, dec);
}

#[test]
fn test_option_controlflow_none_roundtrip() {
    let val: Option<ControlFlow<u32, String>> = None;
    let enc = encode_to_vec(&val).expect("encode Option<ControlFlow>::None");
    let (dec, _): (Option<ControlFlow<u32, String>>, _) =
        decode_from_slice(&enc).expect("decode Option<ControlFlow>::None");
    assert_eq!(val, dec);
}

#[test]
fn test_controlflow_with_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let cf: ControlFlow<u32, String> = ControlFlow::Break(7u32);
    let enc = oxicode::encode_to_vec_with_config(&cf, cfg).expect("encode ControlFlow fixed_int");
    let (dec, _): (ControlFlow<u32, String>, _) =
        oxicode::decode_from_slice_with_config(&enc, cfg).expect("decode ControlFlow fixed_int");
    assert_eq!(cf, dec);
}

#[test]
fn test_controlflow_with_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let cf: ControlFlow<u32, String> = ControlFlow::Continue("big".to_string());
    let enc = oxicode::encode_to_vec_with_config(&cf, cfg).expect("encode ControlFlow big_endian");
    let (dec, _): (ControlFlow<u32, String>, _) =
        oxicode::decode_from_slice_with_config(&enc, cfg).expect("decode ControlFlow big_endian");
    assert_eq!(cf, dec);
}

#[test]
fn test_ordering_controlflow_tuple_roundtrip() {
    let val: (Ordering, ControlFlow<u32, String>) =
        (Ordering::Less, ControlFlow::Continue("ok".to_string()));
    let enc = encode_to_vec(&val).expect("encode (Ordering, ControlFlow)");
    let (dec, _): ((Ordering, ControlFlow<u32, String>), _) =
        decode_from_slice(&enc).expect("decode (Ordering, ControlFlow)");
    assert_eq!(val, dec);
}

#[test]
fn test_ordering_encoded_size_is_one_byte() {
    // All Ordering variants encode as i8 (varint), which is exactly 1 byte.
    for ord in [Ordering::Less, Ordering::Equal, Ordering::Greater] {
        let enc = encode_to_vec(&ord).expect("encode Ordering variant");
        assert_eq!(
            1,
            enc.len(),
            "Ordering::{:?} should encode as 1 byte, got {} bytes: {:?}",
            ord,
            enc.len(),
            enc
        );
    }
}
