//! Advanced streaming tests using encode_into_std_write and decode_from_std_read
//! with std::io::Cursor as the backing I/O object.

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
    config, decode_from_slice, decode_from_std_read, encode_into_std_write, encode_to_vec, Decode,
    Encode,
};
use std::io::Cursor;

// ---- Test 1: encode_into_std_write with Cursor<Vec<u8>> ----

#[test]
fn test_encode_into_std_write_cursor() {
    let mut buf = Cursor::new(Vec::new());
    let written = encode_into_std_write(42u32, &mut buf, config::standard())
        .expect("encode_into_std_write failed");
    assert!(written > 0, "must write at least one byte");
    assert_eq!(
        buf.get_ref().len(),
        written,
        "cursor length must match written bytes"
    );
}

// ---- Test 2: decode_from_std_read from Cursor<Vec<u8>> ----

#[test]
fn test_decode_from_std_read_cursor() {
    let bytes = encode_to_vec(&99u32).expect("encode_to_vec failed");
    let cursor = Cursor::new(bytes);
    let value: u32 =
        decode_from_std_read(cursor, config::standard()).expect("decode_from_std_read failed");
    assert_eq!(value, 99u32);
}

// ---- Test 3: Roundtrip encode_into_std_write then decode_from_std_read ----

#[test]
fn test_roundtrip_encode_into_write_decode_from_read() {
    let original: u64 = 0xDEAD_BEEF_CAFE_0042;
    let mut buf = Cursor::new(Vec::new());
    let written =
        encode_into_std_write(original, &mut buf, config::standard()).expect("encode failed");
    assert!(written > 0);

    buf.set_position(0);
    let decoded: u64 = decode_from_std_read(buf, config::standard()).expect("decode failed");
    assert_eq!(decoded, original);
}

// ---- Test 4: Multiple values written sequentially to same cursor ----

#[test]
fn test_multiple_values_written_sequentially_to_cursor() {
    let mut buf = Cursor::new(Vec::new());
    for i in 0u32..5 {
        encode_into_std_write(i, &mut buf, config::standard()).expect("encode failed");
    }
    assert!(!buf.get_ref().is_empty(), "buffer must not be empty");
    // Total bytes must be at least 5 (one per value)
    assert!(buf.get_ref().len() >= 5);
}

// ---- Test 5: Multiple values read sequentially from cursor ----

#[test]
fn test_multiple_values_read_sequentially_from_cursor() {
    let mut write_buf = Cursor::new(Vec::new());
    let values: [u32; 5] = [10, 20, 30, 40, 50];
    for &v in &values {
        encode_into_std_write(v, &mut write_buf, config::standard()).expect("encode failed");
    }

    let bytes = write_buf.into_inner();
    let mut read_buf = Cursor::new(bytes);
    for &expected in &values {
        let got: u32 =
            decode_from_std_read(&mut read_buf, config::standard()).expect("decode failed");
        assert_eq!(got, expected);
    }
}

// ---- Test 6: Large Vec<u8> (10000 bytes) via streaming ----

#[test]
fn test_large_vec_u8_streaming() {
    let data: Vec<u8> = (0u8..=255).cycle().take(10_000).collect();

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(data.clone(), &mut buf, config::standard())
        .expect("encode large Vec<u8> failed");

    buf.set_position(0);
    let decoded: Vec<u8> =
        decode_from_std_read(buf, config::standard()).expect("decode large Vec<u8> failed");

    assert_eq!(decoded.len(), 10_000);
    assert_eq!(decoded, data);
}

// ---- Test 7: String via streaming roundtrip ----

#[test]
fn test_string_streaming_roundtrip() {
    let original = "Hello, streaming OxiCode world! 🦀".to_string();

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(original.clone(), &mut buf, config::standard())
        .expect("encode String failed");

    buf.set_position(0);
    let decoded: String =
        decode_from_std_read(buf, config::standard()).expect("decode String failed");

    assert_eq!(decoded, original);
}

// ---- Test 8: Struct via streaming roundtrip (using derive) ----

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct StreamPoint {
    x: f64,
    y: f64,
    label: String,
}

#[test]
fn test_struct_streaming_roundtrip() {
    let original = StreamPoint {
        x: std::f64::consts::PI,
        y: -std::f64::consts::E,
        label: "origin-adjacent".to_string(),
    };

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(original.clone(), &mut buf, config::standard())
        .expect("encode struct failed");

    buf.set_position(0);
    let decoded: StreamPoint =
        decode_from_std_read(buf, config::standard()).expect("decode struct failed");

    assert_eq!(decoded, original);
}

// ---- Test 9: Vec<String> via streaming ----

#[test]
fn test_vec_string_streaming_roundtrip() {
    let original: Vec<String> = (0..20).map(|i| format!("entry-{:03}", i)).collect();

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(original.clone(), &mut buf, config::standard())
        .expect("encode Vec<String> failed");

    buf.set_position(0);
    let decoded: Vec<String> =
        decode_from_std_read(buf, config::standard()).expect("decode Vec<String> failed");

    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 20);
}

// ---- Test 10: Option<u64> via streaming ----

#[test]
fn test_option_u64_streaming_roundtrip() {
    let some_val: Option<u64> = Some(0xCAFE_BABE_0000_0001);
    let none_val: Option<u64> = None;

    let mut buf_some = Cursor::new(Vec::new());
    encode_into_std_write(some_val, &mut buf_some, config::standard())
        .expect("encode Some(u64) failed");
    buf_some.set_position(0);
    let decoded_some: Option<u64> =
        decode_from_std_read(buf_some, config::standard()).expect("decode Some(u64) failed");
    assert_eq!(decoded_some, some_val);

    let mut buf_none = Cursor::new(Vec::new());
    encode_into_std_write(none_val, &mut buf_none, config::standard()).expect("encode None failed");
    buf_none.set_position(0);
    let decoded_none: Option<u64> =
        decode_from_std_read(buf_none, config::standard()).expect("decode None failed");
    assert_eq!(decoded_none, none_val);
}

// ---- Test 11: Streaming with fixed int config ----

#[test]
fn test_streaming_with_fixed_int_config() {
    let value: u32 = 0xDEAD_BEEF;
    let fixed_config = config::standard().with_fixed_int_encoding();

    let mut buf = Cursor::new(Vec::new());
    let written = encode_into_std_write(value, &mut buf, fixed_config)
        .expect("encode with fixed int config failed");
    // u32 with fixed encoding is always 4 bytes
    assert_eq!(written, 4, "fixed u32 must be exactly 4 bytes");

    buf.set_position(0);
    let decoded: u32 =
        decode_from_std_read(buf, fixed_config).expect("decode with fixed int config failed");
    assert_eq!(decoded, value);
}

// ---- Test 12: Streaming with big endian config ----

#[test]
fn test_streaming_with_big_endian_config() {
    let value: u32 = 0x0102_0304;
    let be_config = config::standard().with_big_endian();

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(value, &mut buf, be_config)
        .expect("encode with big endian config failed");

    buf.set_position(0);
    let decoded: u32 =
        decode_from_std_read(buf, be_config).expect("decode with big endian config failed");
    assert_eq!(decoded, value);
}

// ---- Test 13: Write empty Vec<u8> via streaming ----

#[test]
fn test_write_empty_vec_u8_streaming() {
    let empty: Vec<u8> = Vec::new();

    let mut buf = Cursor::new(Vec::new());
    let written = encode_into_std_write(empty.clone(), &mut buf, config::standard())
        .expect("encode empty Vec<u8> failed");
    assert!(written > 0, "length-prefix must still produce bytes");

    buf.set_position(0);
    let decoded: Vec<u8> =
        decode_from_std_read(buf, config::standard()).expect("decode empty Vec<u8> failed");
    assert!(decoded.is_empty(), "decoded must be empty Vec");
}

// ---- Test 14: Read from empty cursor returns error ----

#[test]
fn test_read_from_empty_cursor_returns_error() {
    let empty_cursor = Cursor::new(Vec::<u8>::new());
    let result: Result<u32, _> = decode_from_std_read(empty_cursor, config::standard());
    assert!(
        result.is_err(),
        "decoding from empty cursor must return an error"
    );
}

// ---- Test 15: Write 5 u32 values, read back 5 ----

#[test]
fn test_write_5_u32_read_back_5() {
    let values: [u32; 5] = [111, 222, 333, 444, 555];
    let mut buf = Cursor::new(Vec::new());
    for &v in &values {
        encode_into_std_write(v, &mut buf, config::standard()).expect("encode u32 failed");
    }

    buf.set_position(0);
    let mut decoded = [0u32; 5];
    for slot in &mut decoded {
        *slot = decode_from_std_read(&mut buf, config::standard()).expect("decode u32 failed");
    }
    assert_eq!(decoded, values);
}

// ---- Test 16: Write enum via streaming roundtrip ----

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum StreamCommand {
    Noop,
    Move { dx: i32, dy: i32 },
    Write(String),
    SetColor(u8, u8, u8),
}

#[test]
fn test_enum_streaming_roundtrip() {
    let variants = vec![
        StreamCommand::Noop,
        StreamCommand::Move { dx: -5, dy: 10 },
        StreamCommand::Write("hello enum".to_string()),
        StreamCommand::SetColor(255, 128, 0),
    ];

    for original in variants {
        let mut buf = Cursor::new(Vec::new());
        encode_into_std_write(original.clone(), &mut buf, config::standard())
            .expect("encode enum failed");

        buf.set_position(0);
        let decoded: StreamCommand =
            decode_from_std_read(buf, config::standard()).expect("decode enum failed");
        assert_eq!(decoded, original);
    }
}

// ---- Test 17: Write bool values via streaming ----

#[test]
fn test_bool_streaming_roundtrip() {
    for original in [true, false] {
        let mut buf = Cursor::new(Vec::new());
        let written = encode_into_std_write(original, &mut buf, config::standard())
            .expect("encode bool failed");
        assert_eq!(written, 1, "bool must encode to exactly 1 byte");

        buf.set_position(0);
        let decoded: bool =
            decode_from_std_read(buf, config::standard()).expect("decode bool failed");
        assert_eq!(decoded, original);
    }
}

// ---- Test 18: Write i128 via streaming ----

#[test]
fn test_i128_streaming_roundtrip() {
    let value: i128 = i128::MIN;

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(value, &mut buf, config::standard()).expect("encode i128 failed");

    buf.set_position(0);
    let decoded: i128 = decode_from_std_read(buf, config::standard()).expect("decode i128 failed");
    assert_eq!(decoded, value);
}

// ---- Test 19: Write u128 via streaming ----

#[test]
fn test_u128_streaming_roundtrip() {
    let value: u128 = u128::MAX;

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(value, &mut buf, config::standard()).expect("encode u128 failed");

    buf.set_position(0);
    let decoded: u128 = decode_from_std_read(buf, config::standard()).expect("decode u128 failed");
    assert_eq!(decoded, value);
}

// ---- Test 20: Large struct with multiple fields via streaming ----

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct BigRecord {
    id: u64,
    name: String,
    tags: Vec<String>,
    score: f64,
    active: bool,
    flags: [u8; 16],
}

#[test]
fn test_large_struct_streaming_roundtrip() {
    let original = BigRecord {
        id: 0xABCD_EF01_2345_6789,
        name: "streaming-big-record".to_string(),
        tags: (0..10).map(|i| format!("tag-{}", i)).collect(),
        score: std::f64::consts::PI,
        active: true,
        flags: [0xAB; 16],
    };

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(original.clone(), &mut buf, config::standard())
        .expect("encode BigRecord failed");

    buf.set_position(0);
    let decoded: BigRecord =
        decode_from_std_read(buf, config::standard()).expect("decode BigRecord failed");

    assert_eq!(decoded, original);
}

// ---- Test 21: Write then rewind cursor, then decode ----

#[test]
fn test_write_rewind_then_decode() {
    let original: Vec<u32> = (0u32..50).collect();

    let mut buf = Cursor::new(Vec::new());
    encode_into_std_write(original.clone(), &mut buf, config::standard())
        .expect("encode Vec<u32> failed");

    // Explicitly rewind to position 0
    buf.set_position(0);

    let decoded: Vec<u32> =
        decode_from_std_read(buf, config::standard()).expect("decode after rewind failed");

    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 50);
}

// ---- Test 22: Streaming size matches encode_to_vec length ----

#[test]
fn test_streaming_size_matches_encode_to_vec_length() {
    let value: Vec<u64> = (0u64..100).collect();

    // Encode via encode_to_vec (reference path)
    let reference_bytes = encode_to_vec(&value).expect("encode_to_vec failed");

    // Encode via encode_into_std_write
    let mut buf = Cursor::new(Vec::new());
    let written = encode_into_std_write(value.clone(), &mut buf, config::standard())
        .expect("encode_into_std_write failed");

    assert_eq!(
        written,
        reference_bytes.len(),
        "bytes written via encode_into_std_write ({}) must match encode_to_vec length ({})",
        written,
        reference_bytes.len()
    );
    assert_eq!(
        buf.get_ref().as_slice(),
        reference_bytes.as_slice(),
        "encoded bytes must be identical"
    );

    // Verify roundtrip from the streaming buffer
    buf.set_position(0);
    let decoded: Vec<u64> = decode_from_std_read(buf, config::standard()).expect("decode failed");
    assert_eq!(decoded, value);

    // Also verify via decode_from_slice on reference bytes
    let (from_slice, _): (Vec<u64>, _) =
        decode_from_slice(&reference_bytes).expect("decode_from_slice failed");
    assert_eq!(from_slice, value);
}
