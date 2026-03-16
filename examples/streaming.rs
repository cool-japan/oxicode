//! Streaming serialization example for OxiCode
//!
//! This example demonstrates how to use OxiCode's streaming features
//! for encoding and decoding large sequences of items incrementally.
//!
//! Run with: cargo run --example streaming

use oxicode::streaming::{
    BufferStreamingDecoder, BufferStreamingEncoder, StreamingConfig, StreamingDecoder,
    StreamingEncoder,
};
use oxicode::{Decode, Encode};
use std::io::Cursor;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Measurement {
    sensor_id: u32,
    timestamp: u64,
    value: f64,
    unit: String,
}

impl Measurement {
    fn new(sensor_id: u32, timestamp: u64, value: f64) -> Self {
        Self {
            sensor_id,
            timestamp,
            value,
            unit: "celsius".to_string(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OxiCode Streaming Serialization Example\n");

    // Example 1: Buffer streaming (in-memory)
    println!("1. Buffer streaming (in-memory):");

    let measurements: Vec<Measurement> = (0..100)
        .map(|i| Measurement::new(i % 10, 1700000000 + i as u64, 20.0 + (i as f64) * 0.1))
        .collect();

    // Encode with streaming
    let mut encoder = BufferStreamingEncoder::new();
    for measurement in &measurements {
        encoder.write_item(measurement)?;
    }
    let encoded = encoder.finish();

    println!("   Encoded {} measurements", measurements.len());
    println!("   Total bytes: {}", encoded.len());
    println!(
        "   Bytes per item: {:.1}",
        encoded.len() as f64 / measurements.len() as f64
    );

    // Decode with streaming — read_item returns Result<Option<T>>
    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let mut decoded_items: Vec<Measurement> = Vec::new();
    while let Some(item) = decoder.read_item::<Measurement>()? {
        decoded_items.push(item);
    }

    assert_eq!(measurements.len(), decoded_items.len());
    assert_eq!(measurements[0], decoded_items[0]);
    assert_eq!(measurements[99], decoded_items[99]);
    println!("   All {} items decoded correctly", decoded_items.len());

    // Example 2: Streaming with custom chunk size
    println!("\n2. Streaming with custom chunk configuration:");

    let config = StreamingConfig::default()
        .with_chunk_size(1024)
        .with_max_buffer(1024 * 1024); // 1MB max buffer

    let mut encoder = BufferStreamingEncoder::with_config(config);
    for i in 0u32..1000 {
        encoder.write_item(&i)?;
    }
    let encoded = encoder.finish();
    println!("   Encoded 1000 u32 values");
    println!("   Total bytes: {}", encoded.len());

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let mut count = 0u32;
    let mut sum = 0u64;
    while let Some(val) = decoder.read_item::<u32>()? {
        assert_eq!(val, count, "value mismatch at index {}", count);
        sum += val as u64;
        count += 1;
    }
    assert_eq!(count, 1000);
    println!("   Verified {} values, sum = {}", count, sum);

    // Example 3: Streaming to/from std::io (Vec<u8> as Writer/Reader)
    println!("\n3. Streaming to/from std::io (Vec<u8> as Writer/Reader):");

    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut io_encoder = StreamingEncoder::new(&mut buffer);
        for i in 0u32..50 {
            io_encoder.write_item(&(i, format!("item_{}", i)))?;
        }
        io_encoder.finish()?;
    }

    println!(
        "   Encoded 50 (u32, String) pairs to {} bytes",
        buffer.len()
    );

    let cursor = Cursor::new(buffer);
    let mut io_decoder = StreamingDecoder::new(cursor);
    let mut pairs: Vec<(u32, String)> = Vec::new();
    while let Some(pair) = io_decoder.read_item::<(u32, String)>()? {
        pairs.push(pair);
    }

    assert_eq!(pairs.len(), 50);
    assert_eq!(pairs[0], (0, "item_0".to_string()));
    assert_eq!(pairs[49], (49, "item_49".to_string()));
    println!("   Decoded {} pairs from buffer", pairs.len());

    // Example 4: Progress tracking
    println!("\n4. Progress tracking:");

    let mut enc = BufferStreamingEncoder::new();
    for i in 0u64..200 {
        enc.write_item(&i)?;
    }
    let encoded = enc.finish();

    let mut dec = BufferStreamingDecoder::new(&encoded);
    while dec.read_item::<u64>()?.is_some() {}

    let progress = dec.progress();
    println!("   Items processed: {}", progress.items_processed);
    println!("   Bytes processed: {}", progress.bytes_processed);
    println!("   Chunks processed: {}", progress.chunks_processed);
    assert_eq!(progress.items_processed, 200);
    println!("   Progress tracking verified");

    // Example 5: read_all convenience method
    println!("\n5. read_all convenience method:");

    let mut enc2 = BufferStreamingEncoder::new();
    for i in 0u32..50 {
        enc2.write_item(&i)?;
    }
    let encoded2 = enc2.finish();

    let mut dec2 = BufferStreamingDecoder::new(&encoded2);
    let all_items = dec2.read_all::<u32>()?;
    assert_eq!(all_items.len(), 50);
    assert_eq!(all_items[0], 0);
    assert_eq!(all_items[49], 49);
    println!("   read_all decoded {} items in one call", all_items.len());

    println!("\nStreaming example completed successfully!");
    Ok(())
}
