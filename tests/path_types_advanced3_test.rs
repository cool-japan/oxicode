#![cfg(feature = "std")]
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
use std::path::PathBuf;

#[test]
fn test_pathbuf_simple_path_roundtrip() {
    let path = PathBuf::from("/tmp/test.bin");
    let encoded = encode_to_vec(&path).expect("Failed to encode simple path");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode simple path");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_empty_path_roundtrip() {
    let path = PathBuf::new();
    let encoded = encode_to_vec(&path).expect("Failed to encode empty path");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode empty path");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_unicode_characters_roundtrip() {
    let path = PathBuf::from("/tmp/\u{65E5}\u{672C}\u{8A9E}/\u{30D5}\u{30A1}\u{30A4}\u{30EB}.txt");
    let encoded = encode_to_vec(&path).expect("Failed to encode unicode path");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode unicode path");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_relative_path_roundtrip() {
    let path = PathBuf::from("relative/path/to/file");
    let encoded = encode_to_vec(&path).expect("Failed to encode relative path");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode relative path");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_with_spaces_roundtrip() {
    let path = PathBuf::from("/path with spaces/file");
    let encoded = encode_to_vec(&path).expect("Failed to encode path with spaces");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode path with spaces");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_deep_nested_path_roundtrip() {
    let path = PathBuf::from("/a/b/c/d/e/f.txt");
    let encoded = encode_to_vec(&path).expect("Failed to encode deep nested path");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode deep nested path");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_encodes_same_as_string_of_same_path() {
    let path = PathBuf::from("/tmp/example/file.bin");
    let path_str = path.to_string_lossy().into_owned();

    let encoded_path = encode_to_vec(&path).expect("Failed to encode PathBuf");
    let encoded_str = encode_to_vec(&path_str).expect("Failed to encode String");

    assert_eq!(
        encoded_path, encoded_str,
        "PathBuf and equivalent String should encode identically"
    );
}

#[test]
fn test_vec_pathbuf_roundtrip() {
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("/tmp/a"),
        PathBuf::from("/tmp/b"),
        PathBuf::from("/tmp/c"),
    ];
    let encoded = encode_to_vec(&paths).expect("Failed to encode Vec<PathBuf>");
    let (decoded, _): (Vec<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<PathBuf>");
    assert_eq!(paths, decoded);
}

#[test]
fn test_option_pathbuf_some_roundtrip() {
    let path: Option<PathBuf> = Some(PathBuf::from("/tmp/some_file.dat"));
    let encoded = encode_to_vec(&path).expect("Failed to encode Option<PathBuf> Some");
    let (decoded, _): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<PathBuf> Some");
    assert_eq!(path, decoded);
}

#[test]
fn test_option_pathbuf_none_roundtrip() {
    let path: Option<PathBuf> = None;
    let encoded = encode_to_vec(&path).expect("Failed to encode Option<PathBuf> None");
    let (decoded, _): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<PathBuf> None");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_with_fixed_int_config() {
    let path = PathBuf::from("/tmp/fixed_int_test.bin");
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&path, cfg)
        .expect("Failed to encode PathBuf with fixed-int config");
    let (decoded, _): (PathBuf, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode PathBuf with fixed-int config");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_with_big_endian_config() {
    let path = PathBuf::from("/tmp/big_endian_test.bin");
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&path, cfg)
        .expect("Failed to encode PathBuf with big-endian config");
    let (decoded, _): (PathBuf, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode PathBuf with big-endian config");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_consumed_bytes_equals_encoded_length() {
    let path = PathBuf::from("/tmp/consumed_bytes_test.txt");
    let encoded = encode_to_vec(&path).expect("Failed to encode PathBuf for consumed bytes check");
    let (_, consumed): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode PathBuf for consumed bytes check");
    assert_eq!(
        consumed,
        encoded.len(),
        "Consumed bytes should equal total encoded length"
    );
}

#[test]
fn test_pathbuf_single_component_roundtrip() {
    let path = PathBuf::from("hello");
    let encoded = encode_to_vec(&path).expect("Failed to encode single component path");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode single component path");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_with_extension_roundtrip() {
    let path = PathBuf::from("file.rs");
    let encoded = encode_to_vec(&path).expect("Failed to encode path with extension");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode path with extension");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_from_temp_dir_roundtrip() {
    let mut path = std::env::temp_dir();
    path.push("oxicode_test_roundtrip.tmp");
    let encoded = encode_to_vec(&path).expect("Failed to encode temp_dir path");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode temp_dir path");
    assert_eq!(path, decoded);
}

#[test]
fn test_vec_of_10_pathbufs_roundtrip() {
    let paths: Vec<PathBuf> = (0..10)
        .map(|i| PathBuf::from(format!("/tmp/file_{}.bin", i)))
        .collect();
    let encoded = encode_to_vec(&paths).expect("Failed to encode Vec of 10 PathBufs");
    let (decoded, _): (Vec<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec of 10 PathBufs");
    assert_eq!(paths, decoded);
}

#[test]
fn test_pathbuf_with_dots_roundtrip() {
    let path = PathBuf::from("./relative/../path");
    let encoded = encode_to_vec(&path).expect("Failed to encode path with dots");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode path with dots");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_with_numbers_roundtrip() {
    let path = PathBuf::from("/path/123/456");
    let encoded = encode_to_vec(&path).expect("Failed to encode path with numbers");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode path with numbers");
    assert_eq!(path, decoded);
}

#[test]
fn test_two_pathbufs_same_value_produce_same_bytes() {
    let path_a = PathBuf::from("/tmp/identical/path.txt");
    let path_b = PathBuf::from("/tmp/identical/path.txt");
    let encoded_a = encode_to_vec(&path_a).expect("Failed to encode first PathBuf");
    let encoded_b = encode_to_vec(&path_b).expect("Failed to encode second PathBuf");
    assert_eq!(
        encoded_a, encoded_b,
        "Same path values should produce identical encoded bytes"
    );
}

#[test]
fn test_different_pathbufs_produce_different_bytes() {
    let path_a = PathBuf::from("/tmp/path_alpha.txt");
    let path_b = PathBuf::from("/tmp/path_beta.txt");
    let encoded_a = encode_to_vec(&path_a).expect("Failed to encode first distinct PathBuf");
    let encoded_b = encode_to_vec(&path_b).expect("Failed to encode second distinct PathBuf");
    assert_ne!(
        encoded_a, encoded_b,
        "Different path values should produce different encoded bytes"
    );
}

#[test]
fn test_pathbuf_with_non_ascii_filename_roundtrip() {
    let path = PathBuf::from("/tmp/\u{4E2D}\u{6587}\u{76EE}\u{5F55}/\u{6587}\u{4EF6}\u{540D}.dat");
    let encoded = encode_to_vec(&path).expect("Failed to encode non-ASCII filename path");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("Failed to decode non-ASCII filename path");
    assert_eq!(path, decoded);
}
