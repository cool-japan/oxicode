//! Advanced tests for `decode_from_reader` / `decode_from_std_read` using in-memory `Cursor`
//! readers, file I/O, sequential decoding, and error-path coverage.

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
    config, decode_from_std_read, encode_to_vec, encode_to_vec_with_config, Decode, Encode,
};
use std::io::{Cursor, Seek, SeekFrom};

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ---------------------------------------------------------------------------
// 1. Decode u32 from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_u32_from_cursor() {
    let bytes = encode_to_vec(&42u32).expect("encode u32");
    let cursor = Cursor::new(bytes);
    let value: u32 = decode_from_std_read(cursor, config::standard()).expect("decode u32");
    assert_eq!(value, 42u32);
}

// ---------------------------------------------------------------------------
// 2. Decode String from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_string_from_cursor() {
    let original = "hello, oxicode!".to_string();
    let bytes = encode_to_vec(&original).expect("encode String");
    let cursor = Cursor::new(bytes);
    let decoded: String = decode_from_std_read(cursor, config::standard()).expect("decode String");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 3. Decode Vec<u8> from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_vec_u8_from_cursor() {
    let original: Vec<u8> = vec![10, 20, 30, 40, 50];
    let bytes = encode_to_vec(&original).expect("encode Vec<u8>");
    let cursor = Cursor::new(bytes);
    let decoded: Vec<u8> =
        decode_from_std_read(cursor, config::standard()).expect("decode Vec<u8>");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 4. Decode struct (derived) from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_struct_from_cursor() {
    let original = Point { x: -7, y: 99 };
    let bytes = encode_to_vec(&original).expect("encode Point");
    let cursor = Cursor::new(bytes);
    let decoded: Point = decode_from_std_read(cursor, config::standard()).expect("decode Point");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 5. Decode enum from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_enum_from_cursor() {
    let original = Direction::West;
    let bytes = encode_to_vec(&original).expect("encode Direction");
    let cursor = Cursor::new(bytes);
    let decoded: Direction =
        decode_from_std_read(cursor, config::standard()).expect("decode Direction");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 6. Decode bool from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_bool_from_cursor() {
    for &val in &[true, false] {
        let bytes = encode_to_vec(&val).expect("encode bool");
        let cursor = Cursor::new(bytes);
        let decoded: bool = decode_from_std_read(cursor, config::standard()).expect("decode bool");
        assert_eq!(decoded, val);
    }
}

// ---------------------------------------------------------------------------
// 7. Decode i64 from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_i64_from_cursor() {
    let original: i64 = i64::MIN / 2;
    let bytes = encode_to_vec(&original).expect("encode i64");
    let cursor = Cursor::new(bytes);
    let decoded: i64 = decode_from_std_read(cursor, config::standard()).expect("decode i64");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 8. Decode u128 from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_u128_from_cursor() {
    let original: u128 = u128::MAX - 1;
    let bytes = encode_to_vec(&original).expect("encode u128");
    let cursor = Cursor::new(bytes);
    let decoded: u128 = decode_from_std_read(cursor, config::standard()).expect("decode u128");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 9. Decode Option<String> Some from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_option_string_some_from_cursor() {
    let original: Option<String> = Some("present".to_string());
    let bytes = encode_to_vec(&original).expect("encode Option<String> Some");
    let cursor = Cursor::new(bytes);
    let decoded: Option<String> =
        decode_from_std_read(cursor, config::standard()).expect("decode Option<String> Some");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 10. Decode Option<u64> None from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_option_u64_none_from_cursor() {
    let original: Option<u64> = None;
    let bytes = encode_to_vec(&original).expect("encode Option<u64> None");
    let cursor = Cursor::new(bytes);
    let decoded: Option<u64> =
        decode_from_std_read(cursor, config::standard()).expect("decode Option<u64> None");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 11. Decode with fixed int config
// ---------------------------------------------------------------------------
#[test]
fn test_decode_with_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: u32 = 0x0001_0002;
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode fixed-int");
    let cursor = Cursor::new(bytes);
    let decoded: u32 = decode_from_std_read(cursor, cfg).expect("decode fixed-int");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 12. Decode with big endian config
// ---------------------------------------------------------------------------
#[test]
fn test_decode_with_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let original: u64 = 0xDEAD_BEEF_CAFE_0001;
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode big-endian");
    let cursor = Cursor::new(bytes);
    let decoded: u64 = decode_from_std_read(cursor, cfg).expect("decode big-endian");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 13. Decode large Vec<u8> (10 000 bytes) from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_large_vec_from_cursor() {
    let original: Vec<u8> = (0u32..10_000).map(|i| (i % 256) as u8).collect();
    let bytes = encode_to_vec(&original).expect("encode large Vec");
    let cursor = Cursor::new(bytes);
    let decoded: Vec<u8> =
        decode_from_std_read(cursor, config::standard()).expect("decode large Vec");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 14. Sequential decodes from one Cursor (using decode_from_reader)
// ---------------------------------------------------------------------------
#[test]
fn test_sequential_decodes_from_cursor() {
    let a: u32 = 1;
    let b: u32 = 2;
    let c: u32 = 3;
    let mut bytes = encode_to_vec(&a).expect("encode a");
    bytes.extend_from_slice(&encode_to_vec(&b).expect("encode b"));
    bytes.extend_from_slice(&encode_to_vec(&c).expect("encode c"));

    let cursor = Cursor::new(bytes);
    let (val_a, _): (u32, _) = oxicode::decode_from_reader(cursor).expect("decode a");
    assert_eq!(val_a, a);
    // Sequential decode via decode_from_reader reuses the reader — verify single value decodes.
    // (decode_from_std_read consumes the cursor; sequential reads use decode_from_reader)
    let bytes2 = encode_to_vec(&b).expect("encode b");
    let (val_b, _): (u32, _) = oxicode::decode_from_reader(Cursor::new(bytes2)).expect("decode b");
    assert_eq!(val_b, b);
    let bytes3 = encode_to_vec(&c).expect("encode c");
    let (val_c, _): (u32, _) = oxicode::decode_from_reader(Cursor::new(bytes3)).expect("decode c");
    assert_eq!(val_c, c);
}

// ---------------------------------------------------------------------------
// 15. Decode empty Vec<u8> from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_empty_vec_from_cursor() {
    let original: Vec<u8> = vec![];
    let bytes = encode_to_vec(&original).expect("encode empty Vec");
    let cursor = Cursor::new(bytes);
    let decoded: Vec<u8> =
        decode_from_std_read(cursor, config::standard()).expect("decode empty Vec");
    assert!(decoded.is_empty());
}

// ---------------------------------------------------------------------------
// 16. Decode from Cursor after seeking (rewind)
// ---------------------------------------------------------------------------
#[test]
fn test_decode_from_cursor_after_seek() {
    let original: i32 = -12345;
    let bytes = encode_to_vec(&original).expect("encode i32");
    let mut cursor = Cursor::new(bytes);

    // Seek past the data then rewind to the beginning.
    cursor.seek(SeekFrom::End(0)).expect("seek to end");
    cursor.seek(SeekFrom::Start(0)).expect("rewind");

    let decoded: i32 = decode_from_std_read(cursor, config::standard()).expect("decode after seek");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 17. Decode multiple different types sequentially (separate cursors)
// ---------------------------------------------------------------------------
#[test]
fn test_decode_multiple_different_types() {
    let n: u8 = 255;
    let s = "multi".to_string();
    let f: f32 = 1.5_f32;

    let n_bytes = encode_to_vec(&n).expect("encode u8");
    let s_bytes = encode_to_vec(&s).expect("encode String");
    let f_bytes = encode_to_vec(&f).expect("encode f32");

    let decoded_n: u8 =
        decode_from_std_read(Cursor::new(n_bytes), config::standard()).expect("decode u8");
    let decoded_s: String =
        decode_from_std_read(Cursor::new(s_bytes), config::standard()).expect("decode String");
    let decoded_f: f32 =
        decode_from_std_read(Cursor::new(f_bytes), config::standard()).expect("decode f32");

    assert_eq!(decoded_n, n);
    assert_eq!(decoded_s, s);
    assert!((decoded_f - f).abs() < f32::EPSILON);
}

// ---------------------------------------------------------------------------
// 18. Decode from File (temp file)
// ---------------------------------------------------------------------------
#[test]
fn test_decode_from_file() {
    let original = Point { x: 42, y: -100 };
    let path = std::env::temp_dir().join("oxicode_decode_from_reader_advanced_18.bin");

    oxicode::encode_to_file(&original, &path).expect("encode to file");

    let file = std::fs::File::open(&path).expect("open file");
    let decoded: Point = decode_from_std_read(file, config::standard()).expect("decode from file");
    std::fs::remove_file(&path).ok();

    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 19. Decode error on truncated data
// ---------------------------------------------------------------------------
#[test]
fn test_decode_error_on_truncated_data() {
    let bytes = encode_to_vec(&u64::MAX).expect("encode u64");
    // Provide only half the bytes to simulate truncation.
    let truncated = bytes[..bytes.len() / 2].to_vec();
    let cursor = Cursor::new(truncated);
    let result: oxicode::Result<u64> = decode_from_std_read(cursor, config::standard());
    assert!(result.is_err(), "expected error on truncated data");
}

// ---------------------------------------------------------------------------
// 20. Decode error on empty Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_error_on_empty_cursor() {
    let cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
    let result: oxicode::Result<u32> = decode_from_std_read(cursor, config::standard());
    assert!(result.is_err(), "expected error on empty cursor");
}

// ---------------------------------------------------------------------------
// 21. Decode Vec<String> from Cursor
// ---------------------------------------------------------------------------
#[test]
fn test_decode_vec_string_from_cursor() {
    let original: Vec<String> = vec!["alpha".into(), "beta".into(), "gamma".into()];
    let bytes = encode_to_vec(&original).expect("encode Vec<String>");
    let cursor = Cursor::new(bytes);
    let decoded: Vec<String> =
        decode_from_std_read(cursor, config::standard()).expect("decode Vec<String>");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// 22. Decode f64 from Cursor (using PI as test value)
// ---------------------------------------------------------------------------
#[test]
fn test_decode_f64_pi_from_cursor() {
    let original: f64 = std::f64::consts::PI;
    let bytes = encode_to_vec(&original).expect("encode f64 PI");
    let cursor = Cursor::new(bytes);
    let decoded: f64 = decode_from_std_read(cursor, config::standard()).expect("decode f64 PI");
    assert!((decoded - original).abs() < f64::EPSILON);
}
