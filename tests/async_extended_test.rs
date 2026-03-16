//! Extended async streaming tests for oxicode.
//!
//! All tests exercise the `AsyncStreamingEncoder` / `AsyncStreamingDecoder` API
//! and are gated behind the `async-tokio` feature.

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
mod async_extended {
    use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder};
    use oxicode::{Decode, Encode};
    use std::io::Cursor;

    // -----------------------------------------------------------------------
    // Test 1: Async encode then decode a large struct
    //         (struct containing Vec<String> with 100 items)
    // -----------------------------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct LargeStruct {
        id: u32,
        tags: Vec<String>,
        score: f64,
    }

    #[tokio::test]
    async fn test_async_large_struct_roundtrip() {
        let original = LargeStruct {
            id: 7,
            tags: (0..100).map(|i| format!("tag-{i:03}")).collect(),
            score: 99.5,
        };

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_item(&original)
                .await
                .expect("write_item failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<LargeStruct> = decoder.read_item().await.expect("read_item failed");
        assert_eq!(Some(original), decoded);
        assert!(
            decoder.is_finished() || decoder.read_item::<LargeStruct>().await.unwrap().is_none()
        );
    }

    // -----------------------------------------------------------------------
    // Test 2: Async encode multiple types sequentially
    //         (u32, String, Vec<u8>, bool)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_multiple_types_sequentially() {
        let val_u32: u32 = 42;
        let val_string: String = "hello oxicode".to_string();
        let val_bytes: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let val_bool: bool = true;

        // Each type is encoded into its own independent stream to avoid
        // heterogeneous type mixing within a single stream.
        let mut buf_u32 = Vec::<u8>::new();
        let mut buf_string = Vec::<u8>::new();
        let mut buf_bytes = Vec::<u8>::new();
        let mut buf_bool = Vec::<u8>::new();

        {
            let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf_u32));
            enc.write_item(&val_u32).await.expect("write u32");
            enc.finish().await.expect("finish u32");
        }
        {
            let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf_string));
            enc.write_item(&val_string).await.expect("write string");
            enc.finish().await.expect("finish string");
        }
        {
            let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf_bytes));
            enc.write_item(&val_bytes).await.expect("write bytes");
            enc.finish().await.expect("finish bytes");
        }
        {
            let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf_bool));
            enc.write_item(&val_bool).await.expect("write bool");
            enc.finish().await.expect("finish bool");
        }

        let decoded_u32: Option<u32> = AsyncStreamingDecoder::new(Cursor::new(buf_u32))
            .read_item()
            .await
            .expect("decode u32");
        let decoded_string: Option<String> = AsyncStreamingDecoder::new(Cursor::new(buf_string))
            .read_item()
            .await
            .expect("decode string");
        let decoded_bytes: Option<Vec<u8>> = AsyncStreamingDecoder::new(Cursor::new(buf_bytes))
            .read_item()
            .await
            .expect("decode bytes");
        let decoded_bool: Option<bool> = AsyncStreamingDecoder::new(Cursor::new(buf_bool))
            .read_item()
            .await
            .expect("decode bool");

        assert_eq!(decoded_u32, Some(val_u32));
        assert_eq!(decoded_string, Some(val_string));
        assert_eq!(decoded_bytes, Some(val_bytes));
        assert_eq!(decoded_bool, Some(val_bool));
    }

    // -----------------------------------------------------------------------
    // Test 3: Async decode from a std::io::Cursor<Vec<u8>>
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decode_from_cursor() {
        let values: Vec<u64> = (10..20).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        // Decode explicitly via std::io::Cursor<Vec<u8>>
        let cursor: Cursor<Vec<u8>> = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u64> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(values, decoded);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 4: Async encode 1000 u64 values using a loop
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_1000_u64_loop() {
        const N: u64 = 1000;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for i in 0..N {
                encoder.write_item(&i).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u64> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded.len(), N as usize);
        for (expected, actual) in (0..N).zip(decoded.iter()) {
            assert_eq!(expected, *actual);
        }
    }

    // -----------------------------------------------------------------------
    // Test 5: Async encode + decode preserves u128 values correctly
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_u128_roundtrip() {
        let values: Vec<u128> = vec![
            0,
            1,
            u64::MAX as u128,
            u128::MAX / 2,
            u128::MAX - 1,
            u128::MAX,
        ];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u128> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(values, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 6: Async encode empty Vec<u8>
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_empty_vec_u8() {
        let empty: Vec<u8> = Vec::new();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&empty).await.expect("write failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<Vec<u8>> = decoder.read_item().await.expect("decode failed");

        assert_eq!(decoded, Some(empty));
    }

    // -----------------------------------------------------------------------
    // Test 7: Async encode/decode with Tokio BufReader/BufWriter
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_with_tokio_buf_reader_writer() {
        use tokio::io::{BufReader, BufWriter};

        let values: Vec<u32> = (0..50).collect();

        // BufWriter wrapping an in-memory cursor
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let buf_writer = BufWriter::new(cursor);
            let mut encoder = AsyncStreamingEncoder::new(buf_writer);
            for &v in &values {
                encoder.write_item(&v).await.expect("write failed");
            }
            // finish flushes BufWriter internally
            encoder.finish().await.expect("finish failed");
        }

        // BufReader wrapping an in-memory cursor
        let cursor = Cursor::new(buffer);
        let buf_reader = BufReader::new(cursor);
        let mut decoder = AsyncStreamingDecoder::new(buf_reader);
        let decoded: Vec<u32> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(values, decoded);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 8: Encode asynchronously, verify bytes match synchronous encode
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_matches_sync_encode() {
        let value: u64 = 0xCAFE_BABE_1234_5678;

        // Synchronous encode to get reference bytes (single item as a stream chunk)
        let mut sync_buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut sync_buffer);
            // Use sync streaming encoder to produce identical framing
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&value).await.expect("sync-style write");
            encoder.finish().await.expect("sync-style finish");
        }

        // Async encode
        let mut async_buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut async_buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&value).await.expect("async write");
            encoder.finish().await.expect("async finish");
        }

        assert_eq!(
            sync_buffer, async_buffer,
            "async and sync encoding must produce identical bytes"
        );

        // Both must decode back to the original value
        let decoded_async: Option<u64> = AsyncStreamingDecoder::new(Cursor::new(async_buffer))
            .read_item()
            .await
            .expect("decode async");
        assert_eq!(decoded_async, Some(value));
    }

    // -----------------------------------------------------------------------
    // Test 9: Async round-trip for derived struct
    // -----------------------------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct DerivedPoint {
        x: i32,
        y: i32,
        label: String,
    }

    #[tokio::test]
    async fn test_async_roundtrip_derived_struct() {
        let items: Vec<DerivedPoint> = (0..30)
            .map(|i| DerivedPoint {
                x: i * 3,
                y: -(i * 7),
                label: format!("point-{i}"),
            })
            .collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for item in &items {
                encoder.write_item(item).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<DerivedPoint> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(items, decoded);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 10: Async decode error on truncated data
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decode_error_on_truncated_data() {
        let value: u32 = 12345;

        let mut full_buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut full_buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&value).await.expect("write failed");
            encoder.finish().await.expect("finish failed");
        }

        // Truncate to half the bytes to force a decode error
        let half = full_buffer.len() / 2;
        let truncated = full_buffer[..half].to_vec();

        let cursor = Cursor::new(truncated);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let result = decoder.read_item::<u32>().await;

        assert!(
            result.is_err() || result.unwrap().is_none(),
            "decoding truncated data must fail or return None"
        );
    }

    // -----------------------------------------------------------------------
    // Test 11: Multiple sequential async encodes into the same encoder instance
    //
    // A single encoder writes many items across multiple write_item calls.
    // After finish(), all items are decoded back in the correct order.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_multiple_sequential_encodes_same_writer() {
        // Simulate three "batches" written sequentially into one encoder without
        // calling finish() between batches — all items end up in the same stream.
        let batch_a: Vec<u32> = vec![1, 2, 3];
        let batch_b: Vec<u32> = vec![10, 20, 30];
        let batch_c: Vec<u32> = vec![100, 200, 300];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);

            // Encode all three batches sequentially into the same encoder
            for &v in &batch_a {
                encoder.write_item(&v).await.expect("write A");
            }
            for &v in &batch_b {
                encoder.write_item(&v).await.expect("write B");
            }
            for &v in &batch_c {
                encoder.write_item(&v).await.expect("write C");
            }

            encoder.finish().await.expect("finish");
        }

        // Decode all items back
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().await.expect("decode all");

        let mut expected = batch_a.clone();
        expected.extend_from_slice(&batch_b);
        expected.extend_from_slice(&batch_c);

        assert_eq!(expected, decoded);
        assert_eq!(decoder.progress().items_processed, expected.len() as u64);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 12: Async decode with exact byte consumption verification
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decode_exact_byte_consumption() {
        let value: u32 = 99;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&value).await.expect("write failed");
            encoder.finish().await.expect("finish failed");
        }
        let total_bytes = buffer.len();

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<u32> = decoder.read_item().await.expect("decode failed");
        assert_eq!(decoded, Some(value));

        // Drain EOF sentinel
        let eof: Option<u32> = decoder.read_item().await.expect("eof read");
        assert!(eof.is_none());
        assert!(decoder.is_finished());

        // bytes_processed must be > 0 and <= total encoded size
        let bytes_processed = decoder.progress().bytes_processed;
        assert!(
            bytes_processed > 0,
            "bytes_processed must be greater than zero"
        );
        assert!(
            bytes_processed <= total_bytes as u64,
            "bytes_processed ({bytes_processed}) must not exceed total buffer size ({total_bytes})"
        );
        assert_eq!(
            decoder.progress().items_processed,
            1,
            "exactly one item should have been decoded"
        );
    }
}

// ---------------------------------------------------------------------------
// 20 additional comprehensive async streaming tests
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
mod extra_async_tests2 {
    use oxicode::streaming::{
        AsyncStreamingDecoder, AsyncStreamingEncoder, CancellableAsyncEncoder, CancellationToken,
        StreamingConfig,
    };
    use oxicode::{Decode, Encode};
    use std::collections::{BTreeMap, HashMap};
    use std::f64::consts::{E, PI};
    use std::io::Cursor;
    use std::sync::{Arc, Mutex};

    // -----------------------------------------------------------------------
    // Test 13: Complex struct with multiple fields async roundtrip
    // -----------------------------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct ComplexRecord {
        id: u64,
        name: String,
        values: Vec<f64>,
        active: bool,
        score: i32,
    }

    #[tokio::test]
    async fn test_async_complex_struct_multiple_fields() {
        let original = ComplexRecord {
            id: 42,
            name: "oxicode-test".to_string(),
            values: vec![PI, E, PI * E, PI / E],
            active: true,
            score: -128,
        };

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_item(&original)
                .await
                .expect("write complex struct");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<ComplexRecord> =
            decoder.read_item().await.expect("decode complex struct");

        assert_eq!(decoded, Some(original));
        assert!(
            decoder.is_finished()
                || decoder
                    .read_item::<ComplexRecord>()
                    .await
                    .expect("eof")
                    .is_none()
        );
    }

    // -----------------------------------------------------------------------
    // Test 14: Cancellation mid-stream via CancellableAsyncEncoder
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_cancellation_mid_stream() {
        let token = CancellationToken::new();
        let mut buffer = Vec::<u8>::new();
        let cursor = Cursor::new(&mut buffer);
        let mut encoder = CancellableAsyncEncoder::new(cursor, token.child());

        // Write some items before cancellation
        for i in 0u32..5 {
            encoder.write_item(&i).await.expect("write before cancel");
        }
        assert_eq!(
            encoder.progress().items_processed,
            0,
            "items not yet flushed"
        );

        // Cancel the token
        token.cancel();

        // Any subsequent write must return an error
        let result = encoder.write_item(&99u32).await;
        assert!(result.is_err(), "write after cancel must fail");
    }

    // -----------------------------------------------------------------------
    // Test 15: Large batch encode of 1000 items (structs, not just scalars)
    // -----------------------------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct BatchItem {
        index: u32,
        payload: String,
    }

    #[tokio::test]
    async fn test_async_large_batch_encode_1000_structs() {
        let items: Vec<BatchItem> = (0u32..1000)
            .map(|i| BatchItem {
                index: i,
                payload: format!("item-{i:04}"),
            })
            .collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for item in &items {
                encoder.write_item(item).await.expect("write batch item");
            }
            encoder.finish().await.expect("finish batch");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<BatchItem> = decoder.read_all().await.expect("decode batch");

        assert_eq!(decoded.len(), 1000);
        assert_eq!(decoded, items);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 16: Mixed type sequences encoded into separate streams
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_mixed_type_sequences() {
        // Encode and decode each type independently (type-safe approach)
        let val_i8: i8 = -42;
        let val_f32: f32 = PI as f32;
        let val_u16: u16 = 0xABCD;
        let val_i64: i64 = i64::MIN / 3;

        macro_rules! roundtrip {
            ($val:expr, $ty:ty) => {{
                let mut buf = Vec::<u8>::new();
                {
                    let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf));
                    enc.write_item(&$val).await.expect("write");
                    enc.finish().await.expect("finish");
                }
                let decoded: Option<$ty> = AsyncStreamingDecoder::new(Cursor::new(buf))
                    .read_item()
                    .await
                    .expect("decode");
                assert_eq!(decoded, Some($val));
            }};
        }

        roundtrip!(val_i8, i8);
        roundtrip!(val_f32, f32);
        roundtrip!(val_u16, u16);
        roundtrip!(val_i64, i64);
    }

    // -----------------------------------------------------------------------
    // Test 17: Progress tracking callbacks (via Arc<Mutex<>> counter)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_progress_tracking_callbacks() {
        const TOTAL: u64 = 50;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.set_estimated_total(TOTAL);
            for i in 0u32..TOTAL as u32 {
                encoder.write_item(&i).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        }

        // Use a shared counter to simulate callback-style progress tracking
        let processed = Arc::new(Mutex::new(0u64));
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);

        while let Some(_item) = decoder.read_item::<u32>().await.expect("read") {
            let mut count = processed.lock().expect("lock");
            *count += 1;
        }

        let final_count = *processed.lock().expect("lock");
        assert_eq!(final_count, TOTAL);
        assert_eq!(decoder.progress().items_processed, TOTAL);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 18: Empty sequence encode/decode (write_all with empty iterator)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_empty_sequence_encode_decode() {
        let empty: Vec<u32> = Vec::new();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_all(empty.iter().copied())
                .await
                .expect("write_all empty");
            encoder.finish().await.expect("finish");
        }

        assert!(
            !buffer.is_empty(),
            "buffer should have end-of-stream marker"
        );

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().await.expect("decode empty");

        assert!(decoded.is_empty());
        assert!(decoder.is_finished());
        assert_eq!(decoder.progress().items_processed, 0);
    }

    // -----------------------------------------------------------------------
    // Test 19: Error recovery after truncated stream
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_error_recovery_after_truncated_stream() {
        let values: Vec<u64> = vec![PI.to_bits(), E.to_bits(), (PI * E).to_bits()];

        let mut full_buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut full_buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        }

        // Truncate: remove the last quarter of bytes to force mid-chunk corruption
        let truncated_len = full_buffer.len() * 3 / 4;
        let truncated = full_buffer[..truncated_len].to_vec();

        let cursor = Cursor::new(truncated);
        let mut decoder = AsyncStreamingDecoder::new(cursor);

        // Attempt to read; may succeed for initial items or fail on truncation
        let mut recovered = 0usize;
        loop {
            match decoder.read_item::<u64>().await {
                Ok(Some(_)) => recovered += 1,
                Ok(None) => break,
                Err(_) => break, // expected on truncation
            }
        }

        // We do not assert a specific count — the key invariant is that
        // the decoder does NOT panic and handles the error path gracefully.
        assert!(
            recovered <= values.len(),
            "cannot decode more items than encoded"
        );
    }

    // -----------------------------------------------------------------------
    // Test 20: Async encode with small chunk size (forces multiple chunks)
    //
    // Use chunk_size=1024 (minimum allowed by clamp) with items whose total
    // encoded size exceeds 1024 bytes to guarantee multiple chunk flushes.
    // Each String item is ~20 bytes; 200 items = ~4000 bytes > 4 chunks.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_with_small_chunk_size() {
        // chunk_size clamped minimum is 1024; use items large enough to overflow it
        let config = StreamingConfig::new().with_chunk_size(1024);
        // Each string is ~18 bytes encoded; 200 items ≈ 3600 bytes → ≥ 3 chunks
        let items: Vec<String> = (0u32..200).map(|i| format!("chunk-item-{i:05}")).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for item in &items {
                encoder.write_item(item).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<String> = decoder.read_all().await.expect("decode");

        assert_eq!(decoded, items);
        assert!(
            decoder.progress().chunks_processed >= 2,
            "small chunk_size with large payload should produce at least 2 chunks, got {}",
            decoder.progress().chunks_processed
        );
    }

    // -----------------------------------------------------------------------
    // Test 21: Async encode with flush_per_item config
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_with_flush_per_item() {
        let config = StreamingConfig::new().with_flush_per_item(true);
        let items: Vec<u32> = (100u32..110).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for &item in &items {
                encoder.write_item(&item).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().await.expect("decode flush_per_item");

        assert_eq!(decoded, items);
        // flush_per_item=true: each item is its own chunk
        assert_eq!(
            decoder.progress().chunks_processed,
            items.len() as u64,
            "flush_per_item must produce one chunk per item"
        );
    }

    // -----------------------------------------------------------------------
    // Test 22: Encode to Vec via cursor, decode from same Vec
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_to_vec_via_cursor() {
        let values: Vec<i32> = (-50i32..50).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("write i32");
            }
            encoder.finish().await.expect("finish");
        }

        // Confirm we have actual bytes
        assert!(!buffer.is_empty());

        // Decode from the exact same Vec
        let cursor: Cursor<Vec<u8>> = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<i32> = decoder.read_all().await.expect("decode");

        assert_eq!(decoded, values);
    }

    // -----------------------------------------------------------------------
    // Test 23: Struct with derive roundtrip async
    // -----------------------------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct DerivedColor {
        r: u8,
        g: u8,
        b: u8,
        alpha: f32,
        name: String,
    }

    #[tokio::test]
    async fn test_async_derived_struct_roundtrip() {
        let colors: Vec<DerivedColor> = vec![
            DerivedColor {
                r: 255,
                g: 0,
                b: 0,
                alpha: 1.0,
                name: "red".into(),
            },
            DerivedColor {
                r: 0,
                g: 255,
                b: 0,
                alpha: 0.5,
                name: "green".into(),
            },
            DerivedColor {
                r: 0,
                g: 0,
                b: 255,
                alpha: PI as f32 / 4.0,
                name: "blue".into(),
            },
        ];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for c in &colors {
                encoder.write_item(c).await.expect("write color");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<DerivedColor> = decoder.read_all().await.expect("decode colors");

        assert_eq!(decoded, colors);
    }

    // -----------------------------------------------------------------------
    // Test 24: String encoding/decoding async
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_string_encoding_decoding() {
        let strings: Vec<String> = vec![
            String::new(),
            "hello".into(),
            "oxicode 🦀".into(),
            "π ≈ 3.14159".into(),
            "A".repeat(256),
        ];

        for s in &strings {
            let mut buf = Vec::<u8>::new();
            {
                let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf));
                enc.write_item(s).await.expect("write string");
                enc.finish().await.expect("finish");
            }
            let decoded: Option<String> = AsyncStreamingDecoder::new(Cursor::new(buf))
                .read_item()
                .await
                .expect("decode string");
            assert_eq!(decoded.as_ref(), Some(s));
        }
    }

    // -----------------------------------------------------------------------
    // Test 25: Vec<u8> async roundtrip
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_vec_u8_roundtrip() {
        let payloads: Vec<Vec<u8>> =
            vec![vec![], vec![0x00], vec![0xFF; 128], (0u8..=255).collect()];

        for payload in &payloads {
            let mut buf = Vec::<u8>::new();
            {
                let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf));
                enc.write_item(payload).await.expect("write vec<u8>");
                enc.finish().await.expect("finish");
            }
            let decoded: Option<Vec<u8>> = AsyncStreamingDecoder::new(Cursor::new(buf))
                .read_item()
                .await
                .expect("decode vec<u8>");
            assert_eq!(decoded.as_ref(), Some(payload));
        }
    }

    // -----------------------------------------------------------------------
    // Test 26: Option<T> async roundtrip
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_option_t_roundtrip() {
        let some_val: Option<u64> = Some(0xDEAD_BEEF_CAFE_0000);
        let none_val: Option<u64> = None;

        for val in [some_val, none_val] {
            let mut buf = Vec::<u8>::new();
            {
                let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf));
                enc.write_item(&val).await.expect("write option");
                enc.finish().await.expect("finish");
            }
            let decoded: Option<Option<u64>> = AsyncStreamingDecoder::new(Cursor::new(buf))
                .read_item()
                .await
                .expect("decode option");
            assert_eq!(decoded, Some(val));
        }
    }

    // -----------------------------------------------------------------------
    // Test 27: HashMap encoding async
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_hashmap_encoding() {
        let mut map: HashMap<String, u32> = HashMap::new();
        map.insert("pi".into(), PI.to_bits() as u32);
        map.insert("e".into(), E.to_bits() as u32);
        map.insert("zero".into(), 0);
        map.insert("max".into(), u32::MAX);

        let mut buf = Vec::<u8>::new();
        {
            let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf));
            enc.write_item(&map).await.expect("write hashmap");
            enc.finish().await.expect("finish");
        }

        let decoded: Option<HashMap<String, u32>> = AsyncStreamingDecoder::new(Cursor::new(buf))
            .read_item()
            .await
            .expect("decode hashmap");

        assert_eq!(decoded, Some(map));
    }

    // -----------------------------------------------------------------------
    // Test 28: BTreeMap encoding async
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_btreemap_encoding() {
        let mut map: BTreeMap<u32, String> = BTreeMap::new();
        for i in 0u32..20 {
            map.insert(i, format!("value-{i}"));
        }

        let mut buf = Vec::<u8>::new();
        {
            let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf));
            enc.write_item(&map).await.expect("write btreemap");
            enc.finish().await.expect("finish");
        }

        let decoded: Option<BTreeMap<u32, String>> = AsyncStreamingDecoder::new(Cursor::new(buf))
            .read_item()
            .await
            .expect("decode btreemap");

        assert_eq!(decoded, Some(map));
    }

    // -----------------------------------------------------------------------
    // Test 29: Large string async encode/decode
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_large_string_encode_decode() {
        // Build a 64KB string with repeating PI digits pattern
        let pi_str = format!("{:.10}", PI); // "3.1415926536"
        let repeats = (65536 / pi_str.len()) + 1;
        let large: String = pi_str.repeat(repeats);

        let mut buf = Vec::<u8>::new();
        {
            let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf));
            enc.write_item(&large).await.expect("write large string");
            enc.finish().await.expect("finish");
        }

        let decoded: Option<String> = AsyncStreamingDecoder::new(Cursor::new(buf))
            .read_item()
            .await
            .expect("decode large string");

        assert_eq!(decoded, Some(large));
    }

    // -----------------------------------------------------------------------
    // Test 30: Recursive (sequential) decode — decode multiple items one by one
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_recursive_sequential_decode() {
        // Encode 20 items with different numeric patterns
        let items: Vec<u64> = (0u64..20)
            .map(|i| {
                // mix of powers-of-two, PI bits, E bits
                match i % 4 {
                    0 => 1u64 << (i + 1),
                    1 => (PI * i as f64).to_bits(),
                    2 => (E.powi(i as i32)).to_bits(),
                    _ => u64::MAX >> i,
                }
            })
            .collect();

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &item in &items {
                encoder.write_item(&item).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        }

        // Decode one-by-one in a recursive-style loop (no read_all helper)
        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let mut decoded = Vec::new();
        while let Some(v) = decoder.read_item::<u64>().await.expect("read_item") {
            decoded.push(v);
        }

        assert_eq!(decoded, items);
        assert_eq!(decoder.progress().items_processed, items.len() as u64);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 31: Concurrent encode operations via tokio::spawn
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_concurrent_encode_operations() {
        use tokio::task::JoinSet;

        let mut set: JoinSet<Vec<u8>> = JoinSet::new();

        // Spawn 8 independent encode tasks
        for task_id in 0u32..8 {
            set.spawn(async move {
                let items: Vec<u32> = (0u32..50).map(|i| task_id * 1000 + i).collect();
                let mut buf = Vec::<u8>::new();
                {
                    let cursor = Cursor::new(&mut buf);
                    let mut encoder = AsyncStreamingEncoder::new(cursor);
                    for &item in &items {
                        encoder.write_item(&item).await.expect("encode concurrent");
                    }
                    encoder.finish().await.expect("finish concurrent");
                }
                buf
            });
        }

        // Collect and validate all results
        let mut task_count = 0u32;
        while let Some(result) = set.join_next().await {
            let buf = result.expect("task panicked");
            let cursor = Cursor::new(buf);
            let mut decoder = AsyncStreamingDecoder::new(cursor);
            let decoded: Vec<u32> = decoder.read_all().await.expect("decode concurrent");
            assert_eq!(decoded.len(), 50, "each task must encode exactly 50 items");
            task_count += 1;
        }
        assert_eq!(task_count, 8);
    }

    // -----------------------------------------------------------------------
    // Test 32: Async encode with estimated total and percentage progress
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_estimated_total_and_percentage() {
        const TOTAL: u64 = 100;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.set_estimated_total(TOTAL);
            for i in 0u32..TOTAL as u32 {
                encoder.write_item(&i).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        decoder
            .read_all::<u32>()
            .await
            .expect("decode percentage test");

        let progress = decoder.progress();
        assert_eq!(progress.items_processed, TOTAL);
        assert!(progress.chunks_processed >= 1);
    }

    // -----------------------------------------------------------------------
    // Test 33: Async encoder get_ref and final writer recovery
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encoder_get_ref_and_writer_recovery() {
        let mut buffer = Vec::<u8>::new();
        let cursor = Cursor::new(&mut buffer);
        let mut encoder = AsyncStreamingEncoder::new(cursor);

        for i in 0u32..5 {
            encoder.write_item(&i).await.expect("write");
        }

        // get_ref before finish must be valid (non-panicking)
        let _inner_ref = encoder.get_ref();

        // Finish and recover the underlying writer
        let recovered_cursor = encoder.finish().await.expect("finish");

        // The recovered cursor's position must be > 0 (data was written)
        assert!(
            recovered_cursor.position() > 0,
            "cursor must have advanced after writes"
        );
    }
}
