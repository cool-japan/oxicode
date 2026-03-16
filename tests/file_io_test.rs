//! Integration tests for file I/O convenience functions

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
#[cfg(feature = "std")]
mod file_io_tests {
    use oxicode::{Decode, Encode};
    use std::env::temp_dir;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TestData {
        id: u64,
        name: String,
        values: Vec<f32>,
    }

    fn test_data() -> TestData {
        TestData {
            id: 42,
            name: "test_file_io".to_string(),
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        }
    }

    #[test]
    fn test_encode_decode_file_roundtrip() {
        let data = test_data();
        let path = temp_dir().join("oxicode_test_roundtrip.bin");

        oxicode::encode_to_file(&data, &path).expect("encode_to_file failed");
        let decoded: TestData = oxicode::decode_from_file(&path).expect("decode_from_file failed");

        assert_eq!(data, decoded);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_file_contents_match_encode_to_vec() {
        let data = test_data();
        let path = temp_dir().join("oxicode_test_contents.bin");

        oxicode::encode_to_file(&data, &path).expect("encode_to_file failed");
        let file_bytes = std::fs::read(&path).expect("read failed");
        let vec_bytes = oxicode::encode_to_vec(&data).expect("encode_to_vec failed");

        assert_eq!(file_bytes, vec_bytes);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_with_legacy_config() {
        use oxicode::config;

        let data = test_data();
        let path = temp_dir().join("oxicode_test_legacy.bin");

        oxicode::encode_to_file_with_config(&data, &path, config::legacy()).expect("encode failed");
        let decoded: TestData =
            oxicode::decode_from_file_with_config(&path, config::legacy()).expect("decode failed");

        assert_eq!(data, decoded);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_decode_nonexistent_file_error() {
        let path = temp_dir().join("oxicode_nonexistent_xyz_abc.bin");
        let result = oxicode::decode_from_file::<TestData>(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_overwrite_existing_file() {
        let path = temp_dir().join("oxicode_test_overwrite.bin");

        // Write first value
        let first: u32 = 100;
        oxicode::encode_to_file(&first, &path).expect("first encode failed");

        // Overwrite with second value
        let second: u32 = 200;
        oxicode::encode_to_file(&second, &path).expect("second encode failed");

        let decoded: u32 = oxicode::decode_from_file(&path).expect("decode failed");
        assert_eq!(second, decoded);
        std::fs::remove_file(&path).ok();
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LargeRecord {
        id: u64,
        data: Vec<u8>,
        label: String,
    }

    #[test]
    fn test_large_file_roundtrip() {
        let record = LargeRecord {
            id: 42,
            data: (0u8..=255).cycle().take(50_000).collect(),
            label: "x".repeat(10_000),
        };
        let path = temp_dir().join("oxicode_test_large_file.bin");

        oxicode::encode_to_file(&record, &path).expect("encode large file");
        let loaded: LargeRecord = oxicode::decode_from_file(&path).expect("decode large file");

        assert_eq!(record.id, loaded.id);
        assert_eq!(record.data.len(), loaded.data.len());
        assert_eq!(record.label.len(), loaded.label.len());
        assert_eq!(record, loaded);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_multiple_sequential_writes() {
        let path = temp_dir().join("oxicode_test_sequential_writes.bin");

        for val in [111u64, 222u64, 333u64] {
            oxicode::encode_to_file(&val, &path).expect("sequential encode");
        }

        let decoded: u64 =
            oxicode::decode_from_file(&path).expect("decode after sequential writes");
        assert_eq!(333u64, decoded);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_various_types_roundtrip() {
        let base = "oxicode_test_various";

        let v_u8: u8 = 255;
        let p = temp_dir().join(format!("{base}_u8.bin"));
        oxicode::encode_to_file(&v_u8, &p).expect("encode u8");
        let r: u8 = oxicode::decode_from_file(&p).expect("decode u8");
        assert_eq!(v_u8, r);
        std::fs::remove_file(&p).ok();

        let v_u64: u64 = u64::MAX;
        let p = temp_dir().join(format!("{base}_u64.bin"));
        oxicode::encode_to_file(&v_u64, &p).expect("encode u64");
        let r: u64 = oxicode::decode_from_file(&p).expect("decode u64");
        assert_eq!(v_u64, r);
        std::fs::remove_file(&p).ok();

        let v_f64: f64 = std::f64::consts::PI;
        let p = temp_dir().join(format!("{base}_f64.bin"));
        oxicode::encode_to_file(&v_f64, &p).expect("encode f64");
        let r: f64 = oxicode::decode_from_file(&p).expect("decode f64");
        assert_eq!(v_f64.to_bits(), r.to_bits());
        std::fs::remove_file(&p).ok();

        let v_bool = true;
        let p = temp_dir().join(format!("{base}_bool.bin"));
        oxicode::encode_to_file(&v_bool, &p).expect("encode bool");
        let r: bool = oxicode::decode_from_file(&p).expect("decode bool");
        assert_eq!(v_bool, r);
        std::fs::remove_file(&p).ok();

        let v_str = "hello oxicode".to_string();
        let p = temp_dir().join(format!("{base}_string.bin"));
        oxicode::encode_to_file(&v_str, &p).expect("encode string");
        let r: String = oxicode::decode_from_file(&p).expect("decode string");
        assert_eq!(v_str, r);
        std::fs::remove_file(&p).ok();

        let v_vec: Vec<u32> = (0..100).collect();
        let p = temp_dir().join(format!("{base}_vec.bin"));
        oxicode::encode_to_file(&v_vec, &p).expect("encode vec");
        let r: Vec<u32> = oxicode::decode_from_file(&p).expect("decode vec");
        assert_eq!(v_vec, r);
        std::fs::remove_file(&p).ok();
    }
}
