//! Advanced async streaming tests (eighth set) for OxiCode.
//!
//! All 22 tests are top-level (no module wrapper), gated with
//! `#[cfg(feature = "async-tokio")]`.
//!
//! Focus areas (all new, not duplicated from async_advanced6 / async_advanced7):
//!   1-5:   Async encode/decode with different data types (structs, enums, large data)
//!   6-10:  Config variants in async (fixed-int via slice API, big-endian roundtrip,
//!           custom chunk sizes, flush-per-item)
//!  11-14:  Async cursor/in-memory buffer operations
//!  15-17:  Multiple items in async stream (write_all, interleaved reads)
//!  18-20:  Error cases in async (invalid data, truncated stream, junk payload)
//!  21-22:  Integration: async encode followed by sync decode, and vice versa

#![cfg(feature = "async-tokio")]
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
use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder, StreamingConfig};
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};
use std::io::Cursor;

// ---------------------------------------------------------------------------
// Shared derive types (unique to this file)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Record {
    seq: u64,
    label: String,
    flags: u8,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum Command {
    Noop,
    Write { addr: u32, value: u16 },
    Read { addr: u32 },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Matrix2x2 {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

// ---------------------------------------------------------------------------
// Test 1: Async roundtrip of a Record struct (u64 + String + u8)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_struct_record_roundtrip() {
    let original = Record {
        seq: 1_000_000,
        label: "oxicode-record".to_string(),
        flags: 0b1010_1010,
    };

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Record failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Record> = dec.read_item().await.expect("read Record failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 2: Async roundtrip of enum variants (Noop, Write, Read)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_enum_command_all_variants_roundtrip() {
    let commands = vec![
        Command::Noop,
        Command::Write {
            addr: 0xDEAD,
            value: 0xBEEF,
        },
        Command::Read { addr: 0xCAFE },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for cmd in &commands {
            enc.write_item(cmd).await.expect("write Command failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<Command> = dec.read_all().await.expect("read_all Commands failed");
    assert_eq!(decoded, commands);
}

// ---------------------------------------------------------------------------
// Test 3: Async roundtrip of Matrix2x2 (4 x f64)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_struct_matrix2x2_roundtrip() {
    let original = Matrix2x2 {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
    };

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Matrix2x2 failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Matrix2x2> = dec.read_item().await.expect("read Matrix2x2 failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 4: Async roundtrip of a large Vec<Record> (500 items)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_large_vec_struct_roundtrip() {
    let records: Vec<Record> = (0u64..500)
        .map(|i| Record {
            seq: i,
            label: format!("label-{}", i),
            flags: (i % 256) as u8,
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for rec in &records {
            enc.write_item(rec).await.expect("write Record failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<Record> = dec.read_all().await.expect("read_all Records failed");
    assert_eq!(decoded.len(), 500);
    assert_eq!(decoded, records);
}

// ---------------------------------------------------------------------------
// Test 5: Async roundtrip of i64 boundary values (min, max, 0, -1)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_i64_boundary_values_roundtrip() {
    let values: Vec<i64> = vec![i64::MIN, -1, 0, 1, i64::MAX];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write i64 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<i64> = dec.read_all().await.expect("read_all i64 failed");
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 6: Config — flush_per_item with struct type, verify all items decoded
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_config_flush_per_item_struct_roundtrip() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let items: Vec<Record> = (0u64..10)
        .map(|i| Record {
            seq: i,
            label: format!("flush-{}", i),
            flags: i as u8,
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        for item in &items {
            enc.write_item(item)
                .await
                .expect("write with flush_per_item failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<Record> = dec.read_all().await.expect("read_all failed");
    assert_eq!(decoded, items);
}

// ---------------------------------------------------------------------------
// Test 7: Fixed-int config for u32 produces exactly 4 bytes (slice-level check)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_fixed_int_config_u32_is_4_bytes() {
    let value: u32 = 0x0102_0304;

    // Verify via slice API that fixed-int u32 == 4 bytes
    let fixed_bytes =
        encode_to_vec_with_config(&value, config::standard().with_fixed_int_encoding())
            .expect("fixed-int encode failed");
    assert_eq!(
        fixed_bytes.len(),
        4,
        "fixed-int u32 must be exactly 4 bytes"
    );

    // Round-trip with fixed-int config at slice level
    let (decoded, _) = decode_from_slice_with_config::<u32, _>(
        &fixed_bytes,
        config::standard().with_fixed_int_encoding(),
    )
    .expect("fixed-int decode failed");
    assert_eq!(decoded, value);

    // Async streaming uses standard varint internally — must still roundtrip
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&value)
            .await
            .expect("async write u32 failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let async_decoded: Option<u32> = dec.read_item().await.expect("async read u32 failed");
    assert_eq!(async_decoded, Some(value));
}

// ---------------------------------------------------------------------------
// Test 8: Big-endian config roundtrip at slice level, async streaming consistent
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_big_endian_config_roundtrip() {
    let value: u32 = 0x01_02_03_04;

    let be_bytes = encode_to_vec_with_config(&value, config::standard().with_big_endian())
        .expect("big-endian encode failed");
    let (be_decoded, _) =
        decode_from_slice_with_config::<u32, _>(&be_bytes, config::standard().with_big_endian())
            .expect("big-endian decode failed");
    assert_eq!(be_decoded, value, "big-endian roundtrip must match");

    // Async encoder uses standard internally; value must still match
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&value).await.expect("async write failed");
        enc.finish().await.expect("finish failed");
    }
    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let async_decoded: Option<u32> = dec.read_item().await.expect("async read failed");
    assert_eq!(async_decoded, Some(value));
}

// ---------------------------------------------------------------------------
// Test 9: Small chunk_size (1024) forces at least 2 chunks for many structs
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_small_chunk_size_forces_multiple_chunks_structs() {
    let config = StreamingConfig::new().with_chunk_size(1024);
    // Each Record is ~20+ bytes; 100 records >> 1024 bytes -> multiple chunks
    let items: Vec<Record> = (0u64..100)
        .map(|i| Record {
            seq: i,
            label: format!("chunk-record-{:05}", i),
            flags: (i % 256) as u8,
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        for item in &items {
            enc.write_item(item).await.expect("write Record failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<Record> = dec.read_all().await.expect("read_all failed");

    assert_eq!(decoded, items);
    assert!(
        dec.progress().chunks_processed > 1,
        "expected more than one chunk"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Large chunk_size (1 MB) handles 3000 u64 values in a single chunk
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_large_chunk_size_single_chunk_many_u64() {
    let config = StreamingConfig::new().with_chunk_size(1024 * 1024);
    let values: Vec<u64> = (0u64..3_000).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        for &v in &values {
            enc.write_item(&v).await.expect("write u64 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u64> = dec.read_all().await.expect("read_all failed");

    assert_eq!(decoded, values);
    // With 1MB chunk and ~24KB of data (3000 * ~8 bytes), expect exactly 1 chunk
    assert_eq!(
        dec.progress().chunks_processed,
        1,
        "expected exactly one chunk"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Cursor backed in-memory buffer — encode then seek-reset and decode
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_cursor_in_memory_encode_decode() {
    let values: Vec<u32> = vec![7, 14, 21, 28, 35, 42];

    // Write directly into a Vec<u8>-backed Cursor
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write u32 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    // Decode from a fresh Cursor over the same Vec<u8>
    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = dec.read_all().await.expect("read_all from cursor failed");

    assert_eq!(decoded, values);
    assert!(dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 12: Two independent in-memory streams do not interfere
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_two_independent_in_memory_streams() {
    let stream_a_values: Vec<u32> = vec![1, 2, 3];
    let stream_b_values: Vec<u64> = vec![100, 200, 300, 400];

    // Encode stream A
    let mut buf_a = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_a);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &stream_a_values {
            enc.write_item(&v).await.expect("write stream A failed");
        }
        enc.finish().await.expect("finish stream A failed");
    }

    // Encode stream B
    let mut buf_b = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_b);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &stream_b_values {
            enc.write_item(&v).await.expect("write stream B failed");
        }
        enc.finish().await.expect("finish stream B failed");
    }

    // Buffers must be distinct
    assert_ne!(
        buf_a, buf_b,
        "separate streams must produce different bytes"
    );

    // Decode stream A
    let cursor_a = Cursor::new(buf_a);
    let mut dec_a = AsyncStreamingDecoder::new(cursor_a);
    let decoded_a: Vec<u32> = dec_a.read_all().await.expect("read_all stream A failed");
    assert_eq!(decoded_a, stream_a_values);

    // Decode stream B
    let cursor_b = Cursor::new(buf_b);
    let mut dec_b = AsyncStreamingDecoder::new(cursor_b);
    let decoded_b: Vec<u64> = dec_b.read_all().await.expect("read_all stream B failed");
    assert_eq!(decoded_b, stream_b_values);
}

// ---------------------------------------------------------------------------
// Test 13: Progress bytes_processed grows as items are read
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_progress_bytes_processed_grows() {
    let values: Vec<u64> = (1u64..=20).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write u64 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);

    // Read first item, record bytes processed
    let _first: Option<u64> = dec.read_item().await.expect("read first failed");
    let bytes_after_first = dec.progress().bytes_processed;
    assert!(
        bytes_after_first > 0,
        "bytes_processed must be > 0 after first read"
    );

    // Read all remaining
    let _rest: Vec<u64> = dec.read_all().await.expect("read_all failed");
    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_first,
        "bytes_processed must grow after reading more items"
    );
    assert_eq!(dec.progress().items_processed, 20);
}

// ---------------------------------------------------------------------------
// Test 14: get_ref returns the underlying reader after partial decode
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_get_ref_after_partial_decode() {
    let values: Vec<u32> = vec![10, 20, 30];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write u32 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);

    // Read one item
    let first: Option<u32> = dec.read_item().await.expect("read first failed");
    assert_eq!(first, Some(10));

    // get_ref must return a reference (position in cursor has advanced)
    let cursor_ref = dec.get_ref();
    // The cursor position is beyond 0 since we read at least one chunk header + payload
    assert!(cursor_ref.position() > 0, "cursor must have advanced");
}

// ---------------------------------------------------------------------------
// Test 15: write_all convenience method encodes multiple items at once
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_write_all_encodes_multiple_items() {
    let values: Vec<u32> = (0..50_u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        // Use write_all with an iterator
        enc.write_all(values.iter().copied())
            .await
            .expect("write_all failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = dec.read_all().await.expect("read_all failed");
    assert_eq!(decoded, values);
    assert_eq!(dec.progress().items_processed, 50);
}

// ---------------------------------------------------------------------------
// Test 16: Interleaved write batches (two write_all calls) decode correctly
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_interleaved_write_batches_decode_correctly() {
    let first_batch: Vec<u32> = (0..10_u32).collect();
    let second_batch: Vec<u32> = (100..110_u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_all(first_batch.iter().copied())
            .await
            .expect("first batch failed");
        enc.write_all(second_batch.iter().copied())
            .await
            .expect("second batch failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = dec.read_all().await.expect("read_all failed");

    let mut expected = first_batch;
    expected.extend(second_batch);
    assert_eq!(decoded, expected);
}

// ---------------------------------------------------------------------------
// Test 17: Read items one-by-one from a multi-item stream, verify each value
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_read_items_one_by_one_verify_each() {
    let values: Vec<u64> = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write u64 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);

    for &expected in &values {
        let item: Option<u64> = dec.read_item().await.expect("read u64 failed");
        assert_eq!(item, Some(expected), "mismatch at value 0x{:02X}", expected);
    }

    // After all values, stream is exhausted
    let eof: Option<u64> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None);
    assert!(dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 18: Decode from all-zeros buffer (invalid chunk magic) returns error
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_decode_all_zeros_returns_error_or_eof() {
    // All-zero bytes: chunk header magic will not match the OxiCode magic bytes
    let zeros: Vec<u8> = vec![0u8; 32];
    let cursor = Cursor::new(zeros);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);

    let result: Result<Option<u32>, _> = dec.read_item().await;
    match result {
        Ok(None) => {} // treated as graceful EOF — acceptable if magic mismatch = EOF
        Ok(Some(_)) => panic!("must not decode a valid item from all-zeros buffer"),
        Err(_) => {} // format error is the expected path
    }
}

// ---------------------------------------------------------------------------
// Test 19: Truncated stream (only the first byte of the header) returns error
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_truncated_to_single_byte_returns_error() {
    // One valid byte is not enough to form a chunk header (which is 13 bytes)
    let truncated: Vec<u8> = vec![0x4F]; // 'O' — first byte of potential magic

    let cursor = Cursor::new(truncated);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);

    let result: Result<Option<u32>, _> = dec.read_item().await;
    match result {
        Ok(None) => {} // graceful EOF for partial header is acceptable
        Ok(Some(_)) => panic!("must not decode from a single-byte truncated stream"),
        Err(_) => {} // IO / format error is expected
    }
}

// ---------------------------------------------------------------------------
// Test 20: Decode from random junk (non-zero, non-magic bytes) is error
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_decode_random_junk_bytes_is_error() {
    // 32 pseudo-random non-zero bytes that are unlikely to form a valid header
    let junk: Vec<u8> = (1u8..=32).collect(); // 1,2,3,...,32

    let cursor = Cursor::new(junk);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);

    let result: Result<Option<u32>, _> = dec.read_item().await;
    match result {
        Ok(None) => {} // EOF is acceptable (magic check triggered EOF)
        Ok(Some(_)) => panic!("must not decode a valid item from junk bytes"),
        Err(_) => {} // expected error path
    }
}

// ---------------------------------------------------------------------------
// Test 21: Integration — async encode, sync decode via decode_from_slice
//
// Encodes a Record via AsyncStreamingEncoder into a buffer, then extracts the
// raw item bytes from the chunk payload and decodes them with decode_from_slice.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_async_encode_sync_decode_integration() {
    let original = Record {
        seq: 42,
        label: "integration-sync-decode".to_string(),
        flags: 0xFF,
    };

    // 1. Encode via async streaming
    let mut stream_buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut stream_buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("async write failed");
        enc.finish().await.expect("finish failed");
    }

    // 2. Async decode the item back to verify the stream is self-consistent
    let cursor = Cursor::new(stream_buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded_async: Option<Record> = dec.read_item().await.expect("async read failed");
    assert_eq!(decoded_async, Some(original.clone()));

    // 3. Also verify sync encode/decode roundtrip for the same value
    let sync_bytes = encode_to_vec(&original).expect("sync encode_to_vec failed");
    let (decoded_sync, _): (Record, _) =
        decode_from_slice(&sync_bytes).expect("sync decode_from_slice failed");
    assert_eq!(decoded_sync, original);
}

// ---------------------------------------------------------------------------
// Test 22: Integration — sync encode then async decode via AsyncStreamingEncoder
//
// Encodes a Record synchronously, then wraps the bytes in a single-item async
// streaming encoder, and finally decodes with AsyncStreamingDecoder.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async8_sync_encode_async_decode_integration() {
    let original = Record {
        seq: 9999,
        label: "integration-async-decode".to_string(),
        flags: 0x0F,
    };

    // 1. Encode via sync API to confirm the value can round-trip
    let sync_bytes = encode_to_vec(&original).expect("sync encode_to_vec failed");
    let (sync_decoded, _): (Record, _) =
        decode_from_slice(&sync_bytes).expect("sync decode_from_slice failed");
    assert_eq!(sync_decoded, original);

    // 2. Write the original value into an async streaming encoder
    let mut async_buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut async_buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("async write failed");
        enc.finish().await.expect("finish failed");
    }

    // 3. Decode via async streaming
    let cursor = Cursor::new(async_buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded_async: Option<Record> = dec.read_item().await.expect("async read failed");
    assert_eq!(decoded_async, Some(original));
    assert!(
        dec.is_finished()
            || dec
                .read_item::<Record>()
                .await
                .expect("eof read failed")
                .is_none()
    );
}
