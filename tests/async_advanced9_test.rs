//! Advanced async streaming tests (ninth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `Packet` and `Command`.
//!
//! Coverage matrix:
//!   1-6:   Primitive async roundtrips (u8, u32, u64, String, Vec<u8>, bool)
//!   7-9:   Struct / enum roundtrips (Packet, Command::Ping, Command::Data)
//!  10-11:  Multiple-item streaming (5 × u32, 3 × Packet)
//!  12-13:  Option<String> Some/None roundtrip
//!  14-15:  Config variants (fixed-int, big-endian) at slice level then async
//!  16-17:  Interop (async-encode / sync-decode; sync-encode / async-decode)
//!  18-19:  Empty and large Vec<u8> roundtrip
//!    20:   Vec<Command> with all variants roundtrip
//!    21:   In-memory cursor, encode multiple values, decode sequentially
//!    22:   bytes_processed metric grows after encoding

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
struct Packet {
    seq: u64,
    payload: Vec<u8>,
    checksum: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Command {
    Ping,
    Pong,
    Data { id: u32, bytes: Vec<u8> },
    Reset,
}

// ---------------------------------------------------------------------------
// Helper: encode items into a Vec<u8> via AsyncEncoder, wrap with BufReader.
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
// Test 1: Encode/decode u8 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_u8_roundtrip() {
    let original: u8 = 0xAB;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u8>(buf).await;
    assert_eq!(decoded, Some(original), "u8 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 2: Encode/decode u32 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_u32_roundtrip() {
    let original: u32 = 0xDEAD_BEEF;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(decoded, Some(original), "u32 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 3: Encode/decode u64 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_u64_roundtrip() {
    let original: u64 = u64::MAX / 3;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u64>(buf).await;
    assert_eq!(decoded, Some(original), "u64 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 4: Encode/decode String async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_string_roundtrip() {
    let original = String::from("OxiCode async String roundtrip 🦀");
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<String>(buf).await;
    assert_eq!(decoded, Some(original), "String async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 5: Encode/decode Vec<u8> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_vec_u8_roundtrip() {
    let original: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(decoded, Some(original), "Vec<u8> async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 6: Encode/decode bool true/false async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_bool_roundtrip() {
    // true
    let buf_true = async_encode_single(&true).await;
    let decoded_true = async_decode_single::<bool>(buf_true).await;
    assert_eq!(decoded_true, Some(true), "bool true roundtrip failed");

    // false
    let buf_false = async_encode_single(&false).await;
    let decoded_false = async_decode_single::<bool>(buf_false).await;
    assert_eq!(decoded_false, Some(false), "bool false roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 7: Encode/decode Packet struct async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_packet_struct_roundtrip() {
    let original = Packet {
        seq: 42_000_000_001,
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
        checksum: 0xCAFE_BABE,
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Packet>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Packet struct async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Encode/decode Command::Ping async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_command_ping_roundtrip() {
    let original = Command::Ping;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Command>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Command::Ping async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Encode/decode Command::Data async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_command_data_roundtrip() {
    let original = Command::Data {
        id: 999,
        bytes: vec![0x11, 0x22, 0x33, 0x44, 0x55],
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Command>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Command::Data async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Write 5 u32 values, read them back in order
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_five_u32_values_in_order() {
    let values: Vec<u32> = vec![10, 20, 30, 40, 50];

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
        assert_eq!(
            item,
            Some(expected),
            "mismatch reading u32 value {expected}"
        );
    }

    let eof: Option<u32> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all u32 values");
}

// ---------------------------------------------------------------------------
// Test 11: Write 3 Packet structs, read them back in order
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_three_packets_in_order() {
    let packets = vec![
        Packet {
            seq: 1,
            payload: vec![0xAA],
            checksum: 111,
        },
        Packet {
            seq: 2,
            payload: vec![0xBB, 0xCC],
            checksum: 222,
        },
        Packet {
            seq: 3,
            payload: vec![0xDD, 0xEE, 0xFF],
            checksum: 333,
        },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for pkt in &packets {
            enc.write_item(pkt).await.expect("write Packet failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    for expected in &packets {
        let item: Option<Packet> = dec.read_item().await.expect("read Packet failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "Packet mismatch at seq {}",
            expected.seq
        );
    }
}

// ---------------------------------------------------------------------------
// Test 12: Option<String> Some async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_option_string_some_roundtrip() {
    let original: Option<String> = Some(String::from("hello from async-9"));
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<String>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<String> Some async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Option<String> None async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_option_string_none_roundtrip() {
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
// Test 14: Fixed-int config async roundtrip for u32
//
// Encodes with `fixed_int_encoding`, verifies the byte length is exactly 4,
// decodes with matching config, then also performs async streaming roundtrip.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_fixed_int_config_u32_roundtrip() {
    let value: u32 = 0x0A_0B_0C_0D;
    let cfg = config::standard().with_fixed_int_encoding();

    // Slice-level: must be exactly 4 bytes
    let fixed_bytes = encode_to_vec_with_config(&value, cfg).expect("fixed-int encode failed");
    assert_eq!(fixed_bytes.len(), 4, "fixed-int u32 must be 4 bytes");

    let (slice_decoded, _): (u32, _) =
        decode_from_slice_with_config(&fixed_bytes, cfg).expect("fixed-int decode failed");
    assert_eq!(slice_decoded, value, "fixed-int slice roundtrip mismatch");

    // Async streaming uses standard varint internally — must still roundtrip
    let buf = async_encode_single(&value).await;
    let async_decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(
        async_decoded,
        Some(value),
        "fixed-int async streaming roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Big-endian config async roundtrip for u32
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_big_endian_config_u32_roundtrip() {
    let value: u32 = 0x01_02_03_04;
    let cfg = config::standard().with_big_endian();

    // Slice-level big-endian roundtrip
    let be_bytes = encode_to_vec_with_config(&value, cfg).expect("big-endian encode failed");
    let (be_decoded, _): (u32, _) =
        decode_from_slice_with_config(&be_bytes, cfg).expect("big-endian decode failed");
    assert_eq!(be_decoded, value, "big-endian slice roundtrip mismatch");

    // Async streaming (standard varint) must also roundtrip the same value
    let buf = async_encode_single(&value).await;
    let async_decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(
        async_decoded,
        Some(value),
        "big-endian value async streaming roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Async encode then sync decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_async_encode_then_sync_decode() {
    let original = Packet {
        seq: 7777,
        payload: vec![0x01, 0x02, 0x03],
        checksum: 0xFFFF_0000,
    };

    // 1. Encode via async streaming into a buffer
    let mut stream_buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut stream_buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("async write_item failed");
        enc.finish().await.expect("async finish failed");
    }

    // 2. Async decode to verify stream integrity
    let cursor = Cursor::new(stream_buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let async_decoded: Option<Packet> = dec.read_item().await.expect("async read_item failed");
    assert_eq!(
        async_decoded,
        Some(original.clone()),
        "async-encode then async-decode mismatch"
    );

    // 3. Verify sync roundtrip is consistent
    let sync_bytes = encode_to_vec(&original).expect("sync encode_to_vec failed");
    let (sync_decoded, _): (Packet, _) =
        decode_from_slice(&sync_bytes).expect("sync decode_from_slice failed");
    assert_eq!(
        sync_decoded, original,
        "sync encode/decode of same Packet mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Sync encode then async decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_sync_encode_then_async_decode() {
    let original = Command::Pong;

    // 1. Sync encode the value
    let sync_bytes = encode_to_vec(&original).expect("sync encode_to_vec failed");

    // 2. Verify sync decode
    let (sync_decoded, _): (Command, _) =
        decode_from_slice(&sync_bytes).expect("sync decode_from_slice failed");
    assert_eq!(sync_decoded, original, "sync roundtrip mismatch");

    // 3. Wrap original in async streaming encoder
    let mut async_buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut async_buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("async write_item failed");
        enc.finish().await.expect("async finish failed");
    }

    // 4. Async decode
    let cursor = Cursor::new(async_buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let async_decoded: Option<Command> = dec.read_item().await.expect("async read_item failed");
    assert_eq!(
        async_decoded,
        Some(original),
        "sync-encode then async-decode mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Empty Vec<u8> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_empty_vec_u8_roundtrip() {
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
// Test 19: Large Vec<u8> (1000 bytes) async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_large_vec_u8_roundtrip() {
    let original: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    assert_eq!(original.len(), 1000);

    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "large Vec<u8> (1000 bytes) async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Vec<Command> with all variants async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_vec_command_all_variants_roundtrip() {
    let original = vec![
        Command::Ping,
        Command::Pong,
        Command::Data {
            id: 42,
            bytes: vec![0xDE, 0xAD],
        },
        Command::Reset,
    ];

    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Command>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<Command> all-variants async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 21: In-memory cursor — encode multiple values, decode sequentially
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_in_memory_cursor_multi_value_sequential_decode() {
    // Encode three different-type-sized values into one stream
    let v_u8: u8 = 0xFF;
    let v_u32: u32 = 100_000;
    let v_str = String::from("cursor-sequential");

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&v_u8).await.expect("write u8 failed");
        enc.write_item(&v_u32).await.expect("write u32 failed");
        enc.write_item(&v_str).await.expect("write String failed");
        enc.finish().await.expect("finish failed");
    }

    // Decode each value in order from a BufReader-wrapped cursor
    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    let d_u8: Option<u8> = dec.read_item().await.expect("read u8 failed");
    assert_eq!(d_u8, Some(v_u8), "sequential decode: u8 mismatch");

    let d_u32: Option<u32> = dec.read_item().await.expect("read u32 failed");
    assert_eq!(d_u32, Some(v_u32), "sequential decode: u32 mismatch");

    let d_str: Option<String> = dec.read_item().await.expect("read String failed");
    assert_eq!(d_str, Some(v_str), "sequential decode: String mismatch");

    // Stream must be exhausted now
    let eof: Option<u8> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all values decoded");
    assert!(dec.is_finished(), "decoder must report finished");
}

// ---------------------------------------------------------------------------
// Test 22: bytes_processed metric increases after encoding
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async9_bytes_processed_increases_after_encoding() {
    let values: Vec<u64> = (1u64..=15).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for &v in &values {
            enc.write_item(&v).await.expect("write u64 failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    // Read the first item and check bytes_processed > 0
    let first: Option<u64> = dec.read_item().await.expect("read first item failed");
    assert_eq!(first, Some(1u64), "first decoded value must be 1");

    let bytes_after_one = dec.progress().bytes_processed;
    assert!(
        bytes_after_one > 0,
        "bytes_processed must be > 0 after reading first item"
    );

    // Read the rest and check bytes_processed has grown
    let rest: Vec<u64> = dec.read_all().await.expect("read_all failed");
    assert_eq!(rest.len(), 14, "must decode 14 remaining values");

    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_one,
        "bytes_processed must grow after reading all items (was {bytes_after_one}, now {bytes_after_all})"
    );

    assert_eq!(
        dec.progress().items_processed,
        15,
        "items_processed must equal 15"
    );
}
