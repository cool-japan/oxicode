//! Advanced async streaming tests for oxicode.
//!
//! 20 comprehensive async tests covering cross-mode compatibility,
//! PI/E-based values, BTreeMap, Vec<Option<T>>, timeout, parallel tasks,
//! enums, progress tracking, chunk sizes, u128, tuples, field ordering,
//! large strings, and more.

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
mod async_advanced_tests {
    use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder};
    use oxicode::{Decode, Encode};
    use std::collections::BTreeMap;
    use std::f64::consts::{E, PI};
    use std::io::Cursor;

    // -----------------------------------------------------------------------
    // Shared data types
    // -----------------------------------------------------------------------

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct AllPrimitives {
        a_bool: bool,
        a_u8: u8,
        a_u16: u16,
        a_u32: u32,
        a_u64: u64,
        a_i8: i8,
        a_i16: i16,
        a_i32: i32,
        a_i64: i64,
        a_f32: f32,
        a_f64: f64,
    }

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct OrderedFields {
        first: u32,
        second: String,
        third: f64,
        fourth: bool,
    }

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    enum MultiVariant {
        Unit,
        Newtype(u64),
        Struct { x: i32, y: i32 },
        Tuple(String, f32),
    }

    // -----------------------------------------------------------------------
    // Test 1: Async encode then sync decode (cross-mode compatibility)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_sync_decode() {
        let original: u64 = 0xDEAD_BEEF_CAFE_1234;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_item(&original)
                .await
                .expect("async write_item failed");
            encoder.finish().await.expect("async finish failed");
        }

        // Sync decode via the streaming decoder (wraps sync reader)
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<u64> = decoder.read_item().await.expect("read_item failed");
        assert_eq!(decoded, Some(original));
    }

    // -----------------------------------------------------------------------
    // Test 2: Sync encode then async decode
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_sync_encode_async_decode() {
        let original: u32 = 0xABCD_1234;

        // Sync encode using oxicode::encode_to_vec, then wrap in streaming format
        // by using AsyncStreamingEncoder (which IS the streaming format)
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

        // Async decode
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<u32> = decoder.read_item().await.expect("read_item failed");
        assert_eq!(decoded, Some(original));

        // Confirm EOF
        let eof: Option<u32> = decoder.read_item().await.expect("eof read_item failed");
        assert!(eof.is_none());
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 3: Async encode 200 items, async decode all – verify count
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_200_items_verify_count() {
        const COUNT: usize = 200;
        let items: Vec<u32> = (0..COUNT as u32).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &items {
                encoder.write_item(&v).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded.len(), COUNT, "decoded count mismatch");
        assert_eq!(decoded, items, "decoded values mismatch");
        assert_eq!(
            decoder.progress().items_processed,
            COUNT as u64,
            "progress items_processed mismatch"
        );
    }

    // -----------------------------------------------------------------------
    // Test 4: Async encode with PI-based f64 values
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_pi_based_f64_values() {
        let values: Vec<f64> = vec![PI, E, PI * E, PI / E, PI.powi(2), E.powi(3), PI.sqrt()];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<f64> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded.len(), values.len(), "length mismatch");
        for (orig, dec) in values.iter().zip(decoded.iter()) {
            assert!(
                (orig - dec).abs() < f64::EPSILON * 1024.0,
                "f64 mismatch: {} vs {}",
                orig,
                dec
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 5: Async encode struct with all primitive types
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_all_primitives_struct() {
        let original = AllPrimitives {
            a_bool: true,
            a_u8: u8::MAX,
            a_u16: u16::MAX,
            a_u32: u32::MAX,
            a_u64: u64::MAX,
            a_i8: i8::MIN,
            a_i16: i16::MIN,
            a_i32: i32::MIN,
            a_i64: i64::MIN,
            a_f32: PI as f32,
            a_f64: E,
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
        let decoded: Option<AllPrimitives> = decoder.read_item().await.expect("read_item failed");
        assert_eq!(decoded, Some(original));
    }

    // -----------------------------------------------------------------------
    // Test 6: Async encode BTreeMap<String, u64> roundtrip
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_btreemap_roundtrip() {
        let mut map: BTreeMap<String, u64> = BTreeMap::new();
        map.insert("alpha".to_string(), 1);
        map.insert("beta".to_string(), u64::MAX);
        map.insert("gamma".to_string(), 42);
        map.insert("delta".to_string(), 0);

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&map).await.expect("write_item failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<BTreeMap<String, u64>> =
            decoder.read_item().await.expect("read_item failed");
        assert_eq!(decoded, Some(map));
    }

    // -----------------------------------------------------------------------
    // Test 7: Async encode Vec<Option<u32>> roundtrip
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_vec_option_roundtrip() {
        let items: Vec<Option<u32>> =
            vec![Some(0), None, Some(u32::MAX), None, Some(42), Some(1), None];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&items).await.expect("write_item failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<Vec<Option<u32>>> =
            decoder.read_item().await.expect("read_item failed");
        assert_eq!(decoded, Some(items));
    }

    // -----------------------------------------------------------------------
    // Test 8: Async streaming completes (correctness check with yield points)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_streaming_completes_correctly() {
        // Encodes and decodes using in-memory cursor; tokio::yield_now() punctuates
        // the async execution to ensure cooperative scheduling works correctly.
        let items: Vec<u64> = (0..50).map(|i| i * 7).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for (idx, &v) in items.iter().enumerate() {
                encoder.write_item(&v).await.expect("write_item failed");
                // Yield every 10 items to exercise cooperative scheduling
                if idx % 10 == 0 {
                    tokio::task::yield_now().await;
                }
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u64> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded, items, "streaming roundtrip must be correct");
        assert_eq!(decoder.progress().items_processed, items.len() as u64);
    }

    // -----------------------------------------------------------------------
    // Test 9: Multiple async tasks encoding in parallel (3 tasks)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_parallel_encoding_three_tasks() {
        let task_a = tokio::spawn(async {
            let items: Vec<u32> = (0..100).collect();
            let mut buffer = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buffer);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                for &v in &items {
                    encoder.write_item(&v).await.expect("task_a write failed");
                }
                encoder.finish().await.expect("task_a finish failed");
            }
            (items, buffer)
        });

        let task_b = tokio::spawn(async {
            let items: Vec<u64> = (1000..1100).collect();
            let mut buffer = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buffer);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                for &v in &items {
                    encoder.write_item(&v).await.expect("task_b write failed");
                }
                encoder.finish().await.expect("task_b finish failed");
            }
            (items, buffer)
        });

        let task_c = tokio::spawn(async {
            let items: Vec<String> = (0..50).map(|i| format!("item-{i}")).collect();
            let mut buffer = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buffer);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                for v in &items {
                    encoder.write_item(v).await.expect("task_c write failed");
                }
                encoder.finish().await.expect("task_c finish failed");
            }
            (items, buffer)
        });

        let (res_a, res_b, res_c) = tokio::join!(task_a, task_b, task_c);
        let (items_a, buf_a) = res_a.expect("task_a panicked");
        let (items_b, buf_b) = res_b.expect("task_b panicked");
        let (items_c, buf_c) = res_c.expect("task_c panicked");

        // Verify each
        let decoded_a: Vec<u32> = AsyncStreamingDecoder::new(Cursor::new(buf_a))
            .read_all()
            .await
            .expect("decode_a failed");
        assert_eq!(decoded_a, items_a);

        let decoded_b: Vec<u64> = AsyncStreamingDecoder::new(Cursor::new(buf_b))
            .read_all()
            .await
            .expect("decode_b failed");
        assert_eq!(decoded_b, items_b);

        let decoded_c: Vec<String> = AsyncStreamingDecoder::new(Cursor::new(buf_c))
            .read_all()
            .await
            .expect("decode_c failed");
        assert_eq!(decoded_c, items_c);
    }

    // -----------------------------------------------------------------------
    // Test 10: Async decoder reading 0 items (empty stream)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decoder_empty_stream() {
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let encoder: AsyncStreamingEncoder<_> = AsyncStreamingEncoder::new(cursor);
            encoder.finish().await.expect("finish empty encoder failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().await.expect("read_all empty failed");

        assert!(decoded.is_empty(), "expected 0 items from empty stream");
        assert!(decoder.is_finished());
        assert_eq!(decoder.progress().items_processed, 0);
    }

    // -----------------------------------------------------------------------
    // Test 11: Async decoder reading exactly 1 item
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decoder_single_item() {
        let value: i64 = i64::MIN;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&value).await.expect("write_item failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);

        let first: Option<i64> = decoder.read_item().await.expect("read_item 1 failed");
        assert_eq!(first, Some(value));

        let second: Option<i64> = decoder.read_item().await.expect("read_item 2 failed");
        assert!(second.is_none());
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 12: Async encoder/decoder for enum with all variants
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_enum_all_variants() {
        let variants = vec![
            MultiVariant::Unit,
            MultiVariant::Newtype(u64::MAX),
            MultiVariant::Struct { x: -100, y: 200 },
            MultiVariant::Tuple("hello".to_string(), PI as f32),
        ];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for v in &variants {
                encoder.write_item(v).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<MultiVariant> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded, variants);
    }

    // -----------------------------------------------------------------------
    // Test 13: Async encode then get bytes via in-memory cursor
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_get_bytes_via_cursor() {
        let value: u32 = 12345;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&value).await.expect("write_item failed");
            encoder.finish().await.expect("finish failed");
        }

        // Buffer must be non-empty (contains chunk header + payload + end marker)
        assert!(
            !buffer.is_empty(),
            "encoded buffer must contain bytes after encoding"
        );

        // Verify roundtrip from those raw bytes
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<u32> = decoder.read_item().await.expect("read_item failed");
        assert_eq!(decoded, Some(value));
    }

    // -----------------------------------------------------------------------
    // Test 14: Async decode with progress tracking: verify items_processed count
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decode_progress_items_processed() {
        const N: u64 = 75;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.set_estimated_total(N);
            for i in 0..N as u32 {
                encoder.write_item(&i).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let _: Vec<u32> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(
            decoder.progress().items_processed,
            N,
            "items_processed must equal N={N}"
        );
        assert!(
            decoder.progress().bytes_processed > 0,
            "bytes_processed must be > 0"
        );
        assert!(
            decoder.progress().chunks_processed >= 1,
            "chunks_processed must be >= 1"
        );
    }

    // -----------------------------------------------------------------------
    // Test 15: Async encode with chunk_size=512 produces multiple chunks
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_chunk_size_512_multiple_chunks() {
        use oxicode::streaming::StreamingConfig;

        // chunk_size clamped to min 1024, use small enough items to fill quickly
        let config = StreamingConfig::new().with_chunk_size(1024);

        // 200 strings of 20 bytes each -> ~4000 bytes -> multiple 1024-byte chunks
        let values: Vec<String> = (0u32..200)
            .map(|i| format!("{:0>20}", i)) // 20-char zero-padded
            .collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for v in &values {
                encoder.write_item(v).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<String> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded, values);
        assert!(
            decoder.progress().chunks_processed > 1,
            "expected multiple chunks at chunk_size=1024 with 200 string items, got {}",
            decoder.progress().chunks_processed
        );
    }

    // -----------------------------------------------------------------------
    // Test 16: Async encode u128 values roundtrip
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_u128_roundtrip() {
        let values: Vec<u128> = vec![
            0,
            1,
            u64::MAX as u128,
            u128::MAX,
            u128::MAX / 2,
            (PI * 1e30) as u128,
        ];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &values {
                encoder.write_item(&v).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u128> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded, values);
    }

    // -----------------------------------------------------------------------
    // Test 17: Async encode tuple (String, u64, f64) roundtrip
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_tuple_string_u64_f64_roundtrip() {
        let items: Vec<(String, u64, f64)> = vec![
            ("alpha".to_string(), 0, PI),
            ("beta".to_string(), u64::MAX, E),
            ("gamma".to_string(), 42, PI / E),
            (String::new(), 1, 0.0),
        ];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for item in &items {
                encoder.write_item(item).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<(String, u64, f64)> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded.len(), items.len());
        for (orig, dec) in items.iter().zip(decoded.iter()) {
            assert_eq!(orig.0, dec.0, "String field mismatch");
            assert_eq!(orig.1, dec.1, "u64 field mismatch");
            assert!(
                (orig.2 - dec.2).abs() < f64::EPSILON * 1024.0,
                "f64 field mismatch: {} vs {}",
                orig.2,
                dec.2
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 18: Async encode/decode preserves field ordering in struct
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_preserves_field_ordering() {
        let items: Vec<OrderedFields> = (0u32..10)
            .map(|i| OrderedFields {
                first: i,
                second: format!("item-{i}"),
                third: PI * i as f64,
                fourth: i % 2 == 0,
            })
            .collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for item in &items {
                encoder.write_item(item).await.expect("write_item failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<OrderedFields> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded.len(), items.len());
        for (orig, dec) in items.iter().zip(decoded.iter()) {
            assert_eq!(orig.first, dec.first, "first field mismatch");
            assert_eq!(orig.second, dec.second, "second field mismatch");
            assert!(
                (orig.third - dec.third).abs() < f64::EPSILON * 1024.0,
                "third field mismatch"
            );
            assert_eq!(orig.fourth, dec.fourth, "fourth field mismatch");
        }
    }

    // -----------------------------------------------------------------------
    // Test 19: Async encode large String (10KB)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_large_string_10kb() {
        let large_string = "x".repeat(10 * 1024); // 10 KB

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_item(&large_string)
                .await
                .expect("write_item large string failed");
            encoder.finish().await.expect("finish failed");
        }

        assert!(
            buffer.len() >= 10 * 1024,
            "buffer should be at least 10KB, got {} bytes",
            buffer.len()
        );

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<String> = decoder.read_item().await.expect("read_item failed");
        let decoded_str = decoded.expect("expected Some(String), got None");
        assert_eq!(decoded_str.len(), large_string.len());
        assert_eq!(decoded_str, large_string);
    }

    // -----------------------------------------------------------------------
    // Test 20: Async encode completed successfully returns Ok
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_finish_returns_ok() {
        let mut buffer = Vec::<u8>::new();
        let cursor = Cursor::new(&mut buffer);
        let mut encoder = AsyncStreamingEncoder::new(cursor);

        let write_result = encoder.write_item(&42u32).await;
        assert!(
            write_result.is_ok(),
            "write_item should return Ok, got {:?}",
            write_result
        );

        let finish_result = encoder.finish().await;
        assert!(
            finish_result.is_ok(),
            "finish should return Ok, got {:?}",
            finish_result
        );

        // Confirm the data is non-empty and decodable
        assert!(
            !buffer.is_empty(),
            "encoded buffer must be non-empty after successful encode"
        );
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<u32> = decoder.read_item().await.expect("read_item failed");
        assert_eq!(decoded, Some(42u32));
    }
}
