//! Advanced PathBuf serialization tests (set 2): 22 tests covering
//! roundtrips, config variants, collections, and wire-format properties.

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

// ── 1. PathBuf::new() (empty path) roundtrip ────────────────────────────────

#[test]
fn test_pathbuf_new_empty_roundtrip() {
    let path = PathBuf::new();
    let encoded = encode_to_vec(&path).expect("encode PathBuf::new()");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode PathBuf::new()");
    assert_eq!(path, decoded);
}

// ── 2. PathBuf::from("/tmp/test.txt") roundtrip ─────────────────────────────

#[test]
fn test_pathbuf_absolute_tmp_roundtrip() {
    let path = PathBuf::from("/tmp/test.txt");
    let encoded = encode_to_vec(&path).expect("encode absolute path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode absolute path");
    assert_eq!(path, decoded);
}

// ── 3. PathBuf::from("relative/path/file.rs") roundtrip ────────────────────

#[test]
fn test_pathbuf_relative_path_roundtrip() {
    let path = PathBuf::from("relative/path/file.rs");
    let encoded = encode_to_vec(&path).expect("encode relative path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode relative path");
    assert_eq!(path, decoded);
}

// ── 4. PathBuf::from(".") current dir roundtrip ─────────────────────────────

#[test]
fn test_pathbuf_current_dir_roundtrip() {
    let path = PathBuf::from(".");
    let encoded = encode_to_vec(&path).expect("encode current dir");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode current dir");
    assert_eq!(path, decoded);
}

// ── 5. PathBuf::from("/") root roundtrip ────────────────────────────────────

#[test]
fn test_pathbuf_root_roundtrip() {
    let path = PathBuf::from("/");
    let encoded = encode_to_vec(&path).expect("encode root path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode root path");
    assert_eq!(path, decoded);
}

// ── 6. PathBuf with unicode in filename roundtrip ───────────────────────────

#[test]
fn test_pathbuf_unicode_filename_roundtrip() {
    let path = PathBuf::from("/tmp/日本語ファイル.txt");
    let encoded = encode_to_vec(&path).expect("encode unicode path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode unicode path");
    assert_eq!(path, decoded);
}

// ── 7. PathBuf consumed == encoded.len() ────────────────────────────────────

#[test]
fn test_pathbuf_consumed_equals_encoded_len() {
    let path = PathBuf::from("/usr/local/bin/rustc");
    let encoded = encode_to_vec(&path).expect("encode for consumed check");
    let (_, consumed): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

// ── 8. Vec<PathBuf> roundtrip (3 paths) ─────────────────────────────────────

#[test]
fn test_vec_pathbuf_three_paths_roundtrip() {
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("/etc/hosts"),
        PathBuf::from("src/main.rs"),
        PathBuf::from("/var/log/syslog"),
    ];
    let encoded = encode_to_vec(&paths).expect("encode Vec<PathBuf>");
    let (decoded, _): (Vec<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<PathBuf>");
    assert_eq!(paths, decoded);
}

// ── 9. Option<PathBuf> Some roundtrip ───────────────────────────────────────

#[test]
fn test_option_pathbuf_some_roundtrip() {
    let opt: Option<PathBuf> = Some(PathBuf::from("/opt/app/config.yaml"));
    let encoded = encode_to_vec(&opt).expect("encode Option<PathBuf> Some");
    let (decoded, _): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Option<PathBuf> Some");
    assert_eq!(opt, decoded);
}

// ── 10. Option<PathBuf> None roundtrip ──────────────────────────────────────

#[test]
fn test_option_pathbuf_none_roundtrip() {
    let opt: Option<PathBuf> = None;
    let encoded = encode_to_vec(&opt).expect("encode Option<PathBuf> None");
    let (decoded, _): (Option<PathBuf>, usize) =
        decode_from_slice(&encoded).expect("decode Option<PathBuf> None");
    assert_eq!(opt, decoded);
}

// ── 11. Fixed-int config with PathBuf ───────────────────────────────────────

#[test]
fn test_pathbuf_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let path = PathBuf::from("/home/user/documents/report.pdf");
    let encoded = encode_to_vec_with_config(&path, cfg).expect("encode with fixed-int config");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed-int config");
    assert_eq!(path, decoded);
}

// ── 12. Big-endian config with PathBuf ──────────────────────────────────────

#[test]
fn test_pathbuf_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let path = PathBuf::from("/srv/data/archive.tar.gz");
    let encoded = encode_to_vec_with_config(&path, cfg).expect("encode with big-endian config");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with big-endian config");
    assert_eq!(path, decoded);
}

// ── 13. Long path with many components roundtrip ────────────────────────────

#[test]
fn test_pathbuf_long_many_components_roundtrip() {
    let path =
        PathBuf::from("/a/bb/ccc/dddd/eeeee/ffffff/ggggggg/hhhhhhhh/iiiiiiiii/jjjjjjjjjj.rs");
    let encoded = encode_to_vec(&path).expect("encode long path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode long path");
    assert_eq!(path, decoded);
    assert_eq!(path.components().count(), decoded.components().count());
}

// ── 14. PathBuf with spaces in name roundtrip ───────────────────────────────

#[test]
fn test_pathbuf_spaces_in_name_roundtrip() {
    let path = PathBuf::from("my documents/project files/notes.txt");
    let encoded = encode_to_vec(&path).expect("encode path with spaces");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode path with spaces");
    assert_eq!(path, decoded);
}

// ── 15. PathBuf::from("/tmp/file with spaces.txt") roundtrip ────────────────

#[test]
fn test_pathbuf_absolute_with_spaces_roundtrip() {
    let path = PathBuf::from("/tmp/file with spaces.txt");
    let encoded = encode_to_vec(&path).expect("encode absolute path with spaces");
    let (decoded, _): (PathBuf, usize) =
        decode_from_slice(&encoded).expect("decode absolute path with spaces");
    assert_eq!(path, decoded);
}

// ── 16. PathBuf with extension .rs — roundtrip and verify extension preserved

#[test]
fn test_pathbuf_extension_rs_preserved() {
    let path = PathBuf::from("/workspace/myproject/src/lib.rs");
    let encoded = encode_to_vec(&path).expect("encode .rs path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode .rs path");
    assert_eq!(path, decoded);
    assert_eq!(
        decoded.extension().expect("extension must be present"),
        "rs",
        "extension must be 'rs' after roundtrip"
    );
}

// ── 17. Two different PathBufs encode to different bytes (assert_ne!) ────────

#[test]
fn test_two_different_pathbufs_encode_differently() {
    let path_a = PathBuf::from("/tmp/alpha.txt");
    let path_b = PathBuf::from("/tmp/beta.txt");
    let enc_a = encode_to_vec(&path_a).expect("encode path_a");
    let enc_b = encode_to_vec(&path_b).expect("encode path_b");
    assert_ne!(
        enc_a, enc_b,
        "distinct paths must produce distinct encodings"
    );
}

// ── 18. Same path string encodes to same bytes (assert_eq!) ─────────────────

#[test]
fn test_same_path_encodes_to_same_bytes() {
    let path_x = PathBuf::from("/usr/bin/cargo");
    let path_y = PathBuf::from("/usr/bin/cargo");
    let enc_x = encode_to_vec(&path_x).expect("encode path_x");
    let enc_y = encode_to_vec(&path_y).expect("encode path_y");
    assert_eq!(
        enc_x, enc_y,
        "identical paths must produce identical encodings"
    );
}

// ── 19. PathBuf wire size > 0 for non-empty path ────────────────────────────

#[test]
fn test_pathbuf_wire_size_nonzero_for_nonempty() {
    let path = PathBuf::from("/nonzero");
    let encoded = encode_to_vec(&path).expect("encode non-empty path");
    assert!(
        !encoded.is_empty(),
        "encoded non-empty PathBuf must have wire size > 0"
    );
}

// ── 20. PathBuf from std::env::temp_dir() roundtrip ─────────────────────────

#[test]
fn test_pathbuf_from_temp_dir_roundtrip() {
    let path = std::env::temp_dir();
    let encoded = encode_to_vec(&path).expect("encode temp_dir path");
    let (decoded, _): (PathBuf, usize) = decode_from_slice(&encoded).expect("decode temp_dir path");
    assert_eq!(path, decoded);
}

// ── 21. Vec<Option<PathBuf>> roundtrip with Some/None mix ───────────────────

#[test]
fn test_vec_option_pathbuf_mixed_roundtrip() {
    let items: Vec<Option<PathBuf>> = vec![
        Some(PathBuf::from("/first/path")),
        None,
        Some(PathBuf::from("second/relative")),
        None,
        Some(PathBuf::from("/third/absolute/file.bin")),
    ];
    let encoded = encode_to_vec(&items).expect("encode Vec<Option<PathBuf>>");
    let (decoded, _): (Vec<Option<PathBuf>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Option<PathBuf>>");
    assert_eq!(items, decoded);
}

// ── 22. (PathBuf, PathBuf) tuple roundtrip ───────────────────────────────────

#[test]
fn test_pathbuf_tuple_roundtrip() {
    let pair: (PathBuf, PathBuf) = (
        PathBuf::from("/input/data.csv"),
        PathBuf::from("/output/result.parquet"),
    );
    let encoded = encode_to_vec(&pair).expect("encode (PathBuf, PathBuf)");
    let (decoded, _): ((PathBuf, PathBuf), usize) =
        decode_from_slice(&encoded).expect("decode (PathBuf, PathBuf)");
    assert_eq!(pair, decoded);
}
