//! Advanced roundtrip tests (set 2) for PathBuf, OsString, and related types.
//!
//! Covers: empty/root paths, relative/absolute paths, Unicode, spaces,
//! long paths, joined components, byte-size checks, big-endian config,
//! fixed-int config, struct with PathBuf+OsString, inequality, "." path,
//! and extension preservation.

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
use std::ffi::OsString;
use std::path::PathBuf;

// ===== 1. PathBuf::from("") empty path roundtrip =====

#[test]
fn test_pathbuf_empty_roundtrip() {
    let original = PathBuf::from("");
    let encoded = encode_to_vec(&original).expect("encode empty PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode empty PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.as_os_str().is_empty());
}

// ===== 2. PathBuf::from("/") root path roundtrip =====

#[test]
fn test_pathbuf_root_roundtrip() {
    let original = PathBuf::from("/");
    let encoded = encode_to_vec(&original).expect("encode root PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode root PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.has_root());
    assert_eq!(decoded.components().count(), 1);
}

// ===== 3. PathBuf::from("relative/path") roundtrip =====

#[test]
fn test_pathbuf_relative_path_roundtrip2() {
    let original = PathBuf::from("relative/path");
    let encoded = encode_to_vec(&original).expect("encode relative PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode relative PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(!decoded.is_absolute());
    assert_eq!(decoded.file_name().expect("file_name present"), "path");
}

// ===== 4. PathBuf::from("/absolute/path/to/file.txt") roundtrip =====

#[test]
fn test_pathbuf_absolute_path_file_txt_roundtrip() {
    let original = PathBuf::from("/absolute/path/to/file.txt");
    let encoded = encode_to_vec(&original).expect("encode absolute PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode absolute PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.is_absolute());
    assert_eq!(decoded.extension().expect("extension present"), "txt");
    assert_eq!(decoded.file_name().expect("file_name present"), "file.txt");
}

// ===== 5. PathBuf with unicode: "/路径/ファイル" roundtrip =====

#[test]
fn test_pathbuf_unicode_mixed_cjk_roundtrip() {
    let original = PathBuf::from("/路径/ファイル");
    let encoded = encode_to_vec(&original).expect("encode unicode PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode unicode PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.file_name().expect("file_name present"), "ファイル");
    assert_eq!(
        decoded
            .parent()
            .expect("parent exists")
            .file_name()
            .expect("parent name present"),
        "路径"
    );
}

// ===== 6. PathBuf with spaces: "/path with spaces/file name.txt" roundtrip =====

#[test]
fn test_pathbuf_spaces_in_path_roundtrip() {
    let original = PathBuf::from("/path with spaces/file name.txt");
    let encoded = encode_to_vec(&original).expect("encode spaced PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode spaced PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        decoded.file_name().expect("file_name present"),
        "file name.txt"
    );
    assert_eq!(decoded.extension().expect("extension present"), "txt");
}

// ===== 7. Vec<PathBuf> roundtrip =====

#[test]
fn test_vec_pathbuf_multiple_roundtrip() {
    let original: Vec<PathBuf> = vec![
        PathBuf::from("/home/user/docs"),
        PathBuf::from("data/input.csv"),
        PathBuf::from(""),
        PathBuf::from("/"),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<PathBuf> failed");
    let (decoded, consumed): (Vec<PathBuf>, _) =
        decode_from_slice(&encoded).expect("decode Vec<PathBuf> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 4);
}

// ===== 8. Option<PathBuf> Some and None roundtrip =====

#[test]
fn test_option_pathbuf_some_and_none_roundtrip() {
    let some_val: Option<PathBuf> = Some(PathBuf::from("/opt/cache/store"));
    let none_val: Option<PathBuf> = None;

    let enc_some = encode_to_vec(&some_val).expect("encode Some<PathBuf> failed");
    let (dec_some, cons_some): (Option<PathBuf>, _) =
        decode_from_slice(&enc_some).expect("decode Some<PathBuf> failed");
    assert_eq!(dec_some, some_val);
    assert_eq!(cons_some, enc_some.len());
    assert!(dec_some.is_some());

    let enc_none = encode_to_vec(&none_val).expect("encode None<PathBuf> failed");
    let (dec_none, cons_none): (Option<PathBuf>, _) =
        decode_from_slice(&enc_none).expect("decode None<PathBuf> failed");
    assert_eq!(dec_none, none_val);
    assert_eq!(cons_none, enc_none.len());
    assert!(dec_none.is_none());
}

// ===== 9. PathBuf from joined components: PathBuf::from("a").join("b").join("c") =====

#[test]
fn test_pathbuf_joined_components_roundtrip() {
    let original = PathBuf::from("a").join("b").join("c");
    let encoded = encode_to_vec(&original).expect("encode joined PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode joined PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded, PathBuf::from("a/b/c"));
    assert_eq!(decoded.components().count(), 3);
    assert_eq!(decoded.file_name().expect("file_name present"), "c");
}

// ===== 10. Very long path (200+ chars) roundtrip =====

#[test]
fn test_pathbuf_very_long_path_roundtrip() {
    // Build a path exceeding 200 characters
    let segment = "abcdefghij"; // 10 chars
    let mut path_str = String::from("/");
    for i in 0..20 {
        path_str.push_str(&format!("{segment}{i:02}/"));
    }
    path_str.push_str("longfile.bin");
    assert!(
        path_str.len() > 200,
        "path must exceed 200 chars for this test"
    );

    let original = PathBuf::from(&path_str);
    let encoded = encode_to_vec(&original).expect("encode long PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode long PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        decoded.file_name().expect("file_name present"),
        "longfile.bin"
    );
}

// ===== 11. OsString from "hello" roundtrip =====

#[test]
fn test_osstring_hello_roundtrip() {
    let original = OsString::from("hello");
    let encoded = encode_to_vec(&original).expect("encode OsString 'hello' failed");
    let (decoded, consumed): (OsString, _) =
        decode_from_slice(&encoded).expect("decode OsString 'hello' failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.to_string_lossy().as_ref(), "hello");
}

// ===== 12. OsString empty roundtrip =====

#[test]
fn test_osstring_empty_roundtrip() {
    let original = OsString::from("");
    let encoded = encode_to_vec(&original).expect("encode empty OsString failed");
    let (decoded, consumed): (OsString, _) =
        decode_from_slice(&encoded).expect("decode empty OsString failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.is_empty());
}

// ===== 13. OsString from unicode string roundtrip =====

#[test]
fn test_osstring_unicode_roundtrip() {
    let original = OsString::from("ünïcödé_テスト_тест");
    let encoded = encode_to_vec(&original).expect("encode unicode OsString failed");
    let (decoded, consumed): (OsString, _) =
        decode_from_slice(&encoded).expect("decode unicode OsString failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.to_string_lossy().as_ref(), "ünïcödé_テスト_тест");
}

// ===== 14. Vec<OsString> roundtrip =====

#[test]
fn test_vec_osstring_roundtrip() {
    let original: Vec<OsString> = vec![
        OsString::from("PATH=/usr/bin:/bin"),
        OsString::from("HOME=/root"),
        OsString::from(""),
        OsString::from("LANG=ja_JP.UTF-8"),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<OsString> failed");
    let (decoded, consumed): (Vec<OsString>, _) =
        decode_from_slice(&encoded).expect("decode Vec<OsString> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 4);
}

// ===== 15. Option<OsString> roundtrip =====

#[test]
fn test_option_osstring_some_none_roundtrip() {
    let some_val: Option<OsString> = Some(OsString::from("OXICODE_ENV=production"));
    let none_val: Option<OsString> = None;

    let enc_some = encode_to_vec(&some_val).expect("encode Some<OsString> failed");
    let (dec_some, cons_some): (Option<OsString>, _) =
        decode_from_slice(&enc_some).expect("decode Some<OsString> failed");
    assert_eq!(dec_some, some_val);
    assert_eq!(cons_some, enc_some.len());

    let enc_none = encode_to_vec(&none_val).expect("encode None<OsString> failed");
    let (dec_none, cons_none): (Option<OsString>, _) =
        decode_from_slice(&enc_none).expect("decode None<OsString> failed");
    assert_eq!(dec_none, none_val);
    assert_eq!(cons_none, enc_none.len());
}

// ===== 16. PathBuf byte size check (same as String encoding of the path) =====

#[test]
fn test_pathbuf_byte_size_matches_string_encoding() {
    let path_str = "/usr/local/share/oxicode/data.bin";
    let path = PathBuf::from(path_str);
    let str_val = path_str.to_string();

    let enc_path = encode_to_vec(&path).expect("encode PathBuf for size check failed");
    let enc_str = encode_to_vec(&str_val).expect("encode String for size check failed");

    // PathBuf and String should produce identical encoded bytes since PathBuf
    // serializes via its UTF-8 string representation on all supported platforms.
    assert_eq!(
        enc_path.len(),
        enc_str.len(),
        "PathBuf and String must produce same-length encoding for a valid UTF-8 path"
    );
    assert_eq!(
        enc_path, enc_str,
        "PathBuf and String encoding bytes must be identical for UTF-8 paths"
    );
}

// ===== 17. PathBuf with big-endian config roundtrip =====

#[test]
fn test_pathbuf_big_endian_config_roundtrip2() {
    let original = PathBuf::from("/data/big_endian_test/archive.tar.gz");
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode PathBuf big-endian failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode PathBuf big-endian failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.extension().expect("extension present"), "gz");
}

// ===== 18. OsString with fixed-int encoding config roundtrip =====

#[test]
fn test_osstring_fixed_int_config_roundtrip() {
    let original = OsString::from("fixed_int_osstring_value_42");
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode OsString fixed-int failed");
    let (decoded, consumed): (OsString, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode OsString fixed-int failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ===== 19. Struct with PathBuf and OsString fields roundtrip =====

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct FileMetadata {
    path: PathBuf,
    name: OsString,
    size: u64,
}

#[test]
fn test_struct_with_pathbuf_and_osstring_roundtrip() {
    let original = FileMetadata {
        path: PathBuf::from("/var/data/archive/2026/records.db"),
        name: OsString::from("records.db"),
        size: 1_048_576_u64,
    };
    let encoded = encode_to_vec(&original).expect("encode FileMetadata failed");
    let (decoded, consumed): (FileMetadata, _) =
        decode_from_slice(&encoded).expect("decode FileMetadata failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.size, 1_048_576_u64);
    assert_eq!(
        decoded.path.file_name().expect("file_name present"),
        "records.db"
    );
    assert_eq!(decoded.name.to_string_lossy().as_ref(), "records.db");
}

// ===== 20. PathBuf comparison: two different paths decode as different =====

#[test]
fn test_pathbuf_different_paths_decode_as_different() {
    let path_a = PathBuf::from("/home/alice/file.txt");
    let path_b = PathBuf::from("/home/bob/file.txt");

    let enc_a = encode_to_vec(&path_a).expect("encode path_a failed");
    let enc_b = encode_to_vec(&path_b).expect("encode path_b failed");

    let (dec_a, _): (PathBuf, _) = decode_from_slice(&enc_a).expect("decode path_a failed");
    let (dec_b, _): (PathBuf, _) = decode_from_slice(&enc_b).expect("decode path_b failed");

    assert_eq!(dec_a, path_a);
    assert_eq!(dec_b, path_b);
    assert_ne!(dec_a, dec_b, "two distinct paths must not decode as equal");
    // Cross-decoding must also differ: enc_a should not produce path_b
    assert_ne!(enc_a, enc_b, "encoded bytes for distinct paths must differ");
}

// ===== 21. PathBuf::from(".") (current dir) roundtrip =====

#[test]
fn test_pathbuf_current_dir_roundtrip() {
    let original = PathBuf::from(".");
    let encoded = encode_to_vec(&original).expect("encode '.' PathBuf failed");
    let (decoded, consumed): (PathBuf, _) =
        decode_from_slice(&encoded).expect("decode '.' PathBuf failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // "." is a single normal component (CurDir)
    assert_eq!(decoded.components().count(), 1);
    assert_eq!(decoded.to_str().expect("valid UTF-8"), ".");
}

// ===== 22. Path extension detection: roundtrip preserves extension =====

#[test]
fn test_pathbuf_extension_preserved_after_roundtrip() {
    let cases: &[(&str, &str, &str)] = &[
        ("/srv/web/index.html", "html", "index"),
        ("/backup/dump.tar.gz", "gz", "dump.tar"),
        ("nodir.rs", "rs", "nodir"),
        ("/etc/archive.tar.bz2", "bz2", "archive.tar"),
    ];

    for (path_str, expected_ext, expected_stem) in cases {
        let original = PathBuf::from(path_str);
        let encoded = encode_to_vec(&original).expect("encode PathBuf for extension test failed");
        let (decoded, consumed): (PathBuf, _) =
            decode_from_slice(&encoded).expect("decode PathBuf for extension test failed");
        assert_eq!(decoded, original, "path mismatch for {path_str}");
        assert_eq!(consumed, encoded.len(), "consumed mismatch for {path_str}");
        assert_eq!(
            decoded.extension().expect("extension must be present"),
            *expected_ext,
            "extension mismatch for {path_str}"
        );
        assert_eq!(
            decoded.file_stem().expect("file stem must be present"),
            *expected_stem,
            "stem mismatch for {path_str}"
        );
    }
}
