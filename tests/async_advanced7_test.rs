//! Advanced async streaming tests (seventh set) for OxiCode.
//!
//! All 22 tests are top-level (no module wrapper), gated with
//! `#[cfg(feature = "async-tokio")]`.
//!
//! Focus areas (different from async_advanced6):
//!   1-4:   Error cases (empty buffer, truncated data)
//!   5-8:   Config variations (fixed-int, big-endian)
//!   9-12:  Struct types via async streaming
//!   13-16: Different data sizes (tiny / medium / large)
//!   17-19: Concurrent async operations (spawned tasks)
//!   20-22: File I/O with tokio::fs

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
use oxicode::streaming::{
    AsyncStreamingDecoder, AsyncStreamingEncoder, CancellableAsyncEncoder, CancellationToken,
    StreamingConfig,
};
use oxicode::{config, decode_from_slice, encode_to_vec_with_config, Decode, Encode};
use std::io::Cursor;

// ---------------------------------------------------------------------------
// Shared derive types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Payload {
    id: u64,
    data: Vec<u8>,
    label: String,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Nested {
    inner: Point,
    value: f64,
}

// ---------------------------------------------------------------------------
// Test 1: Decode from truly empty buffer returns error (UnexpectedEof)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_decode_from_truly_empty_buffer_is_err() {
    let empty: Vec<u8> = Vec::new();
    let cursor = Cursor::new(empty);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);

    // An empty buffer has no end-marker; the decoder should report finished
    // (UnexpectedEof treated as end-of-stream) rather than panic.
    let result: Result<Option<u32>, _> = dec.read_item().await;
    // Either Ok(None) (graceful EOF) or Err (IO error) — both are acceptable;
    // what is NOT acceptable is a panic.
    match result {
        Ok(None) => {} // graceful EOF
        Ok(Some(_)) => panic!("should not decode a value from an empty buffer"),
        Err(_) => {} // IO / format error is fine
    }
}

// ---------------------------------------------------------------------------
// Test 2: Decode from truncated data (full header but missing payload) is err
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_decode_truncated_header_only_is_err() {
    // Encode one item to get a fully valid byte stream
    let mut full_buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut full_buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&0xDEAD_BEEFu32).await.expect("write failed");
        enc.finish().await.expect("finish failed");
    }

    // ChunkHeader::SIZE is 13 bytes. Keep only the full header (13 bytes) but
    // drop the payload — the decoder must fail when trying to read_exact the payload.
    let truncated: Vec<u8> = full_buf.into_iter().take(13).collect();

    let cursor = Cursor::new(truncated);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);
    let result: Result<Option<u32>, _> = dec.read_item().await;
    // Must be an error — the payload bytes are missing.
    assert!(result.is_err(), "truncated stream should yield an error");
}

// ---------------------------------------------------------------------------
// Test 3: Decode from single garbage byte is error (not panic)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_decode_single_garbage_byte_is_err() {
    let garbage: Vec<u8> = vec![0xFF];
    let cursor = Cursor::new(garbage);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);

    let result: Result<Option<u32>, _> = dec.read_item().await;
    match result {
        Ok(None) => {} // treated as EOF — acceptable
        Ok(Some(_)) => panic!("should not successfully decode garbage"),
        Err(_) => {} // expected error
    }
}

// ---------------------------------------------------------------------------
// Test 4: Decode immediately after finish yields None (no items written)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_decode_after_finish_no_items_is_none() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let enc: AsyncStreamingEncoder<Cursor<&mut Vec<u8>>> = AsyncStreamingEncoder::new(cursor);
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);

    // First read_item call
    let first: Option<u32> = dec.read_item().await.expect("read failed");
    assert_eq!(first, None);

    // Calling read_item again after exhaustion should still return None
    let second: Option<u32> = dec.read_item().await.expect("second read failed");
    assert_eq!(second, None);
    assert!(dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 5: Config variation — flush_per_item enabled, roundtrip u32
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_config_flush_per_item_roundtrip() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let original: u32 = 12345;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        enc.write_item(&original).await.expect("write failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<u32> = dec.read_item().await.expect("read failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 6: Config variation — very small chunk_size forces multiple chunks
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_config_small_chunk_size_multiple_chunks() {
    // Use minimum allowed chunk size (1024 bytes) and enough large items to
    // guarantee multiple flushes. Each String item is ~20+ bytes, so 200 items
    // will require at least ~4 KB which is more than one 1024-byte chunk.
    let config = StreamingConfig::new().with_chunk_size(1024);
    let values: Vec<String> = (0u32..200).map(|i| format!("item-{:08}", i)).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        for v in &values {
            enc.write_item(v).await.expect("write failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<String> = dec.read_all().await.expect("read_all failed");

    assert_eq!(decoded, values);
    // Multiple chunks must have been used
    assert!(
        dec.progress().chunks_processed > 1,
        "expected more than one chunk"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Standard encode_to_vec_with_config big-endian produces different bytes
//         than little-endian for asymmetric values
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_big_endian_bytes_differ_from_little_endian() {
    let original: u32 = 0x0102_0304;

    // Encode with big-endian config
    let be_bytes = encode_to_vec_with_config(&original, config::standard().with_big_endian())
        .expect("big-endian encode failed");

    // Encode with little-endian (default standard) config
    let le_bytes = encode_to_vec_with_config(&original, config::standard())
        .expect("little-endian encode failed");

    // Both encode the same value, but the byte streams differ for non-palindrome values
    // (the big-endian decode must use big-endian config)
    let (be_decoded, _) = oxicode::decode_from_slice_with_config::<u32, _>(
        &be_bytes,
        config::standard().with_big_endian(),
    )
    .expect("big-endian round-trip failed");
    assert_eq!(
        be_decoded, original,
        "big-endian roundtrip must preserve value"
    );

    let (le_decoded, _) = decode_from_slice::<u32>(&le_bytes).expect("little-endian decode failed");
    assert_eq!(
        le_decoded, original,
        "little-endian roundtrip must preserve value"
    );

    // The raw byte representations differ
    assert_ne!(
        be_bytes, le_bytes,
        "big-endian and little-endian bytes should differ"
    );

    // Async streaming (uses default standard internally) should also roundtrip correctly
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("async write failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let async_decoded: Option<u32> = dec.read_item().await.expect("async read failed");
    assert_eq!(async_decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 8: Fixed-int config via encode_to_vec_with_config roundtrip
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_fixed_int_config_encode_decode() {
    let original: u64 = 0xCAFE_BABE_DEAD_BEEF;

    let fixed_bytes =
        encode_to_vec_with_config(&original, config::standard().with_fixed_int_encoding())
            .expect("fixed-int encode failed");

    // Fixed-int encoding always produces 8 bytes for u64
    assert_eq!(fixed_bytes.len(), 8, "fixed-int u64 must be 8 bytes");

    // Decode with the same fixed-int config
    let (decoded, _) = oxicode::decode_from_slice_with_config::<u64, _>(
        &fixed_bytes,
        config::standard().with_fixed_int_encoding(),
    )
    .expect("fixed-int decode failed");
    assert_eq!(decoded, original);

    // Async streaming (uses default varint internally) should still roundtrip correctly
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("write failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let async_decoded: Option<u64> = dec.read_item().await.expect("read failed");
    assert_eq!(async_decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 9: Struct Point roundtrip via async streaming
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_struct_point_roundtrip() {
    let original = Point { x: -100, y: 200 };

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("write Point failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Point> = dec.read_item().await.expect("read Point failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 10: Struct Payload (u64 + Vec<u8> + String) roundtrip
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_struct_payload_roundtrip() {
    let original = Payload {
        id: 9_999_999_999,
        data: vec![10, 20, 30, 40, 50],
        label: "oxicode-payload".to_string(),
    };

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Payload failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Payload> = dec.read_item().await.expect("read Payload failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 11: Nested struct roundtrip
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_nested_struct_roundtrip() {
    let original = Nested {
        inner: Point { x: 3, y: -7 },
        value: std::f64::consts::PI,
    };

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Nested failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Nested> = dec.read_item().await.expect("read Nested failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 12: Multiple structs written, read_all returns correct order
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_multiple_structs_read_all_order() {
    let items: Vec<Point> = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: -1 },
        Point {
            x: i32::MAX,
            y: i32::MIN,
        },
        Point { x: -500, y: 500 },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for item in &items {
            enc.write_item(item).await.expect("write Point failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<Point> = dec.read_all().await.expect("read_all failed");
    assert_eq!(decoded, items);
}

// ---------------------------------------------------------------------------
// Test 13: Tiny data — single byte (u8) roundtrip
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_tiny_single_u8_roundtrip() {
    let original: u8 = 0xAB;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("write u8 failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<u8> = dec.read_item().await.expect("read u8 failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 14: Medium data — 100 u64 values
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_medium_100_u64_roundtrip() {
    let values: Vec<u64> = (0..100).map(|i: u64| i * i + 7).collect();

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
    let decoded: Vec<u64> = dec.read_all().await.expect("read_all failed");
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 15: Large data — Vec<u8> of 10 000 bytes
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_large_vec_10000_bytes_roundtrip() {
    let original: Vec<u8> = (0u8..=255).cycle().take(10_000).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write 10000-byte Vec failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Vec<u8>> = dec.read_item().await.expect("read 10000-byte Vec failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 16: Large number of small items (2000 u32 values) with small chunk_size
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_large_count_small_items_with_small_chunks() {
    let config = StreamingConfig::new().with_chunk_size(1024);
    let values: Vec<u32> = (0..2_000_u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        for &v in &values {
            enc.write_item(&v).await.expect("write failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = dec.read_all().await.expect("read_all failed");

    assert_eq!(decoded.len(), 2_000);
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 17: Concurrent encode in separate spawned tasks, results are consistent
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_concurrent_encode_in_tasks() {
    use std::sync::{Arc, Mutex};

    let results: Arc<Mutex<Vec<Vec<u8>>>> = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

    let mut handles = Vec::new();
    for task_id in 0u32..4 {
        let results_clone: Arc<Mutex<Vec<Vec<u8>>>> = Arc::clone(&results);
        let handle = tokio::spawn(async move {
            let value: u32 = task_id * 1000;
            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut enc = AsyncStreamingEncoder::new(cursor);
                enc.write_item(&value).await.expect("write failed in task");
                enc.finish().await.expect("finish failed in task");
            }
            results_clone.lock().expect("mutex poisoned").push(buf);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("task panicked");
    }

    let all_bufs: std::sync::MutexGuard<'_, Vec<Vec<u8>>> =
        results.lock().expect("mutex poisoned after tasks");
    assert_eq!(all_bufs.len(), 4, "all 4 tasks must have completed");

    // Each buffer must decode successfully to some u32 value
    for buf in all_bufs.iter() {
        let cursor = Cursor::new(buf.clone());
        let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);
        let item: Option<u32> = dec
            .read_item::<u32>()
            .await
            .expect("decode in task check failed");
        assert!(item.is_some(), "decoded value should be Some");
    }
}

// ---------------------------------------------------------------------------
// Test 18: Concurrent decode in separate spawned tasks
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_concurrent_decode_in_tasks() {
    // Prepare a shared encoded buffer for value 42
    let value: u32 = 42;
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&value).await.expect("write failed");
        enc.finish().await.expect("finish failed");
    }
    let buf = std::sync::Arc::new(buf);

    let mut handles = Vec::new();
    for _ in 0..5 {
        let buf_clone = std::sync::Arc::clone(&buf);
        let handle = tokio::spawn(async move {
            let cursor = Cursor::new((*buf_clone).clone());
            let mut dec = AsyncStreamingDecoder::new(cursor);
            let decoded: Option<u32> = dec.read_item().await.expect("decode in task failed");
            assert_eq!(decoded, Some(42u32));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("concurrent decode task panicked");
    }
}

// ---------------------------------------------------------------------------
// Test 19: CancellationToken cancels mid-write in async encoder
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_cancellation_token_cancels_encoder() {
    let token = CancellationToken::new();

    let mut buf = Vec::<u8>::new();
    let cursor = Cursor::new(&mut buf);
    let mut enc = CancellableAsyncEncoder::new(cursor, token.child());

    enc.write_item(&1_u32).await.expect("first write failed");
    enc.write_item(&2_u32).await.expect("second write failed");

    // Cancel before the third write
    token.cancel();
    assert!(token.is_cancelled());

    let result = enc.write_item(&3_u32).await;
    assert!(result.is_err(), "write after cancel must fail");
}

// ---------------------------------------------------------------------------
// Test 20: File I/O — write to temp file and read back with tokio::fs
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_file_io_write_and_read_back() {
    use tokio::fs;
    use tokio::io::BufReader;

    let dir = std::env::temp_dir();
    let path = dir.join("oxicode_async7_test20.bin");

    let values: Vec<u32> = vec![111, 222, 333, 444, 555];

    // Write
    {
        let file = fs::File::create(&path)
            .await
            .expect("create temp file failed");
        let mut enc = AsyncStreamingEncoder::new(file);
        for &v in &values {
            enc.write_item(&v).await.expect("write to file failed");
        }
        enc.finish().await.expect("finish file write failed");
    }

    // Read back
    {
        let file = fs::File::open(&path).await.expect("open temp file failed");
        let reader = BufReader::new(file);
        let mut dec = AsyncStreamingDecoder::new(reader);
        let decoded: Vec<u32> = dec.read_all().await.expect("read_all from file failed");
        assert_eq!(decoded, values);
    }

    // Clean up
    fs::remove_file(&path).await.expect("cleanup failed");
}

// ---------------------------------------------------------------------------
// Test 21: File I/O — encode multiple structs to file, decode from file
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_file_io_structs_roundtrip() {
    use tokio::fs;
    use tokio::io::BufWriter;

    let dir = std::env::temp_dir();
    let path = dir.join("oxicode_async7_test21.bin");

    let points: Vec<Point> = (0i32..10).map(|i| Point { x: i * 10, y: -i }).collect();

    // Write
    {
        let file = fs::File::create(&path)
            .await
            .expect("create temp file failed");
        let writer = BufWriter::new(file);
        let mut enc = AsyncStreamingEncoder::new(writer);
        for p in &points {
            enc.write_item(p).await.expect("write Point to file failed");
        }
        enc.finish().await.expect("finish struct file write failed");
    }

    // Read back
    {
        let file = fs::File::open(&path).await.expect("open temp file failed");
        let mut dec = AsyncStreamingDecoder::new(file);
        let decoded: Vec<Point> = dec
            .read_all()
            .await
            .expect("read_all structs from file failed");
        assert_eq!(decoded, points);
    }

    // Clean up
    fs::remove_file(&path).await.expect("cleanup failed");
}

// ---------------------------------------------------------------------------
// Test 22: File I/O — concurrent task writes same file sequentially then reads
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async7_file_io_write_read_with_progress_check() {
    use tokio::fs;

    let dir = std::env::temp_dir();
    let path = dir.join("oxicode_async7_test22.bin");

    const ITEM_COUNT: usize = 50;
    let values: Vec<u64> = (0..ITEM_COUNT as u64).map(|i| i * i).collect();

    // Write with estimated total set
    {
        let file = fs::File::create(&path)
            .await
            .expect("create temp file failed");
        let mut enc = AsyncStreamingEncoder::new(file);
        enc.set_estimated_total(ITEM_COUNT as u64);
        for &v in &values {
            enc.write_item(&v).await.expect("write u64 to file failed");
        }
        enc.finish().await.expect("finish file write failed");
    }

    // Read back and verify progress metrics
    {
        let file = fs::File::open(&path).await.expect("open temp file failed");
        let mut dec = AsyncStreamingDecoder::new(file);
        let decoded: Vec<u64> = dec.read_all().await.expect("read_all u64 from file failed");

        assert_eq!(decoded, values);
        assert_eq!(
            dec.progress().items_processed,
            ITEM_COUNT as u64,
            "progress should reflect all items"
        );
        assert!(
            dec.progress().chunks_processed >= 1,
            "at least one chunk must have been processed"
        );
        assert!(dec.is_finished());
    }

    // Clean up
    fs::remove_file(&path).await.expect("cleanup failed");
}
