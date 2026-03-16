//! Advanced roundtrip tests for PathBuf, OsString, and OsStr scenarios.
//!
//! Covers: simple filenames, absolute/relative paths, Unicode, spaces, empty paths,
//! collections, structs, maps, tuples, and binary representation verification.

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
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsString;
use std::path::PathBuf;

// ===== 1. PathBuf with simple filename roundtrip =====

#[test]
fn test_pathbuf_simple_filename_roundtrip() {
    let path = PathBuf::from("report.txt");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
}

// ===== 2. PathBuf with absolute path roundtrip =====

#[test]
fn test_pathbuf_absolute_path_roundtrip() {
    let path = PathBuf::from("/usr/local/share/doc/oxicode");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
}

// ===== 3. PathBuf with relative path containing multiple components =====

#[test]
fn test_pathbuf_relative_multicomponent_roundtrip() {
    let path = PathBuf::from("src/features/serde/impl.rs");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    // verify individual components are preserved
    let components: Vec<_> = decoded.components().collect();
    assert_eq!(components.len(), 4);
}

// ===== 4. PathBuf with Unicode filename (CJK characters) =====

#[test]
fn test_pathbuf_unicode_cjk_roundtrip() {
    let path = PathBuf::from("/home/用户/文件/テスト.txt");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded.file_name().expect("file_name"), "テスト.txt");
}

// ===== 5. PathBuf with spaces in filename =====

#[test]
fn test_pathbuf_spaces_in_filename_roundtrip() {
    let path = PathBuf::from("/home/user/my documents/project report final.pdf");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
}

// ===== 6. PathBuf empty path roundtrip =====

#[test]
fn test_pathbuf_empty_roundtrip() {
    let path = PathBuf::new();
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert!(decoded.as_os_str().is_empty());
}

// ===== 7. OsString basic ASCII roundtrip =====

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_osstring_basic_ascii_roundtrip() {
    let original = OsString::from("plain-ascii-string_123");
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (OsString, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 8. OsString with Unicode content =====

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_osstring_unicode_roundtrip() {
    let original = OsString::from("こんにちは_мир_🦀");
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (OsString, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 9. OsString empty roundtrip =====

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_osstring_empty_roundtrip() {
    let original = OsString::new();
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (OsString, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

// ===== 10. Vec<PathBuf> roundtrip =====

#[test]
fn test_vec_pathbuf_roundtrip() {
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("/etc/hosts"),
        PathBuf::from("/var/log/syslog"),
        PathBuf::from("relative/path/file.toml"),
        PathBuf::new(),
    ];
    let encoded = encode_to_vec(&paths).expect("encode failed");
    let (decoded, _): (Vec<PathBuf>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(paths, decoded);
    assert_eq!(decoded.len(), 4);
}

// ===== 11. Option<PathBuf> Some/None roundtrip =====

#[test]
fn test_option_pathbuf_some_roundtrip() {
    let opt: Option<PathBuf> = Some(PathBuf::from("/tmp/staging/output.bin"));
    let encoded = encode_to_vec(&opt).expect("encode failed");
    let (decoded, _): (Option<PathBuf>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(opt, decoded);
}

#[test]
fn test_option_pathbuf_none_roundtrip() {
    let opt: Option<PathBuf> = None;
    let encoded = encode_to_vec(&opt).expect("encode failed");
    let (decoded, _): (Option<PathBuf>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(opt, decoded);
    assert!(decoded.is_none());
}

// ===== 12. PathBuf in struct field =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct FileEntry {
    id: u32,
    path: PathBuf,
    label: String,
}

#[test]
fn test_pathbuf_in_struct_field_roundtrip() {
    let entry = FileEntry {
        id: 42,
        path: PathBuf::from("/data/archive/2026/records.db"),
        label: String::from("primary-store"),
    };
    let encoded = encode_to_vec(&entry).expect("encode failed");
    let (decoded, _): (FileEntry, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(entry, decoded);
    assert_eq!(decoded.path.extension().expect("extension"), "db");
}

// ===== 13. OsString in struct field =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct EnvVar {
    key: OsString,
    value: OsString,
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_osstring_in_struct_field_roundtrip() {
    let var = EnvVar {
        key: OsString::from("OXICODE_DATA_DIR"),
        value: OsString::from("/usr/local/share/oxicode"),
    };
    let encoded = encode_to_vec(&var).expect("encode failed");
    let (decoded, _): (EnvVar, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(var, decoded);
}

// ===== 14. HashMap<String, PathBuf> roundtrip =====

#[test]
fn test_hashmap_string_pathbuf_roundtrip() {
    let mut map: HashMap<String, PathBuf> = HashMap::new();
    map.insert(
        "config".to_string(),
        PathBuf::from("/etc/oxicode/config.toml"),
    );
    map.insert("cache".to_string(), PathBuf::from("/var/cache/oxicode"));
    map.insert("log".to_string(), PathBuf::from("/var/log/oxicode.log"));

    let encoded = encode_to_vec(&map).expect("encode failed");
    let (decoded, _): (HashMap<String, PathBuf>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(map, decoded);
    assert_eq!(
        decoded.get("config").expect("key exists"),
        &PathBuf::from("/etc/oxicode/config.toml")
    );
}

// ===== 15. PathBuf with extension =====

#[test]
fn test_pathbuf_with_extension_roundtrip() {
    let path = PathBuf::from("/builds/release/oxicode.so.2.0.1");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded.extension().expect("extension"), "1");
    assert_eq!(decoded.file_stem().expect("stem"), "oxicode.so.2.0");
}

// ===== 16. PathBuf with deep directory nesting =====

#[test]
fn test_pathbuf_deeply_nested_roundtrip() {
    let path = PathBuf::from("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/leaf.dat");
    let encoded = encode_to_vec(&path).expect("encode failed");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(path, decoded);
    assert_eq!(decoded.file_name().expect("file_name"), "leaf.dat");
}

// ===== 17. PathBuf comparing encode_to_vec lengths for different paths =====

#[test]
fn test_pathbuf_encode_length_comparison() {
    let short = PathBuf::from("a.rs");
    let long = PathBuf::from("/very/long/absolute/path/to/some/deeply/nested/directory/file.rs");

    let short_bytes = encode_to_vec(&short).expect("encode short");
    let long_bytes = encode_to_vec(&long).expect("encode long");

    // longer path string must produce more bytes
    assert!(
        long_bytes.len() > short_bytes.len(),
        "longer path should encode to more bytes: long={} short={}",
        long_bytes.len(),
        short_bytes.len()
    );

    // both must roundtrip correctly
    let (short_dec, _): (PathBuf, _) = decode_from_slice(&short_bytes).expect("decode short");
    let (long_dec, _): (PathBuf, _) = decode_from_slice(&long_bytes).expect("decode long");
    assert_eq!(short, short_dec);
    assert_eq!(long, long_dec);
}

// ===== 18. OsString with special characters =====

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_osstring_special_characters_roundtrip() {
    // Tab, newline, null byte are valid in OsString on Unix
    let original =
        OsString::from("file\twith\ttabs and spaces & symbols: <>, \"quotes\", 'single'");
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (OsString, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 19. Vec<OsString> roundtrip =====

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_vec_osstring_roundtrip() {
    let items: Vec<OsString> = vec![
        OsString::from("PATH"),
        OsString::from("/usr/bin:/usr/local/bin"),
        OsString::from("HOME"),
        OsString::from("/root"),
        OsString::new(),
    ];
    let encoded = encode_to_vec(&items).expect("encode failed");
    let (decoded, _): (Vec<OsString>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(items, decoded);
    assert_eq!(decoded.len(), 5);
    assert!(decoded[4].is_empty());
}

// ===== 20. Tuple (PathBuf, OsString) roundtrip =====

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_tuple_pathbuf_osstring_roundtrip() {
    let original: (PathBuf, OsString) = (
        PathBuf::from("/proc/self/exe"),
        OsString::from("LD_PRELOAD=/lib/hook.so"),
    );
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): ((PathBuf, OsString), _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, PathBuf::from("/proc/self/exe"));
    assert_eq!(decoded.1, OsString::from("LD_PRELOAD=/lib/hook.so"));
}

// ===== 21. BTreeMap<u32, PathBuf> roundtrip =====

#[test]
fn test_btreemap_u32_pathbuf_roundtrip() {
    let mut map: BTreeMap<u32, PathBuf> = BTreeMap::new();
    map.insert(1, PathBuf::from("/shard/001/data.bin"));
    map.insert(2, PathBuf::from("/shard/002/data.bin"));
    map.insert(3, PathBuf::from("/shard/003/data.bin"));
    map.insert(100, PathBuf::from("/shard/100/data.bin"));

    let encoded = encode_to_vec(&map).expect("encode failed");
    let (decoded, _): (BTreeMap<u32, PathBuf>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(map, decoded);

    // BTreeMap ordering is preserved
    let keys: Vec<u32> = decoded.keys().copied().collect();
    assert_eq!(keys, vec![1, 2, 3, 100]);
    assert_eq!(
        decoded.get(&100).expect("key 100"),
        &PathBuf::from("/shard/100/data.bin")
    );
}

// ===== 22. PathBuf binary representation: encode then decode and verify path components =====

#[test]
fn test_pathbuf_binary_representation_components() {
    let path = PathBuf::from("/workspace/oxicode/src/features/serde.rs");
    let encoded = encode_to_vec(&path).expect("encode failed");

    // encoded bytes must not be empty
    assert!(!encoded.is_empty(), "encoded bytes must not be empty");

    // the encoded bytes must contain the UTF-8 representation of the path string somewhere
    let path_str = path.to_str().expect("valid UTF-8 path");
    let path_bytes = path_str.as_bytes();
    let contains_path_bytes = encoded
        .windows(path_bytes.len())
        .any(|window| window == path_bytes);
    assert!(
        contains_path_bytes,
        "encoded bytes should contain the raw path string bytes"
    );

    // full roundtrip and component verification
    let (decoded, consumed): (PathBuf, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(consumed, encoded.len(), "all bytes should be consumed");
    assert_eq!(path, decoded);

    let expected_components = ["workspace", "oxicode", "src", "features", "serde.rs"];
    let decoded_parts: Vec<&str> = decoded
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .filter(|s| *s != "/")
        .collect();
    assert_eq!(decoded_parts, expected_components);
    assert_eq!(decoded.extension().expect("extension"), "rs");
    assert_eq!(decoded.file_stem().expect("stem"), "serde");
}
