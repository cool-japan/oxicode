//! Streaming format tests for StreamingEncoder / StreamingDecoder.
//!
//! Tests the streaming encoder/decoder with a variety of value types,
//! configurations, and edge cases. Uses both the buffer-backed API
//! (no std required) and the IO-backed API (std feature).

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
use oxicode::streaming::{
    BufferStreamingDecoder, BufferStreamingEncoder, StreamingConfig, CHUNK_MAGIC,
};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Test 1: Encode then decode a single u32
// ---------------------------------------------------------------------------

#[test]
fn test_sf_single_u32_roundtrip() {
    let original: u32 = 0xABCD_1234;
    let mut encoder = BufferStreamingEncoder::new();
    encoder.write_item(&original).expect("write_item failed");
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded = decoder
        .read_item::<u32>()
        .expect("read_item failed")
        .expect("expected Some(u32)");

    assert_eq!(decoded, original);
    assert_eq!(decoder.read_item::<u32>().expect("read after end"), None);
    assert!(decoder.is_finished());
}

// ---------------------------------------------------------------------------
// Test 2: Encode an empty sequence — decoder immediately returns None
// ---------------------------------------------------------------------------

#[test]
fn test_sf_empty_sequence_decodes_to_none() {
    let encoder = BufferStreamingEncoder::new();
    let encoded = encoder.finish();

    // An empty encoder produces exactly a 13-byte end header.
    assert_eq!(
        encoded.len(),
        13,
        "empty stream must be exactly 13 bytes, got {}",
        encoded.len()
    );

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    // First read must be None, not an error
    let result = decoder
        .read_item::<u32>()
        .expect("read_item on empty stream must not error");
    assert!(result.is_none(), "expected None from empty stream");
    assert!(decoder.is_finished());
}

// ---------------------------------------------------------------------------
// Test 3: Encode 100 u32 values, decode all, verify order and count
// ---------------------------------------------------------------------------

#[test]
fn test_sf_encode_100_u32_decode_all() {
    let values: Vec<u32> = (200u32..300).collect();
    let mut encoder = BufferStreamingEncoder::new();
    for v in &values {
        encoder.write_item(v).expect("write_item failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");

    assert_eq!(decoded.len(), 100);
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 4: Encode String values
// ---------------------------------------------------------------------------

#[test]
fn test_sf_string_values() {
    let strings: Vec<String> = vec![
        "hello".to_string(),
        "streaming".to_string(),
        "format".to_string(),
        "test".to_string(),
    ];

    let mut encoder = BufferStreamingEncoder::new();
    for s in &strings {
        encoder.write_item(s).expect("write_item(String) failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<String> = decoder.read_all().expect("read_all(String) failed");

    assert_eq!(decoded, strings);
}

// ---------------------------------------------------------------------------
// Test 5: Encode struct values, verify roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[test]
fn test_sf_struct_values_roundtrip() {
    let points: Vec<Point3D> = (0..8)
        .map(|i| Point3D {
            x: i as f32 * 1.5,
            y: i as f32 * -2.0,
            z: i as f32 * 0.5,
        })
        .collect();

    let mut encoder = BufferStreamingEncoder::new();
    for p in &points {
        encoder.write_item(p).expect("write_item(Point3D) failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<Point3D> = decoder.read_all().expect("read_all(Point3D) failed");

    assert_eq!(decoded, points);
}

// ---------------------------------------------------------------------------
// Test 6: StreamingEncoder backed by Vec<u8> via IO path
// ---------------------------------------------------------------------------

#[cfg(feature = "std")]
#[test]
fn test_sf_encoder_to_vec_backing_store() {
    use oxicode::streaming::StreamingEncoder;

    let mut backing: Vec<u8> = Vec::new();
    let values: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    {
        let mut encoder = StreamingEncoder::new(&mut backing);
        for v in &values {
            encoder.write_item(v).expect("write_item failed");
        }
        encoder.finish().expect("finish failed");
    }

    assert!(
        !backing.is_empty(),
        "Vec<u8> backing must be non-empty after encoding"
    );
    // The backing bytes start with the CHUNK_MAGIC bytes
    assert_eq!(
        &backing[..4],
        &CHUNK_MAGIC,
        "encoded bytes must start with OXIS magic"
    );
}

// ---------------------------------------------------------------------------
// Test 7: StreamingDecoder from a Cursor (slice-like reader)
// ---------------------------------------------------------------------------

#[cfg(feature = "std")]
#[test]
fn test_sf_decoder_from_cursor_slice() {
    use oxicode::streaming::{StreamingDecoder, StreamingEncoder};
    use std::io::Cursor;

    let values: Vec<u64> = vec![100, 200, 300, 400, 500];
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut encoder = StreamingEncoder::new(&mut buf);
        for v in &values {
            encoder.write_item(v).expect("write_item failed");
        }
        encoder.finish().expect("finish failed");
    }

    // Decode via Cursor wrapping a &[u8] slice
    let cursor = Cursor::new(buf.as_slice());
    let mut decoder = StreamingDecoder::new(cursor);
    let decoded: Vec<u64> = decoder.read_all().expect("read_all from cursor failed");

    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 8: Multiple types encoded in separate streams in one test
// ---------------------------------------------------------------------------

#[test]
fn test_sf_multiple_types_separate_streams() {
    // u32 stream
    let u32_stream = {
        let mut e = BufferStreamingEncoder::new();
        e.write_item(&42u32).expect("write u32");
        e.finish()
    };
    // i64 stream
    let i64_stream = {
        let mut e = BufferStreamingEncoder::new();
        e.write_item(&-99i64).expect("write i64");
        e.finish()
    };
    // bool stream
    let bool_stream = {
        let mut e = BufferStreamingEncoder::new();
        e.write_item(&true).expect("write bool");
        e.finish()
    };

    let u32_val = BufferStreamingDecoder::new(&u32_stream)
        .read_item::<u32>()
        .expect("decode u32")
        .expect("expected Some(u32)");
    let i64_val = BufferStreamingDecoder::new(&i64_stream)
        .read_item::<i64>()
        .expect("decode i64")
        .expect("expected Some(i64)");
    let bool_val = BufferStreamingDecoder::new(&bool_stream)
        .read_item::<bool>()
        .expect("decode bool")
        .expect("expected Some(bool)");

    assert_eq!(u32_val, 42u32);
    assert_eq!(i64_val, -99i64);
    assert!(bool_val);
}

// ---------------------------------------------------------------------------
// Test 9: Flush behavior — flush_per_item produces one chunk per item
// ---------------------------------------------------------------------------

#[cfg(feature = "std")]
#[test]
fn test_sf_flush_per_item_produces_one_chunk_per_item() {
    use oxicode::streaming::{StreamingDecoder, StreamingEncoder};
    use std::io::Cursor;

    let config = StreamingConfig::new().with_flush_per_item(true);
    let values: Vec<u32> = vec![10, 20, 30];

    let mut buf: Vec<u8> = Vec::new();
    {
        let mut encoder = StreamingEncoder::with_config(&mut buf, config);
        for v in &values {
            encoder.write_item(v).expect("write_item failed");
        }
        encoder.finish().expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = StreamingDecoder::new(cursor);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");

    assert_eq!(decoded, values);
    // With flush_per_item, each of the 3 items must land in its own chunk
    assert_eq!(
        decoder.progress().chunks_processed,
        3,
        "expected 3 chunks (one per item), got {}",
        decoder.progress().chunks_processed
    );
}

// ---------------------------------------------------------------------------
// Test 10: Large data — encode and decode 10 000 items
// ---------------------------------------------------------------------------

#[test]
fn test_sf_large_data_10000_items() {
    let count: u32 = 10_000;
    let values: Vec<u32> = (0..count).collect();

    let mut encoder = BufferStreamingEncoder::new();
    for v in &values {
        encoder.write_item(v).expect("write_item failed");
    }
    let encoded = encoder.finish();
    assert!(
        !encoded.is_empty(),
        "encoded buffer must not be empty for 10000 items"
    );

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all(10000 items) failed");

    assert_eq!(decoded.len(), count as usize);
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 11: Byte count verification — encoded bytes match expected minimum
// ---------------------------------------------------------------------------

#[test]
fn test_sf_byte_count_verification() {
    // Encode 5 u32 values.  Each u32 encodes as at least 1 byte (varint).
    // Minimum layout: 1 data chunk header (13) + ≥5 payload bytes + 1 end header (13) = ≥31.
    let values: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut encoder = BufferStreamingEncoder::new();
    for v in &values {
        encoder.write_item(v).expect("write_item failed");
    }
    let encoded = encoder.finish();

    // 2 headers × 13 bytes = 26; 5 payload bytes minimum
    assert!(
        encoded.len() >= 31,
        "encoded length should be >= 31 bytes, got {}",
        encoded.len()
    );

    // Verify the end-of-stream end-header magic appears in the last 13 bytes
    let last_13 = &encoded[encoded.len() - 13..];
    assert_eq!(
        &last_13[..4],
        &CHUNK_MAGIC,
        "last chunk header must start with OXIS magic"
    );
    // End chunk type byte = 1
    assert_eq!(last_13[4], 1u8, "last chunk must be of type End (1)");
}

// ---------------------------------------------------------------------------
// Test 12: Encode with standard (default) StreamingConfig
// ---------------------------------------------------------------------------

#[test]
fn test_sf_encode_with_standard_config() {
    let config = StreamingConfig::default();
    let values: Vec<u32> = (0u32..20).collect();
    let mut encoder = BufferStreamingEncoder::with_config(config);
    for v in &values {
        encoder.write_item(v).expect("write_item failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");

    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 13: Decoder position tracking via progress().bytes_processed
// ---------------------------------------------------------------------------

#[test]
fn test_sf_decoder_position_tracking() {
    let values: Vec<u32> = vec![0xFFFF_FFFFu32; 10];
    let mut encoder = BufferStreamingEncoder::new();
    for v in &values {
        encoder.write_item(v).expect("write_item failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    // Read exactly 5 items
    for _ in 0..5 {
        decoder
            .read_item::<u32>()
            .expect("read_item failed")
            .expect("expected Some(u32)");
    }
    let bytes_after_5 = decoder.progress().bytes_processed;
    assert!(
        bytes_after_5 > 0,
        "bytes_processed must be > 0 after reading 5 items"
    );

    // Read the remaining 5
    for _ in 0..5 {
        decoder
            .read_item::<u32>()
            .expect("read_item failed")
            .expect("expected Some(u32)");
    }
    let bytes_after_10 = decoder.progress().bytes_processed;
    assert!(
        bytes_after_10 > bytes_after_5,
        "bytes_processed must grow after reading more items"
    );
    assert_eq!(decoder.progress().items_processed, 10);
}

// ---------------------------------------------------------------------------
// Test 14: Sequential encode then sequential decode — item-by-item
// ---------------------------------------------------------------------------

#[test]
fn test_sf_sequential_encode_then_sequential_decode() {
    let count: usize = 15;
    let mut encoder = BufferStreamingEncoder::new();
    for i in 0u32..count as u32 {
        encoder.write_item(&i).expect("write_item failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let mut collected: Vec<u32> = Vec::with_capacity(count);
    while let Some(item) = decoder.read_item::<u32>().expect("read_item failed") {
        collected.push(item);
    }

    assert_eq!(collected.len(), count);
    for (idx, &v) in collected.iter().enumerate() {
        assert_eq!(v, idx as u32, "value mismatch at index {}", idx);
    }
    assert!(decoder.is_finished());
}

// ---------------------------------------------------------------------------
// Test 15: Encode 1000 (u32, String) pairs as separate streams
// ---------------------------------------------------------------------------

#[test]
fn test_sf_1000_u32_string_pairs() {
    let pairs: Vec<(u32, String)> = (0u32..1000).map(|i| (i, format!("val-{}", i))).collect();

    // Stream the u32 values
    let mut u32_enc = BufferStreamingEncoder::new();
    for (n, _) in &pairs {
        u32_enc.write_item(n).expect("write u32");
    }
    let u32_buf = u32_enc.finish();

    // Stream the String values
    let mut str_enc = BufferStreamingEncoder::new();
    for (_, s) in &pairs {
        str_enc.write_item(s).expect("write String");
    }
    let str_buf = str_enc.finish();

    let u32_decoded: Vec<u32> = BufferStreamingDecoder::new(&u32_buf)
        .read_all()
        .expect("read_all u32 failed");
    let str_decoded: Vec<String> = BufferStreamingDecoder::new(&str_buf)
        .read_all()
        .expect("read_all String failed");

    assert_eq!(u32_decoded.len(), 1000);
    assert_eq!(str_decoded.len(), 1000);
    for (idx, ((n, s), (dn, ds))) in pairs
        .iter()
        .zip(u32_decoded.iter().zip(str_decoded.iter()))
        .enumerate()
    {
        assert_eq!(*n, *dn, "u32 mismatch at index {}", idx);
        assert_eq!(s, ds, "String mismatch at index {}", idx);
    }
}

// ---------------------------------------------------------------------------
// Test 16: Empty strings in stream
// ---------------------------------------------------------------------------

#[test]
fn test_sf_empty_strings_in_stream() {
    let strings: Vec<String> = vec![
        String::new(),
        "non-empty".to_string(),
        String::new(),
        String::new(),
        "also-non-empty".to_string(),
    ];

    let mut encoder = BufferStreamingEncoder::new();
    for s in &strings {
        encoder
            .write_item(s)
            .expect("write_item empty string failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<String> = decoder
        .read_all()
        .expect("read_all with empty strings failed");

    assert_eq!(decoded, strings);
}

// ---------------------------------------------------------------------------
// Test 17: Unicode strings in stream
// ---------------------------------------------------------------------------

#[test]
fn test_sf_unicode_strings_in_stream() {
    let strings: Vec<String> = vec![
        "日本語テスト".to_string(),
        "中文测试".to_string(),
        "한국어 테스트".to_string(),
        "Ünïcödé Ström".to_string(),
        "مرحبا بالعالم".to_string(),
        "🦀 Rust streaming 🚀".to_string(),
    ];

    let mut encoder = BufferStreamingEncoder::new();
    for s in &strings {
        encoder
            .write_item(s)
            .expect("write_item unicode string failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<String> = decoder.read_all().expect("read_all unicode strings failed");

    assert_eq!(decoded, strings);
}

// ---------------------------------------------------------------------------
// Test 18: Encode None/Some options in stream
// ---------------------------------------------------------------------------

#[test]
fn test_sf_option_none_some_in_stream() {
    let options: Vec<Option<u32>> = vec![Some(1), None, Some(42), None, None, Some(0xFFFF_FFFF)];

    let mut encoder = BufferStreamingEncoder::new();
    for opt in &options {
        encoder.write_item(opt).expect("write_item(Option) failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<Option<u32>> = decoder.read_all().expect("read_all(Option) failed");

    assert_eq!(decoded, options);
}

// ---------------------------------------------------------------------------
// Test 19: Nested struct stream — struct containing Option and Vec fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Record {
    id: u32,
    label: String,
    tags: Vec<String>,
    score: Option<f64>,
}

#[test]
fn test_sf_nested_struct_with_option_and_vec_fields() {
    let records: Vec<Record> = (0u32..6)
        .map(|i| Record {
            id: i,
            label: format!("record-{}", i),
            tags: (0..i).map(|t| format!("tag-{}", t)).collect(),
            score: if i % 2 == 0 {
                Some(i as f64 * 1.5)
            } else {
                None
            },
        })
        .collect();

    let mut encoder = BufferStreamingEncoder::new();
    for rec in &records {
        encoder.write_item(rec).expect("write_item(Record) failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<Record> = decoder.read_all().expect("read_all(Record) failed");

    assert_eq!(decoded, records);
}

// ---------------------------------------------------------------------------
// Test 20: Stream encode + decode roundtrip identity — bytes are deterministic
// ---------------------------------------------------------------------------

#[test]
fn test_sf_roundtrip_identity_deterministic_encoding() {
    let values: Vec<u32> = (1000u32..1050).collect();

    let encode_once = |vals: &[u32]| -> Vec<u8> {
        let mut enc = BufferStreamingEncoder::new();
        for v in vals {
            enc.write_item(v).expect("write_item failed");
        }
        enc.finish()
    };

    let buf1 = encode_once(&values);
    let buf2 = encode_once(&values);

    // Encoding must be deterministic
    assert_eq!(buf1, buf2, "streaming encoding must be deterministic");

    // Decode from first buffer, re-encode, compare bytes
    let mut decoder = BufferStreamingDecoder::new(&buf1);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");
    assert_eq!(decoded, values, "decoded values must match original");

    let buf3 = encode_once(&decoded);
    assert_eq!(
        buf1, buf3,
        "re-encoding decoded values must produce identical bytes"
    );
}
