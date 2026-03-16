//! Advanced tests for OsString and PathBuf encoding in OxiCode (set 2).
//! 22 top-level #[test] functions — no cfg(test) module wrapper.

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
use std::ffi::OsString;
use std::path::PathBuf;

// ===== Test 1: OsString empty roundtrip =====

#[test]
fn test_osstring_empty_roundtrip() {
    let os = OsString::new();
    let encoded = encode_to_vec(&os).expect("encode OsString empty");
    let (decoded, _bytes): (OsString, usize) =
        decode_from_slice(&encoded).expect("decode OsString empty");
    assert_eq!(os, decoded);
}

// ===== Test 2: OsString "hello" roundtrip =====

#[test]
fn test_osstring_hello_roundtrip() {
    let os = OsString::from("hello");
    let encoded = encode_to_vec(&os).expect("encode OsString hello");
    let (decoded, _bytes): (OsString, usize) =
        decode_from_slice(&encoded).expect("decode OsString hello");
    assert_eq!(os, decoded);
}

// ===== Test 3: OsString with ASCII path chars roundtrip =====

#[test]
fn test_osstring_ascii_path_chars_roundtrip() {
    let os = OsString::from("/usr/local/bin/rustc");
    let encoded = encode_to_vec(&os).expect("encode OsString ascii path");
    let (decoded, _bytes): (OsString, usize) =
        decode_from_slice(&encoded).expect("decode OsString ascii path");
    assert_eq!(os, decoded);
}

// ===== Test 4: OsString consumed == encoded length =====

#[test]
fn test_osstring_consumed_equals_encoded_length() {
    let os = OsString::from("oxicode-test");
    let encoded = encode_to_vec(&os).expect("encode OsString consumed check");
    let (_decoded, consumed): (OsString, usize) =
        decode_from_slice(&encoded).expect("decode OsString consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ===== Test 5: OsString with spaces roundtrip =====

#[test]
fn test_osstring_with_spaces_roundtrip() {
    let os = OsString::from("hello world with spaces");
    let encoded = encode_to_vec(&os).expect("encode OsString spaces");
    let (decoded, _bytes): (OsString, usize) =
        decode_from_slice(&encoded).expect("decode OsString spaces");
    assert_eq!(os, decoded);
}

// ===== Test 6: OsString from "/usr/bin/something" roundtrip =====

#[test]
fn test_osstring_usr_bin_something_roundtrip() {
    let os = OsString::from("/usr/bin/something");
    let encoded = encode_to_vec(&os).expect("encode OsString /usr/bin/something");
    let (decoded, _bytes): (OsString, usize) =
        decode_from_slice(&encoded).expect("decode OsString /usr/bin/something");
    assert_eq!(os, decoded);
}

// ===== Test 7: Vec<OsString> roundtrip =====

#[test]
fn test_vec_osstring_roundtrip() {
    let items: Vec<OsString> = vec![
        OsString::from("alpha"),
        OsString::from("beta"),
        OsString::from("/usr/share/doc"),
        OsString::from(""),
        OsString::from("gamma delta"),
    ];
    let encoded = encode_to_vec(&items).expect("encode Vec<OsString>");
    let (decoded, _bytes): (Vec<OsString>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<OsString>");
    assert_eq!(items, decoded);
    assert_eq!(items.len(), decoded.len());
}

// ===== Test 8: Option<OsString> Some roundtrip =====

#[test]
fn test_option_osstring_some_roundtrip() {
    let opt: Option<OsString> = Some(OsString::from("optional-value"));
    let encoded = encode_to_vec(&opt).expect("encode Option<OsString> Some");
    let (decoded, _bytes): (Option<OsString>, usize) =
        decode_from_slice(&encoded).expect("decode Option<OsString> Some");
    assert_eq!(opt, decoded);
}

// ===== Test 9: Option<OsString> None roundtrip =====

#[test]
fn test_option_osstring_none_roundtrip() {
    let opt: Option<OsString> = None;
    let encoded = encode_to_vec(&opt).expect("encode Option<OsString> None");
    let (decoded, _bytes): (Option<OsString>, usize) =
        decode_from_slice(&encoded).expect("decode Option<OsString> None");
    assert_eq!(opt, decoded);
    assert!(decoded.is_none());
}

// ===== Test 10: OsString with fixed-int config roundtrip =====

#[test]
fn test_osstring_fixed_int_config_roundtrip() {
    let os = OsString::from("fixed-int-test");
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&os, cfg).expect("encode OsString fixed-int");
    let (decoded, _bytes): (OsString, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode OsString fixed-int");
    assert_eq!(os, decoded);
}

// ===== Test 11: OsString with big-endian config roundtrip =====

#[test]
fn test_osstring_big_endian_config_roundtrip() {
    let os = OsString::from("big-endian-test");
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&os, cfg).expect("encode OsString big-endian");
    let (decoded, _bytes): (OsString, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode OsString big-endian");
    assert_eq!(os, decoded);
}

// ===== Test 12: PathBuf empty roundtrip =====

#[test]
fn test_pathbuf_adv2_empty_roundtrip() {
    let path = PathBuf::from("");
    let encoded = encode_to_vec(&path).expect("encode PathBuf empty");
    let (decoded, _bytes): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf empty");
    assert_eq!(path, decoded);
}

// ===== Test 13: PathBuf "/tmp/test.txt" roundtrip =====

#[test]
fn test_pathbuf_adv2_tmp_test_txt_roundtrip() {
    let path = PathBuf::from("/tmp/test.txt");
    let encoded = encode_to_vec(&path).expect("encode PathBuf /tmp/test.txt");
    let (decoded, _bytes): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf /tmp/test.txt");
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== Test 14: PathBuf relative "src/main.rs" roundtrip =====

#[test]
fn test_pathbuf_adv2_relative_roundtrip() {
    let path = PathBuf::from("src/main.rs");
    let encoded = encode_to_vec(&path).expect("encode PathBuf src/main.rs");
    let (decoded, _bytes): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf src/main.rs");
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== Test 15: PathBuf consumed == encoded length =====

#[test]
fn test_pathbuf_adv2_consumed_equals_encoded_length() {
    let path = PathBuf::from("/home/user/documents/file.bin");
    let encoded = encode_to_vec(&path).expect("encode PathBuf consumed check");
    let (_decoded, consumed): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ===== Test 16: Vec<PathBuf> roundtrip =====

#[test]
fn test_vec_pathbuf_adv2_roundtrip() {
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("/etc/passwd"),
        PathBuf::from("relative/path.toml"),
        PathBuf::from(""),
        PathBuf::from("/var/log/syslog"),
        PathBuf::from("../sibling/file.rs"),
    ];
    let encoded = encode_to_vec(&paths).expect("encode Vec<PathBuf>");
    let (decoded, _bytes): (Vec<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<PathBuf>");
    assert_eq!(paths, decoded);
    assert_eq!(paths.len(), decoded.len());
    for (orig, dec) in paths.iter().zip(decoded.iter()) {
        assert_eq!(orig.as_path(), dec.as_path());
    }
}

// ===== Test 17: Option<PathBuf> Some roundtrip =====

#[test]
fn test_option_pathbuf_adv2_some_roundtrip() {
    let opt: Option<PathBuf> = Some(PathBuf::from("/opt/app/data.bin"));
    let encoded = encode_to_vec(&opt).expect("encode Option<PathBuf> Some");
    let (decoded, _bytes): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Option<PathBuf> Some");
    assert_eq!(opt, decoded);
}

// ===== Test 18: Option<PathBuf> None roundtrip =====

#[test]
fn test_option_pathbuf_adv2_none_roundtrip() {
    let opt: Option<PathBuf> = None;
    let encoded = encode_to_vec(&opt).expect("encode Option<PathBuf> None");
    let (decoded, _bytes): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Option<PathBuf> None");
    assert_eq!(opt, decoded);
    assert!(decoded.is_none());
}

// ===== Test 19: PathBuf with fixed-int config roundtrip =====

#[test]
fn test_pathbuf_adv2_fixed_int_config_roundtrip() {
    let path = PathBuf::from("/usr/share/man/man1/ls.1");
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&path, cfg).expect("encode PathBuf fixed-int");
    let (decoded, _bytes): (PathBuf, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode PathBuf fixed-int");
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== Test 20: PathBuf with big-endian config roundtrip =====

#[test]
fn test_pathbuf_adv2_big_endian_config_roundtrip() {
    let path = PathBuf::from("/var/cache/oxicode/test.bin");
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&path, cfg).expect("encode PathBuf big-endian");
    let (decoded, _bytes): (PathBuf, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode PathBuf big-endian");
    assert_eq!(path, decoded);
    assert_eq!(path.as_path(), decoded.as_path());
}

// ===== Test 21: Struct { name: OsString, path: PathBuf } roundtrip =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct NamedPath {
    name: OsString,
    path: PathBuf,
}

#[test]
fn test_struct_osstring_pathbuf_roundtrip() {
    let value = NamedPath {
        name: OsString::from("my-config"),
        path: PathBuf::from("/etc/app/my-config.toml"),
    };
    let encoded = encode_to_vec(&value).expect("encode NamedPath");
    let (decoded, _bytes): (NamedPath, usize) =
        decode_from_slice(&encoded).expect("decode NamedPath");
    assert_eq!(value, decoded);
    assert_eq!(value.name, decoded.name);
    assert_eq!(value.path.as_path(), decoded.path.as_path());
}

// ===== Test 22: OsString and String of same content produce same wire bytes =====

#[test]
fn test_osstring_and_string_same_content_same_wire_bytes() {
    // OsString::encode delegates to to_string_lossy().encode() which encodes as a &str/String.
    // For pure ASCII content this must be byte-for-byte identical to encoding the same String.
    let content = "identical-wire-format";
    let os = OsString::from(content);
    let s = content.to_string();

    let os_bytes = encode_to_vec(&os).expect("encode OsString wire-bytes");
    let str_bytes = encode_to_vec(&s).expect("encode String wire-bytes");

    assert_eq!(
        os_bytes, str_bytes,
        "OsString and String with identical ASCII content must produce the same wire bytes"
    );
}
