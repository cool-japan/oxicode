//! Advanced property-based tests for OxiCode (set 26)

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
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Record {
    id: u32,
    value: i64,
    tag: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Pending(u32),
}

proptest! {
    #[test]
    fn prop_f32_roundtrip(val in any::<f32>()) {
        let encoded = encode_to_vec(&val).expect("encode f32");
        let (decoded, _): (f32, usize) = decode_from_slice(&encoded).expect("decode f32");
        if val.is_nan() {
            prop_assert!(decoded.is_nan());
        } else {
            prop_assert_eq!(val, decoded);
        }
    }

    #[test]
    fn prop_f64_roundtrip(val in any::<f64>()) {
        let encoded = encode_to_vec(&val).expect("encode f64");
        let (decoded, _): (f64, usize) = decode_from_slice(&encoded).expect("decode f64");
        if val.is_nan() {
            prop_assert!(decoded.is_nan());
        } else {
            prop_assert_eq!(val, decoded);
        }
    }

    #[test]
    fn prop_char_roundtrip(val in any::<char>()) {
        let encoded = encode_to_vec(&val).expect("encode char");
        let (decoded, _): (char, usize) = decode_from_slice(&encoded).expect("decode char");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_bool_u32_tuple_roundtrip(val in (any::<bool>(), any::<u32>())) {
        let encoded = encode_to_vec(&val).expect("encode (bool, u32)");
        let (decoded, _): ((bool, u32), usize) = decode_from_slice(&encoded).expect("decode (bool, u32)");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_u8_u16_u32_triple_roundtrip(val in (any::<u8>(), any::<u16>(), any::<u32>())) {
        let encoded = encode_to_vec(&val).expect("encode (u8, u16, u32)");
        let (decoded, _): ((u8, u16, u32), usize) = decode_from_slice(&encoded).expect("decode (u8, u16, u32)");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_vec_bool_roundtrip(val in prop::collection::vec(any::<bool>(), 0..20)) {
        let encoded = encode_to_vec(&val).expect("encode Vec<bool>");
        let (decoded, _): (Vec<bool>, usize) = decode_from_slice(&encoded).expect("decode Vec<bool>");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_vec_i32_roundtrip(val in prop::collection::vec(any::<i32>(), 0..30)) {
        let encoded = encode_to_vec(&val).expect("encode Vec<i32>");
        let (decoded, _): (Vec<i32>, usize) = decode_from_slice(&encoded).expect("decode Vec<i32>");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_option_u64_roundtrip(val in prop::option::of(any::<u64>())) {
        let encoded = encode_to_vec(&val).expect("encode Option<u64>");
        let (decoded, _): (Option<u64>, usize) = decode_from_slice(&encoded).expect("decode Option<u64>");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_option_string_roundtrip(val in prop::option::of("[a-z]{0,20}")) {
        let encoded = encode_to_vec(&val).expect("encode Option<String>");
        let (decoded, _): (Option<String>, usize) = decode_from_slice(&encoded).expect("decode Option<String>");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_record_roundtrip(
        id in any::<u32>(),
        value in any::<i64>(),
        tag in "[a-z]{0,10}"
    ) {
        let val = Record { id, value, tag };
        let encoded = encode_to_vec(&val).expect("encode Record");
        let (decoded, _): (Record, usize) = decode_from_slice(&encoded).expect("decode Record");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_status_active_roundtrip(_seed in any::<u8>()) {
        let val = Status::Active;
        let encoded = encode_to_vec(&val).expect("encode Status::Active");
        let (decoded, _): (Status, usize) = decode_from_slice(&encoded).expect("decode Status::Active");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_status_pending_roundtrip(n in any::<u32>()) {
        let val = Status::Pending(n);
        let encoded = encode_to_vec(&val).expect("encode Status::Pending");
        let (decoded, _): (Status, usize) = decode_from_slice(&encoded).expect("decode Status::Pending");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_vec_record_roundtrip(
        records in prop::collection::vec(
            (any::<u32>(), any::<i64>(), "[a-z]{0,10}").prop_map(|(id, value, tag)| Record { id, value, tag }),
            0..5
        )
    ) {
        let encoded = encode_to_vec(&records).expect("encode Vec<Record>");
        let (decoded, _): (Vec<Record>, usize) = decode_from_slice(&encoded).expect("decode Vec<Record>");
        prop_assert_eq!(records, decoded);
    }

    #[test]
    fn prop_btreemap_u32_string_roundtrip(
        pairs in prop::collection::vec((any::<u32>(), "[a-z]{0,10}"), 0..10)
    ) {
        use std::collections::BTreeMap;
        let val: BTreeMap<u32, String> = pairs.into_iter().collect();
        let encoded = encode_to_vec(&val).expect("encode BTreeMap<u32, String>");
        let (decoded, _): (BTreeMap<u32, String>, usize) = decode_from_slice(&encoded).expect("decode BTreeMap<u32, String>");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_string_string_tuple_roundtrip(
        val in ("[a-z]{0,15}", "[a-z]{0,15}")
    ) {
        let encoded = encode_to_vec(&val).expect("encode (String, String)");
        let (decoded, _): ((String, String), usize) = decode_from_slice(&encoded).expect("decode (String, String)");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_u8_encode_decode_identity(val in any::<u8>()) {
        let encoded = encode_to_vec(&val).expect("encode u8");
        let (decoded, _): (u8, usize) = decode_from_slice(&encoded).expect("decode u8");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_i8_encode_decode_identity(val in any::<i8>()) {
        let encoded = encode_to_vec(&val).expect("encode i8");
        let (decoded, _): (i8, usize) = decode_from_slice(&encoded).expect("decode i8");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn prop_encode_twice_same_bytes_record(
        id in any::<u32>(),
        value in any::<i64>(),
        tag in "[a-z]{0,10}"
    ) {
        let val = Record { id, value, tag };
        let encoded1 = encode_to_vec(&val).expect("first encode Record");
        let encoded2 = encode_to_vec(&val).expect("second encode Record");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn prop_consumed_bytes_equals_encoded_length_i64(val in any::<i64>()) {
        let encoded = encode_to_vec(&val).expect("encode i64");
        let (_, consumed): (i64, usize) = decode_from_slice(&encoded).expect("decode i64");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn prop_consumed_bytes_equals_encoded_length_vec_u8(
        val in prop::collection::vec(any::<u8>(), 0..32)
    ) {
        let encoded = encode_to_vec(&val).expect("encode Vec<u8>");
        let (_, consumed): (Vec<u8>, usize) = decode_from_slice(&encoded).expect("decode Vec<u8>");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn prop_double_roundtrip_f64(val in any::<f64>()) {
        let encoded1 = encode_to_vec(&val).expect("first encode f64");
        let (decoded1, _): (f64, usize) = decode_from_slice(&encoded1).expect("first decode f64");
        let encoded2 = encode_to_vec(&decoded1).expect("second encode f64");
        if !val.is_nan() {
            prop_assert_eq!(encoded1, encoded2);
        } else {
            // NaN is not equal but should still encode/decode consistently
            prop_assert_eq!(encoded1.len(), encoded2.len());
        }
    }

    #[test]
    fn prop_vec_status_roundtrip(
        items in prop::collection::vec(
            prop_oneof![
                Just(Status::Active),
                any::<u32>().prop_map(Status::Pending),
            ],
            0..8
        )
    ) {
        let encoded = encode_to_vec(&items).expect("encode Vec<Status>");
        let (decoded, _): (Vec<Status>, usize) = decode_from_slice(&encoded).expect("decode Vec<Status>");
        prop_assert_eq!(items, decoded);
    }
}
