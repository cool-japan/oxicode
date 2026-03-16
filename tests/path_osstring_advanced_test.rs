//! Advanced roundtrip tests for PathBuf, OsString, and related types.
//!
//! Covers: absolute/relative paths, empty paths, Unicode, spaces, extensions,
//! collections, options, tuples, and alternative config encodings.

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
use oxicode::{config, decode_from_slice, encode_to_vec};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

// ===== 1. PathBuf::from("/tmp/test.txt") roundtrip =====

#[test]
fn test_pathbuf_tmp_test_txt_roundtrip() {
    let path = PathBuf::from("/tmp/test.txt");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== 2. PathBuf::from("") empty path roundtrip =====

#[test]
fn test_pathbuf_empty_string_roundtrip() {
    let path = PathBuf::from("");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert!(decoded.as_os_str().is_empty());
}

// ===== 3. PathBuf::from("relative/path/file.rs") roundtrip =====

#[test]
fn test_pathbuf_relative_path_roundtrip() {
    let path = PathBuf::from("relative/path/file.rs");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded.file_name().expect("file_name present"), "file.rs");
}

// ===== 4. PathBuf::from("/usr/local/bin") roundtrip =====

#[test]
fn test_pathbuf_usr_local_bin_roundtrip() {
    let path = PathBuf::from("/usr/local/bin");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded.file_name().expect("file_name present"), "bin");
}

// ===== 5. PathBuf::new() empty path roundtrip =====

#[test]
fn test_pathbuf_new_empty_roundtrip() {
    let path = PathBuf::new();
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert!(decoded.components().next().is_none());
}

// ===== 6. Long path with many components roundtrip =====

#[test]
fn test_pathbuf_long_many_components_roundtrip() {
    let path = PathBuf::from("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/deep.txt");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded.file_name().expect("file_name present"), "deep.txt");
    // root + 26 single-letter dirs + leaf = 28 components
    assert_eq!(decoded.components().count(), 28);
}

// ===== 7. Vec<PathBuf> with 3 paths roundtrip =====

#[test]
fn test_vec_pathbuf_three_paths_roundtrip() {
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("/etc/hosts"),
        PathBuf::from("src/main.rs"),
        PathBuf::from("/var/log/syslog"),
    ];
    let encoded = encode_to_vec(&paths).expect("encode failed");
    let (decoded, _): (Vec<PathBuf>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(paths, decoded);
    assert_eq!(decoded.len(), 3);
    for (orig, dec) in paths.iter().zip(decoded.iter()) {
        assert_eq!(orig.as_path(), dec.as_path());
    }
}

// ===== 8. Option<PathBuf> Some/None roundtrip =====

#[test]
fn test_option_pathbuf_some_roundtrip() {
    let opt: Option<PathBuf> = Some(PathBuf::from("/opt/data/store.db"));
    let encoded = encode_to_vec(&opt).expect("encode failed");
    let (decoded, _): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(opt, decoded);
    assert!(decoded.is_some());
}

#[test]
fn test_option_pathbuf_none_roundtrip() {
    let opt: Option<PathBuf> = None;
    let encoded = encode_to_vec(&opt).expect("encode failed");
    let (decoded, _): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(opt, decoded);
    assert!(decoded.is_none());
}

// ===== 9. PathBuf with fixed int encoding =====

#[test]
fn test_pathbuf_fixed_int_encoding_roundtrip() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let path = PathBuf::from("/tmp/fixed.bin");
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&path, cfg).expect("encode with fixed int failed");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed int failed");
    assert_eq!(path, decoded);
    // verify the path string bytes are present in the encoded output
    let path_str = path.to_str().expect("path is valid UTF-8");
    let path_bytes = path_str.as_bytes();
    let contains = encoded.windows(path_bytes.len()).any(|w| w == path_bytes);
    assert!(contains, "encoded bytes must contain raw path string");
}

// ===== 10. PathBuf with big endian config =====

#[test]
fn test_pathbuf_big_endian_config_roundtrip() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let path = PathBuf::from("/usr/share/oxicode");
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&path, cfg).expect("encode with big endian failed");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with big endian failed");
    assert_eq!(path, decoded);
}

// ===== 11. OsString::from("hello world") roundtrip =====

#[test]
fn test_osstring_hello_world_roundtrip() {
    let s = OsString::from("hello world");
    let encoded = encode_to_vec(&s).expect("encode failed");
    let (decoded, _): (OsString, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(s, decoded);
}

// ===== 12. OsString::new() empty OsString roundtrip =====

#[test]
fn test_osstring_new_empty_roundtrip() {
    let s = OsString::new();
    let encoded = encode_to_vec(&s).expect("encode failed");
    let (decoded, _): (OsString, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(s, decoded);
    assert!(decoded.is_empty());
}

// ===== 13. OsString::from("/path/to/file") roundtrip =====

#[test]
fn test_osstring_path_string_roundtrip() {
    let s = OsString::from("/path/to/file");
    let encoded = encode_to_vec(&s).expect("encode failed");
    let (decoded, _): (OsString, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(s, decoded);
    // verify via conversion to PathBuf
    let as_path = Path::new(&decoded);
    assert_eq!(as_path, Path::new("/path/to/file"));
}

// ===== 14. Vec<OsString> with 3 strings roundtrip =====

#[test]
fn test_vec_osstring_three_items_roundtrip() {
    let items: Vec<OsString> = vec![
        OsString::from("HOME"),
        OsString::from("/root"),
        OsString::from("SHELL=/bin/bash"),
    ];
    let encoded = encode_to_vec(&items).expect("encode failed");
    let (decoded, _): (Vec<OsString>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(items, decoded);
    assert_eq!(decoded.len(), 3);
}

// ===== 15. Option<OsString> Some/None roundtrip =====

#[test]
fn test_option_osstring_some_roundtrip() {
    let opt: Option<OsString> = Some(OsString::from("OXICODE_HOME=/usr/local/oxicode"));
    let encoded = encode_to_vec(&opt).expect("encode failed");
    let (decoded, _): (Option<OsString>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(opt, decoded);
    assert!(decoded.is_some());
}

#[test]
fn test_option_osstring_none_roundtrip() {
    let opt: Option<OsString> = None;
    let encoded = encode_to_vec(&opt).expect("encode failed");
    let (decoded, _): (Option<OsString>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(opt, decoded);
    assert!(decoded.is_none());
}

// ===== 16. OsString with fixed int encoding =====

#[test]
fn test_osstring_fixed_int_encoding_roundtrip() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let s = OsString::from("fixed_encoding_test");
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&s, cfg).expect("encode with fixed int failed");
    let (decoded, _): (OsString, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed int failed");
    assert_eq!(s, decoded);
}

// ===== 17. (PathBuf, OsString) tuple roundtrip =====

#[test]
fn test_tuple_pathbuf_osstring_roundtrip() {
    let tuple: (PathBuf, OsString) = (
        PathBuf::from("/proc/self/exe"),
        OsString::from("LD_PRELOAD=/lib/hook.so"),
    );
    let encoded = encode_to_vec(&tuple).expect("encode failed");
    let (decoded, _): ((PathBuf, OsString), usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(tuple, decoded);
    assert_eq!(decoded.0, PathBuf::from("/proc/self/exe"));
    assert_eq!(decoded.1, OsString::from("LD_PRELOAD=/lib/hook.so"));
}

// ===== 18. PathBuf with Unicode: /tmp/日本語/test.txt roundtrip =====

#[test]
fn test_pathbuf_unicode_japanese_roundtrip() {
    let path = PathBuf::from("/tmp/日本語/test.txt");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded.file_name().expect("file_name present"), "test.txt");
    // parent should contain the Japanese directory name
    let parent = decoded.parent().expect("parent exists");
    assert_eq!(parent.file_name().expect("parent name"), "日本語");
}

// ===== 19. OsString with spaces: "hello world foo" roundtrip =====

#[test]
fn test_osstring_spaces_roundtrip() {
    let s = OsString::from("hello world foo");
    let encoded = encode_to_vec(&s).expect("encode failed");
    let (decoded, _): (OsString, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(s, decoded);
    // verify space bytes are intact via string conversion
    let decoded_str = decoded.to_string_lossy();
    assert_eq!(decoded_str.as_ref(), "hello world foo");
    assert_eq!(decoded_str.chars().filter(|c| *c == ' ').count(), 2);
}

// ===== 20. PathBuf built with push roundtrip =====

#[test]
fn test_pathbuf_push_roundtrip() {
    let mut path = PathBuf::from("/tmp");
    path.push("subdir");
    path.push("file.txt");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded, PathBuf::from("/tmp/subdir/file.txt"));
    assert_eq!(decoded.file_name().expect("file_name present"), "file.txt");
    assert_eq!(
        decoded.parent().expect("parent exists"),
        Path::new("/tmp/subdir")
    );
}

// ===== 21. Vec<PathBuf> empty vec roundtrip =====

#[test]
fn test_vec_pathbuf_empty_roundtrip() {
    let paths: Vec<PathBuf> = Vec::new();
    let encoded = encode_to_vec(&paths).expect("encode failed");
    let (decoded, _): (Vec<PathBuf>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(paths, decoded);
    assert!(decoded.is_empty());
}

// ===== 22. PathBuf with extension: "document.pdf" roundtrip =====

#[test]
fn test_pathbuf_extension_pdf_roundtrip() {
    let path = PathBuf::from("document.pdf");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded.extension().expect("extension present"), "pdf");
    assert_eq!(decoded.file_stem().expect("stem present"), "document");
    // single component (no root, no directories)
    assert_eq!(decoded.components().count(), 1);
}
