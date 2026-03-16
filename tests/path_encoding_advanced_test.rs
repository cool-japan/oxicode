//! Advanced tests for PathBuf and path encoding in OxiCode.
//!
//! These tests cover edge cases and scenarios not already covered in pathbuf_test.rs.

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
use std::collections::BTreeMap;
use std::path::PathBuf;

mod path_encoding_advanced_tests {
    use super::*;

    // ===== Helper =====

    fn roundtrip<T>(value: &T) -> T
    where
        T: Encode + for<'de> Decode,
    {
        let encoded = encode_to_vec(value).expect("encode failed");
        let (decoded, _): (T, _) = decode_from_slice(&encoded).expect("decode failed");
        decoded
    }

    // ===== Test 1: PathBuf from "/" roundtrip =====

    #[test]
    fn test_pathbuf_root_slash_roundtrip() {
        let path = PathBuf::from("/");
        let decoded = roundtrip(&path);
        assert_eq!(path, decoded, "Root '/' path must roundtrip correctly");
    }

    // ===== Test 2: PathBuf from "/home/user/file.txt" roundtrip =====

    #[test]
    fn test_pathbuf_home_user_file_roundtrip() {
        let path = PathBuf::from("/home/user/file.txt");
        let decoded = roundtrip(&path);
        assert_eq!(
            path, decoded,
            "'/home/user/file.txt' must roundtrip correctly"
        );
        assert_eq!(path.as_path(), decoded.as_path());
    }

    // ===== Test 3: PathBuf from "relative/path" roundtrip =====

    #[test]
    fn test_pathbuf_simple_relative_path_roundtrip() {
        let path = PathBuf::from("relative/path");
        let decoded = roundtrip(&path);
        assert_eq!(path, decoded, "'relative/path' must roundtrip correctly");
        assert_eq!(path.as_path(), decoded.as_path());
    }

    // ===== Test 4: PathBuf from "" (empty) roundtrip =====

    #[test]
    fn test_pathbuf_empty_string_roundtrip() {
        let path = PathBuf::from("");
        let decoded = roundtrip(&path);
        assert_eq!(path, decoded, "Empty path must roundtrip correctly");
        assert!(
            decoded.as_os_str().is_empty(),
            "Decoded empty path must remain empty"
        );
    }

    // ===== Test 5: PathBuf from "file.rs" (no directory) roundtrip =====

    #[test]
    fn test_pathbuf_filename_only_roundtrip() {
        let path = PathBuf::from("file.rs");
        let decoded = roundtrip(&path);
        assert_eq!(path, decoded, "'file.rs' must roundtrip correctly");
        assert_eq!(
            decoded.file_name().expect("file_name must be present"),
            "file.rs"
        );
        assert!(decoded
            .parent()
            .map(|p| p.as_os_str().is_empty())
            .unwrap_or(true));
    }

    // ===== Test 6: PathBuf with Unicode characters roundtrip =====

    #[cfg(unix)]
    #[test]
    fn test_pathbuf_unicode_extended_roundtrip() {
        let path = PathBuf::from("/data/ñoño/日本語ファイル/αρχείο_тест.bin");
        let decoded = roundtrip(&path);
        assert_eq!(
            path, decoded,
            "Unicode path must roundtrip correctly on Unix"
        );
        assert_eq!(path.as_path(), decoded.as_path());
    }

    #[cfg(not(unix))]
    #[test]
    fn test_pathbuf_unicode_extended_roundtrip() {
        // On non-Unix platforms use ASCII-only path
        let path = PathBuf::from("data/unicode_test_file.bin");
        let decoded = roundtrip(&path);
        assert_eq!(path, decoded);
    }

    // ===== Test 7: PathBuf with spaces in path roundtrip =====

    #[test]
    fn test_pathbuf_spaces_in_path_roundtrip() {
        let path = PathBuf::from("/home/user/my documents/report 2025.pdf");
        let decoded = roundtrip(&path);
        assert_eq!(path, decoded, "Path with spaces must roundtrip correctly");
        assert_eq!(path.as_path(), decoded.as_path());
    }

    // ===== Test 8: PathBuf with dots "../parent/child" roundtrip =====

    #[test]
    fn test_pathbuf_dotdot_relative_roundtrip() {
        let path = PathBuf::from("../parent/child");
        let decoded = roundtrip(&path);
        assert_eq!(
            path, decoded,
            "'../parent/child' with dots must roundtrip correctly"
        );
        assert_eq!(path.as_path(), decoded.as_path());
    }

    // ===== Test 9: Vec<PathBuf> roundtrip =====

    #[test]
    fn test_vec_pathbuf_advanced_roundtrip() {
        let paths: Vec<PathBuf> = vec![
            PathBuf::from("/"),
            PathBuf::from(""),
            PathBuf::from("file.txt"),
            PathBuf::from("/usr/bin/cargo"),
            PathBuf::from("../sibling/module.rs"),
        ];
        let decoded = roundtrip(&paths);
        assert_eq!(paths.len(), decoded.len(), "Vec length must be preserved");
        for (i, (orig, dec)) in paths.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(orig, dec, "Path at index {} must match after roundtrip", i);
        }
    }

    // ===== Test 10: Option<PathBuf> Some roundtrip =====

    #[test]
    fn test_option_pathbuf_some_advanced_roundtrip() {
        let opt: Option<PathBuf> = Some(PathBuf::from("/var/log/system.log"));
        let decoded = roundtrip(&opt);
        assert_eq!(
            opt, decoded,
            "Option::Some(PathBuf) must roundtrip correctly"
        );
        assert!(decoded.is_some(), "Decoded Option must still be Some");
    }

    // ===== Test 11: Option<PathBuf> None roundtrip =====

    #[test]
    fn test_option_pathbuf_none_advanced_roundtrip() {
        let opt: Option<PathBuf> = None;
        let decoded = roundtrip(&opt);
        assert_eq!(
            opt, decoded,
            "Option::None PathBuf must roundtrip correctly"
        );
        assert!(decoded.is_none(), "Decoded Option must still be None");
    }

    // ===== Test 12: Struct with PathBuf field derive roundtrip =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct AppSettings {
        config_path: PathBuf,
        log_path: PathBuf,
        max_retries: u32,
    }

    #[test]
    fn test_struct_with_multiple_pathbuf_fields_roundtrip() {
        let settings = AppSettings {
            config_path: PathBuf::from("/etc/app/settings.toml"),
            log_path: PathBuf::from("/var/log/app/output.log"),
            max_retries: 5,
        };
        let encoded = encode_to_vec(&settings).expect("encode AppSettings failed");
        let (decoded, _): (AppSettings, _) =
            decode_from_slice(&encoded).expect("decode AppSettings failed");
        assert_eq!(
            settings, decoded,
            "Struct with PathBuf fields must roundtrip correctly"
        );
        assert_eq!(
            settings.config_path.as_path(),
            decoded.config_path.as_path()
        );
        assert_eq!(settings.log_path.as_path(), decoded.log_path.as_path());
        assert_eq!(settings.max_retries, decoded.max_retries);
    }

    // ===== Test 13: BTreeMap<String, PathBuf> roundtrip =====

    #[test]
    fn test_btreemap_string_to_pathbuf_roundtrip() {
        let mut map: BTreeMap<String, PathBuf> = BTreeMap::new();
        map.insert("home".to_string(), PathBuf::from("/home/user"));
        map.insert("config".to_string(), PathBuf::from("/etc/app/config.toml"));
        map.insert("temp".to_string(), PathBuf::from("/tmp/scratch"));
        map.insert("relative".to_string(), PathBuf::from("data/output.bin"));

        let decoded = roundtrip(&map);
        assert_eq!(
            map.len(),
            decoded.len(),
            "BTreeMap length must be preserved"
        );
        for (key, orig_path) in &map {
            let dec_path = decoded.get(key).expect("key must exist in decoded map");
            assert_eq!(orig_path, dec_path, "Path for key '{}' must match", key);
        }
    }

    // ===== Test 14: PathBuf encode produces same bytes as String encode (Unix only) =====

    #[cfg(unix)]
    #[test]
    fn test_pathbuf_encodes_same_as_string_on_unix() {
        // On Unix, OsString is internally a UTF-8 string, so PathBuf bytes
        // should match the equivalent String encoding.
        let s = "/usr/local/share/doc/readme.txt".to_string();
        let path = PathBuf::from(&s);

        let string_bytes = encode_to_vec(&s).expect("encode String failed");
        let path_bytes = encode_to_vec(&path).expect("encode PathBuf failed");

        assert_eq!(
            string_bytes, path_bytes,
            "PathBuf and String must produce identical encoded bytes on Unix"
        );
    }

    // ===== Test 15: Long path (200 chars) roundtrip =====

    #[test]
    fn test_pathbuf_long_path_200_chars_roundtrip() {
        // Build a path that is exactly 200 characters long in the filename portion
        let segment = "a".repeat(50);
        let path = PathBuf::from(format!(
            "/very/deep/{}/{}/{}/{}",
            segment, segment, segment, segment
        ));
        let path_str = path.to_str().expect("path must be valid UTF-8");
        assert!(
            path_str.len() >= 200,
            "Path string must be at least 200 characters, got {}",
            path_str.len()
        );
        let decoded = roundtrip(&path);
        assert_eq!(path, decoded, "Long 200-char path must roundtrip correctly");
    }

    // ===== Test 16: Path with special chars "file (1).txt" roundtrip =====

    #[test]
    fn test_pathbuf_special_chars_parens_roundtrip() {
        let path = PathBuf::from("file (1).txt");
        let decoded = roundtrip(&path);
        assert_eq!(
            path, decoded,
            "Path with parentheses 'file (1).txt' must roundtrip correctly"
        );
        assert_eq!(
            decoded.file_name().expect("file_name must be present"),
            "file (1).txt"
        );
    }

    // ===== Test 17: Windows-style path "C:\\Users\\test" roundtrip =====

    #[test]
    fn test_pathbuf_windows_style_path_roundtrip() {
        // On Unix this is treated as a single filename component with backslashes,
        // but it must still roundtrip faithfully regardless of the platform.
        let path = PathBuf::from(r"C:\Users\test");
        let decoded = roundtrip(&path);
        assert_eq!(
            path, decoded,
            r"Windows-style path 'C:\Users\test' must roundtrip correctly"
        );
    }

    // ===== Test 18: PathBuf byte size: empty path is 1 byte (varint 0) =====

    #[test]
    fn test_pathbuf_empty_encodes_to_one_byte() {
        let path = PathBuf::from("");
        let encoded = encode_to_vec(&path).expect("encode empty PathBuf failed");
        assert_eq!(
            encoded.len(),
            1,
            "Empty PathBuf must encode to exactly 1 byte (varint 0), got {} bytes",
            encoded.len()
        );
        assert_eq!(
            encoded[0], 0x00,
            "The single byte must be 0x00 (varint for length 0)"
        );
    }

    // ===== Test 19: PathBuf byte size: "abc" is at least 4 bytes =====

    #[test]
    fn test_pathbuf_abc_encodes_to_at_least_four_bytes() {
        let path = PathBuf::from("abc");
        let encoded = encode_to_vec(&path).expect("encode 'abc' PathBuf failed");
        assert!(
            encoded.len() >= 4,
            "PathBuf 'abc' must encode to at least 4 bytes (1 length varint + 3 chars), got {}",
            encoded.len()
        );
        // The raw bytes must contain the ASCII characters for 'a', 'b', 'c'
        let payload = &encoded[1..];
        assert!(
            payload.contains(&b'a') && payload.contains(&b'b') && payload.contains(&b'c'),
            "Encoded bytes must contain ASCII values for 'a', 'b', 'c'"
        );
    }

    // ===== Test 20: Vec<PathBuf> with 100 paths roundtrip =====

    #[test]
    fn test_vec_pathbuf_100_paths_roundtrip() {
        let paths: Vec<PathBuf> = (0..100_u32)
            .map(|i| PathBuf::from(format!("/data/batch/{:04}/output_{:04}.bin", i, i)))
            .collect();

        let encoded = encode_to_vec(&paths).expect("encode Vec<PathBuf> with 100 paths failed");
        let (decoded, consumed): (Vec<PathBuf>, _) =
            decode_from_slice(&encoded).expect("decode Vec<PathBuf> with 100 paths failed");

        assert_eq!(
            paths.len(),
            decoded.len(),
            "Vec of 100 PathBufs must decode to 100 elements"
        );
        assert_eq!(
            consumed,
            encoded.len(),
            "All encoded bytes must be consumed"
        );
        for (i, (orig, dec)) in paths.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(orig, dec, "Path at index {} must match after roundtrip", i);
        }
    }
}
