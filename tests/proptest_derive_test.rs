//! Property-based tests for OxiCode derive macros using proptest.

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

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct SimplePoint {
    x: i32,
    y: i32,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct NamedValue {
    name: String,
    value: u64,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
enum SimpleStatus {
    Active,
    Inactive,
    Pending,
}

fn make_status(n: u8) -> SimpleStatus {
    match n {
        0 => SimpleStatus::Active,
        1 => SimpleStatus::Inactive,
        _ => SimpleStatus::Pending,
    }
}

proptest! {
    #[test]
    fn prop_simple_point_roundtrip(x: i32, y: i32) {
        let point = SimplePoint { x, y };
        let encoded = encode_to_vec(&point).expect("encode SimplePoint");
        let (decoded, _): (SimplePoint, usize) =
            decode_from_slice(&encoded).expect("decode SimplePoint");
        prop_assert_eq!(point, decoded);
    }
}

proptest! {
    #[test]
    fn prop_simple_point_consumed_equals_len(x: i32, y: i32) {
        let point = SimplePoint { x, y };
        let encoded = encode_to_vec(&point).expect("encode SimplePoint");
        let (_, consumed): (SimplePoint, usize) =
            decode_from_slice(&encoded).expect("decode SimplePoint");
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_named_value_roundtrip(name in any::<String>(), value in any::<u64>()) {
        let nv = NamedValue { name, value };
        let encoded = encode_to_vec(&nv).expect("encode NamedValue");
        let (decoded, _): (NamedValue, usize) =
            decode_from_slice(&encoded).expect("decode NamedValue");
        prop_assert_eq!(nv, decoded);
    }
}

proptest! {
    #[test]
    fn prop_named_value_consumed_equals_len(name in any::<String>(), value in any::<u64>()) {
        let nv = NamedValue { name, value };
        let encoded = encode_to_vec(&nv).expect("encode NamedValue");
        let (_, consumed): (NamedValue, usize) =
            decode_from_slice(&encoded).expect("decode NamedValue");
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_simple_status_roundtrip(n in 0u8..=2u8) {
        let status = make_status(n);
        let encoded = encode_to_vec(&status).expect("encode SimpleStatus");
        let (decoded, _): (SimpleStatus, usize) =
            decode_from_slice(&encoded).expect("decode SimpleStatus");
        prop_assert_eq!(status, decoded);
    }
}

proptest! {
    #[test]
    fn prop_all_simple_status_variants_roundtrip(n in 0u8..=2u8) {
        let status = make_status(n);
        let encoded = encode_to_vec(&status).expect("encode SimpleStatus variant");
        let (decoded, consumed): (SimpleStatus, usize) =
            decode_from_slice(&encoded).expect("decode SimpleStatus variant");
        prop_assert_eq!(status, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_simple_point_roundtrip(
        points in prop::collection::vec(
            (any::<i32>(), any::<i32>()).prop_map(|(x, y)| SimplePoint { x, y }),
            0..=8,
        )
    ) {
        let encoded = encode_to_vec(&points).expect("encode Vec<SimplePoint>");
        let (decoded, _): (Vec<SimplePoint>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<SimplePoint>");
        prop_assert_eq!(points, decoded);
    }
}

proptest! {
    #[test]
    fn prop_option_simple_point_roundtrip(x: i32, y: i32, present: bool) {
        let opt: Option<SimplePoint> = if present {
            Some(SimplePoint { x, y })
        } else {
            None
        };
        let encoded = encode_to_vec(&opt).expect("encode Option<SimplePoint>");
        let (decoded, _): (Option<SimplePoint>, usize) =
            decode_from_slice(&encoded).expect("decode Option<SimplePoint>");
        prop_assert_eq!(opt, decoded);
    }
}

proptest! {
    #[test]
    fn prop_option_named_value_roundtrip(
        name in any::<String>(),
        value in any::<u64>(),
        present: bool,
    ) {
        let opt: Option<NamedValue> = if present {
            Some(NamedValue { name, value })
        } else {
            None
        };
        let encoded = encode_to_vec(&opt).expect("encode Option<NamedValue>");
        let (decoded, _): (Option<NamedValue>, usize) =
            decode_from_slice(&encoded).expect("decode Option<NamedValue>");
        prop_assert_eq!(opt, decoded);
    }
}

proptest! {
    #[test]
    fn prop_tuple_of_simple_points_roundtrip(x1: i32, y1: i32, x2: i32, y2: i32) {
        let pair = (SimplePoint { x: x1, y: y1 }, SimplePoint { x: x2, y: y2 });
        let encoded = encode_to_vec(&pair).expect("encode (SimplePoint, SimplePoint)");
        #[allow(clippy::type_complexity)]
        let (decoded, _): ((SimplePoint, SimplePoint), usize) =
            decode_from_slice(&encoded).expect("decode (SimplePoint, SimplePoint)");
        prop_assert_eq!(pair, decoded);
    }
}

proptest! {
    #[test]
    fn prop_vec_named_value_roundtrip(
        items in prop::collection::vec(
            (any::<String>(), any::<u64>())
                .prop_map(|(name, value)| NamedValue { name, value }),
            0..=6,
        )
    ) {
        let encoded = encode_to_vec(&items).expect("encode Vec<NamedValue>");
        let (decoded, _): (Vec<NamedValue>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<NamedValue>");
        prop_assert_eq!(items, decoded);
    }
}

proptest! {
    #[test]
    fn prop_named_value_and_status_roundtrip(
        name in any::<String>(),
        value in any::<u64>(),
        n in 0u8..=2u8,
    ) {
        let nv = NamedValue { name, value };
        let status = make_status(n);
        let pair = (nv, status);
        let encoded = encode_to_vec(&pair).expect("encode (NamedValue, SimpleStatus)");
        #[allow(clippy::type_complexity)]
        let (decoded, _): ((NamedValue, SimpleStatus), usize) =
            decode_from_slice(&encoded).expect("decode (NamedValue, SimpleStatus)");
        prop_assert_eq!(pair, decoded);
    }
}

proptest! {
    #[test]
    fn prop_simple_point_consumed_wide_range(x in i32::MIN..=i32::MAX, y in i32::MIN..=i32::MAX) {
        let point = SimplePoint { x, y };
        let encoded = encode_to_vec(&point).expect("encode SimplePoint wide");
        let (_, consumed): (SimplePoint, usize) =
            decode_from_slice(&encoded).expect("decode SimplePoint wide");
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_named_value_empty_string_roundtrip(value in any::<u64>()) {
        let nv = NamedValue {
            name: String::new(),
            value,
        };
        let encoded = encode_to_vec(&nv).expect("encode NamedValue empty name");
        let (decoded, _): (NamedValue, usize) =
            decode_from_slice(&encoded).expect("decode NamedValue empty name");
        prop_assert_eq!(nv, decoded);
    }
}

proptest! {
    #[test]
    fn prop_named_value_long_string_roundtrip(base in any::<String>(), value in any::<u64>()) {
        let nv = NamedValue { name: base, value };
        let encoded = encode_to_vec(&nv).expect("encode NamedValue long name");
        let (decoded, _): (NamedValue, usize) =
            decode_from_slice(&encoded).expect("decode NamedValue long name");
        prop_assert_eq!(nv, decoded);
    }
}

proptest! {
    #[test]
    fn prop_vec_simple_status_roundtrip(
        ns in prop::collection::vec(0u8..=2u8, 0..=10)
    ) {
        let statuses: Vec<SimpleStatus> = ns.iter().map(|&n| make_status(n)).collect();
        let encoded = encode_to_vec(&statuses).expect("encode Vec<SimpleStatus>");
        let (decoded, _): (Vec<SimpleStatus>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<SimpleStatus>");
        prop_assert_eq!(statuses, decoded);
    }
}

proptest! {
    #[test]
    fn prop_option_vec_simple_point_roundtrip(
        points in prop::option::of(
            prop::collection::vec(
                (any::<i32>(), any::<i32>()).prop_map(|(x, y)| SimplePoint { x, y }),
                0..=5,
            )
        )
    ) {
        let encoded = encode_to_vec(&points).expect("encode Option<Vec<SimplePoint>>");
        let (decoded, _): (Option<Vec<SimplePoint>>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Vec<SimplePoint>>");
        prop_assert_eq!(points, decoded);
    }
}

proptest! {
    #[test]
    fn prop_vec_simple_point_and_named_value_roundtrip(
        points in prop::collection::vec(
            (any::<i32>(), any::<i32>()).prop_map(|(x, y)| SimplePoint { x, y }),
            0..=5,
        ),
        name in any::<String>(),
        value in any::<u64>(),
    ) {
        let nv = NamedValue { name, value };
        let pair = (points, nv);
        let encoded = encode_to_vec(&pair).expect("encode (Vec<SimplePoint>, NamedValue)");
        #[allow(clippy::type_complexity)]
        let (decoded, _): ((Vec<SimplePoint>, NamedValue), usize) =
            decode_from_slice(&encoded).expect("decode (Vec<SimplePoint>, NamedValue)");
        prop_assert_eq!(pair, decoded);
    }
}

proptest! {
    #[test]
    fn prop_simple_point_x_equals_y_roundtrip(v: i32) {
        let point = SimplePoint { x: v, y: v };
        let encoded = encode_to_vec(&point).expect("encode SimplePoint x==y");
        let (decoded, _): (SimplePoint, usize) =
            decode_from_slice(&encoded).expect("decode SimplePoint x==y");
        prop_assert_eq!(point, decoded);
    }
}

proptest! {
    #[test]
    fn prop_named_value_zero_value_roundtrip(name in any::<String>()) {
        let nv = NamedValue { name, value: 0 };
        let encoded = encode_to_vec(&nv).expect("encode NamedValue value=0");
        let (decoded, _): (NamedValue, usize) =
            decode_from_slice(&encoded).expect("decode NamedValue value=0");
        prop_assert_eq!(nv, decoded);
    }
}

proptest! {
    #[test]
    fn prop_named_value_max_value_roundtrip(name in any::<String>()) {
        let nv = NamedValue {
            name,
            value: u64::MAX,
        };
        let encoded = encode_to_vec(&nv).expect("encode NamedValue value=u64::MAX");
        let (decoded, _): (NamedValue, usize) =
            decode_from_slice(&encoded).expect("decode NamedValue value=u64::MAX");
        prop_assert_eq!(nv, decoded);
    }
}

proptest! {
    #[test]
    fn prop_vec_simple_point_consumed_equals_len(
        points in prop::collection::vec(
            (any::<i32>(), any::<i32>()).prop_map(|(x, y)| SimplePoint { x, y }),
            0..=8,
        )
    ) {
        let encoded = encode_to_vec(&points).expect("encode Vec<SimplePoint> for consumed check");
        let (_, consumed): (Vec<SimplePoint>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<SimplePoint> for consumed check");
        prop_assert_eq!(consumed, encoded.len());
    }
}
