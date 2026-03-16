//! Advanced async streaming tests (fifth set) for OxiCode.
//!
//! All 22 tests are top-level (no module wrapper), gated with
//! `#[cfg(feature = "async-tokio")]`.

// ---------------------------------------------------------------------------
// Shared imports — only compiled when async-tokio feature is active
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
use std::io::Cursor;

// ---------------------------------------------------------------------------
// Shared derive types
// ---------------------------------------------------------------------------

#[cfg(feature = "async-tokio")]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct AdvancedPoint {
    x: u32,
    y: u32,
    label: String,
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
// Test 1: u32 encode/decode via async writer/reader
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_u32_roundtrip() {
    let original: u32 = 123_456;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("write u32 failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<u32> = dec.read_item().await.expect("read u32 failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 2: String encode/decode via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_string_roundtrip() {
    let original = "Hello, OxiCode async!".to_string();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write string failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<String> = dec.read_item().await.expect("read string failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 3: Vec<u8> encode/decode via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_vec_u8_roundtrip() {
    let original: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Vec<u8> failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Vec<u8>> = dec.read_item().await.expect("read Vec<u8> failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 4: bool encode/decode via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_bool_roundtrip() {
    let mut buf_true = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_true);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&true).await.expect("write true failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf_true);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<bool> = dec.read_item().await.expect("read bool failed");
    assert_eq!(decoded, Some(true));

    let mut buf_false = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_false);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&false).await.expect("write false failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor2 = Cursor::new(buf_false);
    let mut dec2 = AsyncStreamingDecoder::new(cursor2);
    let decoded2: Option<bool> = dec2.read_item().await.expect("read bool false failed");
    assert_eq!(decoded2, Some(false));
}

// ---------------------------------------------------------------------------
// Test 5: u64 encode/decode via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_u64_roundtrip() {
    let original: u64 = u64::MAX - 1;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("write u64 failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<u64> = dec.read_item().await.expect("read u64 failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 6: Large Vec<u8> (1000 bytes) via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_large_vec_u8_roundtrip() {
    let original: Vec<u8> = (0u8..=255).cycle().take(1000).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write large Vec<u8> failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Vec<u8>> = dec.read_item().await.expect("read large Vec<u8> failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 7: Multiple sequential writes then reads
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_sequential_writes_then_reads() {
    let values: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    for &expected in &values {
        let decoded: Option<u32> = dec.read_item().await.expect("read item failed");
        assert_eq!(decoded, Some(expected));
    }
    let eof: Option<u32> = dec.read_item().await.expect("read eof failed");
    assert_eq!(eof, None);
    assert!(dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 8: Empty Vec<u8> via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_empty_vec_roundtrip() {
    let original: Vec<u8> = Vec::new();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write empty Vec failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Vec<u8>> = dec.read_item().await.expect("read empty Vec failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 9: Option<u32> Some via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_option_u32_some_roundtrip() {
    let original: Option<u32> = Some(42);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Option Some failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Option<u32>> = dec.read_item().await.expect("read Option Some failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 10: Option<u32> None via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_option_u32_none_roundtrip() {
    let original: Option<u32> = None;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Option None failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Option<u32>> = dec.read_item().await.expect("read Option None failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 11: Struct via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_struct_roundtrip() {
    let original = AdvancedPoint {
        x: 100,
        y: 200,
        label: "test-point".to_string(),
    };

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write struct failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<AdvancedPoint> = dec.read_item().await.expect("read struct failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 12: Enum via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_enum_roundtrip() {
    let variants = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &variants {
            enc.write_item(v).await.expect("write enum variant failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<Direction> = dec.read_all().await.expect("read all enum variants failed");
    assert_eq!(decoded, variants);
}

// ---------------------------------------------------------------------------
// Test 13: Tuple (u32, String) via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_tuple_u32_string_roundtrip() {
    let original: (u32, String) = (777, "tuple-value".to_string());

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("write tuple failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<(u32, String)> = dec.read_item().await.expect("read tuple failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 14: Vec<String> via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_vec_string_roundtrip() {
    let original: Vec<String> = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
        "delta".to_string(),
        "epsilon".to_string(),
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Vec<String> failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Vec<String>> = dec.read_item().await.expect("read Vec<String> failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 15: i32 negative via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_i32_negative_roundtrip() {
    let values: Vec<i32> = vec![i32::MIN, -1_000_000, -1, 0, 1, 1_000_000, i32::MAX];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for v in &values {
            enc.write_item(v).await.expect("write i32 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<i32> = dec.read_all().await.expect("read all i32 failed");
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 16: u128 via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_u128_roundtrip() {
    let original: u128 = u128::MAX / 2 + 1;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("write u128 failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<u128> = dec.read_item().await.expect("read u128 failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 17: i64 min via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_i64_min_roundtrip() {
    let original: i64 = i64::MIN;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write i64::MIN failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<i64> = dec.read_item().await.expect("read i64::MIN failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 18: Verify items_processed after async decode
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_items_processed_after_decode() {
    const N: usize = 37;
    let values: Vec<u32> = (0..N as u32).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<u32> = dec.read_all().await.expect("read_all failed");
    assert_eq!(decoded.len(), N);
    assert_eq!(dec.progress().items_processed, N as u64);
    assert!(dec.progress().bytes_processed > 0);
}

// ---------------------------------------------------------------------------
// Test 19: Async roundtrip with small chunk_size config (fixed_int-style stress)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_roundtrip_small_chunk_config() {
    // Use a very small chunk size (64 bytes) with large string items to force
    // multiple chunk flushes. Each 50-char string encodes to more than 64 bytes.
    let config = StreamingConfig::new().with_chunk_size(64);
    let values: Vec<String> = (0u32..30)
        .map(|i| format!("{:0>50}", i)) // zero-padded to 50 chars
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::with_config(cursor, config);
        for s in &values {
            enc.write_item(s).await.expect("write with config failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<String> = dec.read_all().await.expect("read_all with config failed");
    assert_eq!(decoded, values);
    assert!(
        dec.progress().chunks_processed > 1,
        "expected multiple chunks with 64-byte chunk size, got {}",
        dec.progress().chunks_processed
    );
}

// ---------------------------------------------------------------------------
// Test 20: Async roundtrip standard config (default settings)
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_roundtrip_standard_config() {
    let values: Vec<String> = (0u32..50)
        .map(|i| format!("standard-config-item-{i}"))
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        for s in &values {
            enc.write_item(s).await.expect("write string failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Vec<String> = dec.read_all().await.expect("read_all failed");
    assert_eq!(decoded, values);
    assert!(dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 21: Vec<u32> large (100 elements) via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_large_vec_u32_roundtrip() {
    let original: Vec<u32> = (0u32..100).map(|i| i * i).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write large Vec<u32> failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Vec<u32>> = dec.read_item().await.expect("read large Vec<u32> failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 22: (u8, u16, u32, u64) tuple via async
// ---------------------------------------------------------------------------
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn test_async5_quad_tuple_roundtrip() {
    let original: (u8, u16, u32, u64) = (255, 65535, 4_294_967_295, 18_446_744_073_709_551_615);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write quad tuple failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<(u8, u16, u32, u64)> =
        dec.read_item().await.expect("read quad tuple failed");
    assert_eq!(decoded, Some(original));
}
