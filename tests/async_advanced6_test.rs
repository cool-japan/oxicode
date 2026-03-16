//! Advanced async streaming tests (sixth set) for OxiCode.
//!
//! All 22 tests are top-level (no module wrapper), gated with
//! `#[cfg(feature = "async-tokio")]`.

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
use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder};
use oxicode::{config, decode_from_slice, encode_to_vec, Decode, Encode};
use std::io::Cursor;

// ---------------------------------------------------------------------------
// Shared derive types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct NamedItem {
    id: u32,
    name: String,
}

// ---------------------------------------------------------------------------
// Test 1: Encode/decode single u32
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_single_u32_roundtrip() {
    let original: u32 = 42_u32;

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
// Test 2: Encode/decode single String
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_single_string_roundtrip() {
    let original = "OxiCode async streaming test".to_string();

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
// Test 3: Encode/decode multiple u32 values sequentially
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_multiple_u32_sequential() {
    let values: Vec<u32> = vec![10, 20, 30, 40, 50];

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
    for &expected in &values {
        let decoded: Option<u32> = dec.read_item().await.expect("read u32 failed");
        assert_eq!(decoded, Some(expected));
    }
}

// ---------------------------------------------------------------------------
// Test 4: Encode/decode Vec<u8>
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_vec_u8_roundtrip() {
    let original: Vec<u8> = vec![1, 2, 3, 4, 5, 255, 128, 64];

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
// Test 5: Empty encoder produces only end-marker bytes (no item bytes)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_empty_encoder_no_item_bytes() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let enc: AsyncStreamingEncoder<Cursor<&mut Vec<u8>>> = AsyncStreamingEncoder::new(cursor);
        enc.finish().await.expect("finish failed");
    }

    // Buffer must contain only the end-chunk marker (non-empty) but no item data
    assert!(!buf.is_empty(), "end marker should produce some bytes");

    // Decoding should yield no items
    let cursor = Cursor::new(buf);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);
    let result: Option<u32> = dec
        .read_item()
        .await
        .expect("read from empty stream failed");
    assert_eq!(result, None);
    assert!(dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 6: Encoder then decoder roundtrip for u64
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_u64_roundtrip() {
    let original: u64 = u64::MAX;

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
// Test 7: Multiple different types sequentially (u32 then String)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_mixed_u32_string_sequential() {
    let num: u32 = 999;
    let text = "mixed-type-test".to_string();

    let mut buf_num = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_num);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&num).await.expect("write u32 failed");
        enc.finish().await.expect("finish failed");
    }

    let mut buf_str = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf_str);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&text).await.expect("write string failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor_num = Cursor::new(buf_num);
    let mut dec_num = AsyncStreamingDecoder::new(cursor_num);
    let decoded_num: Option<u32> = dec_num.read_item().await.expect("read u32 failed");
    assert_eq!(decoded_num, Some(num));

    let cursor_str = Cursor::new(buf_str);
    let mut dec_str = AsyncStreamingDecoder::new(cursor_str);
    let decoded_str: Option<String> = dec_str.read_item().await.expect("read string failed");
    assert_eq!(decoded_str, Some(text));
}

// ---------------------------------------------------------------------------
// Test 8: Encode/decode bool values
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_bool_roundtrip() {
    for &flag in &[true, false] {
        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut enc = AsyncStreamingEncoder::new(cursor);
            enc.write_item(&flag).await.expect("write bool failed");
            enc.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buf);
        let mut dec = AsyncStreamingDecoder::new(cursor);
        let decoded: Option<bool> = dec.read_item().await.expect("read bool failed");
        assert_eq!(decoded, Some(flag));
    }
}

// ---------------------------------------------------------------------------
// Test 9: Large data (1000-byte Vec<u8>)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_large_vec_u8_roundtrip() {
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
// Test 10: Encode/decode i32 negative value
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_i32_negative_roundtrip() {
    let original: i32 = -1_234_567;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("write i32 failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<i32> = dec.read_item().await.expect("read i32 failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 11: Encode/decode Option<u32> Some
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_option_u32_some_roundtrip() {
    let original: Option<u32> = Some(77);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Option::Some failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Option<u32>> = dec.read_item().await.expect("read Option::Some failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 12: Encode/decode Option<u32> None
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_option_u32_none_roundtrip() {
    let original: Option<u32> = None;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write Option::None failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<Option<u32>> = dec.read_item().await.expect("read Option::None failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 13: Encode multiple items, read them all with read_all
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_encode_multiple_read_all() {
    let values: Vec<u32> = vec![100, 200, 300, 400, 500];

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
    assert_eq!(decoded, values);
}

// ---------------------------------------------------------------------------
// Test 14: Sequential encode of 10 u32 values, sequential decode
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_ten_u32_sequential_encode_decode() {
    let values: Vec<u32> = (1u32..=10).collect();

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
    let mut results = Vec::<u32>::new();
    for _ in 0..10 {
        let item: Option<u32> = dec.read_item().await.expect("read u32 failed");
        if let Some(v) = item {
            results.push(v);
        }
    }
    assert_eq!(results, values);
}

// ---------------------------------------------------------------------------
// Test 15: Encoder with u64 max value
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_u64_max_value_roundtrip() {
    let original: u64 = u64::MAX;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("write u64::MAX failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<u64> = dec.read_item().await.expect("read u64::MAX failed");
    assert_eq!(decoded, Some(u64::MAX));
}

// ---------------------------------------------------------------------------
// Test 16: Decoder on empty buffer returns None
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_decoder_on_empty_buffer_returns_none() {
    // A valid stream with no items — just write the end marker by finishing immediately
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let enc: AsyncStreamingEncoder<Cursor<&mut Vec<u8>>> = AsyncStreamingEncoder::new(cursor);
        enc.finish().await.expect("finish empty stream failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec: AsyncStreamingDecoder<Cursor<Vec<u8>>> = AsyncStreamingDecoder::new(cursor);
    let result: Option<u32> = dec
        .read_item()
        .await
        .expect("read from empty stream failed");
    assert_eq!(result, None);
}

// ---------------------------------------------------------------------------
// Test 17: Writing zero items produces only the end-marker in the output
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_zero_writes_produces_end_marker_only() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let enc: AsyncStreamingEncoder<Cursor<&mut Vec<u8>>> = AsyncStreamingEncoder::new(cursor);
        enc.finish().await.expect("finish failed");
    }

    // The buffer must be non-empty (end marker written) but decodable as empty
    assert!(!buf.is_empty(), "end marker bytes should be present");
    let marker_size = buf.len();

    // Write one item and verify the buffer grows beyond the marker size
    let mut buf2 = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf2);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&1_u32).await.expect("write u32 failed");
        enc.finish().await.expect("finish failed");
    }

    assert!(
        buf2.len() > marker_size,
        "item bytes should exceed end-marker-only size"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Encode/decode struct { id: u32, name: String }
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_struct_named_item_roundtrip() {
    let original = NamedItem {
        id: 101,
        name: "widget".to_string(),
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
    let decoded: Option<NamedItem> = dec.read_item().await.expect("read struct failed");
    assert_eq!(decoded, Some(original));
}

// ---------------------------------------------------------------------------
// Test 19: Multiple items then partial read (read first item only)
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_multiple_items_partial_read() {
    let values: Vec<u32> = vec![11, 22, 33, 44, 55];

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

    // Read only the first item
    let first: Option<u32> = dec.read_item().await.expect("read first item failed");
    assert_eq!(first, Some(11));

    // Stream is not finished yet
    assert!(!dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 20: read_all returns correct count of items
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_read_all_returns_correct_count() {
    const COUNT: usize = 25;
    let values: Vec<u32> = (0..COUNT as u32).map(|i| i * 3).collect();

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

    assert_eq!(decoded.len(), COUNT);
    assert_eq!(decoded, values);
    assert_eq!(dec.progress().items_processed, COUNT as u64);
}

// ---------------------------------------------------------------------------
// Test 21: Encode with standard config via encode_to_vec, verify async decode matches
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_encode_to_vec_matches_async_streaming_decode() {
    let original: u32 = 54321;

    // Encode via the standard encode_to_vec API
    let standard_bytes = encode_to_vec(&original).expect("encode_to_vec failed");

    // Decode via standard decode_from_slice
    let (from_std, _) =
        decode_from_slice::<u32>(&standard_bytes).expect("decode_from_slice failed");
    assert_eq!(from_std, original);

    // Encode via async streaming and compare item value after decode
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncStreamingEncoder::new(cursor);
        enc.write_item(&original).await.expect("async write failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let mut dec = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<u32> = dec.read_item().await.expect("async read failed");
    assert_eq!(decoded, Some(original));

    // Both paths must decode to the same value
    assert_eq!(decoded, Some(from_std));

    // Verify config module is accessible
    let _cfg = config::standard();
}

// ---------------------------------------------------------------------------
// Test 22: Encode/decode Vec<String>
// ---------------------------------------------------------------------------
#[tokio::test]
async fn test_async6_vec_string_roundtrip() {
    let original: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
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
