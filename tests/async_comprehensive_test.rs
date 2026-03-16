//! Comprehensive async streaming tests for oxicode.
//!
//! 20 tests covering a wide range of async encoding/decoding scenarios,
//! including single values, collections, structs, file I/O, concurrent tasks,
//! timeouts, HashMap, identity roundtrips, sequential decoding, and large data.
//!
//! All tests are gated behind the `async-tokio` feature.

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
mod async_comprehensive {
    use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder};
    use oxicode::{Decode, Encode};
    use std::collections::HashMap;
    use std::io::Cursor;

    // -----------------------------------------------------------------------
    // Shared data types
    // -----------------------------------------------------------------------

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct LargeRecord {
        id: u64,
        name: String,
        tags: Vec<String>,
        scores: Vec<f64>,
        active: bool,
    }

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct PrimitiveBundle {
        flag: bool,
        ch: char,
        byte: u8,
        signed: i32,
        unsigned: u64,
        float32: f32,
        float64: f64,
        short_str: String,
    }

    // -----------------------------------------------------------------------
    // Test 1: Async encode single u32, decode back
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_single_u32_decode_back() {
        let value: u32 = 0xCAFE_BABE;

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_item(&value)
                .await
                .expect("write_item single u32 failed");
            encoder.finish().await.expect("finish failed");
        }

        assert!(
            !buffer.is_empty(),
            "encoded buffer must not be empty for a single u32"
        );

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<u32> = decoder
            .read_item()
            .await
            .expect("read_item single u32 failed");
        assert_eq!(decoded, Some(value), "decoded u32 must equal original");

        // Confirm nothing more to read
        let eof: Option<u32> = decoder.read_item().await.expect("eof check failed");
        assert!(eof.is_none(), "stream must be exhausted after single item");
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 2: Async encode Vec<String> 10 items
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_vec_string_ten_items() {
        let items: Vec<String> = (0..10).map(|i| format!("string-item-{i:02}")).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for s in &items {
                encoder
                    .write_item(s)
                    .await
                    .expect("write_item string failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<String> = decoder.read_all().await.expect("read_all strings failed");

        assert_eq!(decoded.len(), 10, "must decode exactly 10 strings");
        assert_eq!(decoded, items, "decoded strings must match originals");
    }

    // -----------------------------------------------------------------------
    // Test 3: Async encode/decode large struct
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_decode_large_struct() {
        let record = LargeRecord {
            id: u64::MAX / 3,
            name: "LargeRecordTest".repeat(5),
            tags: (0..50).map(|i| format!("label-{i:04}")).collect(),
            scores: (0..50).map(|i| i as f64 * 1.23456789).collect(),
            active: true,
        };

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_item(&record)
                .await
                .expect("write_item large record failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<LargeRecord> = decoder
            .read_item()
            .await
            .expect("read_item large record failed");

        assert_eq!(decoded, Some(record), "large record roundtrip must match");
    }

    // -----------------------------------------------------------------------
    // Test 4: Async encode 100 items in a loop
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_100_items_loop() {
        const N: usize = 100;
        let items: Vec<i64> = (0..N as i64).map(|i| i * i - 5000).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &items {
                encoder.write_item(&v).await.expect("write_item i64 failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<i64> = decoder.read_all().await.expect("read_all i64 failed");

        assert_eq!(decoded.len(), N, "must decode exactly 100 items");
        assert_eq!(decoded, items, "decoded i64 values must match originals");
    }

    // -----------------------------------------------------------------------
    // Test 5: Async decode 100 items in a loop (item-by-item)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decode_100_items_loop() {
        const N: u32 = 100;
        let items: Vec<u32> = (500..500 + N).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &items {
                encoder.write_item(&v).await.expect("write_item u32 failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);

        let mut decoded_items = Vec::with_capacity(N as usize);
        while let Some(item) = decoder
            .read_item::<u32>()
            .await
            .expect("read_item loop failed")
        {
            decoded_items.push(item);
        }

        assert_eq!(
            decoded_items.len(),
            N as usize,
            "loop must decode exactly 100 items"
        );
        assert_eq!(
            decoded_items, items,
            "loop-decoded values must match originals"
        );
        assert!(decoder.is_finished());
    }

    // -----------------------------------------------------------------------
    // Test 6: Async with tokio::fs file write/read
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_tokio_fs_file_write_read() {
        use tokio::fs;

        let dir = std::env::temp_dir();
        let path = dir.join("oxicode_async_comprehensive_test6.bin");

        let items: Vec<u64> = (0u64..30).map(|i| i * 100 + 7).collect();

        // Write to file using tokio::fs
        {
            let file = fs::File::create(&path)
                .await
                .expect("tokio file create failed");
            let mut encoder = AsyncStreamingEncoder::new(file);
            for &v in &items {
                encoder
                    .write_item(&v)
                    .await
                    .expect("write_item to file failed");
            }
            encoder.finish().await.expect("finish to file failed");
        }

        // Read back from file using tokio::fs
        {
            let file = fs::File::open(&path).await.expect("tokio file open failed");
            let mut decoder = AsyncStreamingDecoder::new(file);
            let decoded: Vec<u64> = decoder.read_all().await.expect("read_all from file failed");

            assert_eq!(decoded, items, "file roundtrip must match");
        }

        // Cleanup
        fs::remove_file(&path).await.ok();
    }

    // -----------------------------------------------------------------------
    // Test 7: Async concurrent encode tasks (tokio::spawn 3 tasks)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_concurrent_encode_three_tasks() {
        let handle_a = tokio::spawn(async {
            let data: Vec<u16> = (0u16..128).collect();
            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                for &v in &data {
                    encoder.write_item(&v).await.expect("task_a write failed");
                }
                encoder.finish().await.expect("task_a finish failed");
            }
            (data, buf)
        });

        let handle_b = tokio::spawn(async {
            let data: Vec<i16> = (-64i16..64).collect();
            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                for &v in &data {
                    encoder.write_item(&v).await.expect("task_b write failed");
                }
                encoder.finish().await.expect("task_b finish failed");
            }
            (data, buf)
        });

        let handle_c = tokio::spawn(async {
            let data: Vec<f32> = (0..32).map(|i| i as f32 * 0.1_f32).collect();
            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                for &v in &data {
                    encoder.write_item(&v).await.expect("task_c write failed");
                }
                encoder.finish().await.expect("task_c finish failed");
            }
            (data, buf)
        });

        let (res_a, res_b, res_c) = tokio::join!(handle_a, handle_b, handle_c);
        let (data_a, buf_a) = res_a.expect("task_a panicked");
        let (data_b, buf_b) = res_b.expect("task_b panicked");
        let (data_c, buf_c) = res_c.expect("task_c panicked");

        let decoded_a: Vec<u16> = AsyncStreamingDecoder::new(Cursor::new(buf_a))
            .read_all()
            .await
            .expect("decode task_a failed");
        assert_eq!(decoded_a, data_a, "task_a roundtrip must match");

        let decoded_b: Vec<i16> = AsyncStreamingDecoder::new(Cursor::new(buf_b))
            .read_all()
            .await
            .expect("decode task_b failed");
        assert_eq!(decoded_b, data_b, "task_b roundtrip must match");

        let decoded_c: Vec<f32> = AsyncStreamingDecoder::new(Cursor::new(buf_c))
            .read_all()
            .await
            .expect("decode task_c failed");
        assert_eq!(decoded_c.len(), data_c.len(), "task_c length must match");
        for (orig, dec) in data_c.iter().zip(decoded_c.iter()) {
            assert!(
                (orig - dec).abs() < f32::EPSILON * 1024.0,
                "task_c f32 mismatch: {orig} vs {dec}"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 8: Async encode then decode in separate tasks
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_then_decode_separate_tasks() {
        let items: Vec<u32> = (0..60).map(|i| i * 3 + 1).collect();
        let items_clone = items.clone();

        // Encode in a spawned task
        let encode_handle = tokio::spawn(async move {
            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                for &v in &items_clone {
                    encoder
                        .write_item(&v)
                        .await
                        .expect("encode task write failed");
                }
                encoder.finish().await.expect("encode task finish failed");
            }
            buf
        });

        let buf = encode_handle.await.expect("encode task panicked");

        // Decode in another spawned task
        let decode_handle = tokio::spawn(async move {
            let cursor = Cursor::new(buf);
            let mut decoder = AsyncStreamingDecoder::new(cursor);
            decoder.read_all::<u32>().await.expect("decode task failed")
        });

        let decoded = decode_handle.await.expect("decode task panicked");
        assert_eq!(decoded, items, "separate-task encode/decode must match");
    }

    // -----------------------------------------------------------------------
    // Test 9: Async encode empty (zero items)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_empty_stream() {
        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            // Finish immediately — no items written
            let encoder: AsyncStreamingEncoder<_> = AsyncStreamingEncoder::new(cursor);
            encoder
                .finish()
                .await
                .expect("finish on empty encoder failed");
        }

        // Buffer must still be non-empty: the end-marker chunk is always written
        assert!(
            !buffer.is_empty(),
            "even an empty stream must write the end marker"
        );

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder
            .read_all()
            .await
            .expect("read_all on empty stream failed");

        assert!(decoded.is_empty(), "empty stream must yield zero items");
        assert!(decoder.is_finished());
        assert_eq!(
            decoder.progress().items_processed,
            0,
            "items_processed must be 0 for empty stream"
        );
    }

    // -----------------------------------------------------------------------
    // Test 10: Async encode None/Some variants
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_none_some_variants() {
        let values: Vec<Option<u64>> = vec![
            None,
            Some(0),
            Some(u64::MAX),
            None,
            Some(42),
            None,
            Some(1_000_000),
        ];

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for v in &values {
                encoder
                    .write_item(v)
                    .await
                    .expect("write_item Option<u64> failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<Option<u64>> = decoder
            .read_all()
            .await
            .expect("read_all Option<u64> failed");

        assert_eq!(decoded, values, "Option<u64> roundtrip must match");
    }

    // -----------------------------------------------------------------------
    // Test 11: Async encode bool/char/f64
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_bool_char_f64() {
        let booleans: Vec<bool> = vec![true, false, false, true, true];
        let chars: Vec<char> = vec!['A', 'z', '0', '\n', '€', '中'];
        let floats: Vec<f64> = vec![
            0.0,
            -0.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
            1.0_f64 / 3.0,
            std::f64::consts::TAU,
        ];

        let mut buf_bool = Vec::<u8>::new();
        let mut buf_char = Vec::<u8>::new();
        let mut buf_f64 = Vec::<u8>::new();

        {
            let cursor = Cursor::new(&mut buf_bool);
            let mut enc = AsyncStreamingEncoder::new(cursor);
            for &b in &booleans {
                enc.write_item(&b).await.expect("bool write failed");
            }
            enc.finish().await.expect("bool finish failed");
        }
        {
            let cursor = Cursor::new(&mut buf_char);
            let mut enc = AsyncStreamingEncoder::new(cursor);
            for &c in &chars {
                enc.write_item(&c).await.expect("char write failed");
            }
            enc.finish().await.expect("char finish failed");
        }
        {
            let cursor = Cursor::new(&mut buf_f64);
            let mut enc = AsyncStreamingEncoder::new(cursor);
            for &f in &floats {
                enc.write_item(&f).await.expect("f64 write failed");
            }
            enc.finish().await.expect("f64 finish failed");
        }

        let decoded_bools: Vec<bool> = AsyncStreamingDecoder::new(Cursor::new(buf_bool))
            .read_all()
            .await
            .expect("bool decode failed");
        assert_eq!(decoded_bools, booleans, "bool roundtrip must match");

        let decoded_chars: Vec<char> = AsyncStreamingDecoder::new(Cursor::new(buf_char))
            .read_all()
            .await
            .expect("char decode failed");
        assert_eq!(decoded_chars, chars, "char roundtrip must match");

        let decoded_f64s: Vec<f64> = AsyncStreamingDecoder::new(Cursor::new(buf_f64))
            .read_all()
            .await
            .expect("f64 decode failed");
        assert_eq!(decoded_f64s.len(), floats.len(), "f64 count must match");
        for (orig, dec) in floats.iter().zip(decoded_f64s.iter()) {
            // Handle special float values
            if orig.is_nan() {
                assert!(dec.is_nan(), "NaN must roundtrip as NaN");
            } else if orig.is_infinite() {
                assert_eq!(orig.is_sign_positive(), dec.is_sign_positive());
                assert!(dec.is_infinite(), "Inf must roundtrip as Inf");
            } else {
                assert_eq!(orig.to_bits(), dec.to_bits(), "f64 bits must match");
            }
        }
    }

    // -----------------------------------------------------------------------
    // Test 12: Async streaming 1000 items
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_streaming_1000_items() {
        use oxicode::streaming::StreamingConfig;

        const N: u32 = 1000;
        let config = StreamingConfig::new().with_chunk_size(2048);
        let items: Vec<u32> = (0..N).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for &v in &items {
                encoder
                    .write_item(&v)
                    .await
                    .expect("write_item 1000 items failed");
            }
            encoder.finish().await.expect("finish 1000 items failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder
            .read_all()
            .await
            .expect("read_all 1000 items failed");

        assert_eq!(decoded.len(), N as usize, "must decode 1000 items");
        assert_eq!(decoded, items, "1000-item roundtrip must match");
        assert_eq!(
            decoder.progress().items_processed,
            N as u64,
            "items_processed must equal 1000"
        );
        assert!(
            decoder.progress().chunks_processed > 1,
            "1000 items at 2KB chunks must produce multiple chunks, got {}",
            decoder.progress().chunks_processed
        );
    }

    // -----------------------------------------------------------------------
    // Test 13: Async encode then decode with per-item verification
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_decode_per_item_verification() {
        let items: Vec<(u32, String)> = (0u32..20)
            .map(|i| (i, format!("value-{:04}", i * i)))
            .collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for item in &items {
                encoder
                    .write_item(item)
                    .await
                    .expect("write_item tuple failed");
            }
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);

        let mut idx = 0usize;
        while let Some((num, text)) = decoder
            .read_item::<(u32, String)>()
            .await
            .expect("read_item tuple failed")
        {
            assert_eq!(num, items[idx].0, "tuple.0 mismatch at index {idx}");
            assert_eq!(text, items[idx].1, "tuple.1 mismatch at index {idx}");
            idx += 1;
        }
        assert_eq!(idx, items.len(), "all items must be decoded");
    }

    // -----------------------------------------------------------------------
    // Test 14: Async with timeout (tokio::time::timeout)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_with_timeout() {
        use std::time::Duration;
        use tokio::time::timeout;

        let items: Vec<u32> = (0u32..25).collect();
        let mut buffer = Vec::<u8>::new();

        // Encode within timeout
        let encode_result = timeout(Duration::from_secs(5), async {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &items {
                encoder
                    .write_item(&v)
                    .await
                    .expect("write_item timeout test failed");
            }
            encoder.finish().await.expect("finish timeout test failed");
        })
        .await;

        assert!(
            encode_result.is_ok(),
            "encoding 25 items must not time out in 5 seconds"
        );

        // Decode within timeout
        let buffer_clone = buffer.clone();
        let decode_result = timeout(Duration::from_secs(5), async move {
            let cursor = Cursor::new(buffer_clone);
            let mut decoder = AsyncStreamingDecoder::new(cursor);
            decoder
                .read_all::<u32>()
                .await
                .expect("read_all timeout test failed")
        })
        .await;

        let decoded = decode_result.expect("decoding must not time out in 5 seconds");
        assert_eq!(decoded, items, "timeout-test roundtrip must match");
    }

    // -----------------------------------------------------------------------
    // Test 15: Async encode HashMap
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_hashmap() {
        let mut map: HashMap<u32, String> = HashMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());
        map.insert(100, "hundred".to_string());
        map.insert(u32::MAX, "max".to_string());

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_item(&map)
                .await
                .expect("write_item HashMap failed");
            encoder.finish().await.expect("finish HashMap failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<HashMap<u32, String>> =
            decoder.read_item().await.expect("read_item HashMap failed");

        let decoded_map = decoded.expect("expected Some(HashMap), got None");
        assert_eq!(decoded_map.len(), map.len(), "HashMap size must match");
        for (k, v) in &map {
            assert_eq!(
                decoded_map.get(k),
                Some(v),
                "HashMap value for key {k} must match"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 16: Async roundtrip identity (multiple types)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_roundtrip_identity() {
        // Helper macro to test roundtrip identity for a given type+value
        async fn roundtrip_check<T: Encode + Decode + PartialEq + std::fmt::Debug + Clone>(
            value: T,
        ) -> bool {
            let mut buffer = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buffer);
                let mut encoder = AsyncStreamingEncoder::new(cursor);
                encoder
                    .write_item(&value)
                    .await
                    .expect("write_item identity failed");
                encoder.finish().await.expect("finish identity failed");
            }
            let cursor = Cursor::new(buffer);
            let mut decoder = AsyncStreamingDecoder::new(cursor);
            let decoded: Option<T> = decoder
                .read_item()
                .await
                .expect("read_item identity failed");
            decoded == Some(value)
        }

        assert!(
            roundtrip_check(0u8).await,
            "u8 identity roundtrip must hold"
        );
        assert!(
            roundtrip_check(u8::MAX).await,
            "u8::MAX identity roundtrip must hold"
        );
        assert!(
            roundtrip_check(0i8).await,
            "i8 identity roundtrip must hold"
        );
        assert!(
            roundtrip_check(i8::MIN).await,
            "i8::MIN identity roundtrip must hold"
        );
        assert!(
            roundtrip_check(u32::MAX).await,
            "u32::MAX identity roundtrip must hold"
        );
        assert!(
            roundtrip_check(i64::MIN).await,
            "i64::MIN identity roundtrip must hold"
        );
        assert!(
            roundtrip_check("hello world".to_string()).await,
            "String identity roundtrip must hold"
        );
        assert!(
            roundtrip_check(vec![1u8, 2, 3, 255]).await,
            "Vec<u8> identity roundtrip must hold"
        );
    }

    // -----------------------------------------------------------------------
    // Test 17: Async encode to Vec<u8> backing store (write_all API)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_to_vec_backing() {
        let items: Vec<u64> = (1000u64..1050).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            // Use write_all for a batch
            encoder
                .write_all(items.iter().copied())
                .await
                .expect("write_all failed");
            encoder.finish().await.expect("finish write_all failed");
        }

        // The buffer must contain the items plus at least the end-marker chunk header,
        // so it is always non-empty.  We don't assert on exact byte count here because
        // varint encoding may produce fewer bytes per item than the in-memory size.
        assert!(
            !buffer.is_empty(),
            "encoded buffer must be non-empty after write_all"
        );

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<u64> = decoder
            .read_all()
            .await
            .expect("read_all write_all test failed");

        assert_eq!(decoded, items, "write_all roundtrip must match");
    }

    // -----------------------------------------------------------------------
    // Test 18: Async decode sequential (read_item one by one, count manually)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_decode_sequential() {
        const N: usize = 15;
        let items: Vec<i32> = (-7i32..).take(N).collect();

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in &items {
                encoder
                    .write_item(&v)
                    .await
                    .expect("sequential write failed");
            }
            encoder.finish().await.expect("sequential finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);

        let mut count = 0usize;
        let mut sum: i64 = 0;
        while let Some(v) = decoder
            .read_item::<i32>()
            .await
            .expect("sequential read failed")
        {
            sum += v as i64;
            count += 1;
        }

        let expected_sum: i64 = items.iter().map(|&x| x as i64).sum();
        assert_eq!(count, N, "sequential decode count must equal {N}");
        assert_eq!(sum, expected_sum, "sequential sum must match");
        assert_eq!(
            decoder.progress().items_processed,
            N as u64,
            "progress items_processed must equal {N}"
        );
    }

    // -----------------------------------------------------------------------
    // Test 19: Async encode/decode struct with all primitive types
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_decode_struct_all_primitives() {
        let bundle = PrimitiveBundle {
            flag: true,
            ch: '★',
            byte: 0xAB,
            signed: i32::MIN,
            unsigned: u64::MAX,
            float32: std::f32::consts::FRAC_PI_2,
            float64: std::f64::consts::FRAC_PI_4,
            short_str: "oxicode".to_string(),
        };

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder
                .write_item(&bundle)
                .await
                .expect("write_item PrimitiveBundle failed");
            encoder
                .finish()
                .await
                .expect("finish PrimitiveBundle failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<PrimitiveBundle> = decoder
            .read_item()
            .await
            .expect("read_item PrimitiveBundle failed");

        let dec = decoded.expect("expected Some(PrimitiveBundle), got None");
        assert_eq!(dec.flag, bundle.flag, "flag must match");
        assert_eq!(dec.ch, bundle.ch, "char must match");
        assert_eq!(dec.byte, bundle.byte, "byte must match");
        assert_eq!(dec.signed, bundle.signed, "i32 must match");
        assert_eq!(dec.unsigned, bundle.unsigned, "u64 must match");
        assert!(
            (dec.float32 - bundle.float32).abs() < f32::EPSILON * 1024.0,
            "f32 must match"
        );
        assert!(
            (dec.float64 - bundle.float64).abs() < f64::EPSILON * 1024.0,
            "f64 must match"
        );
        assert_eq!(dec.short_str, bundle.short_str, "string must match");
    }

    // -----------------------------------------------------------------------
    // Test 20: Async encode large data (100 KB)
    // -----------------------------------------------------------------------
    #[tokio::test]
    async fn test_async_encode_large_data_100kb() {
        use oxicode::streaming::StreamingConfig;

        // Construct a payload that totals ~100 KB.
        // Each String is 100 bytes; 1024 strings = ~100 KB.
        let payload: Vec<String> = (0usize..1024)
            .map(|i| format!("{:0>100}", i)) // 100-char zero-padded string
            .collect();

        let config = StreamingConfig::new().with_chunk_size(16 * 1024); // 16 KB chunks

        let mut buffer = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
            for s in &payload {
                encoder
                    .write_item(s)
                    .await
                    .expect("write_item 100KB item failed");
            }
            encoder.finish().await.expect("finish 100KB stream failed");
        }

        // Buffer must be at least 100 KB
        assert!(
            buffer.len() >= 100 * 1024,
            "encoded buffer must be at least 100 KB, got {} bytes",
            buffer.len()
        );

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: Vec<String> = decoder.read_all().await.expect("read_all 100KB failed");

        assert_eq!(
            decoded.len(),
            payload.len(),
            "100KB roundtrip must decode correct count"
        );
        assert_eq!(decoded, payload, "100KB roundtrip must match exactly");
        assert!(
            decoder.progress().chunks_processed > 1,
            "100KB at 16KB chunks must produce multiple chunks, got {}",
            decoder.progress().chunks_processed
        );
    }
}
