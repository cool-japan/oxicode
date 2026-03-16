//! Advanced async streaming tests (third set) for OxiCode.
//!
//! Covers unique scenarios not present in async_streaming_test.rs,
//! async_advanced_test.rs, or async_advanced2_test.rs.
//! All tests are top-level (no module wrapper).

// ---------------------------------------------------------------------------
// Shared types — only compiled when async-tokio feature is active
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
use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder, StreamingConfig};
#[cfg(feature = "async-tokio")]
use oxicode::{Decode, Encode};
#[cfg(feature = "async-tokio")]
use std::collections::HashMap;
#[cfg(feature = "async-tokio")]
use std::io::Cursor;

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct NestedStruct {
    outer: u32,
    label: String,
    inner: InnerStruct,
}

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct InnerStruct {
    value: i64,
    flag: bool,
}

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ZeroSized {
    marker: u8,
}

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum RichEnum {
    Empty,
    SingleU64(u64),
    Pair(u32, u32),
    Named { key: String, val: i32 },
}

// ---------------------------------------------------------------------------
// Test 1: Async encode Vec<u32> then decode Vec<u32> (full-vec as single item)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_vec_u32_as_single_item() {
    let data: Vec<u32> = (100u32..200u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&data)
            .await
            .expect("write Vec<u32> failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<Vec<u32>> = decoder.read_item().await.expect("read Vec<u32> failed");
    assert_eq!(got, Some(data));
}

// ---------------------------------------------------------------------------
// Test 2: Async encode Vec<String> then decode Vec<String>
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_vec_string_as_single_item() {
    let strings: Vec<String> = (0u32..10u32)
        .map(|i| format!("string-item-{i:04}"))
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&strings)
            .await
            .expect("write Vec<String> failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<Vec<String>> = decoder.read_item().await.expect("read Vec<String> failed");
    assert_eq!(got, Some(strings));
}

// ---------------------------------------------------------------------------
// Test 3: Async encode/decode with large chunk size config (no splitting)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_large_chunk_size_config_no_split() {
    // chunk_size max = 16MB, all 1000 items should fit in one chunk
    let config = StreamingConfig::new().with_chunk_size(16 * 1024 * 1024);
    let items: Vec<u32> = (0u32..1000u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
        for v in &items {
            encoder
                .write_item(v)
                .await
                .expect("write with large chunk failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = decoder
        .read_all()
        .await
        .expect("read_all with large chunk failed");

    assert_eq!(decoded, items);
    assert_eq!(
        decoder.progress().chunks_processed,
        1,
        "all items should fit in one chunk"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Async encode/decode with small chunk size (minimum 1024) forces splits
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_minimum_chunk_size_forces_splits() {
    // chunk_size clamped to 1024 minimum; encode strings of ~30 bytes each
    // so that 50 items (~1500 bytes) exceed the 1024-byte chunk boundary.
    let config = StreamingConfig::new().with_chunk_size(1024);
    let items: Vec<String> = (0u64..50u64)
        .map(|i| format!("{:0>30}", i)) // 30-char zero-padded string
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
        for v in &items {
            encoder
                .write_item(v)
                .await
                .expect("write with min chunk failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<String> = decoder
        .read_all()
        .await
        .expect("read_all with min chunk failed");

    assert_eq!(decoded, items);
    assert!(
        decoder.progress().chunks_processed > 1,
        "expected multiple chunks with 1024-byte limit, got {}",
        decoder.progress().chunks_processed
    );
}

// ---------------------------------------------------------------------------
// Test 5: Multiple sequential async encodes to same writer (separate finish calls)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_multiple_sequential_encodes_separate_streams() {
    // Each encode is its own independent stream; we verify each independently
    let sets: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![100, 200, 300], vec![999, 888, 777]];

    for set in &sets {
        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for &v in set {
                encoder.write_item(&v).await.expect("write seq failed");
            }
            encoder.finish().await.expect("finish seq failed");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let mut decoded = Vec::<u32>::new();
        while let Some(v) = decoder.read_item::<u32>().await.expect("read seq failed") {
            decoded.push(v);
        }
        assert_eq!(&decoded, set);
    }
}

// ---------------------------------------------------------------------------
// Test 6: Async encode of nested struct (NestedStruct containing InnerStruct)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_nested_struct_roundtrip() {
    let items: Vec<NestedStruct> = (0u32..5u32)
        .map(|i| NestedStruct {
            outer: i,
            label: format!("nested-{i}"),
            inner: InnerStruct {
                value: -(i as i64 * 100),
                flag: i % 2 == 0,
            },
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for item in &items {
            encoder
                .write_item(item)
                .await
                .expect("write nested struct failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<NestedStruct>::new();
    while let Some(v) = decoder
        .read_item::<NestedStruct>()
        .await
        .expect("read nested struct failed")
    {
        decoded.push(v);
    }
    assert_eq!(decoded, items);
}

// ---------------------------------------------------------------------------
// Test 7: Async encode of RichEnum with all four variants
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_rich_enum_all_variants() {
    let variants = vec![
        RichEnum::Empty,
        RichEnum::SingleU64(u64::MAX),
        RichEnum::Pair(0, u32::MAX),
        RichEnum::Named {
            key: "hello".to_string(),
            val: -42,
        },
        RichEnum::Empty,
        RichEnum::SingleU64(0),
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &variants {
            encoder.write_item(v).await.expect("write RichEnum failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<RichEnum>::new();
    while let Some(v) = decoder
        .read_item::<RichEnum>()
        .await
        .expect("read RichEnum failed")
    {
        decoded.push(v);
    }
    assert_eq!(decoded, variants);
}

// ---------------------------------------------------------------------------
// Test 8: Async encode of Option<String> — Some and None variants
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_option_string_roundtrip() {
    let options: Vec<Option<String>> = vec![
        Some("first".to_string()),
        None,
        Some(String::new()),
        None,
        Some("last".to_string()),
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &options {
            encoder
                .write_item(v)
                .await
                .expect("write Option<String> failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<Option<String>>::new();
    while let Some(v) = decoder
        .read_item::<Option<String>>()
        .await
        .expect("read Option<String> failed")
    {
        decoded.push(v);
    }
    assert_eq!(decoded, options);
}

// ---------------------------------------------------------------------------
// Test 9: Async decode error handling with truncated / corrupted data
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_decode_error_truncated_data() {
    // Build a valid stream, then truncate it so the decoder sees a partial chunk
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&42u64)
            .await
            .expect("write for truncate test failed");
        encoder
            .finish()
            .await
            .expect("finish for truncate test failed");
    }

    // Truncate to half the bytes — guaranteed to be an incomplete chunk payload
    let truncated_len = buf.len() / 2;
    buf.truncate(truncated_len);

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let result = decoder.read_item::<u64>().await;
    // The truncated stream must produce an error (not a panic or Some)
    assert!(
        result.is_err(),
        "truncated data must return an error, got Ok({result:?})"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Async encode of large data — 1000 elements — all decoded correctly
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_large_data_1000_elements() {
    let items: Vec<u64> = (0u64..1000u64).map(|i| i * i).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let config = StreamingConfig::new().with_chunk_size(4096);
        let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
        for v in &items {
            encoder
                .write_item(v)
                .await
                .expect("write 1000 elements failed");
        }
        encoder.finish().await.expect("finish 1000 elements failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u64> = decoder
        .read_all()
        .await
        .expect("read_all 1000 elements failed");

    assert_eq!(decoded.len(), 1000, "expected 1000 elements");
    assert_eq!(decoded, items, "decoded values must match original");
}

// ---------------------------------------------------------------------------
// Test 11: Round-trip through tokio::io::duplex pipe
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_round_trip_duplex_pipe() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let items: Vec<u32> = (0u32..50u32).collect();

    // Encode to bytes first (duplex is half-duplex; encode separately then send)
    let mut encoded = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut encoded);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &items {
            encoder
                .write_item(v)
                .await
                .expect("write for duplex test failed");
        }
        encoder
            .finish()
            .await
            .expect("finish for duplex test failed");
    }

    // Create a duplex pipe and write the encoded bytes through it
    let (mut server, mut client) = tokio::io::duplex(65536);
    server
        .write_all(&encoded)
        .await
        .expect("write to duplex failed");
    // Close the write end so the reader sees EOF
    drop(server);

    // Decode from the client end via a collected buffer (duplex read)
    let mut received = Vec::<u8>::new();
    client
        .read_to_end(&mut received)
        .await
        .expect("read from duplex failed");

    let cursor = Cursor::new(received);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = decoder
        .read_all()
        .await
        .expect("read_all through duplex failed");

    assert_eq!(decoded, items);
}

// ---------------------------------------------------------------------------
// Test 12: Async encode/decode of bool, u8, u64 as separate streams
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_bool_u8_u64_separate_streams() {
    // bool stream
    let bools = vec![true, false, true];
    let mut buf_bool = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_bool);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for b in &bools {
            enc.write_item(b).await.expect("write bool failed");
        }
        enc.finish().await.expect("finish bool failed");
    }

    // u8 stream
    let bytes: Vec<u8> = vec![0, 127, 255];
    let mut buf_u8 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_u8);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for b in &bytes {
            enc.write_item(b).await.expect("write u8 failed");
        }
        enc.finish().await.expect("finish u8 failed");
    }

    // u64 stream
    let u64s: Vec<u64> = vec![0, u64::MAX / 2, u64::MAX];
    let mut buf_u64 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_u64);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &u64s {
            enc.write_item(v).await.expect("write u64 failed");
        }
        enc.finish().await.expect("finish u64 failed");
    }

    // Decode all three
    let mut dec_bool = AsyncStreamingDecoder::new(Cursor::new(buf_bool));
    let mut dec_u8 = AsyncStreamingDecoder::new(Cursor::new(buf_u8));
    let mut dec_u64 = AsyncStreamingDecoder::new(Cursor::new(buf_u64));

    let decoded_bools: Vec<bool> = dec_bool.read_all().await.expect("decode bools failed");
    let decoded_u8s: Vec<u8> = dec_u8.read_all().await.expect("decode u8s failed");
    let decoded_u64s: Vec<u64> = dec_u64.read_all().await.expect("decode u64s failed");

    assert_eq!(decoded_bools, bools);
    assert_eq!(decoded_u8s, bytes);
    assert_eq!(decoded_u64s, u64s);
}

// ---------------------------------------------------------------------------
// Test 13: Async encode of tuple (u32, String, bool, f64) round-trip
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_tuple_u32_string_bool_f64() {
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    let items: Vec<(u32, String, bool, f64)> = vec![
        (0, "zero".to_string(), false, 0.0_f64),
        (1, "one".to_string(), true, pi),
        (u32::MAX, "max".to_string(), false, e),
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for item in &items {
            encoder.write_item(item).await.expect("write tuple failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<(u32, String, bool, f64)>::new();
    while let Some(v) = decoder
        .read_item::<(u32, String, bool, f64)>()
        .await
        .expect("read tuple failed")
    {
        decoded.push(v);
    }
    assert_eq!(decoded.len(), items.len());
    for (orig, dec) in items.iter().zip(decoded.iter()) {
        assert_eq!(orig.0, dec.0, "u32 field mismatch");
        assert_eq!(orig.1, dec.1, "String field mismatch");
        assert_eq!(orig.2, dec.2, "bool field mismatch");
        assert_eq!(orig.3.to_bits(), dec.3.to_bits(), "f64 field mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 14: Async encode of HashMap<String, u32> round-trip
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_hashmap_string_u32_roundtrip() {
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("alpha".to_string(), 1);
    map.insert("beta".to_string(), 2);
    map.insert("gamma".to_string(), 3);
    map.insert("delta".to_string(), u32::MAX);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_item(&map)
            .await
            .expect("write HashMap failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let got: Option<HashMap<String, u32>> = decoder.read_item().await.expect("read HashMap failed");
    assert_eq!(got, Some(map));
}

// ---------------------------------------------------------------------------
// Test 15: Async streaming encoder — verify write_all convenience method
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_write_all_convenience_method() {
    let items: Vec<u32> = (500u32..560u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder
            .write_all(items.clone())
            .await
            .expect("write_all failed");
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = decoder
        .read_all()
        .await
        .expect("read_all after write_all failed");
    assert_eq!(decoded, items);
    assert_eq!(
        decoder.progress().items_processed,
        items.len() as u64,
        "items_processed mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Async encode and verify non-zero bytes_processed counter
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_bytes_processed_after_encode() {
    let items: Vec<u64> = (0u64..40u64).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &items {
            encoder
                .write_item(v)
                .await
                .expect("write for bytes check failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let _: Vec<u64> = decoder
        .read_all()
        .await
        .expect("read_all for bytes check failed");

    assert!(
        decoder.progress().bytes_processed > 0,
        "bytes_processed must be > 0 after encoding 40 items"
    );
    assert_eq!(
        decoder.progress().items_processed,
        items.len() as u64,
        "items_processed must equal 40"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Multiple types async round-trip in the same stream (mixed items)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_multiple_types_same_stream() {
    // Encode u32, String, bool, u64 each as separate top-level streams
    let u32_val: u32 = 0xDEAD;
    let str_val = String::from("multi-type-test");
    let bool_val: bool = true;
    let u64_val: u64 = u64::MAX - 1;

    let mut buf_u32 = Vec::<u8>::new();
    let mut buf_str = Vec::<u8>::new();
    let mut buf_bool = Vec::<u8>::new();
    let mut buf_u64 = Vec::<u8>::new();

    {
        let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf_u32));
        enc.write_item(&u32_val).await.expect("write u32 failed");
        enc.finish().await.expect("finish u32 failed");
    }
    {
        let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf_str));
        enc.write_item(&str_val).await.expect("write str failed");
        enc.finish().await.expect("finish str failed");
    }
    {
        let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf_bool));
        enc.write_item(&bool_val).await.expect("write bool failed");
        enc.finish().await.expect("finish bool failed");
    }
    {
        let mut enc = AsyncStreamingEncoder::new(Cursor::new(&mut buf_u64));
        enc.write_item(&u64_val).await.expect("write u64 failed");
        enc.finish().await.expect("finish u64 failed");
    }

    let decoded_u32: Option<u32> = AsyncStreamingDecoder::new(Cursor::new(buf_u32))
        .read_item()
        .await
        .expect("decode u32 failed");
    let decoded_str: Option<String> = AsyncStreamingDecoder::new(Cursor::new(buf_str))
        .read_item()
        .await
        .expect("decode str failed");
    let decoded_bool: Option<bool> = AsyncStreamingDecoder::new(Cursor::new(buf_bool))
        .read_item()
        .await
        .expect("decode bool failed");
    let decoded_u64: Option<u64> = AsyncStreamingDecoder::new(Cursor::new(buf_u64))
        .read_item()
        .await
        .expect("decode u64 failed");

    assert_eq!(decoded_u32, Some(u32_val));
    assert_eq!(decoded_str, Some(str_val));
    assert_eq!(decoded_bool, Some(bool_val));
    assert_eq!(decoded_u64, Some(u64_val));
}

// ---------------------------------------------------------------------------
// Test 18: Async encode of ZeroSized struct (single marker byte)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_zero_sized_struct_roundtrip() {
    let items: Vec<ZeroSized> = (0u8..10u8).map(|i| ZeroSized { marker: i }).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &items {
            encoder.write_item(v).await.expect("write ZeroSized failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<ZeroSized>::new();
    while let Some(v) = decoder
        .read_item::<ZeroSized>()
        .await
        .expect("read ZeroSized failed")
    {
        decoded.push(v);
    }
    assert_eq!(decoded, items);
}

// ---------------------------------------------------------------------------
// Test 19: Async decode from concatenated in-memory buffers (two separate streams)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_decode_from_concatenated_buffers() {
    // Build two independent streams and concatenate them, then decode each separately
    let first: Vec<u32> = vec![10, 20, 30];
    let second: Vec<u32> = vec![40, 50, 60];

    let mut buf_a = Vec::<u8>::new();
    let mut buf_b = Vec::<u8>::new();

    {
        let cursor = Cursor::new(&mut buf_a);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &first {
            enc.write_item(v).await.expect("write first failed");
        }
        enc.finish().await.expect("finish first failed");
    }
    {
        let cursor = Cursor::new(&mut buf_b);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &second {
            enc.write_item(v).await.expect("write second failed");
        }
        enc.finish().await.expect("finish second failed");
    }

    // Decode each buffer independently (they are separate complete streams)
    let mut dec_a = AsyncStreamingDecoder::new(Cursor::new(buf_a));
    let mut dec_b = AsyncStreamingDecoder::new(Cursor::new(buf_b));

    let decoded_first: Vec<u32> = dec_a.read_all().await.expect("read_all first failed");
    let decoded_second: Vec<u32> = dec_b.read_all().await.expect("read_all second failed");

    assert_eq!(decoded_first, first);
    assert_eq!(decoded_second, second);
}

// ---------------------------------------------------------------------------
// Test 20: Async encode with max_buffer config set explicitly
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_with_max_buffer_config() {
    // max_buffer does not affect correctness — verify round-trip works
    let config = StreamingConfig::new()
        .with_chunk_size(2048)
        .with_max_buffer(1024 * 1024);

    let items: Vec<u64> = (1000u64..1050u64).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);
        for v in &items {
            encoder
                .write_item(v)
                .await
                .expect("write with max_buffer config failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u64> = decoder
        .read_all()
        .await
        .expect("read_all with max_buffer config failed");

    assert_eq!(decoded, items);
}

// ---------------------------------------------------------------------------
// Test 21: Verify is_finished() is false before stream ends and true after
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_is_finished_state_transitions() {
    let values: Vec<u32> = vec![1, 2, 3];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &values {
            encoder
                .write_item(v)
                .await
                .expect("write state test failed");
        }
        encoder.finish().await.expect("finish state test failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);

    assert!(
        !decoder.is_finished(),
        "decoder must not be finished before reading"
    );

    let _v1: Option<u32> = decoder.read_item().await.expect("read 1 failed");
    let _v2: Option<u32> = decoder.read_item().await.expect("read 2 failed");
    let _v3: Option<u32> = decoder.read_item().await.expect("read 3 failed");

    // Not finished yet — end-of-stream marker not consumed until next read
    let eof: Option<u32> = decoder.read_item().await.expect("read eof failed");
    assert!(eof.is_none(), "expected None at end of stream");
    assert!(
        decoder.is_finished(),
        "decoder must be finished after reading end-of-stream marker"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Async encode of i32 values including i32::MIN and i32::MAX
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async3_i32_min_max_and_boundaries() {
    let values: Vec<i32> = vec![i32::MIN, -1_000_000, -1, 0, 1, 1_000_000, i32::MAX];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        for v in &values {
            encoder
                .write_item(v)
                .await
                .expect("write i32 boundary failed");
        }
        encoder.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let mut decoded = Vec::<i32>::new();
    while let Some(v) = decoder
        .read_item::<i32>()
        .await
        .expect("read i32 boundary failed")
    {
        decoded.push(v);
    }
    assert_eq!(decoded, values);
    assert!(decoder.is_finished());
}
