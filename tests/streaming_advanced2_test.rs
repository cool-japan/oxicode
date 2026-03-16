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
    config, decode_from_slice, decode_from_std_read, encode_into_std_write, encode_to_vec, Decode,
    Encode,
};
use std::io::{BufReader, BufWriter, Cursor, Write};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Frame {
    id: u32,
    data: Vec<u8>,
    flags: u8,
}

// Test 1: Write u32 to Cursor<Vec<u8>>, read it back
#[test]
fn test_write_u32_to_cursor_read_back() {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(42u32, &mut cursor, config::standard()).expect("encode u32");
    cursor.set_position(0);
    let decoded: u32 = decode_from_std_read(&mut cursor, config::standard()).expect("decode u32");
    assert_eq!(decoded, 42u32);
}

// Test 2: Write String to Cursor<Vec<u8>>, read it back
#[test]
fn test_write_string_to_cursor_read_back() {
    let original = String::from("hello oxicode");
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(original.clone(), &mut cursor, config::standard())
        .expect("encode String");
    cursor.set_position(0);
    let decoded: String =
        decode_from_std_read(&mut cursor, config::standard()).expect("decode String");
    assert_eq!(decoded, original);
}

// Test 3: Write Frame struct to Cursor, read it back
#[test]
fn test_write_frame_struct_to_cursor_read_back() {
    let frame = Frame {
        id: 7,
        data: vec![1, 2, 3, 4, 5],
        flags: 0xAB,
    };
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(frame.id, &mut cursor, config::standard()).expect("encode frame.id");
    encode_into_std_write(frame.data.clone(), &mut cursor, config::standard())
        .expect("encode frame.data");
    encode_into_std_write(frame.flags, &mut cursor, config::standard())
        .expect("encode frame.flags");
    cursor.set_position(0);
    let id: u32 = decode_from_std_read(&mut cursor, config::standard()).expect("decode id");
    let data: Vec<u8> = decode_from_std_read(&mut cursor, config::standard()).expect("decode data");
    let flags: u8 = decode_from_std_read(&mut cursor, config::standard()).expect("decode flags");
    let decoded = Frame { id, data, flags };
    assert_eq!(
        decoded,
        Frame {
            id: 7,
            data: vec![1, 2, 3, 4, 5],
            flags: 0xAB
        }
    );
}

// Test 4: Write 5 u32 values sequentially, read all 5 back in order
#[test]
fn test_write_5_u32_sequential_read_back_in_order() {
    let values = [10u32, 20, 30, 40, 50];
    let mut cursor = Cursor::new(Vec::<u8>::new());
    for &v in &values {
        encode_into_std_write(v, &mut cursor, config::standard()).expect("encode u32 value");
    }
    cursor.set_position(0);
    for &expected in &values {
        let decoded: u32 =
            decode_from_std_read(&mut cursor, config::standard()).expect("decode u32 value");
        assert_eq!(decoded, expected);
    }
}

// Test 5: Write 3 Frame structs sequentially, read all 3 back
#[test]
fn test_write_3_frames_sequential_read_back() {
    let frames = vec![
        Frame {
            id: 1,
            data: vec![0xAA],
            flags: 0x01,
        },
        Frame {
            id: 2,
            data: vec![0xBB, 0xCC],
            flags: 0x02,
        },
        Frame {
            id: 3,
            data: vec![0xDD, 0xEE, 0xFF],
            flags: 0x03,
        },
    ];
    let mut cursor = Cursor::new(Vec::<u8>::new());
    for frame in &frames {
        let encoded = encode_to_vec(frame).expect("encode frame to vec");
        cursor.write_all(&encoded).expect("write frame bytes");
    }
    cursor.set_position(0);
    for expected in &frames {
        let decoded: Frame =
            decode_from_std_read(&mut cursor, config::standard()).expect("decode frame");
        assert_eq!(&decoded, expected);
    }
}

// Test 6: Write u8, u16, u32 heterogeneous sequence, read back in order
#[test]
fn test_write_heterogeneous_u8_u16_u32_read_back() {
    let val_u8: u8 = 0xFF;
    let val_u16: u16 = 0x1234;
    let val_u32: u32 = 0xDEAD_BEEF;
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(val_u8, &mut cursor, config::standard()).expect("encode u8");
    encode_into_std_write(val_u16, &mut cursor, config::standard()).expect("encode u16");
    encode_into_std_write(val_u32, &mut cursor, config::standard()).expect("encode u32");
    cursor.set_position(0);
    let d_u8: u8 = decode_from_std_read(&mut cursor, config::standard()).expect("decode u8");
    let d_u16: u16 = decode_from_std_read(&mut cursor, config::standard()).expect("decode u16");
    let d_u32: u32 = decode_from_std_read(&mut cursor, config::standard()).expect("decode u32");
    assert_eq!(d_u8, val_u8);
    assert_eq!(d_u16, val_u16);
    assert_eq!(d_u32, val_u32);
}

// Test 7: Write to BufWriter, read from BufReader
#[test]
fn test_write_to_bufwriter_read_from_bufreader() {
    let inner = Vec::<u8>::new();
    let mut buf_writer = BufWriter::new(inner);
    encode_into_std_write(99u32, &mut buf_writer, config::standard())
        .expect("encode into BufWriter");
    let inner = buf_writer.into_inner().expect("flush BufWriter");
    let cursor = Cursor::new(inner);
    let mut buf_reader = BufReader::new(cursor);
    let decoded: u32 =
        decode_from_std_read(&mut buf_reader, config::standard()).expect("decode from BufReader");
    assert_eq!(decoded, 99u32);
}

// Test 8: Cursor position advances after each write
#[test]
fn test_cursor_position_advances_after_each_write() {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let pos_before = cursor.position();
    encode_into_std_write(1u32, &mut cursor, config::standard()).expect("first write");
    let pos_after_first = cursor.position();
    assert!(
        pos_after_first > pos_before,
        "position should advance after first write"
    );
    encode_into_std_write(2u32, &mut cursor, config::standard()).expect("second write");
    let pos_after_second = cursor.position();
    assert!(
        pos_after_second > pos_after_first,
        "position should advance after second write"
    );
}

// Test 9: Cursor position advances after each read
#[test]
fn test_cursor_position_advances_after_each_read() {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(100u32, &mut cursor, config::standard()).expect("encode first");
    encode_into_std_write(200u32, &mut cursor, config::standard()).expect("encode second");
    cursor.set_position(0);
    let pos_start = cursor.position();
    let _: u32 = decode_from_std_read(&mut cursor, config::standard()).expect("first read");
    let pos_after_first_read = cursor.position();
    assert!(
        pos_after_first_read > pos_start,
        "position should advance after first read"
    );
    let _: u32 = decode_from_std_read(&mut cursor, config::standard()).expect("second read");
    let pos_after_second_read = cursor.position();
    assert!(
        pos_after_second_read > pos_after_first_read,
        "position should advance after second read"
    );
}

// Test 10: Write then read: position after read == total bytes written
#[test]
fn test_cursor_position_after_read_equals_total_bytes_written() {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let n1 = encode_into_std_write(42u32, &mut cursor, config::standard()).expect("encode first");
    let n2 = encode_into_std_write(84u32, &mut cursor, config::standard()).expect("encode second");
    let total_written = (n1 + n2) as u64;
    cursor.set_position(0);
    let _: u32 = decode_from_std_read(&mut cursor, config::standard()).expect("read first");
    let _: u32 = decode_from_std_read(&mut cursor, config::standard()).expect("read second");
    assert_eq!(cursor.position(), total_written);
}

// Test 11: Write Vec<u8> to cursor, read back, verify length and content
#[test]
fn test_write_vec_u8_read_back_verify_length_and_content() {
    let data: Vec<u8> = (0u8..16).collect();
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(data.clone(), &mut cursor, config::standard()).expect("encode Vec<u8>");
    cursor.set_position(0);
    let decoded: Vec<u8> =
        decode_from_std_read(&mut cursor, config::standard()).expect("decode Vec<u8>");
    assert_eq!(decoded.len(), data.len());
    assert_eq!(decoded, data);
}

// Test 12: Write Option<String> Some, read back
#[test]
fn test_write_option_string_some_read_back() {
    let value: Option<String> = Some(String::from("present"));
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(value.clone(), &mut cursor, config::standard())
        .expect("encode Option Some");
    cursor.set_position(0);
    let decoded: Option<String> =
        decode_from_std_read(&mut cursor, config::standard()).expect("decode Option Some");
    assert_eq!(decoded, value);
}

// Test 13: Write Option<String> None, read back
#[test]
fn test_write_option_string_none_read_back() {
    let value: Option<String> = None;
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(value.clone(), &mut cursor, config::standard())
        .expect("encode Option None");
    cursor.set_position(0);
    let decoded: Option<String> =
        decode_from_std_read(&mut cursor, config::standard()).expect("decode Option None");
    assert_eq!(decoded, None);
}

// Test 14: Write bool true/false pair, read back both
#[test]
fn test_write_bool_true_false_pair_read_back() {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(true, &mut cursor, config::standard()).expect("encode true");
    encode_into_std_write(false, &mut cursor, config::standard()).expect("encode false");
    cursor.set_position(0);
    let decoded_true: bool =
        decode_from_std_read(&mut cursor, config::standard()).expect("decode true");
    let decoded_false: bool =
        decode_from_std_read(&mut cursor, config::standard()).expect("decode false");
    assert!(decoded_true);
    assert!(!decoded_false);
}

// Test 15: Write to Vec<u8> using &mut Vec<u8> as writer, read back with Cursor
#[test]
fn test_write_to_vec_u8_as_writer_read_back_with_cursor() {
    let mut buf = Vec::<u8>::new();
    encode_into_std_write(777u32, &mut buf, config::standard()).expect("encode into Vec");
    let cursor = Cursor::new(&buf[..]);
    let (decoded, _): (u32, _) = decode_from_slice(&buf).expect("decode from slice");
    let _ = cursor;
    assert_eq!(decoded, 777u32);
}

// Test 16: Fixed-int config: write u64 with encode_into_std_write, read back
#[test]
fn test_fixed_int_config_write_u64_read_back() {
    let value: u64 = 0x0102_0304_0506_0708;
    let fixed_config = config::standard().with_fixed_int_encoding();
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(value, &mut cursor, fixed_config).expect("encode u64 fixed");
    cursor.set_position(0);
    let decoded: u64 = decode_from_std_read(&mut cursor, fixed_config).expect("decode u64 fixed");
    assert_eq!(decoded, value);
}

// Test 17: Standard config: sequential write/read of mixed types
#[test]
fn test_standard_config_sequential_write_read_mixed_types() {
    let cfg = config::standard();
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(255u8, &mut cursor, cfg).expect("encode u8");
    encode_into_std_write(String::from("mixed"), &mut cursor, cfg).expect("encode String");
    encode_into_std_write(3.14f32, &mut cursor, cfg).expect("encode f32");
    cursor.set_position(0);
    let d_u8: u8 = decode_from_std_read(&mut cursor, cfg).expect("decode u8");
    let d_str: String = decode_from_std_read(&mut cursor, cfg).expect("decode String");
    let d_f32: f32 = decode_from_std_read(&mut cursor, cfg).expect("decode f32");
    assert_eq!(d_u8, 255u8);
    assert_eq!(d_str, "mixed");
    assert!((d_f32 - 3.14f32).abs() < 1e-5);
}

// Test 18: Write unit struct, verify position doesn't advance (0 bytes encoded)
#[test]
fn test_write_unit_struct_position_unchanged() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Unit;

    let mut cursor = Cursor::new(Vec::<u8>::new());
    let before = cursor.position();
    encode_into_std_write(Unit, &mut cursor, config::standard()).expect("encode Unit");
    let after = cursor.position();
    assert_eq!(before, after, "unit struct should encode to 0 bytes");
}

// Test 19: Error case: reading from empty Cursor returns error
#[test]
fn test_read_from_empty_cursor_returns_error() {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let result: oxicode::Result<u32> = decode_from_std_read(&mut cursor, config::standard());
    assert!(
        result.is_err(),
        "decoding from empty cursor should return an error"
    );
}

// Test 20: BufReader wrap: read after partial write (only N-1 bytes for u32) returns error
#[test]
fn test_partial_write_returns_decode_error() {
    // A u32 under fixed-int encoding is exactly 4 bytes. Write only 3 bytes.
    let partial: Vec<u8> = vec![0x01, 0x02, 0x03];
    let cursor = Cursor::new(partial);
    let mut buf_reader = BufReader::new(cursor);
    let result: oxicode::Result<u32> = decode_from_std_read(
        &mut buf_reader,
        config::standard().with_fixed_int_encoding(),
    );
    assert!(
        result.is_err(),
        "decoding partial u32 bytes should return an error"
    );
}

// Test 21: Write large Vec<u8> (1000 bytes) to cursor, read back
#[test]
fn test_write_large_vec_u8_1000_bytes_read_back() {
    let data: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let mut cursor = Cursor::new(Vec::<u8>::new());
    encode_into_std_write(data.clone(), &mut cursor, config::standard()).expect("encode large vec");
    cursor.set_position(0);
    let decoded: Vec<u8> =
        decode_from_std_read(&mut cursor, config::standard()).expect("decode large vec");
    assert_eq!(decoded.len(), 1000);
    assert_eq!(decoded, data);
}

// Test 22: Interleave: write u32, read u32, write String, read String from same cursor
#[test]
fn test_interleave_write_read_u32_then_string() {
    let mut cursor = Cursor::new(Vec::<u8>::new());

    // Write u32 at position 0
    encode_into_std_write(12345u32, &mut cursor, config::standard()).expect("write u32");
    let after_u32_write = cursor.position();

    // Seek back and read u32
    cursor.set_position(0);
    let decoded_u32: u32 = decode_from_std_read(&mut cursor, config::standard()).expect("read u32");
    assert_eq!(decoded_u32, 12345u32);

    // Now at after_u32_write position, write String
    cursor.set_position(after_u32_write);
    encode_into_std_write(String::from("interleaved"), &mut cursor, config::standard())
        .expect("write String");

    // Seek back to after_u32_write and read String
    cursor.set_position(after_u32_write);
    let decoded_str: String =
        decode_from_std_read(&mut cursor, config::standard()).expect("read String");
    assert_eq!(decoded_str, "interleaved");
}
