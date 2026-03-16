//! Advanced tests for `encode_to_writer` and `decode_from_reader` (22 tests).

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
    config, decode_from_reader, decode_from_reader_with_config, decode_from_slice, encode_to_vec,
    encode_to_writer, encode_to_writer_with_config,
};
use std::io::Cursor;

// ---------------------------------------------------------------------------
// 1. encode_to_writer u32 into Vec<u8> — bytes match encode_to_vec
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_u32_bytes_match_encode_to_vec() {
    let val: u32 = 0xDEAD_BEEF;
    let vec_bytes = encode_to_vec(&val).expect("encode_to_vec u32");
    let mut writer_bytes: Vec<u8> = Vec::new();
    encode_to_writer(&val, &mut writer_bytes).expect("encode_to_writer u32");
    assert_eq!(vec_bytes, writer_bytes);
}

// ---------------------------------------------------------------------------
// 2. encode_to_writer String into Vec<u8>, decode_from_reader roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_string_decode_from_reader() {
    let original = "oxicode streaming test".to_string();
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer String");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (String, _) = decode_from_reader(cursor).expect("decode_from_reader String");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 3. encode_to_writer u64 — full roundtrip via writer/reader
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_u64_roundtrip() {
    let original: u64 = u64::MAX / 3;
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer u64");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (u64, _) = decode_from_reader(cursor).expect("decode_from_reader u64");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 4. encode_to_writer returns correct byte count
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_returns_correct_byte_count() {
    let val: u32 = 999;
    let mut buf: Vec<u8> = Vec::new();
    let n = encode_to_writer(&val, &mut buf).expect("encode_to_writer count");
    assert_eq!(n, buf.len());
    assert!(n > 0);
}

// ---------------------------------------------------------------------------
// 5. encode_to_writer Vec<u32> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_vec_u32_roundtrip() {
    let original: Vec<u32> = vec![1, 2, 3, 100, 200, u32::MAX];
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer Vec<u32>");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (Vec<u32>, _) =
        decode_from_reader(cursor).expect("decode_from_reader Vec<u32>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 6. encode_to_writer Option<u32> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_option_u32_some_roundtrip() {
    let original: Option<u32> = Some(42);
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer Option Some");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (Option<u32>, _) =
        decode_from_reader(cursor).expect("decode_from_reader Option Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 7. encode_to_writer Option<u32> None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_option_u32_none_roundtrip() {
    let original: Option<u32> = None;
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer Option None");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (Option<u32>, _) =
        decode_from_reader(cursor).expect("decode_from_reader Option None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 8. encode_to_writer bool true and false roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_bool_roundtrip() {
    for &flag in &[true, false] {
        let mut buf: Vec<u8> = Vec::new();
        encode_to_writer(&flag, &mut buf).expect("encode_to_writer bool");
        let cursor = Cursor::new(&buf[..]);
        let (decoded, _): (bool, _) = decode_from_reader(cursor).expect("decode_from_reader bool");
        assert_eq!(flag, decoded);
    }
}

// ---------------------------------------------------------------------------
// 9. Multiple sequential encode_to_writer calls into same buffer
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_sequential_encode_to_writer_into_same_buffer() {
    let a: u32 = 10;
    let b: u32 = 20;
    let c: u32 = 30;
    let mut buf: Vec<u8> = Vec::new();
    let n_a = encode_to_writer(&a, &mut buf).expect("encode a");
    let n_b = encode_to_writer(&b, &mut buf).expect("encode b");
    let n_c = encode_to_writer(&c, &mut buf).expect("encode c");
    assert_eq!(buf.len(), n_a + n_b + n_c);
}

// ---------------------------------------------------------------------------
// 10. Multiple sequential decode_from_reader calls from same cursor
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_sequential_decode_from_reader_from_same_cursor() {
    let a: u32 = 111;
    let b: u32 = 222;
    let c: u32 = 333;
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&a, &mut buf).expect("encode a");
    encode_to_writer(&b, &mut buf).expect("encode b");
    encode_to_writer(&c, &mut buf).expect("encode c");

    let mut cursor = Cursor::new(&buf[..]);
    let (decoded_a, n_a): (u32, _) = decode_from_reader(&mut cursor).expect("decode a");
    let (decoded_b, n_b): (u32, _) = decode_from_reader(&mut cursor).expect("decode b");
    let (decoded_c, n_c): (u32, _) = decode_from_reader(&mut cursor).expect("decode c");

    assert_eq!(decoded_a, a);
    assert_eq!(decoded_b, b);
    assert_eq!(decoded_c, c);
    assert_eq!(n_a + n_b + n_c, buf.len());
}

// ---------------------------------------------------------------------------
// 11. encode_to_writer then decode_from_slice — same wire format
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_then_decode_from_slice_same_format() {
    let original: u32 = 0xABCD_1234;
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer");
    let (decoded, consumed): (u32, _) = decode_from_slice(&buf).expect("decode_from_slice");
    assert_eq!(original, decoded);
    assert_eq!(consumed, buf.len());
}

// ---------------------------------------------------------------------------
// 12. encode_to_vec then decode_from_reader — same wire format
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_vec_then_decode_from_reader_same_format() {
    let original: u64 = 0x1122_3344_5566_7788;
    let buf = encode_to_vec(&original).expect("encode_to_vec");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, n): (u64, _) = decode_from_reader(cursor).expect("decode_from_reader");
    assert_eq!(original, decoded);
    assert_eq!(n, buf.len());
}

// ---------------------------------------------------------------------------
// 13. encode_to_writer_with_config fixed-int u32
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_with_config_fixed_int_u32() {
    let val: u32 = 12345;
    let cfg = config::standard().with_fixed_int_encoding();
    let mut buf: Vec<u8> = Vec::new();
    let n = encode_to_writer_with_config(&val, &mut buf, cfg).expect("encode fixed-int");
    assert_eq!(n, 4, "fixed-int u32 must be 4 bytes");
    assert_eq!(buf.len(), 4);
}

// ---------------------------------------------------------------------------
// 14. encode_to_writer_with_config big-endian u32
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_with_config_big_endian_u32() {
    let val: u32 = 0x0102_0304;
    let cfg_be = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let mut buf_be: Vec<u8> = Vec::new();
    encode_to_writer_with_config(&val, &mut buf_be, cfg_be).expect("encode big-endian");

    let cfg_le = config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let mut buf_le: Vec<u8> = Vec::new();
    encode_to_writer_with_config(&val, &mut buf_le, cfg_le).expect("encode little-endian");

    // The two buffers should be reverses of each other (fixed-int, 4 bytes)
    let reversed: Vec<u8> = buf_be.iter().copied().rev().collect();
    assert_eq!(reversed, buf_le);
}

// ---------------------------------------------------------------------------
// 15. decode_from_reader_with_config fixed-int u32
// ---------------------------------------------------------------------------
#[test]
fn test_decode_from_reader_with_config_fixed_int_u32() {
    let val: u32 = 99999;
    let cfg = config::standard().with_fixed_int_encoding();
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer_with_config(&val, &mut buf, cfg).expect("encode fixed-int");
    assert_eq!(buf.len(), 4);
    let cursor = Cursor::new(&buf[..]);
    let (decoded, n): (u32, _) =
        decode_from_reader_with_config(cursor, cfg).expect("decode fixed-int");
    assert_eq!(decoded, val);
    assert_eq!(n, 4);
}

// ---------------------------------------------------------------------------
// 16. decode_from_reader_with_config big-endian u32
// ---------------------------------------------------------------------------
#[test]
fn test_decode_from_reader_with_config_big_endian_u32() {
    let val: u32 = 0xCAFE_BABE;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer_with_config(&val, &mut buf, cfg).expect("encode big-endian");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, n): (u32, _) =
        decode_from_reader_with_config(cursor, cfg).expect("decode big-endian");
    assert_eq!(decoded, val);
    assert_eq!(n, 4);
}

// ---------------------------------------------------------------------------
// 17. encode_to_writer i128 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_i128_roundtrip() {
    let original: i128 = i128::MIN / 7;
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer i128");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (i128, _) = decode_from_reader(cursor).expect("decode_from_reader i128");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 18. encode_to_writer f64 using std::f64::consts::PI
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_f64_pi_roundtrip() {
    let original: f64 = std::f64::consts::PI;
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer f64 PI");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (f64, _) = decode_from_reader(cursor).expect("decode_from_reader f64 PI");
    assert!((decoded - original).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// 19. encode_to_writer Vec<String> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_vec_string_roundtrip() {
    let original: Vec<String> = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma delta".to_string(),
    ];
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer Vec<String>");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (Vec<String>, _) =
        decode_from_reader(cursor).expect("decode_from_reader Vec<String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 20. encode_to_writer into File (temp dir), decode_from_reader from File
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_into_file_roundtrip() {
    let original: u64 = 0xFEED_FACE_DEAD_BEEF;
    let path = std::env::temp_dir().join("oxicode_encode_to_writer_advanced2_test_20.bin");
    {
        let mut file = std::fs::File::create(&path).expect("create temp file");
        encode_to_writer(&original, &mut file).expect("encode_to_writer into File");
    }
    {
        let file = std::fs::File::open(&path).expect("open temp file");
        let (decoded, _): (u64, _) =
            decode_from_reader(file).expect("decode_from_reader from File");
        assert_eq!(original, decoded);
    }
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// 21. Sequential encode/decode of 3 different types into/from same buffer
// ---------------------------------------------------------------------------
#[test]
fn test_sequential_encode_decode_three_different_types() {
    let a: u32 = 42;
    let b: String = "hello".to_string();
    let c: f64 = std::f64::consts::E;

    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&a, &mut buf).expect("encode u32");
    encode_to_writer(&b, &mut buf).expect("encode String");
    encode_to_writer(&c, &mut buf).expect("encode f64");

    let mut cursor = Cursor::new(&buf[..]);
    let (decoded_a, _): (u32, _) = decode_from_reader(&mut cursor).expect("decode u32");
    let (decoded_b, _): (String, _) = decode_from_reader(&mut cursor).expect("decode String");
    let (decoded_c, _): (f64, _) = decode_from_reader(&mut cursor).expect("decode f64");

    assert_eq!(decoded_a, a);
    assert_eq!(decoded_b, b);
    assert!((decoded_c - c).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// 22. encode_to_writer empty Vec<u8> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_to_writer_empty_vec_u8_roundtrip() {
    let original: Vec<u8> = Vec::new();
    let mut buf: Vec<u8> = Vec::new();
    encode_to_writer(&original, &mut buf).expect("encode_to_writer empty Vec<u8>");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (Vec<u8>, _) =
        decode_from_reader(cursor).expect("decode_from_reader empty Vec<u8>");
    assert!(decoded.is_empty());
}
