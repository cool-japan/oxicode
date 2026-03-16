//! Comprehensive tests for PathBuf and Path Encode/Decode implementations.

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
use std::path::PathBuf;

// ===== Helper =====

fn roundtrip<T>(value: &T) -> T
where
    T: Encode + for<'de> Decode,
{
    let encoded = encode_to_vec(value).expect("encode failed");
    let (decoded, _): (T, _) = decode_from_slice(&encoded).expect("decode failed");
    decoded
}

// ===== Test 1: absolute path roundtrip =====

#[test]
fn test_pathbuf_absolute_roundtrip() {
    let path = PathBuf::from("/tmp/test.txt");
    let decoded = roundtrip(&path);
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== Test 2: relative path roundtrip =====

#[test]
fn test_pathbuf_relative_roundtrip() {
    let path = PathBuf::from("relative/path/file.rs");
    let decoded = roundtrip(&path);
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== Test 3: empty path roundtrip =====

#[test]
fn test_pathbuf_empty_roundtrip() {
    let path = PathBuf::from("");
    let decoded = roundtrip(&path);
    assert_eq!(path, decoded);
}

// ===== Test 4: unicode path roundtrip =====

#[cfg(unix)]
#[test]
fn test_pathbuf_unicode_roundtrip() {
    // Unix paths can contain arbitrary UTF-8 sequences
    let path = PathBuf::from("/tmp/日本語/тест/αρχείο.txt");
    let decoded = roundtrip(&path);
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== Test 5: deeply nested path roundtrip =====

#[test]
fn test_pathbuf_deeply_nested_roundtrip() {
    let path = PathBuf::from("/a/b/c/d/e/f/g.txt");
    let decoded = roundtrip(&path);
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== Test 6: Vec<PathBuf> with 5 paths roundtrip =====

#[test]
fn test_vec_pathbuf_roundtrip() {
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("/etc/hosts"),
        PathBuf::from("/usr/local/bin/cargo"),
        PathBuf::from("relative/dir/file.toml"),
        PathBuf::from("/tmp/output.bin"),
        PathBuf::from("../parent/sibling.rs"),
    ];
    let decoded = roundtrip(&paths);
    assert_eq!(paths, decoded);
    assert_eq!(paths.len(), decoded.len());
    for (original, dec) in paths.iter().zip(decoded.iter()) {
        assert_eq!(original.as_path(), dec.as_path());
    }
}

// ===== Test 7: Option<PathBuf> - Some and None =====

#[test]
fn test_option_pathbuf_some_roundtrip() {
    let opt: Option<PathBuf> = Some(PathBuf::from("/some/optional/path.conf"));
    let decoded = roundtrip(&opt);
    assert_eq!(opt, decoded);
}

#[test]
fn test_option_pathbuf_none_roundtrip() {
    let opt: Option<PathBuf> = None;
    let decoded = roundtrip(&opt);
    assert_eq!(opt, decoded);
}

// ===== Test 8: PathBuf in a struct =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct Config {
    path: PathBuf,
    enabled: bool,
}

#[test]
fn test_struct_with_pathbuf_roundtrip() {
    let config = Config {
        path: PathBuf::from("/etc/app/config.toml"),
        enabled: true,
    };
    let encoded = encode_to_vec(&config).expect("encode failed");
    let (decoded, _): (Config, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(config, decoded);
    assert_eq!(config.path.as_path(), decoded.path.as_path());
    assert_eq!(config.enabled, decoded.enabled);
}

#[test]
fn test_struct_with_pathbuf_disabled_roundtrip() {
    let config = Config {
        path: PathBuf::from("logs/app.log"),
        enabled: false,
    };
    let encoded = encode_to_vec(&config).expect("encode failed");
    let (decoded, _): (Config, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(config, decoded);
}

// ===== Test 9: as_path() comparison after decode =====

#[test]
fn test_pathbuf_as_path_comparison_after_decode() {
    let path = PathBuf::from("/usr/share/doc/readme.txt");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    // Verify via as_path() that the decoded path equals the original
    assert_eq!(
        path.as_path(),
        decoded.as_path(),
        "as_path() comparison must hold after decode"
    );
    // Additional component-level verification
    assert_eq!(path.components().count(), decoded.components().count());
    for (orig_comp, dec_comp) in path.components().zip(decoded.components()) {
        assert_eq!(orig_comp, dec_comp);
    }
}

// ===== Test 10: PathBuf from std::env::temp_dir() =====

#[test]
fn test_pathbuf_temp_dir_roundtrip() {
    let path = std::env::temp_dir();
    let decoded = roundtrip(&path);
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

#[test]
fn test_pathbuf_temp_dir_with_filename_roundtrip() {
    let mut path = std::env::temp_dir();
    path.push("oxicode_pathbuf_test_file.bin");
    let decoded = roundtrip(&path);
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}
