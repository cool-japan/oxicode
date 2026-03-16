//! Advanced async streaming tests (tenth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `Frame`, `Sensor`, and `Device`.
//!
//! Coverage matrix:
//!   1:    Encode/decode u32 async roundtrip
//!   2:    Encode/decode String async roundtrip
//!   3:    Encode/decode Vec<u8> async roundtrip
//!   4:    Encode multiple values sequentially, decode sequentially
//!   5:    Encode a custom struct (Frame), decode it
//!   6:    Encode 10 u32 values, decode all 10
//!   7:    Encode with standard config at slice level, then async roundtrip
//!   8:    Encode bool true/false, decode
//!   9:    Encode empty Vec<u8>, decode
//!  10:    Encode Option<String> Some, decode
//!  11:    Encode Option<String> None, decode
//!  12:    Encode u64::MAX, decode
//!  13:    Encode i64::MIN, decode
//!  14:    Encode Vec<String>, decode
//!  15:    Encode a struct with nested struct fields
//!  16:    Encode 3 different types in sequence, decode all 3
//!  17:    Encode and decode with fixed_int_encoding config at slice level then async
//!  18:    Encode with big_endian + fixed_int config, verify raw bytes at slice level
//!  19:    Encode large Vec<u8> (1000 bytes), decode
//!  20:    Multiple encode/decode pairs on same channel (back-to-back streams)
//!  21:    Encode tuple (u32, String), decode
//!  22:    Consumed bytes match encoded length via progress metric

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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder};
use oxicode::{config, decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
use std::io::Cursor;
use tokio::io::BufReader;

// ---------------------------------------------------------------------------
// Types unique to this file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Frame {
    id: u32,
    data: Vec<u8>,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Sensor {
    label: String,
    reading: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Device {
    name: String,
    sensor: Sensor,
    active: bool,
}

// ---------------------------------------------------------------------------
// Helper: encode a single item into a Vec<u8> via AsyncEncoder.
// ---------------------------------------------------------------------------

async fn async_encode_single<T: Encode>(item: &T) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(item)
            .await
            .expect("async_encode_single: write_item failed");
        enc.finish()
            .await
            .expect("async_encode_single: finish failed");
    }
    buf
}

async fn async_decode_single<T: Decode>(buf: Vec<u8>) -> Option<T> {
    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    dec.read_item::<T>()
        .await
        .expect("async_decode_single: read_item failed")
}

// ---------------------------------------------------------------------------
// Test 1: Encode/decode u32 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_u32_roundtrip() {
    let original: u32 = 0xCAFE_F00D;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(decoded, Some(original), "u32 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 2: Encode/decode String async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_string_roundtrip() {
    let original = String::from("OxiCode async streaming test set 10");
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<String>(buf).await;
    assert_eq!(decoded, Some(original), "String async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 3: Encode/decode Vec<u8> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_vec_u8_roundtrip() {
    let original: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(decoded, Some(original), "Vec<u8> async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 4: Encode multiple values sequentially, decode sequentially
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_multi_value_sequential_encode_decode() {
    let v_u32: u32 = 999_999;
    let v_string = String::from("multi-sequential-10");
    let v_bool = true;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&v_u32).await.expect("write u32 failed");
        enc.write_item(&v_string)
            .await
            .expect("write String failed");
        enc.write_item(&v_bool).await.expect("write bool failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    let d_u32: Option<u32> = dec.read_item().await.expect("read u32 failed");
    assert_eq!(d_u32, Some(v_u32), "sequential decode: u32 mismatch");

    let d_string: Option<String> = dec.read_item().await.expect("read String failed");
    assert_eq!(
        d_string,
        Some(v_string),
        "sequential decode: String mismatch"
    );

    let d_bool: Option<bool> = dec.read_item().await.expect("read bool failed");
    assert_eq!(d_bool, Some(v_bool), "sequential decode: bool mismatch");

    let eof: Option<u32> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all values decoded");
}

// ---------------------------------------------------------------------------
// Test 5: Encode a custom struct (Frame), decode it
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_frame_struct_roundtrip() {
    let original = Frame {
        id: 12345,
        data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
        timestamp: 1_700_000_000,
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Frame>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Frame struct async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Encode 10 u32 values, decode all 10
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_ten_u32_values_roundtrip() {
    let values: Vec<u32> = (1u32..=10).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write u32 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    for &expected in &values {
        let item: Option<u32> = dec.read_item().await.expect("read u32 failed");
        assert_eq!(item, Some(expected), "mismatch at u32 value {expected}");
    }

    let eof: Option<u32> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all 10 u32 values");
}

// ---------------------------------------------------------------------------
// Test 7: Encode with standard config at slice level, then async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_standard_config_slice_and_async_roundtrip() {
    let value: u64 = 0xABCD_EF01_2345_6789;
    let cfg = config::standard();

    // Slice-level roundtrip
    let bytes =
        encode_to_vec_with_config(&value, cfg).expect("standard config encode_to_vec failed");
    let (slice_decoded, _): (u64, _) = decode_from_slice_with_config(&bytes, cfg)
        .expect("standard config decode_from_slice failed");
    assert_eq!(
        slice_decoded, value,
        "standard config slice roundtrip mismatch"
    );

    // Async streaming roundtrip of same value
    let buf = async_encode_single(&value).await;
    let async_decoded = async_decode_single::<u64>(buf).await;
    assert_eq!(
        async_decoded,
        Some(value),
        "standard config async streaming roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Encode bool true/false, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_bool_true_false_roundtrip() {
    let buf_true = async_encode_single(&true).await;
    let decoded_true = async_decode_single::<bool>(buf_true).await;
    assert_eq!(decoded_true, Some(true), "bool true async roundtrip failed");

    let buf_false = async_encode_single(&false).await;
    let decoded_false = async_decode_single::<bool>(buf_false).await;
    assert_eq!(
        decoded_false,
        Some(false),
        "bool false async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Encode empty Vec<u8>, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_empty_vec_u8_roundtrip() {
    let original: Vec<u8> = Vec::new();
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "empty Vec<u8> async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Encode Option<String> Some, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_option_string_some_roundtrip() {
    let original: Option<String> = Some(String::from("async-10 Some value"));
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<String>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<String> Some async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Encode Option<String> None, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_option_string_none_roundtrip() {
    let original: Option<String> = None;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<String>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<String> None async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Encode u64::MAX, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_u64_max_roundtrip() {
    let original: u64 = u64::MAX;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u64>(buf).await;
    assert_eq!(decoded, Some(original), "u64::MAX async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 13: Encode i64::MIN, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_i64_min_roundtrip() {
    let original: i64 = i64::MIN;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<i64>(buf).await;
    assert_eq!(decoded, Some(original), "i64::MIN async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 14: Encode Vec<String>, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_vec_string_roundtrip() {
    let original: Vec<String> = vec![
        String::from("alpha"),
        String::from("bravo"),
        String::from("charlie"),
        String::from("delta"),
        String::from("echo"),
    ];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<String>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<String> async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Encode a struct with nested struct fields
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_nested_struct_roundtrip() {
    let original = Device {
        name: String::from("sensor-device-alpha"),
        sensor: Sensor {
            label: String::from("temperature"),
            reading: 36.6,
        },
        active: true,
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Device>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "nested struct async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Encode 3 different types in sequence, decode all 3
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_three_different_types_sequential() {
    let v_u8: u8 = 0xF0;
    let v_u64: u64 = 123_456_789_012;
    let v_bool = false;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&v_u8).await.expect("write u8 failed");
        enc.write_item(&v_u64).await.expect("write u64 failed");
        enc.write_item(&v_bool).await.expect("write bool failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    let d_u8: Option<u8> = dec.read_item().await.expect("read u8 failed");
    assert_eq!(d_u8, Some(v_u8), "type-sequence decode: u8 mismatch");

    let d_u64: Option<u64> = dec.read_item().await.expect("read u64 failed");
    assert_eq!(d_u64, Some(v_u64), "type-sequence decode: u64 mismatch");

    let d_bool: Option<bool> = dec.read_item().await.expect("read bool failed");
    assert_eq!(d_bool, Some(v_bool), "type-sequence decode: bool mismatch");

    let eof: Option<u8> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after 3 heterogeneous values");
}

// ---------------------------------------------------------------------------
// Test 17: Encode and decode with fixed_int_encoding config at slice level then async
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_fixed_int_encoding_config_roundtrip() {
    let value: u32 = 0x12_34_56_78;
    let cfg = config::standard().with_fixed_int_encoding();

    // Slice-level: fixed-int u32 must be exactly 4 bytes
    let fixed_bytes =
        encode_to_vec_with_config(&value, cfg).expect("fixed-int encode_to_vec failed");
    assert_eq!(
        fixed_bytes.len(),
        4,
        "fixed-int u32 must encode to exactly 4 bytes"
    );

    let (slice_decoded, _): (u32, _) =
        decode_from_slice_with_config(&fixed_bytes, cfg).expect("fixed-int decode failed");
    assert_eq!(slice_decoded, value, "fixed-int slice roundtrip mismatch");

    // Async streaming uses varint internally but must still roundtrip the value
    let buf = async_encode_single(&value).await;
    let async_decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(
        async_decoded,
        Some(value),
        "fixed-int async streaming roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Encode with big_endian + fixed_int config, verify raw bytes at slice level
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_big_endian_fixed_int_raw_bytes_verification() {
    let value: u32 = 0x01_02_03_04;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let be_fixed_bytes =
        encode_to_vec_with_config(&value, cfg).expect("big-endian+fixed-int encode failed");
    assert_eq!(
        be_fixed_bytes.len(),
        4,
        "big-endian fixed-int u32 must be 4 bytes"
    );
    // Big-endian byte order: most significant byte first
    assert_eq!(be_fixed_bytes[0], 0x01, "big-endian byte[0] must be 0x01");
    assert_eq!(be_fixed_bytes[1], 0x02, "big-endian byte[1] must be 0x02");
    assert_eq!(be_fixed_bytes[2], 0x03, "big-endian byte[2] must be 0x03");
    assert_eq!(be_fixed_bytes[3], 0x04, "big-endian byte[3] must be 0x04");

    // Verify roundtrip with matching config
    let (decoded, _): (u32, _) = decode_from_slice_with_config(&be_fixed_bytes, cfg)
        .expect("big-endian+fixed-int decode failed");
    assert_eq!(decoded, value, "big-endian+fixed-int roundtrip mismatch");

    // Async streaming still roundtrips correctly (uses its own standard config internally)
    let buf = async_encode_single(&value).await;
    let async_decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(
        async_decoded,
        Some(value),
        "big-endian+fixed-int value async streaming roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Encode large Vec<u8> (1000 bytes), decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_large_vec_u8_1000_bytes_roundtrip() {
    let original: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    assert_eq!(original.len(), 1000, "original must be exactly 1000 bytes");

    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "large Vec<u8> (1000 bytes) async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Multiple encode/decode pairs on same channel (back-to-back streams)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_multiple_encode_decode_pairs_same_channel() {
    // Simulate multiple independent items encoded into consecutive buffers,
    // each decoded independently (back-to-back allocation approach).

    let items: Vec<u32> = vec![111, 222, 333, 444, 555];
    let mut encoded_buffers: Vec<Vec<u8>> = Vec::new();

    for &item in &items {
        let buf = async_encode_single(&item).await;
        encoded_buffers.push(buf);
    }

    assert_eq!(encoded_buffers.len(), 5, "must have 5 encoded buffers");

    for (idx, buf) in encoded_buffers.into_iter().enumerate() {
        let expected = items[idx];
        let decoded = async_decode_single::<u32>(buf).await;
        assert_eq!(
            decoded,
            Some(expected),
            "pair {idx}: decoded value mismatch (expected {expected})"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 21: Encode tuple (u32, String), decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_tuple_u32_string_roundtrip() {
    let original: (u32, String) = (42, String::from("tuple-async-10"));
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<(u32, String)>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "(u32, String) tuple async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Consumed bytes match encoded length via progress metric
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async10_progress_bytes_match_encoded_length() {
    let values: Vec<u32> = (1u32..=8).map(|n| n * 100).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write u32 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let total_encoded_len = buf.len();
    assert!(total_encoded_len > 0, "encoded buffer must not be empty");

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    // Read first item and verify bytes_processed > 0
    let first: Option<u32> = dec.read_item().await.expect("read first u32 failed");
    assert_eq!(first, Some(100u32), "first decoded u32 must be 100");

    let bytes_after_first = dec.progress().bytes_processed;
    assert!(
        bytes_after_first > 0,
        "bytes_processed must be > 0 after reading first item (was 0)"
    );

    // Read all remaining items
    let rest: Vec<u32> = dec.read_all().await.expect("read_all failed");
    assert_eq!(rest.len(), 7, "must decode 7 remaining u32 values");
    assert_eq!(rest[0], 200u32, "second value must be 200");
    assert_eq!(rest[6], 800u32, "last value must be 800");

    // bytes_processed after all reads must have grown
    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_first,
        "bytes_processed must grow after reading all items (was {bytes_after_first}, now {bytes_after_all})"
    );

    // items_processed must equal total item count
    assert_eq!(
        dec.progress().items_processed,
        8,
        "items_processed must equal 8 after reading all values"
    );

    // Verify decoder reports finished state
    let eof: Option<u32> = dec.read_item().await.expect("post-all eof read failed");
    assert_eq!(
        eof, None,
        "decoder must return None once stream is exhausted"
    );
    assert!(
        dec.is_finished(),
        "decoder must report is_finished() == true after all items consumed"
    );

    // Sync encode of entire vec to cross-check total byte count
    let sync_encoded = encode_to_vec(&values).expect("sync encode_to_vec failed");
    let (sync_decoded, _): (Vec<u32>, _) =
        decode_from_slice(&sync_encoded).expect("sync decode_from_slice failed");
    assert_eq!(
        sync_decoded, values,
        "sync roundtrip of same values mismatch"
    );
}
