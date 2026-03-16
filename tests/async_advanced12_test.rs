//! Advanced async streaming tests (twelfth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `Metric`, `Event`, and `Config`.
//!
//! Coverage matrix:
//!   1:    u32 async roundtrip
//!   2:    String async roundtrip
//!   3:    Vec<u8> async roundtrip
//!   4:    bool true async roundtrip
//!   5:    bool false async roundtrip
//!   6:    f64 async roundtrip
//!   7:    u64::MAX async roundtrip
//!   8:    Metric struct async roundtrip
//!   9:    Event::Start async roundtrip
//!  10:    Event::Stop async roundtrip
//!  11:    Event::Data async roundtrip
//!  12:    Config struct async roundtrip
//!  13:    Vec<u32> async roundtrip
//!  14:    Option<String> Some async roundtrip
//!  15:    Option<String> None async roundtrip
//!  16:    Vec<Metric> multiple structs async roundtrip
//!  17:    Sequential two u32 writes then reads
//!  18:    u128 async roundtrip
//!  19:    Vec<String> async roundtrip
//!  20:    i64::MIN async roundtrip
//!  21:    Vec<Event> async roundtrip
//!  22:    Async encoded bytes match encode_to_vec bytes

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

// ---------------------------------------------------------------------------
// Types unique to this file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Metric {
    name: String,
    value: f64,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Event {
    Start,
    Stop,
    Data(Vec<u8>),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Config {
    id: u32,
    enabled: bool,
    thresholds: Vec<f32>,
}

// ---------------------------------------------------------------------------
// Helper: encode a single item via AsyncEncoder using a tokio duplex pipe.
// The writer half is dropped after encoding so the reader sees EOF on the
// streaming framing layer.
// ---------------------------------------------------------------------------

async fn duplex_encode_single<T: Encode>(item: &T) -> Vec<u8> {
    use tokio::io::AsyncReadExt;

    let (writer, mut reader) = tokio::io::duplex(4096);
    let mut encoder = AsyncEncoder::new(writer);
    encoder
        .write_item(item)
        .await
        .expect("duplex_encode_single: write_item failed");
    encoder
        .finish()
        .await
        .expect("duplex_encode_single: finish failed");

    let mut buf = Vec::new();
    reader
        .read_to_end(&mut buf)
        .await
        .expect("duplex_encode_single: read_to_end failed");
    buf
}

async fn duplex_decode_single<T: Decode>(encoded: Vec<u8>) -> Option<T> {
    use std::io::Cursor;
    use tokio::io::BufReader;

    // Wrap in a Cursor so the decoder sees an in-memory stream with proper EOF.
    let cursor = Cursor::new(encoded);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);
    decoder
        .read_item::<T>()
        .await
        .expect("duplex_decode_single: read_item failed")
}

// ---------------------------------------------------------------------------
// Test 1: u32 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_u32_roundtrip() {
    let val: u32 = 0xDEAD_BEEF;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<u32>(encoded).await;
    assert_eq!(decoded, Some(val), "u32 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 2: String async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_string_roundtrip() {
    let val = String::from("oxicode-async-advanced-12");
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<String>(encoded).await;
    assert_eq!(decoded, Some(val), "String async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 3: Vec<u8> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_vec_u8_roundtrip() {
    let val: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0xFF, 0x7F, 0x80];
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Vec<u8>>(encoded).await;
    assert_eq!(decoded, Some(val), "Vec<u8> async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 4: bool true async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_bool_true_roundtrip() {
    let val = true;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<bool>(encoded).await;
    assert_eq!(decoded, Some(true), "bool true async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 5: bool false async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_bool_false_roundtrip() {
    let val = false;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<bool>(encoded).await;
    assert_eq!(decoded, Some(false), "bool false async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 6: f64 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_f64_roundtrip() {
    let val: f64 = std::f64::consts::E;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<f64>(encoded).await;
    assert_eq!(decoded, Some(val), "f64 async roundtrip failed");
    if let Some(d) = decoded {
        assert_eq!(
            d.to_bits(),
            val.to_bits(),
            "f64 bit-exact representation mismatch"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 7: u64::MAX async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_u64_max_roundtrip() {
    let val: u64 = u64::MAX;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<u64>(encoded).await;
    assert_eq!(decoded, Some(u64::MAX), "u64::MAX async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 8: Metric struct async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_metric_struct_roundtrip() {
    let val = Metric {
        name: String::from("cpu.usage"),
        value: 87.5_f64,
        tags: vec![
            String::from("host:server-01"),
            String::from("env:production"),
        ],
    };
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Metric>(encoded).await;
    assert_eq!(decoded, Some(val), "Metric struct async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 9: Event::Start async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_event_start_roundtrip() {
    let val = Event::Start;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Event>(encoded).await;
    assert_eq!(
        decoded,
        Some(Event::Start),
        "Event::Start async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Event::Stop async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_event_stop_roundtrip() {
    let val = Event::Stop;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Event>(encoded).await;
    assert_eq!(
        decoded,
        Some(Event::Stop),
        "Event::Stop async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Event::Data async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_event_data_roundtrip() {
    let val = Event::Data(vec![0x01, 0x02, 0x03, 0xAB, 0xCD, 0xEF]);
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Event>(encoded).await;
    assert_eq!(
        decoded,
        Some(Event::Data(vec![0x01, 0x02, 0x03, 0xAB, 0xCD, 0xEF])),
        "Event::Data async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Config struct async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_config_struct_roundtrip() {
    let val = Config {
        id: 42,
        enabled: true,
        thresholds: vec![0.1_f32, 0.5_f32, 0.9_f32, 1.0_f32],
    };
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Config>(encoded).await;
    assert_eq!(decoded, Some(val), "Config struct async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 13: Vec<u32> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_vec_u32_roundtrip() {
    let val: Vec<u32> = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Vec<u32>>(encoded).await;
    assert_eq!(decoded, Some(val), "Vec<u32> async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 14: Option<String> Some async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_option_string_some_roundtrip() {
    let val: Option<String> = Some(String::from("present-value"));
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Option<String>>(encoded).await;
    assert_eq!(
        decoded,
        Some(Some(String::from("present-value"))),
        "Option<String> Some async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Option<String> None async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_option_string_none_roundtrip() {
    let val: Option<String> = None;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Option<String>>(encoded).await;
    assert_eq!(
        decoded,
        Some(None),
        "Option<String> None async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Vec<Metric> multiple structs async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_vec_metric_roundtrip() {
    let val: Vec<Metric> = vec![
        Metric {
            name: String::from("mem.used"),
            value: 1024.0_f64,
            tags: vec![String::from("host:node-1")],
        },
        Metric {
            name: String::from("disk.io"),
            value: 256.75_f64,
            tags: vec![String::from("device:sda"), String::from("mode:read")],
        },
        Metric {
            name: String::from("net.rx"),
            value: 0.0_f64,
            tags: vec![],
        },
    ];
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Vec<Metric>>(encoded).await;
    assert_eq!(
        decoded,
        Some(val),
        "Vec<Metric> multiple structs async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Sequential two u32 writes then reads
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_sequential_two_u32_writes_reads() {
    use std::io::Cursor;
    use tokio::io::BufReader;

    let first: u32 = 111_222_333;
    let second: u32 = 444_555_666;

    // Encode both values into an in-memory buffer via a Cursor writer
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncEncoder::new(cursor);
        encoder
            .write_item(&first)
            .await
            .expect("write first u32 failed");
        encoder
            .write_item(&second)
            .await
            .expect("write second u32 failed");
        encoder.finish().await.expect("finish failed");
    }

    // Decode sequentially from the same buffer
    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    let decoded_first: Option<u32> = decoder.read_item().await.expect("read first u32 failed");
    assert_eq!(decoded_first, Some(first), "sequential first u32 mismatch");

    let decoded_second: Option<u32> = decoder.read_item().await.expect("read second u32 failed");
    assert_eq!(
        decoded_second,
        Some(second),
        "sequential second u32 mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 18: u128 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_u128_roundtrip() {
    let val: u128 = u128::MAX / 2 + 1;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<u128>(encoded).await;
    assert_eq!(decoded, Some(val), "u128 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 19: Vec<String> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_vec_string_roundtrip() {
    let val: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
        String::from("delta"),
        String::from("epsilon"),
    ];
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Vec<String>>(encoded).await;
    assert_eq!(decoded, Some(val), "Vec<String> async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 20: i64::MIN async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_i64_min_roundtrip() {
    let val: i64 = i64::MIN;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<i64>(encoded).await;
    assert_eq!(decoded, Some(i64::MIN), "i64::MIN async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 21: Vec<Event> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_vec_event_roundtrip() {
    let val: Vec<Event> = vec![
        Event::Start,
        Event::Data(vec![0xDE, 0xAD]),
        Event::Stop,
        Event::Start,
        Event::Data(vec![]),
    ];
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Vec<Event>>(encoded).await;
    assert_eq!(
        decoded,
        Some(vec![
            Event::Start,
            Event::Data(vec![0xDE, 0xAD]),
            Event::Stop,
            Event::Start,
            Event::Data(vec![]),
        ]),
        "Vec<Event> async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Async encoded bytes match encode_to_vec bytes
//
// Verifies that a value encoded via the async streaming path produces the
// same raw payload bytes as the synchronous `encode_to_vec` API.
// The async framing adds a chunk header around the payload, so we cannot
// compare the full buffers directly.  Instead we decode the async-encoded
// bytes back to a value and then sync-encode that value, checking equality.
// We also verify the sync bytes are present verbatim inside the async buffer.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async12_async_encoded_bytes_match_encode_to_vec() {
    let val: u32 = 0x1234_5678;

    // Sync encode
    let sync_bytes = encode_to_vec(&val).expect("sync encode_to_vec failed");

    // Async encode via duplex helper
    let async_bytes = duplex_encode_single(&val).await;

    // The async buffer contains the streaming framing (chunk header) wrapping the
    // same payload bytes as sync_bytes.  Check that sync_bytes appears inside async_bytes.
    let payload_present = async_bytes
        .windows(sync_bytes.len())
        .any(|w| w == sync_bytes.as_slice());
    assert!(
        payload_present,
        "async-encoded buffer must contain the sync-encoded payload bytes verbatim"
    );

    // Round-trip via decode_from_slice on the sync bytes
    let (sync_decoded, consumed): (u32, _) =
        decode_from_slice(&sync_bytes).expect("decode_from_slice failed");
    assert_eq!(sync_decoded, val, "sync decode roundtrip mismatch");
    assert_eq!(consumed, sync_bytes.len(), "consumed byte count mismatch");

    // The async buffer must decode back to the same value
    let async_decoded = duplex_decode_single::<u32>(async_bytes).await;
    assert_eq!(
        async_decoded,
        Some(val),
        "async decode of async-encoded bytes mismatch"
    );

    // Verify both encode to the same sync bytes (cross-check)
    let re_encoded =
        encode_to_vec(&async_decoded.expect("async_decoded was None")).expect("re-encode failed");
    assert_eq!(
        re_encoded, sync_bytes,
        "re-encoded bytes do not match original sync bytes"
    );

    // Confirm config::standard() is accessible (referenced in task spec)
    let _cfg = config::standard();
}
