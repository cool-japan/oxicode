#![cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

/// A rigid body in the physics simulation, carrying position and velocity.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimBody {
    body_id: u32,
    mass: f64,
    pos_x: f64,
    pos_y: f64,
    pos_z: f64,
    vel_x: f64,
    vel_y: f64,
    vel_z: f64,
}

/// A single time-step snapshot of the simulation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimFrame {
    frame: u64,
    timestep: f64,
    bodies: Vec<SimBody>,
}

/// The final output of a completed simulation run.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimResult {
    sim_id: u64,
    total_frames: u64,
    final_frame: SimFrame,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_body(id: u32) -> SimBody {
    SimBody {
        body_id: id,
        mass: 1.0 + id as f64 * 0.1,
        pos_x: id as f64 * 0.5,
        pos_y: id as f64 * -0.5,
        pos_z: id as f64 * 0.25,
        vel_x: 0.01 * id as f64,
        vel_y: -0.01 * id as f64,
        vel_z: 0.005 * id as f64,
    }
}

fn make_frame(frame_idx: u64, body_count: usize) -> SimFrame {
    SimFrame {
        frame: frame_idx,
        timestep: 0.016,
        bodies: (0..body_count as u32).map(make_body).collect(),
    }
}

fn make_sim_result(sim_id: u64, body_count: usize) -> SimResult {
    SimResult {
        sim_id,
        total_frames: 1000,
        final_frame: make_frame(999, body_count),
    }
}

// ---------------------------------------------------------------------------
// 1. SimBody roundtrip — LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_sim_body_lz4_roundtrip() {
    let body = make_body(42);
    let encoded = encode_to_vec(&body).expect("encode SimBody");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress SimBody");
    let decompressed = decompress(&compressed).expect("lz4 decompress SimBody");
    let (decoded, _): (SimBody, usize) = decode_from_slice(&decompressed).expect("decode SimBody");
    assert_eq!(body, decoded);
}

// ---------------------------------------------------------------------------
// 2. SimBody roundtrip — Zstd
// ---------------------------------------------------------------------------
#[test]
fn test_sim_body_zstd_roundtrip() {
    let body = make_body(7);
    let encoded = encode_to_vec(&body).expect("encode SimBody");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress SimBody");
    let decompressed = decompress(&compressed).expect("zstd decompress SimBody");
    let (decoded, _): (SimBody, usize) = decode_from_slice(&decompressed).expect("decode SimBody");
    assert_eq!(body, decoded);
}

// ---------------------------------------------------------------------------
// 3. SimFrame roundtrip — LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_sim_frame_lz4_roundtrip() {
    let frame = make_frame(0, 50);
    let encoded = encode_to_vec(&frame).expect("encode SimFrame");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress SimFrame");
    let decompressed = decompress(&compressed).expect("lz4 decompress SimFrame");
    let (decoded, _): (SimFrame, usize) =
        decode_from_slice(&decompressed).expect("decode SimFrame");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// 4. SimFrame roundtrip — Zstd
// ---------------------------------------------------------------------------
#[test]
fn test_sim_frame_zstd_roundtrip() {
    let frame = make_frame(10, 50);
    let encoded = encode_to_vec(&frame).expect("encode SimFrame");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress SimFrame");
    let decompressed = decompress(&compressed).expect("zstd decompress SimFrame");
    let (decoded, _): (SimFrame, usize) =
        decode_from_slice(&decompressed).expect("decode SimFrame");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// 5. SimResult roundtrip — LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_sim_result_lz4_roundtrip() {
    let result = make_sim_result(1, 20);
    let encoded = encode_to_vec(&result).expect("encode SimResult");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress SimResult");
    let decompressed = decompress(&compressed).expect("lz4 decompress SimResult");
    let (decoded, _): (SimResult, usize) =
        decode_from_slice(&decompressed).expect("decode SimResult");
    assert_eq!(result, decoded);
}

// ---------------------------------------------------------------------------
// 6. SimResult roundtrip — Zstd
// ---------------------------------------------------------------------------
#[test]
fn test_sim_result_zstd_roundtrip() {
    let result = make_sim_result(99, 20);
    let encoded = encode_to_vec(&result).expect("encode SimResult");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress SimResult");
    let decompressed = decompress(&compressed).expect("zstd decompress SimResult");
    let (decoded, _): (SimResult, usize) =
        decode_from_slice(&decompressed).expect("decode SimResult");
    assert_eq!(result, decoded);
}

// ---------------------------------------------------------------------------
// 7. Empty SimFrame (no bodies) — LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_empty_sim_frame_lz4_roundtrip() {
    let frame = SimFrame {
        frame: 0,
        timestep: 0.0,
        bodies: vec![],
    };
    let encoded = encode_to_vec(&frame).expect("encode empty SimFrame");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress empty SimFrame");
    let decompressed = decompress(&compressed).expect("lz4 decompress empty SimFrame");
    let (decoded, _): (SimFrame, usize) =
        decode_from_slice(&decompressed).expect("decode empty SimFrame");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// 8. Empty SimFrame (no bodies) — Zstd
// ---------------------------------------------------------------------------
#[test]
fn test_empty_sim_frame_zstd_roundtrip() {
    let frame = SimFrame {
        frame: 0,
        timestep: 0.0,
        bodies: vec![],
    };
    let encoded = encode_to_vec(&frame).expect("encode empty SimFrame");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress empty SimFrame");
    let decompressed = decompress(&compressed).expect("zstd decompress empty SimFrame");
    let (decoded, _): (SimFrame, usize) =
        decode_from_slice(&decompressed).expect("decode empty SimFrame");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// 9. Single-body SimFrame — LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_single_body_sim_frame_lz4_roundtrip() {
    let frame = make_frame(0, 1);
    let encoded = encode_to_vec(&frame).expect("encode single-body SimFrame");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress single-body SimFrame");
    let decompressed = decompress(&compressed).expect("lz4 decompress single-body SimFrame");
    let (decoded, _): (SimFrame, usize) =
        decode_from_slice(&decompressed).expect("decode single-body SimFrame");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// 10. Single-body SimFrame — Zstd
// ---------------------------------------------------------------------------
#[test]
fn test_single_body_sim_frame_zstd_roundtrip() {
    let frame = make_frame(0, 1);
    let encoded = encode_to_vec(&frame).expect("encode single-body SimFrame");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress single-body SimFrame");
    let decompressed = decompress(&compressed).expect("zstd decompress single-body SimFrame");
    let (decoded, _): (SimFrame, usize) =
        decode_from_slice(&decompressed).expect("decode single-body SimFrame");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// 11. Large simulation (1 000 bodies) compression ratio — LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_large_simulation_lz4_compression_ratio() {
    let frame = make_frame(500, 1_000);
    let encoded = encode_to_vec(&frame).expect("encode large SimFrame");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large SimFrame");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 compressed ({} B) should be smaller than encoded ({} B) for 1000-body frame",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 12. Large simulation (1 000 bodies) compression ratio — Zstd
// ---------------------------------------------------------------------------
#[test]
fn test_large_simulation_zstd_compression_ratio() {
    let frame = make_frame(500, 1_000);
    let encoded = encode_to_vec(&frame).expect("encode large SimFrame");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress large SimFrame");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} B) should be smaller than encoded ({} B) for 1000-body frame",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 13. Large simulation — LZ4 roundtrip correctness
// ---------------------------------------------------------------------------
#[test]
fn test_large_simulation_lz4_roundtrip_correctness() {
    let frame = make_frame(0, 1_000);
    let encoded = encode_to_vec(&frame).expect("encode large SimFrame");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large SimFrame");
    let decompressed = decompress(&compressed).expect("lz4 decompress large SimFrame");
    let (decoded, _): (SimFrame, usize) =
        decode_from_slice(&decompressed).expect("decode large SimFrame");
    assert_eq!(
        frame, decoded,
        "large 1000-body SimFrame must survive LZ4 roundtrip unchanged"
    );
}

// ---------------------------------------------------------------------------
// 14. Large simulation — Zstd roundtrip correctness
// ---------------------------------------------------------------------------
#[test]
fn test_large_simulation_zstd_roundtrip_correctness() {
    let frame = make_frame(0, 1_000);
    let encoded = encode_to_vec(&frame).expect("encode large SimFrame");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress large SimFrame");
    let decompressed = decompress(&compressed).expect("zstd decompress large SimFrame");
    let (decoded, _): (SimFrame, usize) =
        decode_from_slice(&decompressed).expect("decode large SimFrame");
    assert_eq!(
        frame, decoded,
        "large 1000-body SimFrame must survive Zstd roundtrip unchanged"
    );
}

// ---------------------------------------------------------------------------
// 15. Cross-algorithm: LZ4 bytes != Zstd bytes, but both decode to same value
// ---------------------------------------------------------------------------
#[test]
fn test_cross_algorithm_compressed_bytes_differ_but_decode_equal() {
    let frame = make_frame(42, 30);
    let encoded = encode_to_vec(&frame).expect("encode SimFrame for cross-algorithm test");

    let lz4_compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress for cross-algorithm test");
    let zstd_compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress for cross-algorithm test");

    // The compressed byte sequences must differ between algorithms.
    assert_ne!(
        lz4_compressed, zstd_compressed,
        "LZ4 and Zstd must produce different compressed byte sequences"
    );

    // Both must decompress back to the same original payload.
    let lz4_decompressed = decompress(&lz4_compressed).expect("lz4 decompress cross-algorithm");
    let zstd_decompressed = decompress(&zstd_compressed).expect("zstd decompress cross-algorithm");

    assert_eq!(
        lz4_decompressed, zstd_decompressed,
        "LZ4 and Zstd must decompress to identical payloads"
    );

    let (lz4_frame, _): (SimFrame, usize) =
        decode_from_slice(&lz4_decompressed).expect("decode LZ4 cross-algorithm");
    let (zstd_frame, _): (SimFrame, usize) =
        decode_from_slice(&zstd_decompressed).expect("decode Zstd cross-algorithm");

    assert_eq!(frame, lz4_frame);
    assert_eq!(frame, zstd_frame);
}

// ---------------------------------------------------------------------------
// 16. Truncated LZ4 data returns an error
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_lz4_data_returns_error() {
    let frame = make_frame(1, 10);
    let encoded = encode_to_vec(&frame).expect("encode SimFrame for truncation test");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress for truncation test");

    // Truncate to half the compressed length — must not panic, must return Err.
    let half = compressed.len() / 2;
    let truncated = &compressed[..half];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress of truncated LZ4 data must return an error, not Ok"
    );
}

// ---------------------------------------------------------------------------
// 17. Truncated Zstd data returns an error
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_zstd_data_returns_error() {
    let frame = make_frame(1, 10);
    let encoded = encode_to_vec(&frame).expect("encode SimFrame for truncation test");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress for truncation test");

    let half = compressed.len() / 2;
    let truncated = &compressed[..half];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress of truncated Zstd data must return an error, not Ok"
    );
}

// ---------------------------------------------------------------------------
// 18. Garbage bytes — decompression returns an error (LZ4 path via invalid header)
// ---------------------------------------------------------------------------
#[test]
fn test_garbage_bytes_lz4_path_decompression_error() {
    // Bytes that do not match any compression magic header.
    let garbage: Vec<u8> = (0u8..=127).cycle().take(64).collect();
    // compress+decompress with LZ4 then corrupt the compressed payload
    let body = make_body(0);
    let enc = encode_to_vec(&body).expect("encode body");
    let mut compressed = compress(&enc, Compression::Lz4).expect("lz4 compress");
    // Overwrite the payload portion with garbage
    let header_len = compressed.len().min(8);
    for byte in compressed.iter_mut().skip(header_len) {
        *byte ^= 0xFF;
    }
    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress of corrupted LZ4 payload must return an error"
    );
    // Also verify pure random garbage returns error
    let result2 = decompress(&garbage);
    assert!(
        result2.is_err(),
        "decompress of random garbage must return an error"
    );
}

// ---------------------------------------------------------------------------
// 19. Garbage bytes — Zstd path: corrupted compressed payload returns error
// ---------------------------------------------------------------------------
#[test]
fn test_garbage_bytes_zstd_path_decompression_error() {
    let garbage: Vec<u8> = (128u8..=255).cycle().take(64).collect();
    let body = make_body(1);
    let enc = encode_to_vec(&body).expect("encode body");
    let mut compressed = compress(&enc, Compression::Zstd).expect("zstd compress");
    // Flip bits in the payload to corrupt it
    let header_len = compressed.len().min(8);
    for byte in compressed.iter_mut().skip(header_len) {
        *byte ^= 0xAA;
    }
    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress of corrupted Zstd payload must return an error"
    );
    // Also verify pure random garbage returns error
    let result2 = decompress(&garbage);
    assert!(
        result2.is_err(),
        "decompress of random garbage must return an error"
    );
}

// ---------------------------------------------------------------------------
// 20. Multi-frame sequence — LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_multi_frame_sequence_lz4_roundtrip() {
    // Encode multiple SimFrames and verify they all survive compress+decompress.
    let frames: Vec<SimFrame> = (0u64..10).map(|i| make_frame(i, 15)).collect();
    for frame in &frames {
        let encoded = encode_to_vec(frame).expect("encode SimFrame in sequence");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("lz4 compress SimFrame in sequence");
        let decompressed = decompress(&compressed).expect("lz4 decompress SimFrame in sequence");
        let (decoded, _): (SimFrame, usize) =
            decode_from_slice(&decompressed).expect("decode SimFrame in sequence");
        assert_eq!(
            frame, &decoded,
            "frame {} must survive LZ4 roundtrip",
            frame.frame
        );
    }
}

// ---------------------------------------------------------------------------
// 21. Multi-frame sequence — Zstd
// ---------------------------------------------------------------------------
#[test]
fn test_multi_frame_sequence_zstd_roundtrip() {
    let frames: Vec<SimFrame> = (0u64..10).map(|i| make_frame(i, 15)).collect();
    for frame in &frames {
        let encoded = encode_to_vec(frame).expect("encode SimFrame in sequence");
        let compressed =
            compress(&encoded, Compression::Zstd).expect("zstd compress SimFrame in sequence");
        let decompressed = decompress(&compressed).expect("zstd decompress SimFrame in sequence");
        let (decoded, _): (SimFrame, usize) =
            decode_from_slice(&decompressed).expect("decode SimFrame in sequence");
        assert_eq!(
            frame, &decoded,
            "frame {} must survive Zstd roundtrip",
            frame.frame
        );
    }
}

// ---------------------------------------------------------------------------
// 22. SimResult with extreme float values — both algorithms
// ---------------------------------------------------------------------------
#[test]
fn test_sim_result_extreme_floats_both_algorithms() {
    let extreme_body = SimBody {
        body_id: 0,
        mass: f64::MAX,
        pos_x: f64::MIN_POSITIVE,
        pos_y: -f64::MAX,
        pos_z: 0.0,
        vel_x: f64::INFINITY,
        vel_y: f64::NEG_INFINITY,
        vel_z: f64::NAN,
    };
    let frame = SimFrame {
        frame: u64::MAX,
        timestep: f64::EPSILON,
        bodies: vec![extreme_body],
    };
    let result = SimResult {
        sim_id: u64::MAX,
        total_frames: u64::MAX,
        final_frame: frame,
    };

    let encoded = encode_to_vec(&result).expect("encode extreme SimResult");

    // LZ4 path
    let lz4_compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress extreme SimResult");
    let lz4_decompressed = decompress(&lz4_compressed).expect("lz4 decompress extreme SimResult");
    let (lz4_decoded, _): (SimResult, usize) =
        decode_from_slice(&lz4_decompressed).expect("decode extreme SimResult via LZ4");

    // Zstd path
    let zstd_compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress extreme SimResult");
    let zstd_decompressed =
        decompress(&zstd_compressed).expect("zstd decompress extreme SimResult");
    let (zstd_decoded, _): (SimResult, usize) =
        decode_from_slice(&zstd_decompressed).expect("decode extreme SimResult via Zstd");

    // NaN fields prevent direct PartialEq comparison; compare the raw decoded payloads instead.
    assert_eq!(
        lz4_decompressed, zstd_decompressed,
        "LZ4 and Zstd decompressed payloads must be byte-identical for extreme-float SimResult"
    );

    // Non-NaN fields are directly comparable.
    assert_eq!(lz4_decoded.sim_id, result.sim_id);
    assert_eq!(lz4_decoded.total_frames, result.total_frames);
    assert_eq!(lz4_decoded.final_frame.frame, result.final_frame.frame);
    assert_eq!(zstd_decoded.sim_id, result.sim_id);
    assert_eq!(zstd_decoded.total_frames, result.total_frames);
    assert_eq!(zstd_decoded.final_frame.frame, result.final_frame.frame);
}
