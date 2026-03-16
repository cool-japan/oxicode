//! Advanced roundtrip tests for smart pointer types: Box, Rc, Arc, Cow.
//!
//! Covers Box<T>, Box<[T]>, Box<str>, Rc<T>, Rc<[T]>, Rc<str>,
//! Arc<T>, Arc<[T]>, Arc<str>, Cow<str>, Cow<[u8]>, nested pointers,
//! and wire-format compatibility assertions.

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
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

mod smart_pointer_advanced_tests {
    use super::*;
    use std::f64::consts::{E, PI};

    // ===== Test 1: Box<u32> roundtrip =====

    #[test]
    fn test_box_u32_roundtrip() {
        let original: Box<u32> = Box::new(4_294_967_295_u32);
        let enc = encode_to_vec(&original).expect("encode Box<u32>");
        let (dec, _): (Box<u32>, _) = decode_from_slice(&enc).expect("decode Box<u32>");
        assert_eq!(original, dec);
    }

    // ===== Test 2: Box<String> roundtrip =====

    #[allow(clippy::box_collection)]
    #[test]
    fn test_box_string_roundtrip() {
        let original: Box<String> = Box::new("smart pointer advanced test".to_string());
        let enc = encode_to_vec(&original).expect("encode Box<String>");
        let (dec, _): (Box<String>, _) = decode_from_slice(&enc).expect("decode Box<String>");
        assert_eq!(original, dec);
    }

    // ===== Test 3: Box<Vec<u8>> roundtrip =====

    #[allow(clippy::box_collection)]
    #[test]
    fn test_box_vec_u8_roundtrip() {
        let original: Box<Vec<u8>> = Box::new(vec![0u8, 1, 2, 64, 128, 200, 255]);
        let enc = encode_to_vec(&original).expect("encode Box<Vec<u8>>");
        let (dec, _): (Box<Vec<u8>>, _) = decode_from_slice(&enc).expect("decode Box<Vec<u8>>");
        assert_eq!(original, dec);
    }

    // ===== Test 4: Box<[u8]> roundtrip (byte slice box) =====

    #[test]
    fn test_box_u8_slice_roundtrip() {
        let original: Box<[u8]> =
            vec![10u8, 20, 30, 40, 50, 60, 70, 80, 90, 100].into_boxed_slice();
        let enc = encode_to_vec(&original).expect("encode Box<[u8]>");
        let (dec, _): (Box<[u8]>, _) = decode_from_slice(&enc).expect("decode Box<[u8]>");
        assert_eq!(&*original, &*dec);
    }

    // ===== Test 5: Box<str> roundtrip =====

    #[test]
    fn test_box_str_roundtrip() {
        let original: Box<str> = "oxicode box str advanced".into();
        let enc = encode_to_vec(&original).expect("encode Box<str>");
        let (dec, _): (Box<str>, _) = decode_from_slice(&enc).expect("decode Box<str>");
        assert_eq!(&*original, &*dec);
    }

    // ===== Test 6: Rc<u32> roundtrip =====

    #[test]
    fn test_rc_u32_roundtrip() {
        let original: Rc<u32> = Rc::new(1_234_567_u32);
        let enc = encode_to_vec(&original).expect("encode Rc<u32>");
        let (dec, _): (Rc<u32>, _) = decode_from_slice(&enc).expect("decode Rc<u32>");
        assert_eq!(*original, *dec);
    }

    // ===== Test 7: Rc<String> roundtrip =====

    #[allow(clippy::box_collection)]
    #[test]
    fn test_rc_string_roundtrip() {
        let original: Rc<String> = Rc::new("rc string advanced".to_string());
        let enc = encode_to_vec(&original).expect("encode Rc<String>");
        let (dec, _): (Rc<String>, _) = decode_from_slice(&enc).expect("decode Rc<String>");
        assert_eq!(*original, *dec);
    }

    // ===== Test 8: Rc<[u8]> roundtrip =====

    #[test]
    fn test_rc_u8_slice_roundtrip() {
        let original: Rc<[u8]> = Rc::from(vec![5u8, 10, 15, 20, 25, 30].as_slice());
        let enc = encode_to_vec(&original).expect("encode Rc<[u8]>");
        let (dec, _): (Rc<[u8]>, _) = decode_from_slice(&enc).expect("decode Rc<[u8]>");
        assert_eq!(&*original, &*dec);
    }

    // ===== Test 9: Rc<str> roundtrip =====

    #[test]
    fn test_rc_str_roundtrip() {
        let original: Rc<str> = Rc::from("rc str slice roundtrip");
        let enc = encode_to_vec(&original).expect("encode Rc<str>");
        let (dec, _): (Rc<str>, _) = decode_from_slice(&enc).expect("decode Rc<str>");
        assert_eq!(&*original, &*dec);
    }

    // ===== Test 10: Arc<u32> roundtrip =====

    #[test]
    fn test_arc_u32_roundtrip() {
        let original: Arc<u32> = Arc::new(99_999_999_u32);
        let enc = encode_to_vec(&original).expect("encode Arc<u32>");
        let (dec, _): (Arc<u32>, _) = decode_from_slice(&enc).expect("decode Arc<u32>");
        assert_eq!(*original, *dec);
    }

    // ===== Test 11: Arc<String> roundtrip =====

    #[allow(clippy::box_collection)]
    #[test]
    fn test_arc_string_roundtrip() {
        let original: Arc<String> = Arc::new("arc string advanced roundtrip".to_string());
        let enc = encode_to_vec(&original).expect("encode Arc<String>");
        let (dec, _): (Arc<String>, _) = decode_from_slice(&enc).expect("decode Arc<String>");
        assert_eq!(*original, *dec);
    }

    // ===== Test 12: Arc<[u8]> roundtrip =====

    #[test]
    fn test_arc_u8_slice_roundtrip() {
        let original: Arc<[u8]> = Arc::from(vec![255u8, 128, 64, 32, 16, 8, 4, 2, 1, 0].as_slice());
        let enc = encode_to_vec(&original).expect("encode Arc<[u8]>");
        let (dec, _): (Arc<[u8]>, _) = decode_from_slice(&enc).expect("decode Arc<[u8]>");
        assert_eq!(&*original, &*dec);
    }

    // ===== Test 13: Arc<str> roundtrip =====

    #[test]
    fn test_arc_str_roundtrip() {
        let original: Arc<str> = Arc::from("arc str advanced roundtrip test");
        let enc = encode_to_vec(&original).expect("encode Arc<str>");
        let (dec, _): (Arc<str>, _) = decode_from_slice(&enc).expect("decode Arc<str>");
        assert_eq!(&*original, &*dec);
    }

    // ===== Test 14: Cow<str> owned variant roundtrip =====

    #[test]
    fn test_cow_str_owned_roundtrip() {
        let original: Cow<str> = Cow::Owned("cow owned str roundtrip".to_string());
        let enc = encode_to_vec(&original).expect("encode Cow<str> owned");
        let (dec, _): (Cow<str>, _) = decode_from_slice(&enc).expect("decode Cow<str> owned");
        assert_eq!(&*original, &*dec);
    }

    // ===== Test 15: Cow<[u8]> owned variant roundtrip =====

    #[test]
    fn test_cow_u8_slice_owned_roundtrip() {
        let original: Cow<[u8]> = Cow::Owned(vec![1u8, 3, 5, 7, 9, 11, 13, 15]);
        let enc = encode_to_vec(&original).expect("encode Cow<[u8]> owned");
        let (dec, _): (Cow<[u8]>, _) = decode_from_slice(&enc).expect("decode Cow<[u8]> owned");
        assert_eq!(&*original, &*dec);
    }

    // ===== Test 16: Box<struct_with_pi> - derived struct in Box =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PhysicsConstants {
        pi: f64,
        e: f64,
        scale: u32,
        label: String,
    }

    #[test]
    fn test_box_derived_struct_with_pi_e_roundtrip() {
        let original: Box<PhysicsConstants> = Box::new(PhysicsConstants {
            pi: PI,
            e: E,
            scale: 1_000_000,
            label: "physics constants box".to_string(),
        });
        let enc = encode_to_vec(&original).expect("encode Box<PhysicsConstants>");
        let (dec, _): (Box<PhysicsConstants>, _) =
            decode_from_slice(&enc).expect("decode Box<PhysicsConstants>");
        assert_eq!(original.pi, dec.pi);
        assert_eq!(original.e, dec.e);
        assert_eq!(original.scale, dec.scale);
        assert_eq!(original.label, dec.label);
        // Sanity check the constants were not mangled
        assert!((dec.pi - PI).abs() < f64::EPSILON);
        assert!((dec.e - E).abs() < f64::EPSILON);
    }

    // ===== Test 17: Nested Box<Box<u32>> roundtrip =====

    #[test]
    fn test_nested_box_box_u32_roundtrip() {
        let original: Box<Box<u32>> = Box::new(Box::new(42_u32));
        let enc = encode_to_vec(&original).expect("encode Box<Box<u32>>");
        let (dec, _): (Box<Box<u32>>, _) = decode_from_slice(&enc).expect("decode Box<Box<u32>>");
        assert_eq!(**original, **dec);
    }

    // ===== Test 18: Vec<Box<String>> roundtrip =====

    #[allow(clippy::box_collection)]
    #[test]
    fn test_vec_box_string_roundtrip() {
        let original: Vec<Box<String>> = vec![
            Box::new("first element".to_string()),
            Box::new("second element".to_string()),
            Box::new(String::new()),
            Box::new("fourth element with unicode: αβγδ".to_string()),
        ];
        let enc = encode_to_vec(&original).expect("encode Vec<Box<String>>");
        let (dec, _): (Vec<Box<String>>, _) =
            decode_from_slice(&enc).expect("decode Vec<Box<String>>");
        assert_eq!(original.len(), dec.len());
        for (a, b) in original.iter().zip(dec.iter()) {
            assert_eq!(**a, **b);
        }
    }

    // ===== Test 19: Box produces same bytes as plain value =====

    #[test]
    fn test_box_produces_same_bytes_as_plain_value() {
        let plain: u32 = 987_654_321_u32;
        let boxed: Box<u32> = Box::new(plain);

        let enc_plain = encode_to_vec(&plain).expect("encode plain u32");
        let enc_boxed = encode_to_vec(&boxed).expect("encode Box<u32>");

        assert_eq!(
            enc_plain, enc_boxed,
            "Box<u32> must produce identical wire bytes to plain u32"
        );

        // Also verify with str types
        let plain_str = String::from("wire-format-check");
        let boxed_str: Box<str> = plain_str.as_str().into();

        let enc_plain_str = encode_to_vec(&plain_str).expect("encode plain String");
        let enc_boxed_str = encode_to_vec(&boxed_str).expect("encode Box<str>");

        assert_eq!(
            enc_plain_str, enc_boxed_str,
            "Box<str> must produce identical wire bytes to String for the same content"
        );
    }

    // ===== Test 20: Arc produces same bytes as plain value =====

    #[test]
    fn test_arc_produces_same_bytes_as_plain_value() {
        let plain: u64 = 1_234_567_890_123_456_789_u64;
        let arc: Arc<u64> = Arc::new(plain);

        let enc_plain = encode_to_vec(&plain).expect("encode plain u64");
        let enc_arc = encode_to_vec(&arc).expect("encode Arc<u64>");

        assert_eq!(
            enc_plain, enc_arc,
            "Arc<u64> must produce identical wire bytes to plain u64"
        );

        // Also verify with byte-slice types
        let plain_vec: Vec<u8> = vec![42u8, 84, 126, 168, 210];
        let arc_slice: Arc<[u8]> = Arc::from(plain_vec.as_slice());

        let enc_plain_vec = encode_to_vec(&plain_vec).expect("encode plain Vec<u8>");
        let enc_arc_slice = encode_to_vec(&arc_slice).expect("encode Arc<[u8]>");

        assert_eq!(
            enc_plain_vec, enc_arc_slice,
            "Arc<[u8]> must produce identical wire bytes to Vec<u8> for the same content"
        );
    }
}
