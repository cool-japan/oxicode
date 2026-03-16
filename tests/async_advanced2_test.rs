//! Advanced async streaming tests (second set) for OxiCode.
//!
//! Covers unique scenarios not present in async_streaming_test.rs,
//! async_advanced_test.rs, async_comprehensive_test.rs, or async_extended_test.rs.

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
#[cfg(feature = "async-tokio")]
use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder, StreamingConfig};
#[cfg(feature = "async-tokio")]
use oxicode::{Decode, Encode};
#[cfg(feature = "async-tokio")]
use std::io::Cursor;

// ---------------------------------------------------------------------------
// Shared types (only compiled when async-tokio feature is active)
// ---------------------------------------------------------------------------

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Point2D {
    x: i32,
    y: i32,
}

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ---------------------------------------------------------------------------
// Test 1: Write then read single u32
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_single_u32_roundtrip() {
    let value: u32 = 0xDEAD_BEEF;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder.write_item(&value).await.expect("write_item failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<u32> = decoder.read_item().await.expect("read_item failed");
    assert_eq!(got, Some(value));
    let eof: Option<u32> = decoder.read_item().await.expect("read after eof failed");
    assert!(eof.is_none());
}

// ---------------------------------------------------------------------------
// Test 2: Write then read multiple u32 values, verify all values and order
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_multiple_u32_values() {
    let values: Vec<u32> = vec![1, 2, 3, 5, 8, 13, 21, 34];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &values {
            encoder.write_item(v).await.expect("write_item failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<u32>::new();
    while let Some(v) = decoder.read_item::<u32>().await.expect("read_item failed") {
        decoded.push(v);
    }
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 3: Write then read String
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_string_roundtrip() {
    let s = String::from("hello, async oxicode!");

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder.write_item(&s).await.expect("write_item failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<String> = decoder.read_item().await.expect("read_item failed");
    assert_eq!(got, Some(s));
}

// ---------------------------------------------------------------------------
// Test 4: Write then read Vec<u8>
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_vec_u8_roundtrip() {
    let data: Vec<u8> = (0u8..=255u8).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder.write_item(&data).await.expect("write_item failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<Vec<u8>> = decoder.read_item().await.expect("read_item failed");
    assert_eq!(got, Some(data));
}

// ---------------------------------------------------------------------------
// Test 5: Write then read Option<u64> — Some and None variants
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_option_u64_roundtrip() {
    let some_val: Option<u64> = Some(u64::MAX / 2);
    let none_val: Option<u64> = None;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&some_val)
            .await
            .expect("write some failed");
        encoder
            .write_item(&none_val)
            .await
            .expect("write none failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got_some: Option<Option<u64>> = decoder.read_item().await.expect("read some failed");
    let got_none: Option<Option<u64>> = decoder.read_item().await.expect("read none failed");
    assert_eq!(got_some, Some(some_val));
    assert_eq!(got_none, Some(none_val));
}

// ---------------------------------------------------------------------------
// Test 6: Write then read bool values (true, false, true)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_bool_values_roundtrip() {
    let bools = vec![true, false, true, false, false, true];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for b in &bools {
            encoder.write_item(b).await.expect("write_item failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<bool>::new();
    while let Some(v) = decoder.read_item::<bool>().await.expect("read_item failed") {
        decoded.push(v);
    }
    assert_eq!(decoded, bools);
}

// ---------------------------------------------------------------------------
// Test 7: Write then read i64 including negative values
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_i64_including_negative() {
    let values: Vec<i64> = vec![i64::MIN, -1, 0, 1, i64::MAX];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &values {
            encoder.write_item(v).await.expect("write_item failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<i64>::new();
    while let Some(v) = decoder.read_item::<i64>().await.expect("read_item failed") {
        decoded.push(v);
    }
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 8: Empty stream — no items written — first read returns None
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_empty_stream_returns_none() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let encoder = AsyncStreamingEncoder::<_>::new(cursor);
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let result: Option<u32> = decoder
        .read_item()
        .await
        .expect("read on empty stream failed");
    assert!(result.is_none());
    assert!(decoder.is_finished());
}

// ---------------------------------------------------------------------------
// Test 9: Write 100 u32 values, read back all 100 via while-let loop
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_100_u32_while_let_loop() {
    let count = 100u32;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for i in 0..count {
            encoder.write_item(&i).await.expect("write_item failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<u32>::new();
    while let Some(v) = decoder.read_item::<u32>().await.expect("read_item failed") {
        decoded.push(v);
    }

    assert_eq!(decoded.len(), count as usize);
    let expected: Vec<u32> = (0..count).collect();
    assert_eq!(decoded, expected);
}

// ---------------------------------------------------------------------------
// Test 10: Write mixed types sequentially in separate streams, read back
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_mixed_types_separate_streams() {
    // u32 stream
    let mut buf_u32 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_u32);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&42u32).await.expect("write u32 failed");
        enc.finish().await.expect("finish u32 failed");
    }
    // String stream
    let mut buf_str = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_str);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&String::from("mixed"))
            .await
            .expect("write str failed");
        enc.finish().await.expect("finish str failed");
    }
    // bool stream
    let mut buf_bool = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_bool);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&true).await.expect("write bool failed");
        enc.finish().await.expect("finish bool failed");
    }

    let mut dec_u32 = AsyncStreamingDecoder::new(Cursor::new(buf_u32));
    let mut dec_str = AsyncStreamingDecoder::new(Cursor::new(buf_str));
    let mut dec_bool = AsyncStreamingDecoder::new(Cursor::new(buf_bool));

    assert_eq!(
        dec_u32.read_item::<u32>().await.expect("read u32 failed"),
        Some(42u32)
    );
    assert_eq!(
        dec_str
            .read_item::<String>()
            .await
            .expect("read str failed"),
        Some(String::from("mixed"))
    );
    assert_eq!(
        dec_bool
            .read_item::<bool>()
            .await
            .expect("read bool failed"),
        Some(true)
    );
}

// ---------------------------------------------------------------------------
// Test 11: Write then read large String (1000 chars)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_large_string_1000_chars() {
    let large: String = "x".repeat(1000);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&large)
            .await
            .expect("write large string failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<String> = decoder.read_item().await.expect("read large string failed");
    assert_eq!(got, Some(large));
}

// ---------------------------------------------------------------------------
// Test 12: Write then read Vec<String>
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_vec_string_roundtrip() {
    let items: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
        String::from("delta"),
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&items)
            .await
            .expect("write Vec<String> failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<Vec<String>> = decoder.read_item().await.expect("read Vec<String> failed");
    assert_eq!(got, Some(items));
}

// ---------------------------------------------------------------------------
// Test 13: Async with in-memory Vec<u8> buffer (not file), verify no file I/O
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_in_memory_buffer_no_file() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder.write_item(&0xCAFEu32).await.expect("write failed");
        encoder.write_item(&0xBABEu32).await.expect("write failed");
        encoder.finish().await.expect("finish failed");
    }

    // The buffer is non-empty — data was written to memory only
    assert!(!buf.is_empty());

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let first: Option<u32> = decoder.read_item().await.expect("read first failed");
    let second: Option<u32> = decoder.read_item().await.expect("read second failed");
    assert_eq!(first, Some(0xCAFEu32));
    assert_eq!(second, Some(0xBABEu32));
}

// ---------------------------------------------------------------------------
// Test 14: Write struct, read struct back
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_struct_roundtrip() {
    let points = vec![
        Point2D { x: -10, y: 20 },
        Point2D { x: 0, y: 0 },
        Point2D { x: 100, y: -50 },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for p in &points {
            encoder.write_item(p).await.expect("write struct failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<Point2D>::new();
    while let Some(p) = decoder
        .read_item::<Point2D>()
        .await
        .expect("read struct failed")
    {
        decoded.push(p);
    }
    assert_eq!(decoded, points);
}

// ---------------------------------------------------------------------------
// Test 15: Write enum values, read back
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_enum_roundtrip() {
    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::North,
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for d in &directions {
            encoder.write_item(d).await.expect("write enum failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<Direction>::new();
    while let Some(d) = decoder
        .read_item::<Direction>()
        .await
        .expect("read enum failed")
    {
        decoded.push(d);
    }
    assert_eq!(decoded, directions);
}

// ---------------------------------------------------------------------------
// Test 16: Write 0 items, read 0 items (alias for empty stream, different assert focus)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_zero_items_written_zero_read() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let encoder = AsyncStreamingEncoder::<_>::new(cursor);
        encoder.finish().await.expect("finish empty encoder failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut count = 0usize;
    while decoder
        .read_item::<u64>()
        .await
        .expect("read on zero-item stream failed")
        .is_some()
    {
        count += 1;
    }
    assert_eq!(count, 0);
}

// ---------------------------------------------------------------------------
// Test 17: Write 1 item, read 1 item, then next read returns None
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_one_item_then_none() {
    let val: u32 = 99;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&val)
            .await
            .expect("write one item failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let first: Option<u32> = decoder.read_item().await.expect("read first item failed");
    let second: Option<u32> = decoder.read_item().await.expect("read after last failed");
    assert_eq!(first, Some(val));
    assert!(second.is_none());
    assert!(decoder.is_finished());
}

// ---------------------------------------------------------------------------
// Test 18: Write Vec<u8> of 10000 bytes, read back
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_vec_u8_10000_bytes() {
    let data: Vec<u8> = (0u32..10000).map(|i| (i % 256) as u8).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&data)
            .await
            .expect("write 10k bytes failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<Vec<u8>> = decoder.read_item().await.expect("read 10k bytes failed");
    assert_eq!(got, Some(data));
}

// ---------------------------------------------------------------------------
// Test 19: Multiple sequential writes then sequential reads (interleaved writes)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_sequential_writes_then_reads() {
    let a: u32 = 111;
    let b: u32 = 222;
    let c: u32 = 333;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder.write_item(&a).await.expect("write a failed");
        encoder.write_item(&b).await.expect("write b failed");
        encoder.write_item(&c).await.expect("write c failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let ra: Option<u32> = decoder.read_item().await.expect("read a failed");
    let rb: Option<u32> = decoder.read_item().await.expect("read b failed");
    let rc: Option<u32> = decoder.read_item().await.expect("read c failed");
    let rd: Option<u32> = decoder.read_item().await.expect("read eof failed");
    assert_eq!(ra, Some(a));
    assert_eq!(rb, Some(b));
    assert_eq!(rc, Some(c));
    assert!(rd.is_none());
}

// ---------------------------------------------------------------------------
// Test 20: Write then read u128 values (including MIN and MAX)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_u128_min_max_roundtrip() {
    let values: Vec<u128> = vec![u128::MIN, 1, u128::MAX / 3, u128::MAX];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &values {
            encoder.write_item(v).await.expect("write u128 failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<u128>::new();
    while let Some(v) = decoder.read_item::<u128>().await.expect("read u128 failed") {
        decoded.push(v);
    }
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 21: Write then read f64 values (use computed values not literals)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_f64_computed_values() {
    // Use computed values to avoid clippy::approx_constant warnings
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    let sqrt2 = std::f64::consts::SQRT_2;
    let values: Vec<f64> = vec![-pi, -e, -sqrt2, 0.0_f64, sqrt2, e, pi];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &values {
            encoder.write_item(v).await.expect("write f64 failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<f64>::new();
    while let Some(v) = decoder.read_item::<f64>().await.expect("read f64 failed") {
        decoded.push(v);
    }
    assert_eq!(decoded.len(), values.len());
    for (a, b) in values.iter().zip(decoded.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch: {a} vs {b}");
    }
}

// ---------------------------------------------------------------------------
// Test 22: Write then read with custom config (small chunk size + flush_per_item)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async2_custom_config_small_chunk_flush_per_item() {
    // Small chunk size forces multiple chunks; flush_per_item ensures each item
    // is flushed immediately — exercises the most aggressive flushing path.
    let config = StreamingConfig::new()
        .with_chunk_size(64)
        .with_flush_per_item(true);

    let items: Vec<u32> = (200u32..215u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
        for item in &items {
            encoder
                .write_item(item)
                .await
                .expect("write with config failed");
        }
        encoder.finish().await.expect("finish with config failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<u32>::new();
    while let Some(v) = decoder
        .read_item::<u32>()
        .await
        .expect("read with config failed")
    {
        decoded.push(v);
    }

    assert_eq!(decoded, items);
    // Each item was flushed individually, so we expect as many chunks as items
    assert_eq!(
        decoder.progress().chunks_processed,
        items.len() as u64,
        "expected one chunk per item with flush_per_item"
    );
}
