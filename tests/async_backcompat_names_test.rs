//! Backward-compatibility test for the pre-0.2.5 short async type names.
//!
//! `AsyncEncoder`/`AsyncDecoder` were the historical short aliases for
//! `AsyncStreamingEncoder`/`AsyncStreamingDecoder`, reachable from three
//! public paths in the 0.2.4 API: `oxicode::streaming`, `oxicode::async_tokio`,
//! and `oxicode::async_io`. This test exercises all three paths with real
//! encode/decode round-trips so a future rename or removal of any of them
//! is caught by the test suite, not just by `cargo-semver-checks`.
//!
//! Gated by the `async-tokio` feature at the file level.
#![cfg(feature = "async-tokio")]
#![allow(deprecated)]

use std::io::Cursor;

#[tokio::test]
async fn streaming_module_old_names_round_trip() {
    use oxicode::streaming::{AsyncDecoder, AsyncEncoder};

    let cursor = Cursor::new(Vec::new());
    let mut encoder = AsyncEncoder::new(cursor);
    encoder.write_item(&42u32).await.expect("write_item");
    let cursor = encoder.finish().await.expect("finish");

    let mut decoder = AsyncDecoder::new(Cursor::new(cursor.into_inner()));
    let value: u32 = decoder
        .read_item()
        .await
        .expect("read_item")
        .expect("item present");
    assert_eq!(value, 42);
}

#[tokio::test]
async fn async_tokio_convenience_module_old_names_round_trip() {
    use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder};

    let cursor = Cursor::new(Vec::new());
    let mut encoder = AsyncEncoder::new(cursor);
    encoder.write_item(&7u8).await.expect("write_item");
    let cursor = encoder.finish().await.expect("finish");

    let mut decoder = AsyncDecoder::new(Cursor::new(cursor.into_inner()));
    let value: u8 = decoder
        .read_item()
        .await
        .expect("read_item")
        .expect("item present");
    assert_eq!(value, 7);
}

#[tokio::test]
async fn async_io_convenience_module_old_names_round_trip() {
    use oxicode::async_io::{AsyncDecoder, AsyncEncoder};

    let cursor = Cursor::new(Vec::new());
    let mut encoder = AsyncEncoder::new(cursor);
    encoder.write_item(&"hello").await.expect("write_item");
    let cursor = encoder.finish().await.expect("finish");

    let mut decoder = AsyncDecoder::new(Cursor::new(cursor.into_inner()));
    let value: String = decoder
        .read_item()
        .await
        .expect("read_item")
        .expect("item present");
    assert_eq!(value, "hello");
}
