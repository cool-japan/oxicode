//! Advanced roundtrip tests for special Rust types.
//! This module focuses on distinct values and type combinations
//! not exhaustively covered by the other test suites.

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
mod special_types_advanced_tests {
    use oxicode::{decode_from_slice, encode_to_vec};
    use std::cell::{Cell, RefCell};
    use std::cmp::{Ordering, Reverse};
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::mem::ManuallyDrop;
    use std::num::Wrapping;
    use std::ops::{Bound, ControlFlow, Range, RangeInclusive};
    use std::path::PathBuf;
    use std::time::Duration;

    // ===== 1. Ordering::Less roundtrip =====

    #[test]
    fn test_ordering_less_roundtrip() {
        let original = Ordering::Less;
        let bytes = encode_to_vec(&original).expect("Failed to encode Ordering::Less");
        let (decoded, _): (Ordering, _) =
            decode_from_slice(&bytes).expect("Failed to decode Ordering::Less");
        assert_eq!(original, decoded, "Ordering::Less must survive roundtrip");
    }

    // ===== 2. Ordering::Equal roundtrip =====

    #[test]
    fn test_ordering_equal_roundtrip() {
        let original = Ordering::Equal;
        let bytes = encode_to_vec(&original).expect("Failed to encode Ordering::Equal");
        let (decoded, _): (Ordering, _) =
            decode_from_slice(&bytes).expect("Failed to decode Ordering::Equal");
        assert_eq!(original, decoded, "Ordering::Equal must survive roundtrip");
    }

    // ===== 3. Ordering::Greater roundtrip =====

    #[test]
    fn test_ordering_greater_roundtrip() {
        let original = Ordering::Greater;
        let bytes = encode_to_vec(&original).expect("Failed to encode Ordering::Greater");
        let (decoded, _): (Ordering, _) =
            decode_from_slice(&bytes).expect("Failed to decode Ordering::Greater");
        assert_eq!(
            original, decoded,
            "Ordering::Greater must survive roundtrip"
        );
    }

    // ===== 4. Infallible via Result<T, Infallible> =====
    // Infallible cannot be instantiated; test it as the error arm of Result.

    #[test]
    fn test_infallible_as_result_error_type() {
        let ok_value: Result<u64, Infallible> = Ok(0xDEAD_BEEF_CAFE_u64);
        let bytes = encode_to_vec(&ok_value).expect("Failed to encode Result<u64, Infallible>");
        let (decoded, _): (Result<u64, Infallible>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Result<u64, Infallible>");
        assert_eq!(
            ok_value, decoded,
            "Result<u64, Infallible>::Ok must survive roundtrip"
        );
    }

    // ===== 5. ControlFlow<u32, String>::Continue("ok") roundtrip =====

    #[test]
    fn test_control_flow_continue_string_roundtrip() {
        let cf: ControlFlow<u32, String> = ControlFlow::Continue("ok".to_string());
        let bytes = encode_to_vec(&cf).expect("Failed to encode ControlFlow::Continue");
        let (decoded, _): (ControlFlow<u32, String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode ControlFlow::Continue");
        assert_eq!(
            cf, decoded,
            "ControlFlow::Continue(\"ok\") must survive roundtrip"
        );
    }

    // ===== 6. ControlFlow<u32, String>::Break(42) roundtrip =====

    #[test]
    fn test_control_flow_break_u32_roundtrip() {
        let cf: ControlFlow<u32, String> = ControlFlow::Break(42u32);
        let bytes = encode_to_vec(&cf).expect("Failed to encode ControlFlow::Break");
        let (decoded, _): (ControlFlow<u32, String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode ControlFlow::Break");
        assert_eq!(cf, decoded, "ControlFlow::Break(42) must survive roundtrip");
    }

    // ===== 7. Wrapping<u32> at MAX roundtrip =====

    #[test]
    fn test_wrapping_u32_max_roundtrip() {
        let original = Wrapping(u32::MAX);
        let bytes = encode_to_vec(&original).expect("Failed to encode Wrapping<u32>::MAX");
        let (decoded, _): (Wrapping<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Wrapping<u32>::MAX");
        assert_eq!(
            original, decoded,
            "Wrapping(u32::MAX) must survive roundtrip"
        );
    }

    // ===== 8. Wrapping<i64> at MIN roundtrip =====

    #[test]
    fn test_wrapping_i64_min_roundtrip() {
        let original = Wrapping(i64::MIN);
        let bytes = encode_to_vec(&original).expect("Failed to encode Wrapping<i64>::MIN");
        let (decoded, _): (Wrapping<i64>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Wrapping<i64>::MIN");
        assert_eq!(
            original, decoded,
            "Wrapping(i64::MIN) must survive roundtrip"
        );
    }

    // ===== 9. Reverse<u32> roundtrip =====

    #[test]
    fn test_reverse_u32_max_roundtrip() {
        let original = Reverse(u32::MAX);
        let bytes = encode_to_vec(&original).expect("Failed to encode Reverse<u32>");
        let (decoded, _): (Reverse<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Reverse<u32>");
        assert_eq!(
            original, decoded,
            "Reverse(u32::MAX) must survive roundtrip"
        );
    }

    // ===== 10. Cell<u32> roundtrip =====

    #[test]
    fn test_cell_u32_max_roundtrip() {
        let original = Cell::new(u32::MAX);
        let bytes = encode_to_vec(&original).expect("Failed to encode Cell<u32>");
        let (decoded, _): (Cell<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Cell<u32>");
        assert_eq!(
            original.get(),
            decoded.get(),
            "Cell(u32::MAX) must survive roundtrip"
        );
    }

    // ===== 11. RefCell<String> roundtrip =====

    #[test]
    fn test_refcell_string_advanced_roundtrip() {
        let original = RefCell::new("oxicode advanced test".to_string());
        let bytes = encode_to_vec(&original).expect("Failed to encode RefCell<String>");
        let (decoded, _): (RefCell<String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode RefCell<String>");
        assert_eq!(
            *original.borrow(),
            *decoded.borrow(),
            "RefCell<String> must survive roundtrip"
        );
    }

    // ===== 12. ManuallyDrop<u64> roundtrip =====

    #[test]
    fn test_manually_drop_u64_roundtrip() {
        let original = ManuallyDrop::new(u64::MAX);
        let bytes = encode_to_vec(&original).expect("Failed to encode ManuallyDrop<u64>");
        let (decoded, _): (ManuallyDrop<u64>, _) =
            decode_from_slice(&bytes).expect("Failed to decode ManuallyDrop<u64>");
        assert_eq!(
            *original, *decoded,
            "ManuallyDrop<u64> must survive roundtrip"
        );
    }

    // ===== 13. PhantomData<String> roundtrip =====

    #[test]
    fn test_phantom_data_string_roundtrip() {
        let original: PhantomData<String> = PhantomData;
        let bytes = encode_to_vec(&original).expect("Failed to encode PhantomData<String>");
        let (decoded, _): (PhantomData<String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode PhantomData<String>");
        // PhantomData is a zero-size type; equality holds trivially.
        let _: (PhantomData<String>, PhantomData<String>) = (original, decoded);
    }

    // ===== 14. Range<u32> roundtrip =====

    #[test]
    fn test_range_u32_advanced_roundtrip() {
        let original: Range<u32> = Range {
            start: 0,
            end: u32::MAX,
        };
        let bytes = encode_to_vec(&original).expect("Failed to encode Range<u32>");
        let (decoded, _): (Range<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Range<u32>");
        assert_eq!(
            original, decoded,
            "Range<u32>(0..u32::MAX) must survive roundtrip"
        );
    }

    // ===== 15. RangeInclusive<u32> roundtrip =====

    #[test]
    fn test_range_inclusive_u32_advanced_roundtrip() {
        let original: RangeInclusive<u32> = RangeInclusive::new(0, u32::MAX);
        let bytes = encode_to_vec(&original).expect("Failed to encode RangeInclusive<u32>");
        let (decoded, _): (RangeInclusive<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode RangeInclusive<u32>");
        assert_eq!(
            original, decoded,
            "RangeInclusive<u32>(0..=u32::MAX) must survive roundtrip"
        );
    }

    // ===== 16. Bound<u32>::Included roundtrip =====

    #[test]
    fn test_bound_included_u32_advanced_roundtrip() {
        let original: Bound<u32> = Bound::Included(u32::MAX);
        let bytes = encode_to_vec(&original).expect("Failed to encode Bound::Included");
        let (decoded, _): (Bound<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Bound::Included");
        assert_eq!(
            original, decoded,
            "Bound::Included(u32::MAX) must survive roundtrip"
        );
    }

    // ===== 17. Bound<u32>::Excluded roundtrip =====

    #[test]
    fn test_bound_excluded_u32_advanced_roundtrip() {
        let original: Bound<u32> = Bound::Excluded(0u32);
        let bytes = encode_to_vec(&original).expect("Failed to encode Bound::Excluded");
        let (decoded, _): (Bound<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Bound::Excluded");
        assert_eq!(
            original, decoded,
            "Bound::Excluded(0) must survive roundtrip"
        );
    }

    // ===== 18. Bound<u32>::Unbounded roundtrip =====

    #[test]
    fn test_bound_unbounded_u32_advanced_roundtrip() {
        let original: Bound<u32> = Bound::Unbounded;
        let bytes = encode_to_vec(&original).expect("Failed to encode Bound::Unbounded");
        let (decoded, _): (Bound<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Bound::Unbounded");
        assert_eq!(original, decoded, "Bound::Unbounded must survive roundtrip");
    }

    // ===== 19. OsString roundtrip (ASCII content) =====

    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn test_osstring_ascii_advanced_roundtrip() {
        use std::ffi::OsString;
        let original = OsString::from("oxicode/special_types/advanced");
        let bytes = encode_to_vec(&original).expect("Failed to encode OsString");
        let (decoded, _): (OsString, _) =
            decode_from_slice(&bytes).expect("Failed to decode OsString");
        assert_eq!(original, decoded, "OsString (ASCII) must survive roundtrip");
    }

    // ===== 20. PathBuf roundtrip =====

    #[test]
    fn test_pathbuf_advanced_roundtrip() {
        let original = PathBuf::from("/usr/share/oxicode/advanced/data.bin");
        let bytes = encode_to_vec(&original).expect("Failed to encode PathBuf");
        let (decoded, _): (PathBuf, _) =
            decode_from_slice(&bytes).expect("Failed to decode PathBuf");
        assert_eq!(original, decoded, "PathBuf must survive roundtrip");
    }

    // ===== 21. Duration::from_millis(1000) roundtrip =====

    #[test]
    fn test_duration_from_millis_1000_roundtrip() {
        let original = Duration::from_millis(1000);
        let bytes = encode_to_vec(&original).expect("Failed to encode Duration::from_millis(1000)");
        let (decoded, _): (Duration, _) =
            decode_from_slice(&bytes).expect("Failed to decode Duration::from_millis(1000)");
        assert_eq!(
            original, decoded,
            "Duration::from_millis(1000) must survive roundtrip"
        );
        // Verify the decoded value equals exactly one second.
        assert_eq!(decoded.as_secs(), 1, "1000 ms must equal 1 second");
        assert_eq!(
            decoded.subsec_nanos(),
            0,
            "1000 ms must have zero subsecond nanos"
        );
    }

    // ===== 22. Vec of Ordering values [Less, Equal, Greater, Less] roundtrip =====

    #[test]
    fn test_vec_of_orderings_roundtrip() {
        let original = vec![
            Ordering::Less,
            Ordering::Equal,
            Ordering::Greater,
            Ordering::Less,
        ];
        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Ordering>");
        let (decoded, _): (Vec<Ordering>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<Ordering>");
        assert_eq!(original, decoded, "Vec<Ordering> must survive roundtrip");
        assert_eq!(decoded.len(), 4, "Decoded Vec must have 4 elements");
    }
}
