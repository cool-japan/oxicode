#![cfg(feature = "compression-lz4")]
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

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BuildArtifact {
    artifact_id: u64,
    name: String,
    version: String,
    size_bytes: u64,
    checksums: Vec<(String, String)>, // (algo, hex)
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BuildStatus {
    Success { duration_ms: u32 },
    Failed { exit_code: i32, stderr: String },
    Cached,
    Skipped(String),
}

// Test 1: BuildArtifact roundtrip via LZ4
#[test]
fn test_build_artifact_lz4_roundtrip() {
    let artifact = BuildArtifact {
        artifact_id: 1001u64,
        name: String::from("libfoo.so"),
        version: String::from("1.2.3"),
        size_bytes: 204_800u64,
        checksums: vec![
            (String::from("sha256"), String::from("deadbeefcafe0000")),
            (String::from("md5"), String::from("aabbccddeeff0011")),
        ],
    };
    let encoded = encode_to_vec(&artifact).expect("Failed to encode BuildArtifact");
    let compressed = compress_lz4(&encoded).expect("Failed to compress BuildArtifact");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress BuildArtifact");
    let (decoded, _): (BuildArtifact, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode BuildArtifact");
    assert_eq!(artifact, decoded);
}

// Test 2: BuildStatus::Success roundtrip via LZ4
#[test]
fn test_build_status_success_lz4_roundtrip() {
    let status = BuildStatus::Success {
        duration_ms: 4_200u32,
    };
    let encoded = encode_to_vec(&status).expect("Failed to encode BuildStatus::Success");
    let compressed = compress_lz4(&encoded).expect("Failed to compress BuildStatus::Success");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress BuildStatus::Success");
    let (decoded, _): (BuildStatus, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode BuildStatus::Success");
    assert_eq!(status, decoded);
}

// Test 3: BuildStatus::Failed roundtrip via LZ4
#[test]
fn test_build_status_failed_lz4_roundtrip() {
    let status = BuildStatus::Failed {
        exit_code: -1i32,
        stderr: String::from("error: linker `cc` not found\nnote: run with `RUST_BACKTRACE=1`"),
    };
    let encoded = encode_to_vec(&status).expect("Failed to encode BuildStatus::Failed");
    let compressed = compress_lz4(&encoded).expect("Failed to compress BuildStatus::Failed");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress BuildStatus::Failed");
    let (decoded, _): (BuildStatus, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode BuildStatus::Failed");
    assert_eq!(status, decoded);
}

// Test 4: BuildStatus::Cached roundtrip via LZ4
#[test]
fn test_build_status_cached_lz4_roundtrip() {
    let status = BuildStatus::Cached;
    let encoded = encode_to_vec(&status).expect("Failed to encode BuildStatus::Cached");
    let compressed = compress_lz4(&encoded).expect("Failed to compress BuildStatus::Cached");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress BuildStatus::Cached");
    let (decoded, _): (BuildStatus, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode BuildStatus::Cached");
    assert_eq!(status, decoded);
}

// Test 5: BuildStatus::Skipped roundtrip via LZ4
#[test]
fn test_build_status_skipped_lz4_roundtrip() {
    let status = BuildStatus::Skipped(String::from("no sources changed"));
    let encoded = encode_to_vec(&status).expect("Failed to encode BuildStatus::Skipped");
    let compressed = compress_lz4(&encoded).expect("Failed to compress BuildStatus::Skipped");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress BuildStatus::Skipped");
    let (decoded, _): (BuildStatus, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode BuildStatus::Skipped");
    assert_eq!(status, decoded);
}

// Test 6: Vec<BuildArtifact> multiple items roundtrip via LZ4
#[test]
fn test_vec_build_artifact_lz4_roundtrip() {
    let artifacts: Vec<BuildArtifact> = (0..8u64)
        .map(|i| BuildArtifact {
            artifact_id: i * 100,
            name: format!("artifact-{}.o", i),
            version: format!("{}.0.0", i + 1),
            size_bytes: (i + 1) * 1024,
            checksums: vec![
                (String::from("sha256"), format!("{:064x}", i * 0xABCDEFu64)),
                (String::from("sha512"), format!("{:0128x}", i * 0xFEDCBAu64)),
            ],
        })
        .collect();
    let encoded = encode_to_vec(&artifacts).expect("Failed to encode Vec<BuildArtifact>");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Vec<BuildArtifact>");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress Vec<BuildArtifact>");
    let (decoded, _): (Vec<BuildArtifact>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Vec<BuildArtifact>");
    assert_eq!(artifacts, decoded);
}

// Test 7: Vec<BuildStatus> mixed variants roundtrip via LZ4
#[test]
fn test_vec_build_status_mixed_lz4_roundtrip() {
    let statuses: Vec<BuildStatus> = vec![
        BuildStatus::Success {
            duration_ms: 1_200u32,
        },
        BuildStatus::Cached,
        BuildStatus::Failed {
            exit_code: 127i32,
            stderr: String::from("command not found"),
        },
        BuildStatus::Skipped(String::from("feature disabled")),
        BuildStatus::Success {
            duration_ms: 890u32,
        },
        BuildStatus::Cached,
    ];
    let encoded = encode_to_vec(&statuses).expect("Failed to encode Vec<BuildStatus>");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Vec<BuildStatus>");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress Vec<BuildStatus>");
    let (decoded, _): (Vec<BuildStatus>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Vec<BuildStatus>");
    assert_eq!(statuses, decoded);
}

// Test 8: Large repetitive BuildArtifact data compresses smaller
#[test]
fn test_large_repetitive_build_artifact_compress_smaller() {
    let artifact = BuildArtifact {
        artifact_id: 9999u64,
        name: String::from("big-lib.a"),
        version: String::from("9.9.9"),
        size_bytes: 999_999u64,
        checksums: vec![(String::from("sha256"), "a".repeat(64))],
    };
    let encoded = encode_to_vec(&artifact).expect("Failed to encode single artifact");
    // Replicate the pattern by creating a Vec with many identical items
    let many: Vec<BuildArtifact> = (0..200)
        .map(|_| BuildArtifact {
            artifact_id: artifact.artifact_id,
            name: artifact.name.clone(),
            version: artifact.version.clone(),
            size_bytes: artifact.size_bytes,
            checksums: artifact.checksums.clone(),
        })
        .collect();
    let many_encoded = encode_to_vec(&many).expect("Failed to encode many artifacts");
    let many_compressed = compress_lz4(&many_encoded).expect("Failed to compress many artifacts");
    assert!(
        many_compressed.len() < many_encoded.len(),
        "Compressed size {} should be less than original size {}",
        many_compressed.len(),
        many_encoded.len()
    );
    // Also verify single artifact roundtrips correctly
    let compressed = compress_lz4(&encoded).expect("Failed to compress single artifact");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress single artifact");
    let (decoded, _): (BuildArtifact, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode single artifact");
    assert_eq!(artifact, decoded);
}

// Test 9: u64 max value LZ4 roundtrip
#[test]
fn test_u64_max_lz4_roundtrip() {
    let val: u64 = u64::MAX;
    let encoded = encode_to_vec(&val).expect("Failed to encode u64::MAX");
    let compressed = compress_lz4(&encoded).expect("Failed to compress u64::MAX");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress u64::MAX");
    let (decoded, _): (u64, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode u64::MAX");
    assert_eq!(val, decoded);
}

// Test 10: i32 min value LZ4 roundtrip
#[test]
fn test_i32_min_lz4_roundtrip() {
    let val: i32 = i32::MIN;
    let encoded = encode_to_vec(&val).expect("Failed to encode i32::MIN");
    let compressed = compress_lz4(&encoded).expect("Failed to compress i32::MIN");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress i32::MIN");
    let (decoded, _): (i32, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode i32::MIN");
    assert_eq!(val, decoded);
}

// Test 11: Empty Vec<BuildArtifact> LZ4 roundtrip
#[test]
fn test_empty_vec_build_artifact_lz4_roundtrip() {
    let val: Vec<BuildArtifact> = Vec::new();
    let encoded = encode_to_vec(&val).expect("Failed to encode empty Vec<BuildArtifact>");
    let compressed = compress_lz4(&encoded).expect("Failed to compress empty Vec<BuildArtifact>");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress empty Vec<BuildArtifact>");
    let (decoded, _): (Vec<BuildArtifact>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode empty Vec<BuildArtifact>");
    assert_eq!(val, decoded);
}

// Test 12: Option<BuildArtifact> Some LZ4 roundtrip
#[test]
fn test_option_build_artifact_some_lz4_roundtrip() {
    let val: Option<BuildArtifact> = Some(BuildArtifact {
        artifact_id: 77u64,
        name: String::from("optional-artifact"),
        version: String::from("0.1.0"),
        size_bytes: 512u64,
        checksums: vec![(String::from("crc32"), String::from("deadbeef"))],
    });
    let encoded = encode_to_vec(&val).expect("Failed to encode Option<BuildArtifact> Some");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Option<BuildArtifact> Some");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress Option<BuildArtifact> Some");
    let (decoded, _): (Option<BuildArtifact>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Option<BuildArtifact> Some");
    assert_eq!(val, decoded);
}

// Test 13: Option<BuildArtifact> None LZ4 roundtrip
#[test]
fn test_option_build_artifact_none_lz4_roundtrip() {
    let val: Option<BuildArtifact> = None;
    let encoded = encode_to_vec(&val).expect("Failed to encode Option<BuildArtifact> None");
    let compressed = compress_lz4(&encoded).expect("Failed to compress Option<BuildArtifact> None");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress Option<BuildArtifact> None");
    let (decoded, _): (Option<BuildArtifact>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode Option<BuildArtifact> None");
    assert_eq!(val, decoded);
}

// Test 14: Compress same BuildArtifact three times - all identical
#[test]
fn test_compress_build_artifact_three_times_identical() {
    let artifact = BuildArtifact {
        artifact_id: 42u64,
        name: String::from("idempotent.lib"),
        version: String::from("3.0.0"),
        size_bytes: 8_192u64,
        checksums: vec![(String::from("sha1"), String::from("cafebabe0000cafe"))],
    };
    let encoded = encode_to_vec(&artifact).expect("Failed to encode artifact for idempotency test");
    let c1 = compress_lz4(&encoded).expect("Failed to compress first time");
    let c2 = compress_lz4(&encoded).expect("Failed to compress second time");
    let c3 = compress_lz4(&encoded).expect("Failed to compress third time");
    assert_eq!(c1, c2, "First and second compressions should be identical");
    assert_eq!(c2, c3, "Second and third compressions should be identical");
}

// Test 15: Decompress bad data returns error
#[test]
fn test_decompress_bad_data_returns_error() {
    let bad_data: Vec<u8> = vec![0xDEu8, 0xADu8, 0xBEu8, 0xEFu8, 0x00u8, 0xFFu8, 0x42u8];
    let result = decompress_lz4(&bad_data);
    assert!(
        result.is_err(),
        "Decompressing invalid LZ4 data should return an error"
    );
}

// Test 16: Compressed output is non-empty even for a minimal BuildStatus
#[test]
fn test_compressed_output_nonempty_for_minimal_status() {
    let status = BuildStatus::Cached;
    let encoded = encode_to_vec(&status).expect("Failed to encode minimal BuildStatus");
    let compressed = compress_lz4(&encoded).expect("Failed to compress minimal BuildStatus");
    assert!(
        !compressed.is_empty(),
        "Compressed output should be non-empty even for minimal BuildStatus"
    );
}

// Test 17: Large repetitive bytes (10000 of 0xBB) compress smaller
#[test]
fn test_large_repetitive_bytes_compress_smaller() {
    let data: Vec<u8> = vec![0xBBu8; 10_000];
    let encoded = encode_to_vec(&data).expect("Failed to encode large repetitive bytes");
    let compressed = compress_lz4(&encoded).expect("Failed to compress large repetitive bytes");
    assert!(
        compressed.len() < encoded.len(),
        "Compressed size {} should be less than original size {}",
        compressed.len(),
        encoded.len()
    );
}

// Test 18: String with repeated pattern LZ4 roundtrip
#[test]
fn test_repeated_pattern_string_lz4_roundtrip() {
    let val: String = "build-artifact-checksum-".repeat(400);
    let encoded = encode_to_vec(&val).expect("Failed to encode repeated String");
    let compressed = compress_lz4(&encoded).expect("Failed to compress repeated String");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress repeated String");
    let (decoded, _): (String, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode repeated String");
    assert_eq!(val, decoded);
}

// Test 19: Unicode string in BuildArtifact name LZ4 roundtrip
#[test]
fn test_unicode_build_artifact_name_lz4_roundtrip() {
    let artifact = BuildArtifact {
        artifact_id: 888u64,
        name: String::from("アーティファクト_\u{1F527}_build.so"),
        version: String::from("2.0.\u{03B1}"),
        size_bytes: 4_096u64,
        checksums: vec![(
            String::from("sha256"),
            String::from("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        )],
    };
    let encoded = encode_to_vec(&artifact).expect("Failed to encode unicode BuildArtifact");
    let compressed = compress_lz4(&encoded).expect("Failed to compress unicode BuildArtifact");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress unicode BuildArtifact");
    let (decoded, _): (BuildArtifact, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode unicode BuildArtifact");
    assert_eq!(artifact, decoded);
}

// Test 20: Decompressed bytes exactly match original encoded bytes for BuildArtifact
#[test]
fn test_decompressed_bytes_exactly_match_encoded_build_artifact() {
    let artifact = BuildArtifact {
        artifact_id: 12345u64,
        name: String::from("exact-match-artifact.a"),
        version: String::from("4.5.6"),
        size_bytes: 65_536u64,
        checksums: vec![
            (
                String::from("sha256"),
                String::from("aabbccddeeff00112233445566778899"),
            ),
            (
                String::from("sha512"),
                String::from("ffeeddccbbaa99887766554433221100"),
            ),
            (String::from("crc32"), String::from("deadbeef")),
        ],
    };
    let encoded = encode_to_vec(&artifact).expect("Failed to encode artifact for exact match test");
    let compressed = compress_lz4(&encoded).expect("Failed to compress for exact match test");
    let decompressed =
        decompress_lz4(&compressed).expect("Failed to decompress for exact match test");
    assert_eq!(
        encoded, decompressed,
        "Decompressed bytes must exactly equal original encoded bytes"
    );
}

// Test 21: LCG pseudo-random Vec<u32> (300 items) roundtrip via LZ4
#[test]
fn test_lcg_random_vec_u32_lz4_roundtrip() {
    // Linear congruential generator: x_{n+1} = (a * x_n + c) mod 2^32
    let a: u32 = 1_664_525u32;
    let c: u32 = 1_013_904_223u32;
    let mut state: u32 = 0xCAFE_BABEu32;
    let items: Vec<u32> = (0..300)
        .map(|_| {
            state = state.wrapping_mul(a).wrapping_add(c);
            state
        })
        .collect();
    let encoded = encode_to_vec(&items).expect("Failed to encode LCG Vec<u32>");
    let compressed = compress_lz4(&encoded).expect("Failed to compress LCG Vec<u32>");
    let decompressed = decompress_lz4(&compressed).expect("Failed to decompress LCG Vec<u32>");
    let (decoded, _): (Vec<u32>, usize) =
        decode_from_slice(&decompressed).expect("Failed to decode LCG Vec<u32>");
    assert_eq!(items, decoded);
}

// Test 22: BuildArtifact with empty checksums LZ4 roundtrip
#[test]
fn test_build_artifact_empty_checksums_lz4_roundtrip() {
    let artifact = BuildArtifact {
        artifact_id: 0u64,
        name: String::from("no-checksum-artifact"),
        version: String::from("0.0.1"),
        size_bytes: 0u64,
        checksums: vec![],
    };
    let encoded =
        encode_to_vec(&artifact).expect("Failed to encode BuildArtifact with empty checksums");
    let compressed =
        compress_lz4(&encoded).expect("Failed to compress BuildArtifact with empty checksums");
    let decompressed = decompress_lz4(&compressed)
        .expect("Failed to decompress BuildArtifact with empty checksums");
    let (decoded, _): (BuildArtifact, usize) = decode_from_slice(&decompressed)
        .expect("Failed to decode BuildArtifact with empty checksums");
    assert_eq!(artifact, decoded);
}
