//! Adversarial hardening tests for the compression module (WP-D).
//!
//! These exercise the public `oxicode::compression` API against crafted,
//! malicious payloads: decompression bombs, header collisions, frames that omit
//! the content-size field, and unknown/short headers. They complement the
//! in-module unit tests and lock in the DoS protections added for the WP-D
//! findings (TODO.md lines 326-370).

#![cfg(all(
    feature = "alloc",
    feature = "compression-lz4",
    feature = "compression-zstd"
))]

use oxicode::compression::{
    compress, decompress, decompress_or_passthrough, decompress_with_limit, is_compressed,
    Compression, DEFAULT_MAX_DECOMPRESSED_SIZE,
};
use oxicode::Error;

/// oxicode compression framing: `b"OXC"` + version(1) + codec id.
const OXC_MAGIC: [u8; 3] = [0x4F, 0x58, 0x43];
const OXC_VERSION: u8 = 1;
const CODEC_NONE: u8 = 0;
const CODEC_LZ4: u8 = 1;
const CODEC_ZSTD: u8 = 2;

const ZSTD_MAGIC: [u8; 4] = [0x28, 0xB5, 0x2F, 0xFD];
const ZSTD_MAX_BLOCK_SIZE: u32 = 128 * 1024;
const FHD_SINGLE_SEGMENT: u8 = 0x20;

const LZ4_FRAME_MAGIC: u32 = 0x184D2204;

fn oxc_wrap(codec: u8, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(5 + payload.len());
    out.extend_from_slice(&OXC_MAGIC);
    out.push(OXC_VERSION);
    out.push(codec);
    out.extend_from_slice(payload);
    out
}

/// A raw zstd frame with `n` maximum-size RLE blocks: ~128 KiB regenerated per
/// ~4 input bytes. Declares a content size of 1 (a lie the real decoder would
/// only notice after fully expanding the bomb).
fn zstd_rle_bomb(n: u32) -> Vec<u8> {
    let mut frame = Vec::new();
    frame.extend_from_slice(&ZSTD_MAGIC);
    frame.push(FHD_SINGLE_SEGMENT); // single-segment, 1-byte content size
    frame.push(1u8); // declared content size = 1
    for i in 0..n {
        let last = if i == n - 1 { 1 } else { 0 };
        let block_header: u32 = (ZSTD_MAX_BLOCK_SIZE << 3) | (1 << 1) | last; // RLE
        frame.push((block_header & 0xFF) as u8);
        frame.push(((block_header >> 8) & 0xFF) as u8);
        frame.push(((block_header >> 16) & 0xFF) as u8);
        frame.push(0xAB);
    }
    frame
}

#[test]
fn zstd_rle_bomb_rejected_before_expansion() {
    // ~4 MiB of declared regeneration from a few hundred input bytes, capped at
    // 256 KiB: must be rejected without allocating megabytes.
    let bomb = oxc_wrap(CODEC_ZSTD, &zstd_rle_bomb(32));
    let err = decompress_with_limit(&bomb, 256 * 1024).expect_err("zstd bomb must be rejected");
    assert!(
        matches!(err, Error::LimitExceeded { .. }),
        "expected LimitExceeded, got {err:?}"
    );
}

#[test]
fn zstd_frame_omitting_content_size_rejected() {
    // descriptor 0 => neither single-segment nor content-size flag => FCS omitted.
    let mut frame = Vec::new();
    frame.extend_from_slice(&ZSTD_MAGIC);
    frame.push(0u8); // descriptor
    frame.push(0u8); // window descriptor
    frame.extend_from_slice(&[0x01, 0x00, 0x00]); // one empty last raw block
    let wrapped = oxc_wrap(CODEC_ZSTD, &frame);

    let err = decompress(&wrapped).expect_err("omitted content size must be rejected");
    assert!(
        matches!(err, Error::InvalidData { .. }),
        "expected InvalidData, got {err:?}"
    );
}

#[test]
fn lz4_frame_omitting_content_size_rejected() {
    // Standard LZ4 frame magic with the FLG Content_Size bit (0x08) cleared.
    let mut frame = Vec::new();
    frame.extend_from_slice(&LZ4_FRAME_MAGIC.to_le_bytes());
    frame.push(0x60); // FLG: version=01, block-independent, no content size
    frame.push(0x40); // BD (block max size field)
    let wrapped = oxc_wrap(CODEC_LZ4, &frame);

    let err = decompress(&wrapped).expect_err("LZ4 frame without content size must be rejected");
    assert!(
        matches!(err, Error::InvalidData { .. }),
        "expected InvalidData, got {err:?}"
    );
}

#[test]
fn unknown_codec_id_reports_unknown() {
    let wrapped = oxc_wrap(9, b"whatever");
    let err = decompress(&wrapped).expect_err("codec 9 is unknown");
    match err {
        Error::InvalidData { message } => {
            assert!(message.contains("unknown"), "unexpected message: {message}");
        }
        other => panic!("expected InvalidData, got {other:?}"),
    }
}

#[test]
fn header_collision_is_recognized_as_compressed() {
    // A raw payload that happens to start with the 5-byte OXC header collides
    // with the detection heuristic. This is a documented hazard: is_compressed
    // returns true even though the bytes are not really compressed.
    let colliding = oxc_wrap(CODEC_NONE, b"raw serialized bytes that are not compressed");
    assert!(
        is_compressed(&colliding),
        "a payload with a valid OXC header is detected as compressed"
    );

    // A payload with an out-of-range codec id (>2) is NOT detected, thanks to
    // the stricter codec check added in is_compressed.
    let mut not_compressed = colliding.clone();
    not_compressed[4] = 0x7F;
    assert!(
        !is_compressed(&not_compressed),
        "an invalid codec id must not be detected as compressed"
    );
}

#[test]
fn passthrough_returns_non_matching_input_verbatim() {
    let raw = b"\x00\x01\x02 not an OXC header at all";
    let out = decompress_or_passthrough(raw).expect("passthrough must succeed");
    assert_eq!(out.as_slice(), raw.as_slice());
}

#[test]
fn short_header_reports_unexpected_end() {
    let err = decompress(&[0x4F, 0x58]).expect_err("too short to hold a header");
    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "expected UnexpectedEnd, got {err:?}"
    );
}

#[test]
fn default_cap_is_256_mib() {
    assert_eq!(DEFAULT_MAX_DECOMPRESSED_SIZE, 256 * 1024 * 1024);
}

#[test]
fn honest_roundtrips_survive_the_cap() {
    let data = b"Hardening test payload -- roundtrip through both codecs.".repeat(64);

    for codec in [
        Compression::Lz4,
        Compression::Zstd,
        Compression::ZstdLevel(19),
    ] {
        let compressed = compress(&data, codec).expect("compress");
        let restored = decompress(&compressed).expect("decompress");
        assert_eq!(restored, data, "roundtrip mismatch for {codec:?}");

        // A tight but sufficient cap still allows the honest payload.
        let restored2 =
            decompress_with_limit(&compressed, data.len() + 4096).expect("decompress with cap");
        assert_eq!(restored2, data);
    }
}
