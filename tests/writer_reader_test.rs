//! Tests for encode_to_writer and decode_from_reader convenience functions.

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
use oxicode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Sample {
    id: u32,
    name: String,
    values: Vec<f64>,
}

#[test]
fn test_encode_to_writer_cursor() {
    let s = Sample {
        id: 42,
        name: "hello".to_string(),
        values: vec![1.0, 2.0, 3.0],
    };
    let mut buf = Vec::new();
    let n = oxicode::encode_to_writer(&s, &mut buf).expect("encode");
    assert!(n > 0);
    assert_eq!(n, buf.len());
}

#[test]
fn test_decode_from_reader_cursor() {
    let s = Sample {
        id: 99,
        name: "world".to_string(),
        values: vec![4.0, 5.0],
    };
    let buf = oxicode::encode_to_vec(&s).expect("encode");
    let cursor = std::io::Cursor::new(&buf);
    let (decoded, n): (Sample, _) = oxicode::decode_from_reader(cursor).expect("decode");
    assert_eq!(s, decoded);
    assert_eq!(n, buf.len());
}

#[test]
fn test_writer_reader_roundtrip_file() {
    let s = Sample {
        id: 1,
        name: "file_test".to_string(),
        values: vec![0.1, 0.2],
    };
    let dir = std::env::temp_dir();
    let path = dir.join("oxicode_writer_reader_test.bin");
    {
        let f = std::fs::File::create(&path).expect("create");
        oxicode::encode_to_writer(&s, f).expect("encode");
    }
    {
        let f = std::fs::File::open(&path).expect("open");
        let (decoded, _): (Sample, _) = oxicode::decode_from_reader(f).expect("decode");
        assert_eq!(s, decoded);
    }
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_encode_to_vec_with_size_hint() {
    let s = Sample {
        id: 7,
        name: "hint".to_string(),
        values: vec![1.5; 10],
    };
    let buf1 = oxicode::encode_to_vec(&s).expect("encode");
    let buf2 = oxicode::encode_to_vec_with_size_hint(&s, 128).expect("encode_hint");
    assert_eq!(buf1, buf2);
}

#[test]
fn test_encode_to_vec_with_size_hint_undersized() {
    // Even with a bad hint (too small), should work correctly
    let s = Sample {
        id: 1,
        name: "x".repeat(1000),
        values: vec![1.0; 100],
    };
    let buf1 = oxicode::encode_to_vec(&s).expect("encode");
    let buf2 = oxicode::encode_to_vec_with_size_hint(&s, 1).expect("encode_hint");
    assert_eq!(buf1, buf2);
}

#[test]
fn test_encode_to_writer_with_config_standard() {
    let s = Sample {
        id: 10,
        name: "cfg_test".to_string(),
        values: vec![1.0, 2.0],
    };
    let mut buf = Vec::new();
    let n = oxicode::encode_to_writer_with_config(&s, &mut buf, oxicode::config::standard())
        .expect("encode_to_writer_with_config");
    assert!(n > 0);
    assert_eq!(n, buf.len());
}

#[test]
fn test_encode_to_writer_with_config_matches_encode_to_vec() {
    #[allow(clippy::approx_constant)]
    let s = Sample {
        id: 77,
        name: "match_test".to_string(),
        values: vec![3.14],
    };
    let config = oxicode::config::standard();

    let vec_bytes = oxicode::encode_to_vec_with_config(&s, config).expect("encode_to_vec");
    let mut writer_bytes = Vec::new();
    oxicode::encode_to_writer_with_config(&s, &mut writer_bytes, config)
        .expect("encode_to_writer_with_config");

    assert_eq!(vec_bytes, writer_bytes);
}

#[test]
fn test_decode_from_reader_with_config_roundtrip() {
    let s = Sample {
        id: 55,
        name: "decode_cfg".to_string(),
        values: vec![9.0, 8.0, 7.0],
    };
    let config = oxicode::config::standard();

    let encoded = oxicode::encode_to_vec_with_config(&s, config).expect("encode");
    let cursor = std::io::Cursor::new(&encoded);
    let (decoded, n): (Sample, _) =
        oxicode::decode_from_reader_with_config(cursor, config).expect("decode");
    assert_eq!(s, decoded);
    assert_eq!(n, encoded.len());
}

#[test]
fn test_decode_from_reader_with_config_fixed_int() {
    let config = oxicode::config::standard().with_fixed_int_encoding();
    let val: u32 = 12345;

    let encoded = oxicode::encode_to_vec_with_config(&val, config).expect("encode");
    assert_eq!(encoded.len(), 4, "fixed int u32 should always be 4 bytes");

    let cursor = std::io::Cursor::new(&encoded);
    let (decoded, n): (u32, _) =
        oxicode::decode_from_reader_with_config(cursor, config).expect("decode");
    assert_eq!(decoded, val);
    assert_eq!(n, 4);
}

#[test]
fn test_encode_to_writer_with_config_fixed_int_matches_vec() {
    let config = oxicode::config::standard().with_fixed_int_encoding();
    let val: u64 = u64::MAX;

    let vec_bytes = oxicode::encode_to_vec_with_config(&val, config).expect("encode_to_vec");
    let mut writer_bytes = Vec::new();
    oxicode::encode_to_writer_with_config(&val, &mut writer_bytes, config)
        .expect("encode_to_writer_with_config");

    assert_eq!(vec_bytes, writer_bytes);
    assert_eq!(writer_bytes.len(), 8, "u64 with fixed encoding is 8 bytes");
}
