//! WP-C streaming & async hardening regression tests.
//!
//! Covers the fixes for TODO.md lines 265–324:
//! chunk-size DoS bound, truncation detection, cancellation safety, generic
//! codec config threading, Metadata / zero-item chunk skipping, zero-sized item
//! round-trip, encode-time size enforcement, backpressure, and decoder poisoning.

#![cfg(all(feature = "async-tokio", feature = "std"))]
use oxicode::streaming::{
    BufferStreamingDecoder, BufferStreamingEncoder, ChunkHeader, StreamingConfig, StreamingDecoder,
    StreamingEncoder, MAX_CHUNK_SIZE,
};
use oxicode::Error;
use std::io::Cursor;

/// Build the encoded byte payload for a single item using the standard codec.
fn item_bytes(v: u32) -> Vec<u8> {
    oxicode::encode_to_vec(&v).expect("encode item")
}

// ─────────────────────────── Allocation-DoS bound ───────────────────────────

#[test]
fn forged_giant_chunk_header_is_rejected_before_allocating_std() {
    // 'OXIS' + type Data + payload_len = 0xFFFF_FFFF + item_count = 1.
    let forged = ChunkHeader::data(u32::MAX, 1).to_bytes();
    let mut decoder = StreamingDecoder::new(Cursor::new(forged.to_vec()));
    let result = decoder.read_item::<u32>();
    match result {
        Err(Error::LimitExceeded { limit, found }) => {
            assert!(limit as usize <= MAX_CHUNK_SIZE);
            assert_eq!(found, u32::MAX as u64);
        }
        other => panic!("expected LimitExceeded, got {other:?}"),
    }
}

#[test]
fn forged_giant_chunk_header_is_rejected_before_allocating_buffer() {
    let forged = ChunkHeader::data(u32::MAX, 1).to_bytes();
    let mut decoder = BufferStreamingDecoder::new(&forged);
    match decoder.read_item::<u32>() {
        Err(Error::LimitExceeded { found, .. }) => assert_eq!(found, u32::MAX as u64),
        other => panic!("expected LimitExceeded, got {other:?}"),
    }
}

#[test]
fn max_buffer_size_backpressure_bounds_decode() {
    // Encode a chunk whose payload is a few hundred bytes.
    let mut enc = BufferStreamingEncoder::new();
    for i in 0..100u32 {
        enc.write_item(&i).expect("write");
    }
    let encoded = enc.finish();

    // A decoder configured with a tiny max_buffer_size must refuse the chunk.
    let tight = StreamingConfig::new().with_max_buffer(16);
    let mut decoder =
        BufferStreamingDecoder::new_with_configs(&encoded, tight, oxicode::config::standard());
    match decoder.read_item::<u32>() {
        Err(Error::LimitExceeded { limit, .. }) => assert_eq!(limit, 16),
        other => panic!("expected LimitExceeded from backpressure, got {other:?}"),
    }
}

// ─────────────────────────── Truncation detection ───────────────────────────

#[test]
fn truncation_without_end_chunk_is_an_error_buffer() {
    let mut enc = BufferStreamingEncoder::new();
    for i in 0..10u32 {
        enc.write_item(&i).expect("write");
    }
    let encoded = enc.finish();

    // Truncate at several offsets; every prefix that drops the End chunk must
    // error rather than silently returning a short vec.
    for cut in [
        encoded.len() - 1,
        encoded.len() - ChunkHeader::SIZE,
        encoded.len() / 2,
        ChunkHeader::SIZE + 1,
    ] {
        let truncated = &encoded[..cut];
        let mut decoder = BufferStreamingDecoder::new(truncated);
        let result: Result<Vec<u32>, _> = decoder.read_all();
        assert!(
            result.is_err(),
            "truncation at {cut} must be reported as an error"
        );
    }
}

#[test]
fn truncation_without_end_chunk_is_an_error_std() {
    let mut buffer = Vec::new();
    {
        let mut enc = StreamingEncoder::new(&mut buffer);
        for i in 0..10u32 {
            enc.write_item(&i).expect("write");
        }
        enc.finish().expect("finish");
    }
    // Drop the trailing End chunk entirely.
    let truncated = buffer[..buffer.len() - ChunkHeader::SIZE].to_vec();
    let mut decoder = StreamingDecoder::new(Cursor::new(truncated));
    let result: Result<Vec<u32>, _> = decoder.read_all();
    assert!(result.is_err(), "missing End chunk must be an error");
}

// ─────────────────────── Metadata / zero-item skipping ──────────────────────

/// Hand-craft [Data(1,2)][Metadata][Data(3,4)][End] and assert all four items
/// are read back, i.e. the Metadata chunk does not truncate the stream.
fn crafted_stream_with_metadata() -> Vec<u8> {
    let mut stream = Vec::new();

    let mut payload_a = Vec::new();
    payload_a.extend_from_slice(&item_bytes(1));
    payload_a.extend_from_slice(&item_bytes(2));
    stream.extend_from_slice(&ChunkHeader::data(payload_a.len() as u32, 2).to_bytes());
    stream.extend_from_slice(&payload_a);

    let meta = [0xAAu8, 0xBB, 0xCC];
    stream.extend_from_slice(&ChunkHeader::metadata(meta.len() as u32).to_bytes());
    stream.extend_from_slice(&meta);

    let mut payload_b = Vec::new();
    payload_b.extend_from_slice(&item_bytes(3));
    payload_b.extend_from_slice(&item_bytes(4));
    stream.extend_from_slice(&ChunkHeader::data(payload_b.len() as u32, 2).to_bytes());
    stream.extend_from_slice(&payload_b);

    stream.extend_from_slice(&ChunkHeader::end().to_bytes());
    stream
}

#[test]
fn metadata_chunk_is_skipped_buffer() {
    let stream = crafted_stream_with_metadata();
    let mut decoder = BufferStreamingDecoder::new(&stream);
    let got: Vec<u32> = decoder.read_all().expect("read_all");
    assert_eq!(got, vec![1, 2, 3, 4]);
}

#[test]
fn metadata_chunk_is_skipped_std() {
    let stream = crafted_stream_with_metadata();
    let mut decoder = StreamingDecoder::new(Cursor::new(stream));
    let got: Vec<u32> = decoder.read_all().expect("read_all");
    assert_eq!(got, vec![1, 2, 3, 4]);
}

// ───────────────────────────── Zero-sized items ─────────────────────────────

#[test]
fn zero_sized_items_survive_roundtrip_buffer() {
    let mut enc = BufferStreamingEncoder::new();
    for _ in 0..1000 {
        enc.write_item(&()).expect("write unit");
    }
    let encoded = enc.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let got: Vec<()> = decoder.read_all().expect("read_all");
    assert_eq!(got.len(), 1000, "all zero-sized items must be preserved");
}

#[test]
fn zero_sized_items_survive_roundtrip_std() {
    let mut buffer = Vec::new();
    {
        let mut enc = StreamingEncoder::new(&mut buffer);
        for _ in 0..1000 {
            enc.write_item(&()).expect("write unit");
        }
        enc.finish().expect("finish");
    }
    let mut decoder = StreamingDecoder::new(Cursor::new(buffer));
    let got: Vec<()> = decoder.read_all().expect("read_all");
    assert_eq!(got.len(), 1000);
}

// ───────────────────────── Encode-time size enforcement ─────────────────────

#[test]
fn oversized_single_item_is_rejected_at_encode() {
    // A Vec<u8> larger than MAX_CHUNK_SIZE cannot fit in a single chunk.
    let big: Vec<u8> = vec![0u8; MAX_CHUNK_SIZE + 1];
    let mut enc = BufferStreamingEncoder::new();
    match enc.write_item(&big) {
        Err(Error::LimitExceeded { limit, found }) => {
            assert_eq!(limit, MAX_CHUNK_SIZE as u64);
            assert!(found > MAX_CHUNK_SIZE as u64);
        }
        other => panic!("expected LimitExceeded for oversized item, got {other:?}"),
    }
}

// ────────────────────────────── Decoder poison ──────────────────────────────

#[test]
fn decoder_is_poisoned_after_error_and_retry_fails_deterministically() {
    // Craft a stream whose first chunk claims more payload than is present.
    let mut stream = Vec::new();
    stream.extend_from_slice(&ChunkHeader::data(64, 1).to_bytes());
    stream.extend_from_slice(&[0u8; 4]); // far fewer than 64 payload bytes

    let mut decoder = BufferStreamingDecoder::new(&stream);
    let first = decoder.read_item::<u32>();
    assert!(first.is_err(), "short payload must error");

    // A retry must return a deterministic failed-state error, never Ok/None
    // (which could misframe payload bytes as a header).
    match decoder.read_item::<u32>() {
        Err(Error::InvalidData { message }) => {
            assert!(message.contains("failed state"));
        }
        other => panic!("expected poisoned failed-state error, got {other:?}"),
    }
}

// ─────────────────────── Generic codec config threading ─────────────────────

#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn async_encoder_decoder_honor_fixed_int_config() {
    use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder};

    let codec = oxicode::config::standard().with_fixed_int_encoding();

    let mut buffer = Vec::new();
    {
        let cursor = Cursor::new(&mut buffer);
        let mut encoder = AsyncStreamingEncoder::new_with_config(cursor, codec);
        for i in 0u32..25 {
            encoder.write_item(&i).await.expect("write");
        }
        encoder.finish().await.expect("finish");
    }

    let mut decoder = AsyncStreamingDecoder::new_with_config(Cursor::new(buffer), codec);
    let got: Vec<u32> = decoder.read_all().await.expect("read_all");
    assert_eq!(got, (0..25).collect::<Vec<_>>());
}

#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn sync_fixed_int_encode_decodes_through_async() {
    use oxicode::streaming::AsyncStreamingDecoder;

    let codec = oxicode::config::standard().with_fixed_int_encoding();

    let mut buffer = Vec::new();
    {
        let mut encoder = StreamingEncoder::new_with_config(&mut buffer, codec);
        for i in 0u32..15 {
            encoder.write_item(&i).expect("write");
        }
        encoder.finish().expect("finish");
    }

    let mut decoder = AsyncStreamingDecoder::new_with_config(Cursor::new(buffer), codec);
    let got: Vec<u32> = decoder.read_all().await.expect("read_all");
    assert_eq!(got, (0..15).collect::<Vec<_>>());
}

// ───────────────────────── Async cancellation safety ────────────────────────

/// Dropping a `read_item` future mid-header must not lose the bytes already
/// pulled from the reader: the next call resumes and decodes correctly.
#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn async_decoder_read_is_cancellation_safe() {
    use oxicode::streaming::AsyncStreamingDecoder;
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;

    // Full encoded stream for three items (values chosen for multi-byte varints).
    let mut enc = BufferStreamingEncoder::new();
    enc.write_item(&100_000u32).expect("write");
    enc.write_item(&200_000u32).expect("write");
    enc.write_item(&300_000u32).expect("write");
    let full = enc.finish();

    let (mut tx, rx) = tokio::io::duplex(4096);
    let mut decoder = AsyncStreamingDecoder::new(rx);

    // Deliver a partial header (5 of 13 bytes), then interrupt a read.
    tx.write_all(&full[..5]).await.expect("write partial");
    let interrupted =
        tokio::time::timeout(Duration::from_millis(50), decoder.read_item::<u32>()).await;
    assert!(interrupted.is_err(), "read must be interrupted mid-header");

    // Deliver the remainder and resume: no bytes lost, decode is correct.
    tx.write_all(&full[5..]).await.expect("write rest");

    let a: Option<u32> = decoder.read_item().await.expect("resume a");
    let b: Option<u32> = decoder.read_item().await.expect("resume b");
    let c: Option<u32> = decoder.read_item().await.expect("resume c");
    let d: Option<u32> = decoder.read_item().await.expect("end");
    assert_eq!(
        (a, b, c, d),
        (Some(100_000), Some(200_000), Some(300_000), None)
    );
}

/// A gated writer that accepts only a bounded number of total bytes, then
/// stalls (returns `Pending`) until the gate is opened — used to interrupt a
/// write mid-flush and verify the encoder resumes without corrupting the frame.
#[cfg(feature = "async-tokio")]
struct GatedWriter {
    sink: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    allow: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

#[cfg(feature = "async-tokio")]
impl tokio::io::AsyncWrite for GatedWriter {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        use std::sync::atomic::Ordering;
        let this = self.get_mut();
        let mut sink = this.sink.lock().expect("sink lock");
        let allow = this.allow.load(Ordering::SeqCst);
        if sink.len() >= allow {
            return std::task::Poll::Pending;
        }
        let can = (allow - sink.len()).min(buf.len());
        sink.extend_from_slice(&buf[..can]);
        std::task::Poll::Ready(Ok(can))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }
}

#[cfg(feature = "async-tokio")]
#[tokio::test]
async fn async_encoder_write_is_cancellation_safe() {
    use oxicode::streaming::AsyncStreamingEncoder;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    let sink = Arc::new(Mutex::new(Vec::new()));
    let allow = Arc::new(AtomicUsize::new(4)); // accept only 4 bytes, then stall
    let writer = GatedWriter {
        sink: sink.clone(),
        allow: allow.clone(),
    };

    // flush_per_item so the first write triggers a flush that stalls mid-frame.
    let config = StreamingConfig::new().with_flush_per_item(true);
    let mut encoder = AsyncStreamingEncoder::with_config(writer, config);

    let interrupted =
        tokio::time::timeout(Duration::from_millis(50), encoder.write_item(&111u32)).await;
    assert!(interrupted.is_err(), "write must stall and be interrupted");

    // Open the gate; subsequent calls first drain the interrupted frame.
    allow.store(usize::MAX, Ordering::SeqCst);
    encoder.write_item(&222u32).await.expect("resume write 222");
    encoder.write_item(&333u32).await.expect("write 333");
    encoder.finish().await.expect("finish");

    let bytes = sink.lock().expect("sink").clone();
    let mut decoder = BufferStreamingDecoder::new(&bytes);
    let got: Vec<u32> = decoder.read_all().expect("read_all");
    assert_eq!(
        got,
        vec![111, 222, 333],
        "interrupted frame must resume without corruption or duplication"
    );
}
