//! Custom struct/enum property-based roundtrip tests using proptest
//! (split from proptest_test.rs).
//!
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
use core::cmp::Ordering;
use core::ops::ControlFlow;
use std::collections::LinkedList;

use oxicode::{decode_from_slice, decode_iter_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(value: &T) {
    let encoded = encode_to_vec(value).expect("encode failed");
    let (decoded, bytes_read): (T, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(value, &decoded, "roundtrip failed");
    assert_eq!(bytes_read, encoded.len(), "bytes_read mismatch");
}

// Derived struct roundtrip
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Record {
    id: u64,
    name: String,
    tags: Vec<String>,
    score: Option<f64>,
}

proptest! {
    #[test]
    fn prop_roundtrip_point(x: f32, y: f32) {
        prop_assume!(!x.is_nan() && !y.is_nan());
        roundtrip(&Point { x, y });
    }

    #[test]
    fn prop_roundtrip_record(
        id: u64,
        name: String,
        tags: Vec<String>,
        score in prop::option::of(prop::num::f64::NORMAL),
    ) {
        roundtrip(&Record { id, name, tags, score });
    }

    // Verify encoded_size matches actual encoded length
    #[test]
    fn prop_encoded_size_matches_encode_to_vec_u64(v: u64) {
        let size = oxicode::encoded_size(&v).expect("encoded_size failed");
        let bytes = encode_to_vec(&v).expect("encode_to_vec failed");
        prop_assert_eq!(size, bytes.len());
    }

    #[test]
    fn prop_encoded_size_matches_encode_to_vec_string(v: String) {
        let size = oxicode::encoded_size(&v).expect("encoded_size failed");
        let bytes = encode_to_vec(&v).expect("encode_to_vec failed");
        prop_assert_eq!(size, bytes.len());
    }

    #[test]
    fn prop_encoded_size_matches_encode_to_vec_vec_u32(v: Vec<u32>) {
        let size = oxicode::encoded_size(&v).expect("encoded_size failed");
        let bytes = encode_to_vec(&v).expect("encode_to_vec failed");
        prop_assert_eq!(size, bytes.len());
    }

    // Verify that decoding extra bytes doesn't corrupt things
    #[test]
    fn prop_extra_bytes_ignored(v: u64, extra: Vec<u8>) {
        let mut encoded = encode_to_vec(&v).expect("encode failed");
        let orig_len = encoded.len();
        encoded.extend_from_slice(&extra);
        let (decoded, bytes_read): (u64, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(bytes_read, orig_len);
    }
}

// ===== New types added in recent work =====

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct PropU64(u64);

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipProp {
    value: u32,
    #[oxicode(skip)]
    ignored: u64,
}

proptest! {
    // 1. core::cmp::Ordering roundtrip
    #[test]
    fn prop_roundtrip_ordering(val in prop::sample::select(vec![
        Ordering::Less, Ordering::Equal, Ordering::Greater
    ])) {
        let enc = encode_to_vec(&val).expect("encode");
        let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(val, dec);
    }

    // 2. core::ops::ControlFlow<String, u32> roundtrip
    #[test]
    fn prop_roundtrip_control_flow(is_continue: bool, u: u32, s in ".*") {
        let cf: ControlFlow<String, u32> = if is_continue {
            ControlFlow::Continue(u)
        } else {
            ControlFlow::Break(s)
        };
        let enc = encode_to_vec(&cf).expect("encode");
        let (dec, _): (ControlFlow<String, u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(cf, dec);
    }

    // 3. Box<str> roundtrip
    #[test]
    fn prop_roundtrip_box_str(s in ".*") {
        let original: Box<str> = s.into_boxed_str();
        let enc = encode_to_vec(&original).expect("encode");
        let (dec, _): (Box<str>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(&*original, &*dec);
    }

    // 4. Box<[u32]> roundtrip
    #[test]
    fn prop_roundtrip_box_slice_u32(v: Vec<u32>) {
        let original: Box<[u32]> = v.into_boxed_slice();
        let enc = encode_to_vec(&original).expect("encode");
        let (dec, _): (Box<[u32]>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(&*original, &*dec);
    }

    // 5. LinkedList<u32> roundtrip
    #[test]
    fn prop_roundtrip_linked_list_u32(v: Vec<u32>) {
        let original: LinkedList<u32> = v.into_iter().collect();
        let enc = encode_to_vec(&original).expect("encode");
        let (dec, _): (LinkedList<u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(original, dec);
    }

    // 6. #[oxicode(transparent)] newtype roundtrip
    #[test]
    fn prop_roundtrip_transparent(val: u64) {
        let wrapped = PropU64(val);
        let enc_wrapped = encode_to_vec(&wrapped).expect("encode wrapped");
        let enc_raw = encode_to_vec(&val).expect("encode raw");
        prop_assert_eq!(enc_wrapped.clone(), enc_raw, "transparent must match raw encoding");
        let (dec, _): (PropU64, _) = decode_from_slice(&enc_wrapped).expect("decode");
        prop_assert_eq!(dec.0, val);
    }

    // 7. #[oxicode(skip)] struct: encoding is identical regardless of skipped field value
    #[test]
    fn prop_skip_field_matches_without_ignored(value: u32) {
        let with_skip = SkipProp { value, ignored: u64::MAX };
        let without = SkipProp { value, ignored: 0 };
        let enc_skip = encode_to_vec(&with_skip).expect("encode with skip");
        let enc_no_skip = encode_to_vec(&without).expect("encode without");
        // Both should produce identical bytes (ignored field is never written)
        prop_assert_eq!(enc_skip, enc_no_skip, "skipped field must not affect encoding");
    }

    // 8. DecodeIter consistency: decode_iter_from_slice gives same items as decode_from_slice::<Vec<T>>
    #[test]
    fn prop_decode_iter_consistent_with_vec(items: Vec<u64>) {
        let enc = encode_to_vec(&items).expect("encode");
        let (vec_dec, _): (Vec<u64>, _) = decode_from_slice(&enc).expect("decode vec");
        let iter_dec: Vec<u64> = decode_iter_from_slice::<u64>(&enc)
            .expect("iter")
            .collect::<Result<Vec<_>, _>>()
            .expect("decode iter items");
        prop_assert_eq!(vec_dec, iter_dec);
    }
}

// --- Derive attribute property tests ---

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSkipDefault {
    name: String,
    #[oxicode(skip)]
    skipped: u32,
}

proptest! {
    #[test]
    fn prop_skip_field_default_roundtrip(
        name in proptest::string::string_regex("[a-z]{1,20}").unwrap(),
        value in 0u32..u32::MAX
    ) {
        let original = WithSkipDefault { name: name.clone(), skipped: value };
        let enc = encode_to_vec(&original).expect("encode");
        let (dec, _): (WithSkipDefault, _) = decode_from_slice(&enc).expect("decode");
        // name is preserved, skipped becomes 0 (Default)
        prop_assert_eq!(&dec.name, &name);
        prop_assert_eq!(dec.skipped, 0u32);
    }
}
