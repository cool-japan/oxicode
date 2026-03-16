//! Compression example for OxiCode
//!
//! This example demonstrates the built-in LZ4 and Zstd compression features
//! for reducing the size of serialized data.
//!
//! Run with: cargo run --example compression --features compression-lz4
//! Or with Zstd: cargo run --example compression --features compression-zstd

use oxicode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct LogEntry {
    timestamp: u64,
    level: String,
    message: String,
    fields: Vec<(String, String)>,
}

impl LogEntry {
    fn new(timestamp: u64, level: &str, message: &str) -> Self {
        Self {
            timestamp,
            level: level.to_string(),
            message: message.to_string(),
            fields: vec![
                ("service".to_string(), "oxicode".to_string()),
                ("version".to_string(), "0.2.0".to_string()),
            ],
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OxiCode Compression Example\n");

    // Generate sample log entries (highly compressible repetitive data)
    let logs: Vec<LogEntry> = (0..1000)
        .map(|i| {
            LogEntry::new(
                1700000000 + i,
                if i % 3 == 0 {
                    "INFO"
                } else if i % 3 == 1 {
                    "WARN"
                } else {
                    "ERROR"
                },
                &format!("Processing request {} completed successfully", i),
            )
        })
        .collect();

    // Encode without compression
    let raw_bytes = oxicode::encode_to_vec(&logs)?;
    println!("Uncompressed size: {} bytes", raw_bytes.len());

    // Verify roundtrip without compression
    let (decoded, _): (Vec<LogEntry>, _) = oxicode::decode_from_slice(&raw_bytes)?;
    assert_eq!(logs, decoded);
    println!("Uncompressed roundtrip verified");

    #[cfg(feature = "compression-lz4")]
    {
        use oxicode::compression::{compress, compress_with_stats, decompress, Compression};

        // LZ4 compression
        let lz4_bytes = compress(&raw_bytes, Compression::Lz4)?;
        let (_, stats) = compress_with_stats(&raw_bytes, Compression::Lz4)?;

        println!("\nLZ4 Compression:");
        println!("  Compressed size: {} bytes", lz4_bytes.len());
        println!("  Compression ratio: {:.2}x", stats.ratio());
        println!("  Space savings: {:.1}%", stats.savings_percent());

        // Decompress and verify
        let decompressed = decompress(&lz4_bytes)?;
        let (decoded, _): (Vec<LogEntry>, _) = oxicode::decode_from_slice(&decompressed)?;
        assert_eq!(logs, decoded);
        println!("  LZ4 roundtrip verified");

        // Auto-detection
        assert!(oxicode::compression::is_compressed(&lz4_bytes));
        println!("  Magic header detection works");
    }

    #[cfg(feature = "compression-zstd")]
    {
        use oxicode::compression::{compress, compress_with_stats, decompress, Compression};

        // Zstd default compression
        let zstd_bytes = compress(&raw_bytes, Compression::Zstd)?;
        let (_, stats) = compress_with_stats(&raw_bytes, Compression::Zstd)?;

        println!("\nZstd Compression (level 3):");
        println!("  Compressed size: {} bytes", zstd_bytes.len());
        println!("  Compression ratio: {:.2}x", stats.ratio());
        println!("  Space savings: {:.1}%", stats.savings_percent());

        // Zstd with high compression level
        let zstd_high = compress(&raw_bytes, Compression::ZstdLevel(19))?;
        println!("\nZstd Compression (level 19):");
        println!("  Compressed size: {} bytes", zstd_high.len());
        println!(
            "  Compression ratio: {:.2}x",
            raw_bytes.len() as f64 / zstd_high.len() as f64
        );

        let decompressed = decompress(&zstd_bytes)?;
        let (decoded, _): (Vec<LogEntry>, _) = oxicode::decode_from_slice(&decompressed)?;
        assert_eq!(logs, decoded);
        println!("  Zstd roundtrip verified");
    }

    #[cfg(not(any(feature = "compression-lz4", feature = "compression-zstd")))]
    {
        println!("\nNote: Run with --features compression-lz4 or --features compression-zstd");
        println!("to see compression in action.");
    }

    println!("\nCompression example completed successfully!");
    Ok(())
}
