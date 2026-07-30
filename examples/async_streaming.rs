//! Async streaming serialization example for OxiCode.
//!
//! Demonstrates `AsyncStreamingEncoder`/`AsyncStreamingDecoder` writing and
//! reading items incrementally over tokio's async I/O traits, plus
//! cooperative cancellation via `CancellationToken` /
//! `CancellableAsyncEncoder` / `CancellableAsyncDecoder`.
//!
//! Note: `AsyncStreamingEncoder`/`AsyncStreamingDecoder` frame their output
//! using oxicode's internal chunked container format (a small header per
//! chunk). This is **not** the same byte layout as `oxicode::encode_to_vec`
//! and is not decodable with `decode_from_slice` (or by bincode) — always
//! read an async-streamed payload back with a matching `AsyncStreamingDecoder`
//! / `StreamingDecoder` / `BufferStreamingDecoder`.
//!
//! Run with: cargo run --example async_streaming --features async-tokio
//!
//! (Without the `async-tokio` feature this example still builds — `main`
//! just reports that the feature is disabled, since `AsyncStreamingEncoder`
//! and `AsyncStreamingDecoder` only exist when the feature is enabled.)

#[cfg(feature = "async-tokio")]
mod async_demo {
    use oxicode::streaming::{
        AsyncStreamingDecoder, AsyncStreamingEncoder, CancellableAsyncEncoder, CancellationToken,
    };
    use oxicode::{Decode, Encode};

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    pub struct Event {
        pub id: u32,
        pub payload: String,
    }

    /// Basic async streaming round-trip over an in-memory `Vec<u8>` /
    /// `&[u8]` pair (tokio implements `AsyncWrite` for `Vec<u8>` and
    /// `AsyncRead` for `&[u8]`, so no real I/O is needed for this demo).
    pub async fn basic_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        println!("1. Basic async streaming round-trip:");

        let events: Vec<Event> = (0..50)
            .map(|i| Event {
                id: i,
                payload: format!("event-{i}"),
            })
            .collect();

        // --- Encode ---
        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut encoder = AsyncStreamingEncoder::new(&mut buffer);
            for event in &events {
                encoder.write_item(event).await?;
            }
            encoder.finish().await?;
        }
        println!(
            "   Encoded {} events into {} bytes (async, in-memory)",
            events.len(),
            buffer.len()
        );

        // --- Decode ---
        let mut decoder = AsyncStreamingDecoder::new(buffer.as_slice());
        let mut decoded: Vec<Event> = Vec::new();
        while let Some(event) = decoder.read_item::<Event>().await? {
            decoded.push(event);
        }

        assert_eq!(events, decoded);
        println!("   Decoded and verified {} events\n", decoded.len());
        Ok(())
    }

    /// Demonstrates cooperative cancellation: `CancellableAsyncEncoder`/
    /// `CancellableAsyncDecoder` check a shared `CancellationToken` between
    /// items and return `Error::Cancelled` (rather than corrupting the
    /// stream) once cancellation is requested.
    pub async fn cancellation_demo() -> Result<(), Box<dyn std::error::Error>> {
        println!("2. Cooperative cancellation:");

        let token = CancellationToken::new();
        let mut buffer: Vec<u8> = Vec::new();
        let mut encoder = CancellableAsyncEncoder::new(&mut buffer, token.clone());

        for i in 0u32..1000 {
            if i == 10 {
                token.cancel();
            }
            match encoder.write_item(&i).await {
                Ok(()) => {}
                Err(err) => {
                    println!("   Write stopped at item {i} after cancellation: {err}");
                    break;
                }
            }
        }

        println!("   Token cancelled: {}\n", token.is_cancelled());
        Ok(())
    }
}

#[cfg(feature = "async-tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OxiCode Async Streaming Example\n");
    async_demo::basic_roundtrip().await?;
    async_demo::cancellation_demo().await?;
    Ok(())
}

#[cfg(not(feature = "async-tokio"))]
fn main() {
    println!("This example requires the \"async-tokio\" feature.");
    println!("Run with: cargo run --example async_streaming --features async-tokio");
}
