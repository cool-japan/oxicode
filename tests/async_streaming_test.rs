//! Async streaming integration tests (tokio)

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
mod async_streaming {
    use oxicode::streaming::{
        AsyncStreamingDecoder, AsyncStreamingEncoder, CancellableAsyncDecoder,
        CancellableAsyncEncoder, CancellationToken, StreamingConfig,
    };
    use oxicode::{Decode, Encode};
    use std::io::Cursor;

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct DataPoint {
        id: u32,
        value: f64,
    }

    // -----------------------------------------------------------------------
    // Test 1: Basic async encode/decode roundtrip using in-memory cursor
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_roundtrip_basic() {
        let original = DataPoint {
            id: 42,
            value: 3.14,
        };

        // Encode
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

        // Decode
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<DataPoint> = decoder.read_item().await.expect("read_item failed");
        assert_eq!(Some(original), decoded);

        let eof: Option<DataPoint> = decoder
            .read_item()
            .await
            .expect("read_item after eof failed");
        assert!(eof.is_none());
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 2: Encode/decode multiple items, verify order and count
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_multiple_items() {
        let items: Vec<DataPoint> = (0..100)
            .map(|i| DataPoint {
                id: i,
                value: i as f64 * 0.5,
            })
            .collect();

        // Encode all at once via write_all
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_all(items.clone())
                .await
                .expect("write_all failed");
            encoder.finish().await.expect("finish failed");
        }

        // Decode via read_all
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<DataPoint> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(items.len(), decoded.len());
        assert_eq!(items, decoded);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 3: Cancellation token stops encoding mid-stream
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_cancellation_encoder() {
        let token = CancellationToken::new();
        let child = token.child();

        let mut buffer = Vec::<u8>::new();
        let cursor = Cursor::new(&mut buffer);
        let mut encoder = CancellableAsyncEncoder::new(cursor, child);

        // These should succeed
        encoder
            .write_item(&DataPoint { id: 1, value: 1.0 })
            .await
            .expect("write 1 failed");
        encoder
            .write_item(&DataPoint { id: 2, value: 2.0 })
            .await
            .expect("write 2 failed");

        // Cancel the token
        token.cancel();
        assert!(token.is_cancelled());

        // Next write should fail
        let result = encoder.write_item(&DataPoint { id: 3, value: 3.0 }).await;
        assert!(
            result.is_err(),
            "write after cancellation should return Err"
        );
    }

    // -----------------------------------------------------------------------
    // Test 4: Cancellation token stops decoding mid-stream
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_cancellation_decoder() {
        // Prepare encoded buffer first
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for i in 0..10u32 {
                encoder.write_item(&i).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let token = CancellationToken::new();
        let child = token.child();

        let cursor = Cursor::new(buffer);
        let mut decoder = CancellableAsyncDecoder::new(cursor, child);

        // Read first item fine
        let first: Option<u32> = decoder.read_item().await.expect("first read failed");
        assert_eq!(first, Some(0));

        // Cancel
        token.cancel();

        // Next read should fail
        let result = decoder.read_item::<u32>().await;
        assert!(result.is_err(), "read after cancellation should return Err");
    }

    // -----------------------------------------------------------------------
    // Test 5: Cancellation token – child shares parent state
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_cancellation_token_child_shares_state() {
        let parent = CancellationToken::new();
        assert!(!parent.is_cancelled());

        let child = parent.child();
        assert!(!child.is_cancelled());

        parent.cancel();

        assert!(parent.is_cancelled());
        // Child must also see the cancellation
        assert!(child.is_cancelled());
    }

    // -----------------------------------------------------------------------
    // Test 6: Progress tracking – items_processed matches written count
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_progress_tracking() {
        const N: u64 = 30;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.set_estimated_total(N);
            for i in 0..N as u32 {
                encoder.write_item(&i).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let _: Vec<u32> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoder.progress().items_processed, N);
        assert!(decoder.progress().chunks_processed >= 1);
        assert!(decoder.progress().bytes_processed > 0);
    }

    // -----------------------------------------------------------------------
    // Test 7: Multiple chunks via small chunk_size with larger items
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_multiple_chunks() {
        // Use minimum chunk size (1024 bytes) with large string items to force
        // multiple flushes. Each string item encodes to ~200+ bytes.
        let config = StreamingConfig::new().with_chunk_size(1024);

        // 50 strings of 200 bytes each → ~10000 bytes total → at least 9 chunks at 1024 bytes
        let values: Vec<String> = (0u32..50)
            .map(|i| format!("{:0>200}", i)) // zero-padded to 200 chars
            .collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for v in &values {
                encoder.write_item(v).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<String> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(values, decoded);
        // Small chunk size with large items must have produced more than one chunk
        assert!(
            decoder.progress().chunks_processed > 1,
            "expected multiple chunks, got {}",
            decoder.progress().chunks_processed
        );
    }

    // -----------------------------------------------------------------------
    // Test 8: Item-by-item manual read_item interleaving with is_finished
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_item_by_item_manual() {
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&10u64).await.expect("write 10 failed");
            encoder.write_item(&20u64).await.expect("write 20 failed");
            encoder.write_item(&30u64).await.expect("write 30 failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);

        assert!(!decoder.is_finished());
        assert_eq!(decoder.read_item::<u64>().await.expect("read 1"), Some(10));
        assert_eq!(decoder.read_item::<u64>().await.expect("read 2"), Some(20));
        assert_eq!(decoder.read_item::<u64>().await.expect("read 3"), Some(30));

        let end = decoder.read_item::<u64>().await.expect("read end");
        assert_eq!(end, None);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 9: Cancellable decoder read_all
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_cancellable_decoder_read_all_no_cancel() {
        let values: Vec<u32> = (0..20).collect();
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let token = CancellationToken::new();
        let cursor = Cursor::new(buffer);
        let mut decoder = CancellableAsyncDecoder::new(cursor, token);
        let decoded: Vec<u32> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(values, decoded);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 10: get_ref returns the underlying writer/reader
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_get_ref() {
        let mut buffer = Vec::<u8>::new();
        let cursor = Cursor::new(&mut buffer);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder.write_item(&42u32).await.expect("write failed");
        // get_ref should compile and return a reference
        let _ = encoder.get_ref();
        encoder.finish().await.expect("finish failed");
    }

    // -----------------------------------------------------------------------
    // Test 11: Flush per item mode
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_flush_per_item() {
        let config = StreamingConfig::new().with_flush_per_item(true);
        let values: Vec<u32> = (0..10).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for &v in &values {
                encoder.write_item(&v).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().await.expect("read_all failed");
        assert_eq!(values, decoded);
        // Each item was its own chunk
        assert_eq!(decoder.progress().chunks_processed, values.len() as u64);
    }

    // -----------------------------------------------------------------------
    // Test 12: Cancellable finish after cancellation returns error
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_cancellable_finish_after_cancel() {
        let token = CancellationToken::new();
        let child = token.child();

        let mut buffer = Vec::<u8>::new();
        let cursor = Cursor::new(&mut buffer);
        let encoder = CancellableAsyncEncoder::new(cursor, child);

        token.cancel();
        let result = encoder.finish().await;
        assert!(result.is_err(), "finish after cancel should return Err");
    }
}

// ---------------------------------------------------------------------------
// Extended async streaming tests
// ---------------------------------------------------------------------------

#[cfg(feature = "async-tokio")]
mod extra_async_tests {
    use oxicode::streaming::{
        AsyncStreamingDecoder, AsyncStreamingEncoder, CancellableAsyncDecoder,
        CancellableAsyncEncoder, CancellationToken, StreamingConfig,
    };
    use oxicode::{Decode, Encode};
    use std::io::Cursor;

    /// A struct with a larger payload to stress multi-chunk streaming.
    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct AsyncPacket {
        id: u64,
        payload: Vec<u8>,
        label: String,
    }

    impl AsyncPacket {
        fn new(id: u64, size: usize) -> Self {
            Self {
                id,
                payload: (0..size).map(|i| (i & 0xFF) as u8).collect(),
                label: format!("packet-{id}"),
            }
        }
    }

    // -----------------------------------------------------------------------
    // Test 13: Encode many packets, decode all, verify identity
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_many_packets() {
        let packets: Vec<AsyncPacket> = (0..100).map(|i| AsyncPacket::new(i, 10)).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for packet in &packets {
                encoder.write_item(packet).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<AsyncPacket> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(packets.len(), decoded.len());
        assert_eq!(packets, decoded);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 14: Large payload streaming (> 1 MB)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_streaming_large_payload() {
        // 10 packets of 100 KB each -> 1 MB+ total
        let packets: Vec<AsyncPacket> = (0..10).map(|i| AsyncPacket::new(i, 100_000)).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            // Use a 64 KB chunk size to force multi-chunk operation
            let config = StreamingConfig::new().with_chunk_size(64 * 1024);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for packet in &packets {
                encoder.write_item(packet).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<AsyncPacket> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(packets, decoded);
        // With 10 packets of 100 KB and a 64 KB chunk, we must have multiple chunks
        assert!(
            decoder.progress().chunks_processed > 1,
            "expected multiple chunks for large data, got {}",
            decoder.progress().chunks_processed
        );
    }

    // -----------------------------------------------------------------------
    // Test 15: write_all convenience method with heterogeneous item count
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_write_all_packets() {
        let packets: Vec<AsyncPacket> = (0..50).map(|i| AsyncPacket::new(i, 20)).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_all(packets.clone())
                .await
                .expect("write_all failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<AsyncPacket> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(packets, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 16: Encoder progress matches items written (via encoder.progress())
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encoder_progress() {
        const N: u32 = 200;
        let config = StreamingConfig::new().with_chunk_size(1024);
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            encoder.set_estimated_total(N as u64);
            for i in 0..N {
                encoder.write_item(&i).await.expect("write failed");
            }
            // Before finish, flushed chunks are tracked; unflushed remainder is not yet counted
            encoder.finish().await.expect("finish failed");
        }

        // Verify via decoder that exactly N items were encoded
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().await.expect("read_all failed");
        assert_eq!(decoded.len(), N as usize);
        assert_eq!(decoder.progress().items_processed, N as u64);
    }

    // -----------------------------------------------------------------------
    // Test 17: CancellableAsyncEncoder write_all stops mid-stream on cancel
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_cancellable_encoder_write_all_stops_on_cancel() {
        let token = CancellationToken::new();
        let child = token.child();

        let mut buffer = Vec::<u8>::new();
        let cursor = Cursor::new(&mut buffer);
        let mut encoder = CancellableAsyncEncoder::new(cursor, child);

        // Write a few items before cancelling
        encoder
            .write_item(&1u64)
            .await
            .expect("write 1 should succeed");
        encoder
            .write_item(&2u64)
            .await
            .expect("write 2 should succeed");

        token.cancel();

        // After cancel, write must fail
        let result = encoder.write_item(&3u64).await;
        assert!(
            result.is_err(),
            "write after cancellation must return an error"
        );
    }

    // -----------------------------------------------------------------------
    // Test 18: CancellableAsyncDecoder read_all with no cancel returns all
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_cancellable_decoder_full_stream() {
        let values: Vec<u64> = (0..500).collect();
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let token = CancellationToken::new();
        let cursor = Cursor::new(buffer);
        let mut decoder = CancellableAsyncDecoder::new(cursor, token);
        let decoded: Vec<u64> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(values, decoded);
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 19: Nested struct with optional fields round-trips correctly
    // -----------------------------------------------------------------------
    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct NestedPacket {
        outer_id: u32,
        inner: AsyncPacket,
    }

    #[tokio::test]
    async fn test_async_nested_struct_roundtrip() {
        let items: Vec<NestedPacket> = (0..30)
            .map(|i| NestedPacket {
                outer_id: i,
                inner: AsyncPacket::new(i as u64, 5),
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
        let decoded: Vec<NestedPacket> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(items, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 20: Streaming strings – variable-length items
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_streaming_variable_length_strings() {
        let strings: Vec<String> = (0u32..200)
            .map(|i| {
                // Each string has a different length (0..200 chars)
                "x".repeat(i as usize)
            })
            .collect();

        let config = StreamingConfig::new().with_chunk_size(2048);
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for s in &strings {
                encoder.write_item(s).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<String> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(strings, decoded);
        assert!(
            decoder.progress().chunks_processed >= 1,
            "should have processed at least one chunk"
        );
    }

    // -----------------------------------------------------------------------
    // Test 21: Empty encoder produces a valid (zero-item) stream
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_empty_encoder() {
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let encoder: AsyncStreamingEncoder<_> = AsyncStreamingEncoder::new(cursor);
            encoder
                .finish()
                .await
                .expect("finish of empty encoder failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder
            .read_all()
            .await
            .expect("read_all on empty stream failed");
        assert!(decoded.is_empty());
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 22: Decoder get_ref returns reader reference
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decoder_get_ref() {
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&999u32).await.expect("write failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let decoder = AsyncStreamingDecoder::new(cursor);
        // get_ref must compile and return a reference without consuming the decoder
        let _: &Cursor<Vec<u8>> = decoder.get_ref();
    }

    // -----------------------------------------------------------------------
    // Test 23: Cancellation token – multiple children all see the cancel
    // -----------------------------------------------------------------------
    #[test]
    fn test_cancellation_token_multiple_children() {
        let parent = CancellationToken::new();
        let child_a = parent.child();
        let child_b = parent.child();
        let child_c = child_a.child(); // grandchild

        assert!(!child_a.is_cancelled());
        assert!(!child_b.is_cancelled());
        assert!(!child_c.is_cancelled());

        parent.cancel();

        assert!(child_a.is_cancelled());
        assert!(child_b.is_cancelled());
        assert!(child_c.is_cancelled());
    }

    // -----------------------------------------------------------------------
    // Test 24: Flush-per-item mode produces one chunk per item
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_flush_per_item_one_chunk_per_item() {
        const N: usize = 15;
        let config = StreamingConfig::new().with_flush_per_item(true);
        let values: Vec<u32> = (0..N as u32).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for &v in &values {
                encoder.write_item(&v).await.expect("write failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(values, decoded);
        assert_eq!(
            decoder.progress().chunks_processed,
            N as u64,
            "expected exactly one chunk per item in flush_per_item mode"
        );
    }
}

// ---------------------------------------------------------------------------
// More async tests: sequential multi-encode, large payload, error resilience
// ---------------------------------------------------------------------------

#[cfg(feature = "async-tokio")]
mod more_async_tests {
    use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder, StreamingConfig};
    use oxicode::{Decode, Encode};
    use std::io::Cursor;

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct Config {
        timeout_ms: u64,
        max_retries: u8,
        endpoint: String,
    }

    // -----------------------------------------------------------------------
    // Test 25: Encode each Config separately; decode each back individually.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_sequential_multi_encode_async() {
        let configs = vec![
            Config {
                timeout_ms: 5000,
                max_retries: 3,
                endpoint: "/api/v1".to_string(),
            },
            Config {
                timeout_ms: 1000,
                max_retries: 1,
                endpoint: "/health".to_string(),
            },
            Config {
                timeout_ms: 30000,
                max_retries: 5,
                endpoint: "/api/v2/stream".to_string(),
            },
        ];

        for cfg in &configs {
            let mut buf = Vec::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                encoder.write_item(cfg).await.expect("encode");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncStreamingDecoder::new(cursor);
            let decoded: Option<Config> = decoder.read_item().await.expect("decode");
            assert_eq!(Some(cfg.clone()), decoded);
        }
    }

    // -----------------------------------------------------------------------
    // Test 26: All configs in one stream, decoded sequentially.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_sequential_multi_encode_single_stream() {
        let configs = vec![
            Config {
                timeout_ms: 5000,
                max_retries: 3,
                endpoint: "/api/v1".to_string(),
            },
            Config {
                timeout_ms: 1000,
                max_retries: 1,
                endpoint: "/health".to_string(),
            },
            Config {
                timeout_ms: 30000,
                max_retries: 5,
                endpoint: "/api/v2/stream".to_string(),
            },
        ];

        let mut buf = Vec::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for cfg in &configs {
                encoder.write_item(cfg).await.expect("encode");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        for expected in &configs {
            let decoded: Option<Config> = decoder.read_item().await.expect("decode");
            assert_eq!(Some(expected.clone()), decoded);
        }
        let eof: Option<Config> = decoder.read_item().await.expect("eof");
        assert!(eof.is_none());
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 27: Large payload (100 KB Vec<u8>) exercises internal buffering.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_large_payload() {
        let data: Vec<u8> = (0..=255u8).cycle().take(100_000).collect();

        let mut buf = Vec::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&data).await.expect("encode");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<Vec<u8>> = decoder.read_item().await.expect("decode");
        assert_eq!(Some(data), decoded);
    }

    // -----------------------------------------------------------------------
    // Test 28: Large payload with small chunk size forces multiple flushes.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_large_payload_small_chunks() {
        let data: Vec<u8> = (0..=255u8).cycle().take(50_000).collect();
        let config = StreamingConfig::new().with_chunk_size(4096);

        let mut buf = Vec::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for chunk in data.chunks(10_000) {
                encoder
                    .write_item(&chunk.to_vec())
                    .await
                    .expect("encode chunk");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<Vec<u8>> = decoder.read_all().await.expect("decode all");
        let flat: Vec<u8> = decoded.into_iter().flatten().collect();
        assert_eq!(data, flat);
        assert!(
            decoder.progress().chunks_processed > 1,
            "expected multiple chunks, got {}",
            decoder.progress().chunks_processed
        );
    }

    // -----------------------------------------------------------------------
    // Test 29: u8 boundary values roundtrip correctly.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_u8_boundary_values() {
        let values: Vec<u8> = vec![0, 1, 127, 128, 254, 255];

        let mut buf = Vec::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("encode");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u8> = decoder.read_all().await.expect("decode");
        assert_eq!(values, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 30: Empty string and long string roundtrip.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_string_edge_cases() {
        let strings: Vec<String> = vec![
            String::new(),
            "a".to_string(),
            "hello world".to_string(),
            "x".repeat(10_000),
        ];

        let mut buf = Vec::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for s in &strings {
                encoder.write_item(s).await.expect("encode");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<String> = decoder.read_all().await.expect("decode");
        assert_eq!(strings, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 31: bytes_processed counter is nonzero after encoding items.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_bytes_processed_nonzero() {
        let mut buf = Vec::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for i in 0u32..50 {
                encoder.write_item(&i).await.expect("encode");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let _: Vec<u32> = decoder.read_all().await.expect("decode");
        assert!(
            decoder.progress().bytes_processed > 0,
            "bytes_processed should be > 0"
        );
        assert_eq!(decoder.progress().items_processed, 50);
    }

    // -----------------------------------------------------------------------
    // Test 32: Tuple roundtrip via async streaming.
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_tuple_roundtrip() {
        let items: Vec<(u32, String, bool)> = (0u32..20)
            .map(|i| (i, format!("item-{i}"), i % 2 == 0))
            .collect();

        let mut buf = Vec::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for item in &items {
                encoder.write_item(item).await.expect("encode");
            }
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<(u32, String, bool)> = decoder.read_all().await.expect("decode");
        assert_eq!(items, decoded);
    }
}
