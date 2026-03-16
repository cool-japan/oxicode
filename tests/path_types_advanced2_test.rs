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
    encode_to_vec_with_config, Decode, Encode,
};
use std::path::PathBuf;

// ===== Test 1: PathBuf::from("/usr/local/bin") roundtrip =====

#[test]
fn test_pathbuf_usr_local_bin_roundtrip() {
    let path = PathBuf::from("/usr/local/bin");
    let encoded = encode_to_vec(&path).expect("encode /usr/local/bin");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode /usr/local/bin");
    assert_eq!(
        path, decoded,
        "PathBuf /usr/local/bin must roundtrip unchanged"
    );
}

// ===== Test 2: PathBuf::from("relative/path") roundtrip =====

#[test]
fn test_pathbuf_relative_path_roundtrip() {
    let path = PathBuf::from("relative/path");
    let encoded = encode_to_vec(&path).expect("encode relative/path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode relative/path");
    assert_eq!(path, decoded, "relative/path must roundtrip unchanged");
}

// ===== Test 3: PathBuf::new() (empty) roundtrip =====

#[test]
fn test_pathbuf_new_empty_roundtrip() {
    let path = PathBuf::new();
    let encoded = encode_to_vec(&path).expect("encode empty PathBuf");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode empty PathBuf");
    assert_eq!(path, decoded, "empty PathBuf must roundtrip unchanged");
    assert!(
        decoded.as_os_str().is_empty(),
        "decoded empty PathBuf must have empty OsStr"
    );
}

// ===== Test 4: PathBuf::from(".") roundtrip =====

#[test]
fn test_pathbuf_dot_roundtrip() {
    let path = PathBuf::from(".");
    let encoded = encode_to_vec(&path).expect("encode PathBuf(\".\")");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf(\".\")");
    assert_eq!(path, decoded, "PathBuf(\".\") must roundtrip unchanged");
    assert_eq!(
        path.to_str().expect("dot path is valid UTF-8"),
        decoded.to_str().expect("decoded dot path is valid UTF-8"),
        "string representation of dot path must match"
    );
}

// ===== Test 5: PathBuf::from("..") roundtrip =====

#[test]
fn test_pathbuf_dotdot_roundtrip() {
    let path = PathBuf::from("..");
    let encoded = encode_to_vec(&path).expect("encode PathBuf(\"..\")");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf(\"..\")");
    assert_eq!(path, decoded, "PathBuf(\"..\") must roundtrip unchanged");
    assert_eq!(
        path.to_str().expect("dotdot path is valid UTF-8"),
        decoded
            .to_str()
            .expect("decoded dotdot path is valid UTF-8"),
        "string representation of dotdot path must match"
    );
}

// ===== Test 6: PathBuf with unicode characters roundtrip =====

#[test]
fn test_pathbuf_unicode_roundtrip() {
    let path = PathBuf::from("/tmp/日本語/тест/αρχείο.txt");
    let encoded = encode_to_vec(&path).expect("encode unicode PathBuf");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode unicode PathBuf");
    assert_eq!(path, decoded, "unicode PathBuf must roundtrip unchanged");
    assert_eq!(
        path.to_str().expect("unicode path is valid UTF-8"),
        decoded
            .to_str()
            .expect("decoded unicode path is valid UTF-8"),
        "unicode string representation must match"
    );
}

// ===== Test 7: PathBuf::from("/") roundtrip =====

#[test]
fn test_pathbuf_root_roundtrip() {
    let path = PathBuf::from("/");
    let encoded = encode_to_vec(&path).expect("encode root path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode root path");
    assert_eq!(path, decoded, "root path \"/\" must roundtrip unchanged");
    assert!(decoded.is_absolute(), "decoded root path must be absolute");
}

// ===== Test 8: Vec<PathBuf> roundtrip =====

#[test]
fn test_vec_pathbuf_roundtrip() {
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("relative/path"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("/"),
    ];
    let encoded = encode_to_vec(&paths).expect("encode Vec<PathBuf>");
    let (decoded, _): (Vec<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<PathBuf>");
    assert_eq!(paths, decoded, "Vec<PathBuf> must roundtrip unchanged");
    assert_eq!(paths.len(), decoded.len(), "length must be preserved");
    for (orig, dec) in paths.iter().zip(decoded.iter()) {
        assert_eq!(
            orig.as_path(),
            dec.as_path(),
            "each path must match after decode"
        );
    }
}

// ===== Test 9: Option<PathBuf> Some roundtrip =====

#[test]
fn test_option_pathbuf_some_roundtrip() {
    let opt: Option<PathBuf> = Some(PathBuf::from("/usr/local/bin"));
    let encoded = encode_to_vec(&opt).expect("encode Option<PathBuf> Some");
    let (decoded, _): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Option<PathBuf> Some");
    assert_eq!(
        opt, decoded,
        "Option<PathBuf> Some must roundtrip unchanged"
    );
    assert!(decoded.is_some(), "decoded must be Some");
}

// ===== Test 10: Option<PathBuf> None roundtrip =====

#[test]
fn test_option_pathbuf_none_roundtrip() {
    let opt: Option<PathBuf> = None;
    let encoded = encode_to_vec(&opt).expect("encode Option<PathBuf> None");
    let (decoded, _): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Option<PathBuf> None");
    assert_eq!(
        opt, decoded,
        "Option<PathBuf> None must roundtrip unchanged"
    );
    assert!(decoded.is_none(), "decoded must be None");
}

// ===== Test 11: PathBuf with fixed_int_encoding config roundtrip =====

#[test]
fn test_pathbuf_fixed_int_encoding_config_roundtrip() {
    let path = PathBuf::from("/usr/local/bin");
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&path, cfg).expect("encode PathBuf with fixed_int_encoding");
    let (decoded, _): (PathBuf, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode PathBuf with fixed_int_encoding");
    assert_eq!(
        path, decoded,
        "PathBuf with fixed_int_encoding config must roundtrip unchanged"
    );
}

// ===== Test 12: consumed bytes == encoded length for PathBuf =====

#[test]
fn test_pathbuf_consumed_bytes_equals_encoded_length() {
    let path = PathBuf::from("/usr/local/bin");
    let encoded = encode_to_vec(&path).expect("encode PathBuf for bytes check");
    let (_decoded, consumed): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf for bytes check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ===== Test 13: Two different PathBuf values produce different encodings =====

#[test]
fn test_two_different_pathbuf_produce_different_encodings() {
    let path_a = PathBuf::from("/usr/local/bin");
    let path_b = PathBuf::from("/usr/local/lib");
    let encoded_a = encode_to_vec(&path_a).expect("encode PathBuf a");
    let encoded_b = encode_to_vec(&path_b).expect("encode PathBuf b");
    assert_ne!(
        encoded_a, encoded_b,
        "different PathBuf values must produce different encoded bytes"
    );
}

// ===== Test 14: Struct with PathBuf field roundtrip =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct FileEntry {
    location: PathBuf,
    size: u64,
}

#[test]
fn test_struct_with_pathbuf_field_roundtrip() {
    let entry = FileEntry {
        location: PathBuf::from("/var/log/system.log"),
        size: 4096,
    };
    let encoded = encode_to_vec(&entry).expect("encode FileEntry");
    let (decoded, _): (FileEntry, usize) = decode_from_slice(&encoded).expect("decode FileEntry");
    assert_eq!(
        entry, decoded,
        "FileEntry struct with PathBuf field must roundtrip unchanged"
    );
    assert_eq!(
        entry.location.as_path(),
        decoded.location.as_path(),
        "PathBuf field within struct must match"
    );
    assert_eq!(
        entry.size, decoded.size,
        "u64 field within struct must match"
    );
}

// ===== Test 15: PathBuf encodes same as equivalent String (same underlying bytes) =====

#[test]
fn test_pathbuf_encodes_same_as_string() {
    let path_str = "/usr/local/bin";
    let path = PathBuf::from(path_str);
    let path_as_string = path_str.to_string();
    let encoded_pathbuf = encode_to_vec(&path).expect("encode PathBuf");
    let encoded_string = encode_to_vec(&path_as_string).expect("encode String");
    assert_eq!(
        encoded_pathbuf, encoded_string,
        "PathBuf must encode to the same bytes as its equivalent String"
    );
}

// ===== Test 16: PathBuf with spaces in path roundtrip =====

#[test]
fn test_pathbuf_with_spaces_roundtrip() {
    let path = PathBuf::from("/home/user/my documents/report 2026.pdf");
    let encoded = encode_to_vec(&path).expect("encode PathBuf with spaces");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf with spaces");
    assert_eq!(
        path, decoded,
        "PathBuf with spaces must roundtrip unchanged"
    );
    assert_eq!(
        path.file_name().expect("has file_name"),
        decoded.file_name().expect("decoded has file_name"),
        "file_name() must match after decode"
    );
}

// ===== Test 17: PathBuf with multiple nested directories roundtrip =====

#[test]
fn test_pathbuf_multiple_nested_directories_roundtrip() {
    let path = PathBuf::from("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p.txt");
    let encoded = encode_to_vec(&path).expect("encode deeply nested PathBuf");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode deeply nested PathBuf");
    assert_eq!(
        path, decoded,
        "deeply nested PathBuf must roundtrip unchanged"
    );
    assert_eq!(
        path.components().count(),
        decoded.components().count(),
        "component count must be preserved"
    );
}

// ===== Test 18: PathBuf from temp_dir() roundtrip =====

#[test]
fn test_pathbuf_temp_dir_roundtrip() {
    let path = std::env::temp_dir();
    let encoded = encode_to_vec(&path).expect("encode temp_dir PathBuf");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode temp_dir PathBuf");
    assert_eq!(path, decoded, "temp_dir PathBuf must roundtrip unchanged");
    assert_eq!(
        path.as_path(),
        decoded.as_path(),
        "temp_dir as_path() must match after decode"
    );
}

// ===== Test 19: Vec<Option<PathBuf>> mixed roundtrip =====

#[test]
fn test_vec_option_pathbuf_mixed_roundtrip() {
    let paths: Vec<Option<PathBuf>> = vec![
        Some(PathBuf::from("/usr/local/bin")),
        None,
        Some(PathBuf::from("relative/path")),
        None,
        Some(PathBuf::from(".")),
    ];
    let encoded = encode_to_vec(&paths).expect("encode Vec<Option<PathBuf>>");
    let (decoded, _): (Vec<Option<PathBuf>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Option<PathBuf>>");
    assert_eq!(
        paths, decoded,
        "Vec<Option<PathBuf>> mixed must roundtrip unchanged"
    );
    assert_eq!(paths.len(), decoded.len(), "length must be preserved");
    for (orig, dec) in paths.iter().zip(decoded.iter()) {
        assert_eq!(orig, dec, "each Option<PathBuf> element must match");
    }
}

// ===== Test 20: PathBuf with very long path (200 chars) roundtrip =====

#[test]
fn test_pathbuf_very_long_path_roundtrip() {
    let segment = "abcdefghij"; // 10 chars each
    let long_path = format!(
        "/{}",
        (0..20).map(|_| segment).collect::<Vec<_>>().join("/")
    );
    assert!(
        long_path.len() >= 200,
        "path must be at least 200 chars long"
    );
    let path = PathBuf::from(&long_path);
    let encoded = encode_to_vec(&path).expect("encode long PathBuf");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode long PathBuf");
    assert_eq!(path, decoded, "very long PathBuf must roundtrip unchanged");
    assert_eq!(
        path.to_str().expect("long path is valid UTF-8"),
        decoded.to_str().expect("decoded long path is valid UTF-8"),
        "string representation of long path must match"
    );
}

// ===== Test 21: PathBuf::from("C:\\Windows\\System32") roundtrip (Windows-style path) =====

#[test]
fn test_pathbuf_windows_style_path_roundtrip() {
    let path = PathBuf::from("C:\\Windows\\System32");
    let encoded = encode_to_vec(&path).expect("encode Windows-style PathBuf");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode Windows-style PathBuf");
    assert_eq!(
        path, decoded,
        "Windows-style path string PathBuf must roundtrip unchanged"
    );
    assert_eq!(
        path.to_str().expect("windows path is valid UTF-8"),
        decoded
            .to_str()
            .expect("decoded windows path is valid UTF-8"),
        "Windows-style path string representation must match"
    );
}

// ===== Test 22: PathBuf with dots and special chars roundtrip =====

#[test]
fn test_pathbuf_dots_and_special_chars_roundtrip() {
    let path = PathBuf::from("/home/user/.config/my-app/settings.v2.conf");
    let encoded = encode_to_vec(&path).expect("encode PathBuf with dots and special chars");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf with dots and special chars");
    assert_eq!(
        path, decoded,
        "PathBuf with dots and special chars must roundtrip unchanged"
    );
    assert_eq!(
        path.file_name().expect("has file_name"),
        decoded.file_name().expect("decoded has file_name"),
        "file_name() must match after decode"
    );
    assert_eq!(
        path.extension().expect("has extension"),
        decoded.extension().expect("decoded has extension"),
        "extension() must match after decode"
    );
}
