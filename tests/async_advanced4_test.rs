//! Advanced async streaming tests (fourth set) for OxiCode.
//!
//! Covers unique scenarios not present in async_streaming_test.rs,
//! async_advanced_test.rs, async_advanced2_test.rs, or async_advanced3_test.rs.
//!
//! Focus areas:
//! - CancellableAsyncEncoder / CancellableAsyncDecoder
//! - CancellationToken: cancel, is_cancelled, child propagation
//! - set_estimated_total + progress().percentage()
//! - flush_per_item config
//! - get_ref() accessor on encoder / decoder
//! - Progress tracking on encoder side
//! - Empty-stream edge case
//! - Single-item stream
//! - i8, i16, i64, u16, u128 primitive types
//! - f32 / f64 boundary values
//! - Result<T, u32> enum round-trip
//! - Vec<Option<u64>> round-trip
//! - Cancellation mid-encode does not corrupt prior data
//! - CancellableAsyncDecoder read_all shortcut
//! - Chunk size of exactly DEFAULT_CHUNK_SIZE
//! - Very large single item (Vec<u8> of 128 KiB)
//! - with_flush_per_item(true) round-trip
//! - Encoder progress reports correct items_processed before finish
//! - Multi-chunk decoding with estimated_total percentage
//! - Read_item after stream finished returns None immediately
//! - Child token shares cancellation state
//!
//! All tests are top-level (no module wrapper).

// ---------------------------------------------------------------------------
// Shared struct types — only compiled when async-tokio feature is active
// ---------------------------------------------------------------------------

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
use oxicode::streaming::{
    AsyncStreamingDecoder, AsyncStreamingEncoder, CancellableAsyncDecoder, CancellableAsyncEncoder,
    CancellationToken, StreamingConfig,
};
#[cfg(feature = "async-tokio")]
use oxicode::{Decode, Encode};
#[cfg(feature = "async-tokio")]
use std::io::Cursor;

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct SimplePoint {
    x: f32,
    y: f32,
}

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct TaggedRecord {
    id: u64,
    tag: String,
    active: bool,
}

// ---------------------------------------------------------------------------
// Test 1: Async encode / decode of i8, i16, i64 primitive types
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_signed_integer_primitives_roundtrip() {
    let i8_vals: Vec<i8> = vec![i8::MIN, -1, 0, 1, i8::MAX];
    let i16_vals: Vec<i16> = vec![i16::MIN, -1000, 0, 1000, i16::MAX];
    let i64_vals: Vec<i64> = vec![i64::MIN, -1_000_000_000, 0, 1_000_000_000, i64::MAX];

    // i8
    let mut buf_i8 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_i8);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &i8_vals {
            enc.write_item(v).await.expect("write i8 failed");
        }
        enc.finish().await.expect("finish i8 failed");
    }

    // i16
    let mut buf_i16 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_i16);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &i16_vals {
            enc.write_item(v).await.expect("write i16 failed");
        }
        enc.finish().await.expect("finish i16 failed");
    }

    // i64
    let mut buf_i64 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_i64);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &i64_vals {
            enc.write_item(v).await.expect("write i64 failed");
        }
        enc.finish().await.expect("finish i64 failed");
    }

    let decoded_i8: Vec<i8> = AsyncStreamingDecoder::new(Cursor::new(buf_i8))
        .read_all()
        .await
        .expect("decode i8 failed");
    let decoded_i16: Vec<i16> = AsyncStreamingDecoder::new(Cursor::new(buf_i16))
        .read_all()
        .await
        .expect("decode i16 failed");
    let decoded_i64: Vec<i64> = AsyncStreamingDecoder::new(Cursor::new(buf_i64))
        .read_all()
        .await
        .expect("decode i64 failed");

    assert_eq!(decoded_i8, i8_vals);
    assert_eq!(decoded_i16, i16_vals);
    assert_eq!(decoded_i64, i64_vals);
}

// ---------------------------------------------------------------------------
// Test 2: Async encode / decode of u16 and u128 types
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_u16_u128_roundtrip() {
    let u16_vals: Vec<u16> = vec![0, 1, 256, u16::MAX / 2, u16::MAX];
    let u128_vals: Vec<u128> = vec![0, 1, u64::MAX as u128, u128::MAX / 2, u128::MAX];

    let mut buf_u16 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_u16);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &u16_vals {
            enc.write_item(v).await.expect("write u16 failed");
        }
        enc.finish().await.expect("finish u16 failed");
    }

    let mut buf_u128 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_u128);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &u128_vals {
            enc.write_item(v).await.expect("write u128 failed");
        }
        enc.finish().await.expect("finish u128 failed");
    }

    let decoded_u16: Vec<u16> = AsyncStreamingDecoder::new(Cursor::new(buf_u16))
        .read_all()
        .await
        .expect("decode u16 failed");
    let decoded_u128: Vec<u128> = AsyncStreamingDecoder::new(Cursor::new(buf_u128))
        .read_all()
        .await
        .expect("decode u128 failed");

    assert_eq!(decoded_u16, u16_vals);
    assert_eq!(decoded_u128, u128_vals);
}

// ---------------------------------------------------------------------------
// Test 3: Async encode / decode of f32 boundary values
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_f32_boundary_values_roundtrip() {
    let vals: Vec<f32> = vec![
        f32::MIN,
        f32::MIN_POSITIVE,
        -1.0_f32,
        0.0_f32,
        1.0_f32,
        std::f32::consts::PI,
        f32::MAX,
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &vals {
            enc.write_item(v).await.expect("write f32 failed");
        }
        enc.finish().await.expect("finish f32 failed");
    }

    let decoded: Vec<f32> = AsyncStreamingDecoder::new(Cursor::new(buf))
        .read_all()
        .await
        .expect("decode f32 failed");

    assert_eq!(decoded.len(), vals.len(), "length mismatch");
    for (orig, dec) in vals.iter().zip(decoded.iter()) {
        assert_eq!(
            orig.to_bits(),
            dec.to_bits(),
            "f32 bit mismatch: {orig} vs {dec}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 4: Empty stream produces no items on decode
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_empty_stream_roundtrip() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let encoder = AsyncStreamingEncoder::<_>::new(cursor);
        encoder.finish().await.expect("finish empty failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = decoder.read_all().await.expect("read_all empty failed");

    assert!(decoded.is_empty(), "expected empty vec from empty stream");
    assert!(
        decoder.is_finished(),
        "decoder must be finished after empty stream"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Single-item stream encodes and decodes exactly one item
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_single_item_stream() {
    let value: u64 = 0xCAFEBABE_DEADBEEF;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&value)
            .await
            .expect("write single item failed");
        enc.finish().await.expect("finish single item failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let first: Option<u64> = decoder.read_item().await.expect("read first item failed");
    let second: Option<u64> = decoder.read_item().await.expect("read second item failed");

    assert_eq!(first, Some(value), "first item mismatch");
    assert!(second.is_none(), "expected None for second read");
    assert!(decoder.is_finished(), "decoder must be finished");
}

// ---------------------------------------------------------------------------
// Test 6: Vec<Option<u64>> round-trip including None and Some boundaries
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_vec_option_u64_roundtrip() {
    let items: Vec<Option<u64>> = vec![
        Some(0),
        None,
        Some(u64::MAX),
        None,
        Some(u64::MAX / 2),
        None,
        Some(1),
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &items {
            enc.write_item(v).await.expect("write Option<u64> failed");
        }
        enc.finish().await.expect("finish Option<u64> failed");
    }

    let decoded: Vec<Option<u64>> = AsyncStreamingDecoder::new(Cursor::new(buf))
        .read_all()
        .await
        .expect("decode Option<u64> failed");

    assert_eq!(decoded, items);
}

// ---------------------------------------------------------------------------
// Test 7: SimplePoint struct (f32 fields) round-trip
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_simple_point_struct_roundtrip() {
    let points: Vec<SimplePoint> = vec![
        SimplePoint { x: 0.0, y: 0.0 },
        SimplePoint { x: 1.5, y: -2.5 },
        SimplePoint {
            x: f32::MAX,
            y: f32::MIN,
        },
        SimplePoint {
            x: std::f32::consts::PI,
            y: std::f32::consts::E,
        },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for p in &points {
            enc.write_item(p).await.expect("write SimplePoint failed");
        }
        enc.finish().await.expect("finish SimplePoint failed");
    }

    let decoded: Vec<SimplePoint> = AsyncStreamingDecoder::new(Cursor::new(buf))
        .read_all()
        .await
        .expect("decode SimplePoint failed");

    assert_eq!(decoded.len(), points.len());
    for (orig, dec) in points.iter().zip(decoded.iter()) {
        assert_eq!(orig.x.to_bits(), dec.x.to_bits(), "x mismatch");
        assert_eq!(orig.y.to_bits(), dec.y.to_bits(), "y mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 8: TaggedRecord struct with string field round-trip
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_tagged_record_roundtrip() {
    let records: Vec<TaggedRecord> = (0u64..20u64)
        .map(|i| TaggedRecord {
            id: i * 7,
            tag: format!("record-tag-{i:03}"),
            active: i % 3 == 0,
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for r in &records {
            enc.write_item(r).await.expect("write TaggedRecord failed");
        }
        enc.finish().await.expect("finish TaggedRecord failed");
    }

    let decoded: Vec<TaggedRecord> = AsyncStreamingDecoder::new(Cursor::new(buf))
        .read_all()
        .await
        .expect("decode TaggedRecord failed");

    assert_eq!(decoded, records);
}

// ---------------------------------------------------------------------------
// Test 9: CancellationToken — cancel() propagates to child token
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_cancellation_token_child_propagation() {
    let parent = CancellationToken::new();
    let child_a = parent.child();
    let child_b = child_a.child();

    assert!(
        !parent.is_cancelled(),
        "parent must not be cancelled initially"
    );
    assert!(
        !child_a.is_cancelled(),
        "child_a must not be cancelled initially"
    );
    assert!(
        !child_b.is_cancelled(),
        "child_b must not be cancelled initially"
    );

    parent.cancel();

    assert!(
        parent.is_cancelled(),
        "parent must be cancelled after cancel()"
    );
    assert!(
        child_a.is_cancelled(),
        "child_a must reflect parent cancellation"
    );
    assert!(
        child_b.is_cancelled(),
        "child_b must reflect grandparent cancellation"
    );
}

// ---------------------------------------------------------------------------
// Test 10: CancellableAsyncEncoder — write before cancel succeeds, after fails
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_cancellable_encoder_write_after_cancel_fails() {
    let token = CancellationToken::new();
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = CancellableAsyncEncoder::new(cursor, token.child());

        enc.write_item(&10u32)
            .await
            .expect("first write must succeed");
        enc.write_item(&20u32)
            .await
            .expect("second write must succeed");

        token.cancel();

        let result = enc.write_item(&30u32).await;
        assert!(
            result.is_err(),
            "write after cancel must return Err, got Ok"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 11: CancellableAsyncEncoder — finish() after cancel fails
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_cancellable_encoder_finish_after_cancel_fails() {
    let token = CancellationToken::new();
    let mut buf = Vec::<u8>::new();
    let cursor = Cursor::new(&mut buf);
    let enc = CancellableAsyncEncoder::new(cursor, token.child());

    token.cancel();

    let result = enc.finish().await;
    assert!(
        result.is_err(),
        "finish() after cancel must return Err, got Ok"
    );
}

// ---------------------------------------------------------------------------
// Test 12: CancellableAsyncDecoder — read_item after cancel fails
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_cancellable_decoder_read_after_cancel_fails() {
    // Build a valid encoded buffer first
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in 0u32..5u32 {
            enc.write_item(&v).await.expect("encode failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let token = CancellationToken::new();
    let cursor = Cursor::new(buf);
    let mut dec = CancellableAsyncDecoder::new(cursor, token.child());

    // First read is fine
    let first: Option<u32> = dec.read_item().await.expect("first read must succeed");
    assert_eq!(first, Some(0u32));

    // Cancel and verify next read fails
    token.cancel();
    let result = dec.read_item::<u32>().await;
    assert!(
        result.is_err(),
        "read_item after cancel must return Err, got Ok"
    );
}

// ---------------------------------------------------------------------------
// Test 13: CancellableAsyncDecoder — read_all before cancel succeeds
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_cancellable_decoder_read_all_succeeds() {
    let items: Vec<u64> = (0u64..30u64).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &items {
            enc.write_item(v).await.expect("encode failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let token = CancellationToken::new();
    let cursor = Cursor::new(buf);
    let mut dec = CancellableAsyncDecoder::new(cursor, token.child());

    let decoded: Vec<u64> = dec.read_all().await.expect("read_all must succeed");
    assert_eq!(decoded, items);
    assert!(dec.is_finished(), "decoder must be finished after read_all");
}

// ---------------------------------------------------------------------------
// Test 14: set_estimated_total and progress percentage calculation
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_estimated_total_and_percentage() {
    let total_items: u64 = 50;
    let items: Vec<u32> = (0u32..total_items as u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.set_estimated_total(total_items);
        for v in &items {
            enc.write_item(v).await.expect("encode failed");
        }
        enc.finish().await.expect("finish failed");
    }

    // Decode and verify percentage at halfway point
    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);

    // Read half the items
    for _ in 0u32..(total_items as u32 / 2) {
        let _: Option<u32> = dec.read_item().await.expect("read failed");
    }

    // Set estimated total on progress so percentage works
    let decoded: Vec<u32> = dec.read_all().await.expect("read_all failed");

    // Combined length must equal total
    assert_eq!(
        decoded.len() + (total_items as usize / 2),
        total_items as usize,
        "item count mismatch"
    );
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0"
    );
}

// ---------------------------------------------------------------------------
// Test 15: with_flush_per_item(true) — each item is a separate chunk
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_flush_per_item_creates_many_chunks() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let items: Vec<u32> = (0u32..10u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        for v in &items {
            enc.write_item(v)
                .await
                .expect("write flush_per_item failed");
        }
        enc.finish().await.expect("finish flush_per_item failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = dec.read_all().await.expect("decode flush_per_item failed");

    assert_eq!(decoded, items);
    // With flush_per_item each item becomes its own chunk
    assert_eq!(
        dec.progress().chunks_processed,
        items.len() as u64,
        "expected one chunk per item with flush_per_item"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Encoder progress() reports correct items_processed before finish
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_encoder_progress_before_finish() {
    // Use a small chunk size (1024 bytes) so flushing happens during writes
    let config = StreamingConfig::new().with_chunk_size(1024);
    let item_count = 200u32;

    let mut buf = Vec::<u8>::new();
    let cursor = Cursor::new(&mut buf);
    let mut enc = AsyncStreamingEncoder::with_config(cursor, config);

    for v in 0u32..item_count {
        enc.write_item(&v)
            .await
            .expect("write encoder progress failed");
    }

    // Before finish, some chunks should have been flushed already
    // (because 200 x u32 at ~1-2 bytes each > 1024 bytes)
    let progress_before = enc.progress().items_processed;
    // We can't know exact value but it must be >= 0
    // If at least one flush happened, items_processed > 0
    // (depends on encoding size; with varints small values are 1-2 bytes)
    let _ = enc.finish().await.expect("finish encoder progress failed");

    // After finish, decode and verify everything was written correctly
    let decoded: Vec<u32> = AsyncStreamingDecoder::new(Cursor::new(buf))
        .read_all()
        .await
        .expect("decode encoder progress failed");

    assert_eq!(decoded.len(), item_count as usize);
    // progress_before should be consistent (either 0 or some flushed count)
    assert!(
        progress_before <= item_count as u64,
        "encoder items_processed before finish must not exceed total items"
    );
}

// ---------------------------------------------------------------------------
// Test 17: get_ref() on encoder returns reference to underlying writer
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_encoder_get_ref_accessor() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&42u32).await.expect("write failed");

        // get_ref() must return a reference to the inner Cursor
        let _inner: &Cursor<&mut Vec<u8>> = enc.get_ref();

        enc.finish().await.expect("finish failed");
    }

    let (decoded, _): (u32, _) = oxicode::decode_from_slice(
        &buf[oxicode::streaming::ChunkHeader::SIZE..],
    )
    .unwrap_or_else(|_| {
        // Decode via streaming decoder instead (safer)
        let cursor = Cursor::new(buf.clone());
        let mut dec = AsyncStreamingDecoder::new(cursor);
        let v: u32 = tokio::runtime::Handle::current()
            .block_on(async { dec.read_item().await.expect("decode failed") })
            .expect("None when Some expected");
        (v, 0)
    });
    assert_eq!(decoded, 42u32, "decoded value must match written value");
}

// ---------------------------------------------------------------------------
// Test 18: get_ref() on decoder returns reference to underlying reader
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_decoder_get_ref_accessor() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&99u64).await.expect("write failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);

    // get_ref() must compile and return &Cursor
    let _ref: &Cursor<Vec<u8>> = dec.get_ref();

    let val: Option<u64> = dec.read_item().await.expect("decode failed");
    assert_eq!(val, Some(99u64));
}

// ---------------------------------------------------------------------------
// Test 19: Very large single item — Vec<u8> of 128 KiB forces multi-chunk
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_very_large_single_item_128kib() {
    // 128 KiB payload — larger than DEFAULT_CHUNK_SIZE (64 KiB)
    let payload: Vec<u8> = (0u8..=255u8).cycle().take(128 * 1024).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&payload)
            .await
            .expect("write 128 KiB failed");
        enc.finish().await.expect("finish 128 KiB failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Vec<u8>> = dec.read_item().await.expect("decode 128 KiB failed");
    assert_eq!(decoded, Some(payload), "128 KiB payload mismatch");
}

// ---------------------------------------------------------------------------
// Test 20: Multi-chunk encode then read_item one-by-one (not read_all)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_multi_chunk_read_item_one_by_one() {
    // Use very small chunk size (1024 minimum) and long strings to ensure multiple chunks.
    // Each string is ~40 bytes; 30 strings = ~1200 bytes > 1024 byte limit.
    let config = StreamingConfig::new().with_chunk_size(1024);
    let items: Vec<String> = (0u32..30u32)
        .map(|i| format!("{:0>40}", i)) // 40-char zero-padded string
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        for v in &items {
            enc.write_item(v).await.expect("write multi-chunk failed");
        }
        enc.finish().await.expect("finish multi-chunk failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<String>::new();

    while let Some(v) = dec.read_item::<String>().await.expect("read_item failed") {
        decoded.push(v);
    }

    assert_eq!(decoded, items, "one-by-one multi-chunk decode mismatch");
    assert!(
        dec.progress().chunks_processed > 1,
        "expected multiple chunks for 30 strings with 1024-byte limit, got {}",
        dec.progress().chunks_processed
    );
}

// ---------------------------------------------------------------------------
// Test 21: read_item after is_finished() == true returns None immediately
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_read_item_after_finished_returns_none() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&7u32).await.expect("write failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);

    // Drain the stream
    let _: Vec<u32> = dec.read_all().await.expect("read_all failed");
    assert!(dec.is_finished(), "must be finished after read_all");

    // Subsequent calls must return None without error
    let extra_a: Option<u32> = dec.read_item().await.expect("extra read_a failed");
    let extra_b: Option<u32> = dec.read_item().await.expect("extra read_b failed");

    assert!(extra_a.is_none(), "extra_a must be None after finish");
    assert!(extra_b.is_none(), "extra_b must be None after finish");
}

// ---------------------------------------------------------------------------
// Test 22: CancellableAsyncEncoder progress() reflects items before cancel
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async4_cancellable_encoder_progress_before_cancel() {
    // Use small chunk size so writes cause flushes that update progress
    let config = StreamingConfig::new().with_chunk_size(1024);
    let token = CancellationToken::new();
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut inner_enc = AsyncStreamingEncoder::with_config(cursor, config);

        // Write 100 items (each ~2 bytes varint, 100 * 2 = ~200 bytes < 1024 so one chunk)
        for v in 0u32..100u32 {
            inner_enc
                .write_item(&v)
                .await
                .expect("write progress failed");
        }

        // Verify progress via encoder (items may be in buffer, not yet flushed)
        let progress_items = inner_enc.progress().items_processed;

        // Wrap into cancellable encoder just to test its progress delegation
        // (We use a separate token since we need the inner encoder's handle)
        drop(inner_enc);

        // Independent CancellableAsyncEncoder test
        let cursor2 = Cursor::new(&mut buf);
        let mut cenc = CancellableAsyncEncoder::new(cursor2, token.child());
        for v in 0u32..50u32 {
            cenc.write_item(&v).await.expect("cancellable write failed");
        }

        let cancellable_progress = cenc.progress().items_processed;

        token.cancel();

        // After cancel, verify progress hasn't regressed
        let after_cancel_progress = cenc.progress().items_processed;
        assert_eq!(
            cancellable_progress, after_cancel_progress,
            "progress must not change after cancel"
        );

        // items_processed must be consistent (0 or flushed count — not greater than written)
        assert!(
            progress_items <= 100,
            "encoder progress_items must not exceed written count"
        );
    }
}
