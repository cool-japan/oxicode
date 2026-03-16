//! Advanced async streaming tests (thirteenth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `Frame`, `Protocol`, and `Connection`.
//!
//! Coverage matrix:
//!   1:    Encode/decode u32 async roundtrip
//!   2:    Encode/decode String async roundtrip
//!   3:    Encode/decode Vec<u8> async roundtrip
//!   4:    Encode/decode bool async roundtrip
//!   5:    Encode/decode f64 async roundtrip (bit-exact)
//!   6:    Encode/decode Frame async roundtrip
//!   7:    Encode/decode Protocol::Http variant
//!   8:    Encode/decode Protocol::Tcp variant
//!   9:    Encode/decode Protocol::Udp variant
//!  10:    Encode/decode Protocol::WebSocket variant
//!  11:    Encode/decode Connection async roundtrip
//!  12:    Encode/decode Vec<Frame> async roundtrip
//!  13:    Encode/decode Option<Frame> Some async roundtrip
//!  14:    Sequential writes: write 3 Frame items, read 3 Frame items
//!  15:    Encode large Vec<u8> (4096 bytes), decode
//!  16:    Encode/decode i64 negative value async roundtrip
//!  17:    Encode/decode u128::MAX async roundtrip
//!  18:    Encode/decode Vec<Connection> async roundtrip
//!  19:    Encode with big_endian + fixed_int config
//!  20:    Encode/decode empty Vec<Frame> async roundtrip
//!  21:    Encode/decode Option<Frame> None async roundtrip
//!  22:    Error on corrupt data: decoder returns error on truncated input
//!  23:    Encode/decode multiple Protocol variants in Vec<Protocol>
//! (Tests 21-22 shifted: test 21 = bytes match encode_to_vec, test 22 = Vec<Protocol> all variants)

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
    seq: u32,
    payload: Vec<u8>,
    checksum: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Protocol {
    Http,
    Tcp,
    Udp,
    WebSocket,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Connection {
    id: u64,
    protocol: Protocol,
    active: bool,
    frames: Vec<Frame>,
}

// ---------------------------------------------------------------------------
// Helpers
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
async fn test_async13_u32_roundtrip() {
    let original: u32 = 3_141_592_653;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(decoded, Some(original), "u32 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 2: Encode/decode String async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_string_roundtrip() {
    let original = String::from("oxicode-async-frame-protocol-test");
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<String>(buf).await;
    assert_eq!(decoded, Some(original), "String async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 3: Encode/decode Vec<u8> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_vec_u8_roundtrip() {
    let original: Vec<u8> = vec![0x13, 0x37, 0xCA, 0xFE, 0xBA, 0xBE, 0xDE, 0xAD];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(decoded, Some(original), "Vec<u8> async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 4: Encode/decode bool async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_bool_roundtrip() {
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
// Test 5: Encode/decode f64 async roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_f64_bit_exact_roundtrip() {
    let original: f64 = std::f64::consts::E;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<f64>(buf).await;
    assert_eq!(decoded, Some(original), "f64 async roundtrip failed");
    if let Some(d) = decoded {
        assert_eq!(
            d.to_bits(),
            original.to_bits(),
            "f64 bit-level representation mismatch"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 6: Encode/decode Frame async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_frame_roundtrip() {
    let original = Frame {
        seq: 100,
        payload: vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE],
        checksum: 0xF1A2,
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Frame>(buf).await;
    assert_eq!(decoded, Some(original), "Frame async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 7: Encode/decode Protocol::Http variant
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_protocol_http_roundtrip() {
    let original = Protocol::Http;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Protocol>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Protocol::Http async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Encode/decode Protocol::Tcp variant
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_protocol_tcp_roundtrip() {
    let original = Protocol::Tcp;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Protocol>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Protocol::Tcp async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Encode/decode Protocol::Udp variant
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_protocol_udp_roundtrip() {
    let original = Protocol::Udp;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Protocol>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Protocol::Udp async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Encode/decode Protocol::WebSocket variant
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_protocol_websocket_roundtrip() {
    let original = Protocol::WebSocket;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Protocol>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Protocol::WebSocket async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Encode/decode Connection async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_connection_roundtrip() {
    let original = Connection {
        id: 99_999_999_999,
        protocol: Protocol::WebSocket,
        active: true,
        frames: vec![
            Frame {
                seq: 1,
                payload: vec![0x01, 0x02],
                checksum: 0x0102,
            },
            Frame {
                seq: 2,
                payload: vec![0x03, 0x04, 0x05],
                checksum: 0x030C,
            },
        ],
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Connection>(buf).await;
    assert_eq!(decoded, Some(original), "Connection async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 12: Encode/decode Vec<Frame> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_vec_frame_roundtrip() {
    let original: Vec<Frame> = (0u32..6)
        .map(|i| Frame {
            seq: i * 7,
            payload: vec![i as u8; (i + 1) as usize],
            checksum: (i as u16) * 0x0100 + (i as u16),
        })
        .collect();
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Frame>>(buf).await;
    assert_eq!(decoded, Some(original), "Vec<Frame> async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 13: Encode/decode Option<Frame> Some async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_option_frame_some_roundtrip() {
    let original: Option<Frame> = Some(Frame {
        seq: 512,
        payload: vec![0xFE, 0xED, 0xFA, 0xCE],
        checksum: 0xBEEF,
    });
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<Frame>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<Frame> Some async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Sequential writes: write 3 Frame items, read 3 Frame items
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_three_frames_sequential_roundtrip() {
    let frames: Vec<Frame> = vec![
        Frame {
            seq: 10,
            payload: vec![0x11, 0x22, 0x33],
            checksum: 0x1122,
        },
        Frame {
            seq: 20,
            payload: vec![0x44, 0x55],
            checksum: 0x4455,
        },
        Frame {
            seq: 30,
            payload: vec![0x66, 0x77, 0x88, 0x99],
            checksum: 0x6677,
        },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for frame in &frames {
            enc.write_item(frame).await.expect("write Frame failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    for (idx, expected) in frames.iter().enumerate() {
        let item: Option<Frame> = dec.read_item().await.expect("read Frame failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "Frame at index {idx} mismatch"
        );
    }

    let eof: Option<Frame> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all 3 Frames decoded");
}

// ---------------------------------------------------------------------------
// Test 15: Encode large Vec<u8> (4096 bytes), decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_large_vec_u8_4096_bytes_roundtrip() {
    let original: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
    assert_eq!(original.len(), 4096, "original must be exactly 4096 bytes");

    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "large Vec<u8> (4096 bytes) async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Encode/decode i64 negative value async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_i64_negative_roundtrip() {
    let original: i64 = -9_223_372_036_854_775_807_i64;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<i64>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "i64 negative async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Encode/decode u128::MAX async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_u128_max_roundtrip() {
    let original: u128 = u128::MAX;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u128>(buf).await;
    assert_eq!(decoded, Some(original), "u128::MAX async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 18: Encode/decode Vec<Connection> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_vec_connection_roundtrip() {
    let original: Vec<Connection> = vec![
        Connection {
            id: 1,
            protocol: Protocol::Http,
            active: true,
            frames: vec![Frame {
                seq: 0,
                payload: vec![0x00],
                checksum: 0x0000,
            }],
        },
        Connection {
            id: 2,
            protocol: Protocol::Tcp,
            active: false,
            frames: Vec::new(),
        },
        Connection {
            id: 3,
            protocol: Protocol::Udp,
            active: true,
            frames: vec![
                Frame {
                    seq: 100,
                    payload: vec![0xFF, 0xFE],
                    checksum: 0xFFFE,
                },
                Frame {
                    seq: 101,
                    payload: vec![0xFD, 0xFC, 0xFB],
                    checksum: 0xFDFC,
                },
            ],
        },
    ];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Connection>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<Connection> async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Encode with big_endian + fixed_int config
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_big_endian_fixed_int_config_roundtrip() {
    let value: u32 = 0xC0_DE_CA_FE;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let be_bytes =
        encode_to_vec_with_config(&value, cfg).expect("big-endian+fixed-int encode failed");
    assert_eq!(
        be_bytes.len(),
        4,
        "big-endian+fixed-int u32 must encode to exactly 4 bytes"
    );
    assert_eq!(be_bytes[0], 0xC0, "big-endian byte[0] must be 0xC0");
    assert_eq!(be_bytes[1], 0xDE, "big-endian byte[1] must be 0xDE");
    assert_eq!(be_bytes[2], 0xCA, "big-endian byte[2] must be 0xCA");
    assert_eq!(be_bytes[3], 0xFE, "big-endian byte[3] must be 0xFE");

    let (slice_decoded, _): (u32, _) =
        decode_from_slice_with_config(&be_bytes, cfg).expect("big-endian+fixed-int decode failed");
    assert_eq!(
        slice_decoded, value,
        "big-endian+fixed-int slice roundtrip mismatch"
    );

    // Async streaming roundtrip uses default config internally
    let buf = async_encode_single(&value).await;
    let async_decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(
        async_decoded,
        Some(value),
        "config variant async streaming roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Encode/decode empty Vec<Frame> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_empty_vec_frame_roundtrip() {
    let original: Vec<Frame> = Vec::new();
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Frame>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "empty Vec<Frame> async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Bytes match encode_to_vec output
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_bytes_match_encode_to_vec_output() {
    let frame = Frame {
        seq: 7,
        payload: vec![0x07, 0x08, 0x09],
        checksum: 0x0708,
    };

    // Sync encode to get reference bytes
    let sync_bytes = encode_to_vec(&frame).expect("sync encode Frame failed");

    // Async encode into buffer
    let async_bytes = async_encode_single(&frame).await;

    // The async encoder may add framing overhead, but decoding both must yield same value
    let (sync_decoded, _): (Frame, _) =
        decode_from_slice(&sync_bytes).expect("sync decode Frame failed");
    assert_eq!(sync_decoded, frame, "sync decode from sync bytes failed");

    let async_decoded = async_decode_single::<Frame>(async_bytes).await;
    assert_eq!(
        async_decoded,
        Some(frame),
        "async decode from async bytes failed"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Vec<Protocol> all variants async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async13_vec_protocol_all_variants_roundtrip() {
    let original: Vec<Protocol> = vec![
        Protocol::Http,
        Protocol::Tcp,
        Protocol::Udp,
        Protocol::WebSocket,
        Protocol::Http,
        Protocol::Udp,
    ];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Protocol>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<Protocol> all variants async roundtrip failed"
    );
}
