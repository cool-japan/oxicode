//! Advanced tests for `std::cmp::Ordering` and `std::ops::ControlFlow` serialization in OxiCode.
//! 22 tests covering roundtrip, config variants, structs, arrays, and byte-consumption checks.

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
    encode_to_vec_with_config,
};
use std::cmp::Ordering;
use std::ops::ControlFlow;

// ─── Test 1: Ordering::Less roundtrip ───────────────────────────────────────

#[test]
fn test_adv2_ordering_less_roundtrip() {
    let enc = encode_to_vec(&Ordering::Less).expect("encode Ordering::Less");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode Ordering::Less");
    assert_eq!(Ordering::Less, dec);
}

// ─── Test 2: Ordering::Equal roundtrip ──────────────────────────────────────

#[test]
fn test_adv2_ordering_equal_roundtrip() {
    let enc = encode_to_vec(&Ordering::Equal).expect("encode Ordering::Equal");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode Ordering::Equal");
    assert_eq!(Ordering::Equal, dec);
}

// ─── Test 3: Ordering::Greater roundtrip ────────────────────────────────────

#[test]
fn test_adv2_ordering_greater_roundtrip() {
    let enc = encode_to_vec(&Ordering::Greater).expect("encode Ordering::Greater");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode Ordering::Greater");
    assert_eq!(Ordering::Greater, dec);
}

// ─── Test 4: Vec<Ordering> with all three variants ──────────────────────────

#[test]
fn test_adv2_vec_ordering_all_variants_roundtrip() {
    let variants = vec![Ordering::Less, Ordering::Equal, Ordering::Greater];
    let enc = encode_to_vec(&variants).expect("encode Vec<Ordering>");
    let (dec, _): (Vec<Ordering>, _) = decode_from_slice(&enc).expect("decode Vec<Ordering>");
    assert_eq!(variants, dec);
}

// ─── Test 5: Option<Ordering> Some(Less) ────────────────────────────────────

#[test]
fn test_adv2_option_ordering_some_less_roundtrip() {
    let val: Option<Ordering> = Some(Ordering::Less);
    let enc = encode_to_vec(&val).expect("encode Option<Ordering>::Some(Less)");
    let (dec, _): (Option<Ordering>, _) =
        decode_from_slice(&enc).expect("decode Option<Ordering>::Some(Less)");
    assert_eq!(val, dec);
}

// ─── Test 6: Option<Ordering> None ──────────────────────────────────────────

#[test]
fn test_adv2_option_ordering_none_roundtrip() {
    let val: Option<Ordering> = None;
    let enc = encode_to_vec(&val).expect("encode Option<Ordering>::None");
    let (dec, _): (Option<Ordering>, _) =
        decode_from_slice(&enc).expect("decode Option<Ordering>::None");
    assert_eq!(val, dec);
}

// ─── Test 7: Struct with Ordering field ─────────────────────────────────────

#[test]
fn test_adv2_struct_with_ordering_field_roundtrip() {
    #[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
    struct SortResult {
        name: String,
        order: Ordering,
    }

    let val = SortResult {
        name: "alpha_beta".to_string(),
        order: Ordering::Greater,
    };
    let enc = encode_to_vec(&val).expect("encode SortResult");
    let (dec, _): (SortResult, _) = decode_from_slice(&enc).expect("decode SortResult");
    assert_eq!(val, dec);
}

// ─── Test 8: Ordering bytes_consumed == encoded.len() ───────────────────────

#[test]
fn test_adv2_ordering_bytes_consumed_equals_encoded_len() {
    let enc = encode_to_vec(&Ordering::Equal).expect("encode Ordering::Equal");
    let (_, consumed): (Ordering, _) =
        decode_from_slice(&enc).expect("decode Ordering::Equal for consumed check");
    assert_eq!(consumed, enc.len());
}

// ─── Test 9: Fixed-int config with Ordering ─────────────────────────────────

#[test]
fn test_adv2_ordering_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let enc =
        encode_to_vec_with_config(&Ordering::Less, cfg).expect("encode Ordering::Less fixed_int");
    let (dec, _): (Ordering, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Ordering::Less fixed_int");
    assert_eq!(Ordering::Less, dec);
}

// ─── Test 10: Big-endian config with Ordering ───────────────────────────────

#[test]
fn test_adv2_ordering_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&Ordering::Greater, cfg)
        .expect("encode Ordering::Greater big_endian");
    let (dec, _): (Ordering, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Ordering::Greater big_endian");
    assert_eq!(Ordering::Greater, dec);
}

// ─── Test 11: ControlFlow<String, u32> Continue(42u32) ──────────────────────

#[test]
fn test_adv2_controlflow_string_u32_continue_42_roundtrip() {
    let cf: ControlFlow<String, u32> = ControlFlow::Continue(42u32);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(42u32)");
    let (dec, _): (ControlFlow<String, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(42u32)");
    assert_eq!(cf, dec);
}

// ─── Test 12: ControlFlow<String, u32> Break("stop") ───────────────────────

#[test]
fn test_adv2_controlflow_string_u32_break_stop_roundtrip() {
    let cf: ControlFlow<String, u32> = ControlFlow::Break("stop".to_string());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Break(\"stop\")");
    let (dec, _): (ControlFlow<String, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Break(\"stop\")");
    assert_eq!(cf, dec);
}

// ─── Test 13: ControlFlow<(), ()> Continue(()) ──────────────────────────────

#[test]
fn test_adv2_controlflow_unit_continue_roundtrip() {
    let cf: ControlFlow<(), ()> = ControlFlow::Continue(());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::<(),()>::Continue(())");
    let (dec, _): (ControlFlow<(), ()>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::<(),()>::Continue(())");
    assert_eq!(cf, dec);
}

// ─── Test 14: ControlFlow<(), ()> Break(()) ─────────────────────────────────

#[test]
fn test_adv2_controlflow_unit_break_roundtrip() {
    let cf: ControlFlow<(), ()> = ControlFlow::Break(());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::<(),()>::Break(())");
    let (dec, _): (ControlFlow<(), ()>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::<(),()>::Break(())");
    assert_eq!(cf, dec);
}

// ─── Test 15: ControlFlow<bool, u64> Continue(0u64) ────────────────────────

#[test]
fn test_adv2_controlflow_bool_u64_continue_zero_roundtrip() {
    let cf: ControlFlow<bool, u64> = ControlFlow::Continue(0u64);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::<bool,u64>::Continue(0)");
    let (dec, _): (ControlFlow<bool, u64>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::<bool,u64>::Continue(0)");
    assert_eq!(cf, dec);
}

// ─── Test 16: ControlFlow<bool, u64> Break(true) ────────────────────────────

#[test]
fn test_adv2_controlflow_bool_u64_break_true_roundtrip() {
    let cf: ControlFlow<bool, u64> = ControlFlow::Break(true);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::<bool,u64>::Break(true)");
    let (dec, _): (ControlFlow<bool, u64>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::<bool,u64>::Break(true)");
    assert_eq!(cf, dec);
}

// ─── Test 17: Vec<ControlFlow<String, u32>> mixed variants ──────────────────

#[test]
fn test_adv2_vec_controlflow_mixed_variants_roundtrip() {
    let items: Vec<ControlFlow<String, u32>> = vec![
        ControlFlow::Continue(1u32),
        ControlFlow::Break("halt".to_string()),
        ControlFlow::Continue(255u32),
        ControlFlow::Break("abort".to_string()),
    ];
    let enc = encode_to_vec(&items).expect("encode Vec<ControlFlow<String,u32>>");
    let (dec, _): (Vec<ControlFlow<String, u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<ControlFlow<String,u32>>");
    assert_eq!(items, dec);
}

// ─── Test 18: Option<ControlFlow<bool, u32>> Some(Continue(5)) ──────────────

#[test]
fn test_adv2_option_controlflow_some_continue_5_roundtrip() {
    let val: Option<ControlFlow<bool, u32>> = Some(ControlFlow::Continue(5u32));
    let enc = encode_to_vec(&val).expect("encode Option<ControlFlow>::Some(Continue(5))");
    let (dec, _): (Option<ControlFlow<bool, u32>>, _) =
        decode_from_slice(&enc).expect("decode Option<ControlFlow>::Some(Continue(5))");
    assert_eq!(val, dec);
}

// ─── Test 19: Struct with ControlFlow field ─────────────────────────────────

#[test]
fn test_adv2_struct_with_controlflow_field_roundtrip() {
    #[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
    struct StepResult {
        step: u32,
        flow: ControlFlow<String, u32>,
    }

    let val = StepResult {
        step: 7u32,
        flow: ControlFlow::Continue(42u32),
    };
    let enc = encode_to_vec(&val).expect("encode StepResult");
    let (dec, _): (StepResult, _) = decode_from_slice(&enc).expect("decode StepResult");
    assert_eq!(val, dec);
}

// ─── Test 20: ControlFlow bytes_consumed == encoded.len() ───────────────────

#[test]
fn test_adv2_controlflow_bytes_consumed_equals_encoded_len() {
    let cf: ControlFlow<String, u32> = ControlFlow::Continue(99u32);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(99u32)");
    let (_, consumed): (ControlFlow<String, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(99u32) for consumed check");
    assert_eq!(consumed, enc.len());
}

// ─── Test 21: ControlFlow<Vec<u8>, String> Break with data ──────────────────

#[test]
fn test_adv2_controlflow_vec_u8_string_break_with_data_roundtrip() {
    let cf: ControlFlow<Vec<u8>, String> = ControlFlow::Break(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::<Vec<u8>,String>::Break(bytes)");
    let (dec, _): (ControlFlow<Vec<u8>, String>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::<Vec<u8>,String>::Break(bytes)");
    assert_eq!(cf, dec);
}

// ─── Test 22: [Ordering; 3] array with all three variants ───────────────────

#[test]
fn test_adv2_ordering_array_three_variants_roundtrip() {
    let arr: [Ordering; 3] = [Ordering::Less, Ordering::Equal, Ordering::Greater];
    let enc = encode_to_vec(&arr).expect("encode [Ordering; 3]");
    let (dec, _): ([Ordering; 3], _) = decode_from_slice(&enc).expect("decode [Ordering; 3]");
    assert_eq!(arr, dec);
}
