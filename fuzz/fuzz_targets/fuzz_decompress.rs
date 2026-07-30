#![no_main]
use libfuzzer_sys::fuzz_target;
use oxicode::compression::{compress, decompress, decompress_or_passthrough, decompress_with_limit, Compression};

fuzz_target!(|data: &[u8]| {
    // Raw untrusted bytes straight into the decompression entry points. The
    // first 5 bytes (MAGIC + version + codec id) plus whatever follows are
    // entirely attacker-controlled here: this exercises the header parser,
    // the codec dispatch, and both the LZ4 and Zstd frame parsers behind it.
    // None of these calls may panic, and none may allocate beyond the
    // configured cap regardless of what the (possibly forged) header claims.
    let _ = decompress(data);
    let _ = decompress_or_passthrough(data);
    let _ = decompress_with_limit(data, 1024 * 1024);

    // Structured variant: prepend a genuine oxicode compression header (valid
    // MAGIC + VERSION + a real codec id) so the fuzzer's mutated bytes land
    // in the codec payload position instead of being immediately rejected by
    // the header check. This gives the actual LZ4/Zstd decoders far more
    // exercise than header-only mutation would.
    if data.len() >= 2 {
        let codec_id = data[0];
        let payload = &data[1..];

        for id in [0u8, 1u8, 2u8] {
            // 0 = None (passthrough marker), 1 = Lz4, 2 = Zstd — matches the
            // codec ids `compress`'s header writer assigns internally. We
            // don't rely on any specific numeric mapping being stable; we
            // just want well-formed-looking headers with varying codec ids
            // feeding arbitrary payload bytes into decompress().
            let mut framed = alloc_header(id);
            framed.extend_from_slice(payload);
            let _ = decompress(&framed);
            let _ = decompress_or_passthrough(&framed);
            let _ = decompress_with_limit(&framed, 1024 * 1024);
        }

        // Also exercise codec_id taken directly from fuzzer input, to cover
        // out-of-range / unknown codec ids at the dispatch site.
        let mut framed = alloc_header(codec_id);
        framed.extend_from_slice(payload);
        let _ = decompress(&framed);
    }

    // Round-trip sanity: whatever compress() itself produces must always be
    // decompressible without panicking, even though `data` here is not
    // necessarily compressible-looking input (compress on arbitrary bytes
    // should either succeed with a valid frame or return a clean error).
    if let Ok(compressed) = compress(data, Compression::None) {
        let _ = decompress(&compressed);
    }
    // Both compression-lz4 and compression-zstd are unconditionally enabled
    // for this fuzz crate (see fuzz/Cargo.toml), so no feature gating needed.
    if let Ok(compressed) = compress(data, Compression::Lz4) {
        let _ = decompress(&compressed);
    }
    if let Ok(compressed) = compress(data, Compression::Zstd) {
        let _ = decompress(&compressed);
    }
});

/// Build a syntactically valid oxicode compression header (MAGIC + VERSION +
/// codec id) matching the format documented in `oxicode::compression`
/// (`b"OXC"` + version 1 + codec id), without depending on any private
/// constant from the crate.
fn alloc_header(codec_id: u8) -> alloc::vec::Vec<u8> {
    let mut header = alloc::vec::Vec::with_capacity(5);
    header.extend_from_slice(b"OXC");
    header.push(1); // VERSION
    header.push(codec_id);
    header
}

extern crate alloc;
