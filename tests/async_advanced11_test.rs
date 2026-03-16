//! Advanced async streaming tests (eleventh set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `Packet` and `Command`.
//!
//! Coverage matrix:
//!   1:    Encode/decode Packet async roundtrip
//!   2:    Encode/decode Command::Ping async roundtrip
//!   3:    Encode/decode Command::Pong async roundtrip
//!   4:    Encode/decode Command::Data(vec) async roundtrip
//!   5:    Encode/decode Command::Error("msg") async roundtrip
//!   6:    Encode 5 Packets sequentially, decode all 5
//!   7:    Encode Vec<u8> bytes, decode
//!   8:    Encode i32 negative value, decode
//!   9:    Encode u128::MAX, decode
//!  10:    Encode f64::PI, decode (bit-exact)
//!  11:    Encode Option<Packet> Some, decode
//!  12:    Encode Option<Command> None, decode
//!  13:    Encode Vec<Command> with all variants, decode
//!  14:    Encode 10 u64 values sequentially, decode all
//!  15:    Encode large Vec<u8> (2048 bytes), decode
//!  16:    Encode struct with String field, decode
//!  17:    Encode tuple (u32, String), decode
//!  18:    Encode with big_endian + fixed_int config
//!  19:    Encode bool, decode (true and false)
//!  20:    Multi-type sequence: Packet then Command then u32
//!  21:    Encode empty Vec<u8>, decode
//!  22:    Encode multiple small values, verify consumed bytes match

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
    seq: u32,
    data: Vec<u8>,
    flags: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Command {
    Ping,
    Pong,
    Data(Vec<u8>),
    Error(String),
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
// Test 1: Encode/decode Packet async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_packet_roundtrip() {
    let original = Packet {
        seq: 42,
        data: vec![0x01, 0x02, 0x03, 0x04, 0x05],
        flags: 0b0000_1111,
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Packet>(buf).await;
    assert_eq!(decoded, Some(original), "Packet async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 2: Encode/decode Command::Ping async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_command_ping_roundtrip() {
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
// Test 3: Encode/decode Command::Pong async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_command_pong_roundtrip() {
    let original = Command::Pong;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Command>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Command::Pong async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Encode/decode Command::Data(vec) async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_command_data_roundtrip() {
    let original = Command::Data(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Command>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Command::Data(vec) async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Encode/decode Command::Error("msg") async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_command_error_roundtrip() {
    let original = Command::Error(String::from("connection reset by peer"));
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Command>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Command::Error(String) async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Encode 5 Packets sequentially, decode all 5
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_five_packets_sequential_roundtrip() {
    let packets: Vec<Packet> = (0u32..5)
        .map(|i| Packet {
            seq: i * 10,
            data: vec![i as u8; (i + 1) as usize],
            flags: (i as u8) << 1,
        })
        .collect();

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

    for (idx, expected) in packets.iter().enumerate() {
        let item: Option<Packet> = dec.read_item().await.expect("read Packet failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "packet at index {idx} mismatch"
        );
    }

    let eof: Option<Packet> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all 5 Packets decoded");
}

// ---------------------------------------------------------------------------
// Test 7: Encode Vec<u8> bytes, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_vec_u8_payload_roundtrip() {
    let original: Vec<u8> = vec![0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<u8> payload async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Encode i32 negative value, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_i32_negative_roundtrip() {
    let original: i32 = -987_654_321;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<i32>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "i32 negative async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Encode u128::MAX, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_u128_max_roundtrip() {
    let original: u128 = u128::MAX;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u128>(buf).await;
    assert_eq!(decoded, Some(original), "u128::MAX async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 10: Encode f64::PI, decode (bit-exact)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_f64_pi_bit_exact_roundtrip() {
    let original: f64 = std::f64::consts::PI;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<f64>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "f64::PI bit-exact async roundtrip failed"
    );
    // Extra verification: bits must match exactly
    if let Some(d) = decoded {
        assert_eq!(
            d.to_bits(),
            original.to_bits(),
            "f64::PI bit-level representation mismatch"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 11: Encode Option<Packet> Some, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_option_packet_some_roundtrip() {
    let original: Option<Packet> = Some(Packet {
        seq: 777,
        data: vec![1, 2, 3],
        flags: 0x0F,
    });
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<Packet>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<Packet> Some async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Encode Option<Command> None, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_option_command_none_roundtrip() {
    let original: Option<Command> = None;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<Command>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<Command> None async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Encode Vec<Command> with all variants, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_vec_command_all_variants_roundtrip() {
    let original: Vec<Command> = vec![
        Command::Ping,
        Command::Pong,
        Command::Data(vec![0x10, 0x20, 0x30]),
        Command::Error(String::from("timeout")),
        Command::Ping,
    ];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Command>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<Command> all variants async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Encode 10 u64 values sequentially, decode all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_ten_u64_values_sequential_roundtrip() {
    let values: Vec<u64> = (0u64..10).map(|i| (i + 1) * 1_000_000_007).collect();

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

    for (idx, &expected) in values.iter().enumerate() {
        let item: Option<u64> = dec.read_item().await.expect("read u64 failed");
        assert_eq!(
            item,
            Some(expected),
            "u64 at index {idx} mismatch (expected {expected})"
        );
    }

    let eof: Option<u64> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all 10 u64 values decoded");
}

// ---------------------------------------------------------------------------
// Test 15: Encode large Vec<u8> (2048 bytes), decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_large_vec_u8_2048_bytes_roundtrip() {
    let original: Vec<u8> = (0u8..=255).cycle().take(2048).collect();
    assert_eq!(original.len(), 2048, "original must be exactly 2048 bytes");

    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "large Vec<u8> (2048 bytes) async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Encode struct with String field, decode
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TaggedPacket {
    tag: String,
    packet: Packet,
}

#[tokio::test]
async fn test_async11_struct_with_string_field_roundtrip() {
    let original = TaggedPacket {
        tag: String::from("critical-frame"),
        packet: Packet {
            seq: 9999,
            data: vec![0xCA, 0xFE, 0xBA, 0xBE],
            flags: 0b1010_1010,
        },
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<TaggedPacket>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "struct with String field async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Encode tuple (u32, String), decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_tuple_u32_string_roundtrip() {
    let original: (u32, String) = (31415, String::from("tuple-async-11-pi"));
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<(u32, String)>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "(u32, String) tuple async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Encode with big_endian + fixed_int config
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_big_endian_fixed_int_config_roundtrip() {
    let value: u32 = 0xDE_AD_BE_EF;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    // Slice-level encode and verify byte order
    let be_bytes =
        encode_to_vec_with_config(&value, cfg).expect("big-endian+fixed-int encode failed");
    assert_eq!(
        be_bytes.len(),
        4,
        "big-endian+fixed-int u32 must encode to exactly 4 bytes"
    );
    assert_eq!(be_bytes[0], 0xDE, "big-endian byte[0] must be 0xDE");
    assert_eq!(be_bytes[1], 0xAD, "big-endian byte[1] must be 0xAD");
    assert_eq!(be_bytes[2], 0xBE, "big-endian byte[2] must be 0xBE");
    assert_eq!(be_bytes[3], 0xEF, "big-endian byte[3] must be 0xEF");

    // Roundtrip via slice with same config
    let (slice_decoded, _): (u32, _) =
        decode_from_slice_with_config(&be_bytes, cfg).expect("big-endian+fixed-int decode failed");
    assert_eq!(
        slice_decoded, value,
        "big-endian+fixed-int slice roundtrip mismatch"
    );

    // Async streaming roundtrip (uses its own default config internally)
    let buf = async_encode_single(&value).await;
    let async_decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(
        async_decoded,
        Some(value),
        "big-endian+fixed-int value async streaming roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Encode bool, decode (true and false)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_bool_true_and_false_roundtrip() {
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
// Test 20: Multi-type sequence: Packet then Command then u32
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_multi_type_packet_command_u32_sequence() {
    let pkt = Packet {
        seq: 1001,
        data: vec![0xAA, 0xBB],
        flags: 0x01,
    };
    let cmd = Command::Data(vec![0x11, 0x22, 0x33]);
    let num: u32 = 0xFEED_FACE;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&pkt).await.expect("write Packet failed");
        enc.write_item(&cmd).await.expect("write Command failed");
        enc.write_item(&num).await.expect("write u32 failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    let d_pkt: Option<Packet> = dec.read_item().await.expect("read Packet failed");
    assert_eq!(d_pkt, Some(pkt), "multi-type sequence: Packet mismatch");

    let d_cmd: Option<Command> = dec.read_item().await.expect("read Command failed");
    assert_eq!(d_cmd, Some(cmd), "multi-type sequence: Command mismatch");

    let d_num: Option<u32> = dec.read_item().await.expect("read u32 failed");
    assert_eq!(d_num, Some(num), "multi-type sequence: u32 mismatch");

    let eof: Option<u32> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after Packet+Command+u32 sequence");
}

// ---------------------------------------------------------------------------
// Test 21: Encode empty Vec<u8>, decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_empty_vec_u8_roundtrip() {
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
// Test 22: Encode multiple small values, verify consumed bytes match
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async11_multiple_small_values_consumed_bytes_match() {
    let v_u8: u8 = 0xAB;
    let v_u16: u16 = 0x1234;
    let v_u32: u32 = 0xDEAD_BEEF;
    let v_bool = true;

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&v_u8).await.expect("write u8 failed");
        enc.write_item(&v_u16).await.expect("write u16 failed");
        enc.write_item(&v_u32).await.expect("write u32 failed");
        enc.write_item(&v_bool).await.expect("write bool failed");
        enc.finish().await.expect("finish failed");
    }

    let total_len = buf.len();
    assert!(total_len > 0, "encoded buffer must not be empty");

    // Sync-encode all four values individually to verify cross-decode correctness.
    // Note: the async buffer is larger than the sum of sync-encoded sizes because
    // the async encoder wraps each item with a length-prefixed frame.
    let sync_bytes_u8 = encode_to_vec(&v_u8).expect("sync encode u8 failed");
    let sync_bytes_u16 = encode_to_vec(&v_u16).expect("sync encode u16 failed");
    let sync_bytes_u32 = encode_to_vec(&v_u32).expect("sync encode u32 failed");
    let sync_bytes_bool = encode_to_vec(&v_bool).expect("sync encode bool failed");

    let sum_of_sync_sizes =
        sync_bytes_u8.len() + sync_bytes_u16.len() + sync_bytes_u32.len() + sync_bytes_bool.len();
    assert!(
        total_len >= sum_of_sync_sizes,
        "async-encoded total ({total_len}) must be >= sum of sync-encoded parts ({sum_of_sync_sizes})"
    );

    // Async decode and verify all values
    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    let d_u8: Option<u8> = dec.read_item().await.expect("read u8 failed");
    assert_eq!(d_u8, Some(v_u8), "consumed-bytes check: u8 mismatch");

    let d_u16: Option<u16> = dec.read_item().await.expect("read u16 failed");
    assert_eq!(d_u16, Some(v_u16), "consumed-bytes check: u16 mismatch");

    let d_u32: Option<u32> = dec.read_item().await.expect("read u32 failed");
    assert_eq!(d_u32, Some(v_u32), "consumed-bytes check: u32 mismatch");

    let d_bool: Option<bool> = dec.read_item().await.expect("read bool failed");
    assert_eq!(d_bool, Some(v_bool), "consumed-bytes check: bool mismatch");

    // bytes_processed tracks payload bytes consumed; must be > 0 and <= total framed length.
    let bytes_processed = dec.progress().bytes_processed;
    let total_len_u64 = u64::try_from(total_len).expect("total_len must fit in u64");
    assert!(
        bytes_processed > 0,
        "bytes_processed must be > 0 after decoding 4 values (was 0)"
    );
    assert!(
        bytes_processed <= total_len_u64,
        "bytes_processed ({bytes_processed}) must not exceed total encoded buffer size ({total_len})"
    );

    // items_processed must be 4
    assert_eq!(
        dec.progress().items_processed,
        4,
        "items_processed must be 4 after reading all 4 values"
    );

    let eof: Option<u8> = dec.read_item().await.expect("eof read failed");
    assert_eq!(
        eof, None,
        "decoder must return None once all values consumed"
    );
    assert!(
        dec.is_finished(),
        "decoder must report is_finished() == true after all items consumed"
    );

    // Cross-check sync decode of the individual slices
    let (sync_u8, _): (u8, _) = decode_from_slice(&sync_bytes_u8).expect("sync decode u8 failed");
    assert_eq!(sync_u8, v_u8, "sync decode u8 cross-check failed");

    let (sync_u16, _): (u16, _) =
        decode_from_slice(&sync_bytes_u16).expect("sync decode u16 failed");
    assert_eq!(sync_u16, v_u16, "sync decode u16 cross-check failed");

    let (sync_u32, _): (u32, _) =
        decode_from_slice(&sync_bytes_u32).expect("sync decode u32 failed");
    assert_eq!(sync_u32, v_u32, "sync decode u32 cross-check failed");
}
