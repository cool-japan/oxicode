//! Zstd compression tests — cybersecurity / threat intelligence domain.
//!
//! Exercises oxicode encode/decode combined with Zstd compress/decompress
//! using realistic threat-intelligence data structures.

#![cfg(feature = "compression-zstd")]
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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum ThreatLevel {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum AttackVector {
    Network,
    Local,
    Physical,
    Adjacent,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Indicator {
    ioc_type: String,
    value: String,
    threat_level: ThreatLevel,
    first_seen: u64,
    last_seen: u64,
    confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ThreatActor {
    name: String,
    aliases: Vec<String>,
    ttps: Vec<String>,
    indicators: Vec<Indicator>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ThreatReport {
    id: u64,
    title: String,
    actors: Vec<ThreatActor>,
    attack_vector: AttackVector,
    published_at: u64,
    tags: Vec<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_indicator(ioc_type: &str, value: &str, level: ThreatLevel) -> Indicator {
    Indicator {
        ioc_type: ioc_type.to_string(),
        value: value.to_string(),
        threat_level: level,
        first_seen: 1_700_000_000,
        last_seen: 1_710_000_000,
        confidence: 0.85,
    }
}

fn make_actor(name: &str, n_indicators: usize) -> ThreatActor {
    let indicators = (0..n_indicators)
        .map(|i| {
            make_indicator(
                "ip",
                &format!("10.0.{}.{}", i / 256, i % 256),
                ThreatLevel::High,
            )
        })
        .collect();
    ThreatActor {
        name: name.to_string(),
        aliases: vec!["APT-X".to_string(), "GhostShell".to_string()],
        ttps: vec![
            "T1059".to_string(),
            "T1071".to_string(),
            "T1566".to_string(),
        ],
        indicators,
    }
}

fn make_report(id: u64, n_actors: usize, indicators_per_actor: usize) -> ThreatReport {
    let actors = (0..n_actors)
        .map(|i| make_actor(&format!("ThreatActor_{i}"), indicators_per_actor))
        .collect();
    ThreatReport {
        id,
        title: format!("Threat Report #{id}"),
        actors,
        attack_vector: AttackVector::Network,
        published_at: 1_710_000_000 + id,
        tags: vec![
            "apt".to_string(),
            "ransomware".to_string(),
            "lateral-movement".to_string(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

// 1. Basic Indicator roundtrip
#[test]
fn test_zstd_indicator_roundtrip() {
    let ind = make_indicator("domain", "malware-c2.example.com", ThreatLevel::Critical);
    let encoded = encode_to_vec(&ind).expect("encode indicator");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress indicator");
    let decompressed = decompress(&compressed).expect("decompress indicator");
    let (decoded, _): (Indicator, usize) =
        decode_from_slice(&decompressed).expect("decode indicator");
    assert_eq!(ind, decoded);
}

// 2. ThreatActor roundtrip
#[test]
fn test_zstd_threat_actor_roundtrip() {
    let actor = make_actor("LazarusGroup", 10);
    let encoded = encode_to_vec(&actor).expect("encode actor");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress actor");
    let decompressed = decompress(&compressed).expect("decompress actor");
    let (decoded, _): (ThreatActor, usize) =
        decode_from_slice(&decompressed).expect("decode actor");
    assert_eq!(actor, decoded);
}

// 3. Full ThreatReport roundtrip
#[test]
fn test_zstd_threat_report_roundtrip() {
    let report = make_report(1, 3, 20);
    let encoded = encode_to_vec(&report).expect("encode report");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress report");
    let decompressed = decompress(&compressed).expect("decompress report");
    let (decoded, _): (ThreatReport, usize) =
        decode_from_slice(&decompressed).expect("decode report");
    assert_eq!(report, decoded);
}

// 4. Large report with many indicators (compression reduces size)
#[test]
fn test_zstd_large_report_many_indicators() {
    let report = make_report(42, 5, 500);
    let encoded = encode_to_vec(&report).expect("encode large report");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress large report");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} B) should be smaller than encoded ({} B) for a large repetitive report",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("decompress large report");
    let (decoded, _): (ThreatReport, usize) =
        decode_from_slice(&decompressed).expect("decode large report");
    assert_eq!(report, decoded);
}

// 5. Compressed size < original for highly repetitive IOC data
#[test]
fn test_zstd_repetitive_ioc_compression_ratio() {
    // 1 000 identical indicators — extremely repetitive
    let indicators: Vec<Indicator> = (0..1_000)
        .map(|_| {
            make_indicator(
                "hash",
                "aabbccddeeff00112233445566778899",
                ThreatLevel::High,
            )
        })
        .collect();
    let encoded = encode_to_vec(&indicators).expect("encode indicators");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress indicators");
    assert!(
        compressed.len() < encoded.len(),
        "repetitive IOC list: compressed ({}) should be < encoded ({})",
        compressed.len(),
        encoded.len()
    );
}

// 6. Empty ThreatReport roundtrip
#[test]
fn test_zstd_empty_report() {
    let report = ThreatReport {
        id: 0,
        title: String::new(),
        actors: vec![],
        attack_vector: AttackVector::Local,
        published_at: 0,
        tags: vec![],
    };
    let encoded = encode_to_vec(&report).expect("encode empty report");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress empty report");
    let decompressed = decompress(&compressed).expect("decompress empty report");
    let (decoded, _): (ThreatReport, usize) =
        decode_from_slice(&decompressed).expect("decode empty report");
    assert_eq!(report, decoded);
}

// 7. ThreatLevel::Info variant
#[test]
fn test_zstd_threat_level_info() {
    let ind = make_indicator(
        "url",
        "https://legitimate-looking.example/path",
        ThreatLevel::Info,
    );
    let encoded = encode_to_vec(&ind).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (Indicator, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.threat_level, ThreatLevel::Info);
}

// 8. ThreatLevel::Low variant
#[test]
fn test_zstd_threat_level_low() {
    let ind = make_indicator("email", "spammer@example.net", ThreatLevel::Low);
    let encoded = encode_to_vec(&ind).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (Indicator, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.threat_level, ThreatLevel::Low);
}

// 9. ThreatLevel::Medium variant
#[test]
fn test_zstd_threat_level_medium() {
    let ind = make_indicator("ip", "192.168.100.1", ThreatLevel::Medium);
    let encoded = encode_to_vec(&ind).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (Indicator, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.threat_level, ThreatLevel::Medium);
}

// 10. ThreatLevel::High variant
#[test]
fn test_zstd_threat_level_high() {
    let ind = make_indicator("ip", "10.20.30.40", ThreatLevel::High);
    let encoded = encode_to_vec(&ind).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (Indicator, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.threat_level, ThreatLevel::High);
}

// 11. ThreatLevel::Critical variant
#[test]
fn test_zstd_threat_level_critical() {
    let ind = make_indicator("hash", "deadbeef".repeat(8).as_str(), ThreatLevel::Critical);
    let encoded = encode_to_vec(&ind).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (Indicator, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.threat_level, ThreatLevel::Critical);
}

// 12. AttackVector::Network variant
#[test]
fn test_zstd_attack_vector_network() {
    let report = ThreatReport {
        id: 10,
        title: "Network-based APT".to_string(),
        actors: vec![],
        attack_vector: AttackVector::Network,
        published_at: 1_710_000_010,
        tags: vec!["network".to_string()],
    };
    let encoded = encode_to_vec(&report).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (ThreatReport, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.attack_vector, AttackVector::Network);
}

// 13. AttackVector::Local variant
#[test]
fn test_zstd_attack_vector_local() {
    let report = ThreatReport {
        id: 11,
        title: "Insider Threat".to_string(),
        actors: vec![],
        attack_vector: AttackVector::Local,
        published_at: 1_710_000_011,
        tags: vec!["insider".to_string()],
    };
    let encoded = encode_to_vec(&report).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (ThreatReport, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.attack_vector, AttackVector::Local);
}

// 14. AttackVector::Physical variant
#[test]
fn test_zstd_attack_vector_physical() {
    let report = ThreatReport {
        id: 12,
        title: "Physical Access Campaign".to_string(),
        actors: vec![],
        attack_vector: AttackVector::Physical,
        published_at: 1_710_000_012,
        tags: vec!["physical".to_string()],
    };
    let encoded = encode_to_vec(&report).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (ThreatReport, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.attack_vector, AttackVector::Physical);
}

// 15. AttackVector::Adjacent variant
#[test]
fn test_zstd_attack_vector_adjacent() {
    let report = ThreatReport {
        id: 13,
        title: "VLAN Hopping Attack".to_string(),
        actors: vec![],
        attack_vector: AttackVector::Adjacent,
        published_at: 1_710_000_013,
        tags: vec!["adjacent".to_string()],
    };
    let encoded = encode_to_vec(&report).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (ThreatReport, usize) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(decoded.attack_vector, AttackVector::Adjacent);
}

// 16. Corruption detection — destroy the zstd frame magic bytes
#[test]
fn test_zstd_corruption_detected_single_byte_flip() {
    let report = make_report(99, 2, 5);
    let encoded = encode_to_vec(&report).expect("encode");
    let mut compressed = compress(&encoded, Compression::Zstd).expect("compress");

    // The oxicode compression header is 5 bytes long.  Immediately after it
    // begins the raw zstd frame, which starts with the 4-byte magic
    // 0xFD2F B528.  Overwriting these bytes is guaranteed to make the zstd
    // decoder reject the frame.
    compressed[5] = 0x00;
    compressed[6] = 0x00;
    compressed[7] = 0x00;
    compressed[8] = 0x00;

    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress should fail when the zstd frame magic is destroyed"
    );
}

// 17. Corruption detection — truncated compressed stream
#[test]
fn test_zstd_corruption_detected_truncated_stream() {
    let report = make_report(100, 2, 5);
    let encoded = encode_to_vec(&report).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");

    // Drop the last quarter of the stream
    let truncated = &compressed[..compressed.len() * 3 / 4];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress should fail on truncated zstd stream"
    );
}

// 18. Idempotent decompression — decompress-then-re-decompress raw bytes errors
#[test]
fn test_zstd_double_decompress_raw_fails() {
    let report = make_report(200, 1, 3);
    let encoded = encode_to_vec(&report).expect("encode");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress");
    let decompressed = decompress(&compressed).expect("first decompress");

    // The decompressed bytes are raw oxicode-encoded data, not a compressed stream
    let result = decompress(&decompressed);
    assert!(
        result.is_err(),
        "decompressing raw encoded bytes should return an error (no compression header)"
    );
}

// 19. ZstdLevel(1) roundtrip — fastest compression level
#[test]
fn test_zstd_level_1_report_roundtrip() {
    let report = make_report(301, 2, 50);
    let encoded = encode_to_vec(&report).expect("encode");
    let compressed = compress(&encoded, Compression::ZstdLevel(1)).expect("compress level-1");
    let decompressed = decompress(&compressed).expect("decompress level-1");
    let (decoded, _): (ThreatReport, usize) =
        decode_from_slice(&decompressed).expect("decode level-1");
    assert_eq!(report, decoded);
}

// 20. ZstdLevel(19) roundtrip — high compression level
#[test]
fn test_zstd_level_19_report_roundtrip() {
    let report = make_report(302, 2, 50);
    let encoded = encode_to_vec(&report).expect("encode");
    let compressed = compress(&encoded, Compression::ZstdLevel(19)).expect("compress level-19");
    let decompressed = decompress(&compressed).expect("decompress level-19");
    let (decoded, _): (ThreatReport, usize) =
        decode_from_slice(&decompressed).expect("decode level-19");
    assert_eq!(report, decoded);
}

// 21. Vec<ThreatReport> batch roundtrip
#[test]
fn test_zstd_batch_reports_roundtrip() {
    let reports: Vec<ThreatReport> = (0..20).map(|i| make_report(i, 2, 10)).collect();
    let encoded = encode_to_vec(&reports).expect("encode batch");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress batch");
    let decompressed = decompress(&compressed).expect("decompress batch");
    let (decoded, _): (Vec<ThreatReport>, usize) =
        decode_from_slice(&decompressed).expect("decode batch");
    assert_eq!(reports, decoded);
}

// 22. Mixed threat levels in a single report — all variants present
#[test]
fn test_zstd_all_threat_levels_in_one_report() {
    let levels = [
        ThreatLevel::Info,
        ThreatLevel::Low,
        ThreatLevel::Medium,
        ThreatLevel::High,
        ThreatLevel::Critical,
    ];
    let indicators: Vec<Indicator> = levels
        .iter()
        .enumerate()
        .map(|(i, lvl)| Indicator {
            ioc_type: "domain".to_string(),
            value: format!("c2-{i}.example.com"),
            threat_level: lvl.clone(),
            first_seen: 1_700_000_000 + i as u64,
            last_seen: 1_710_000_000 + i as u64,
            confidence: 0.5 + 0.1 * i as f32,
        })
        .collect();
    let actor = ThreatActor {
        name: "OmniThreat".to_string(),
        aliases: vec!["OT".to_string()],
        ttps: vec!["T1190".to_string()],
        indicators,
    };
    let report = ThreatReport {
        id: 999,
        title: "All Threat Level Coverage".to_string(),
        actors: vec![actor],
        attack_vector: AttackVector::Network,
        published_at: 1_710_999_999,
        tags: vec!["coverage-test".to_string()],
    };

    let encoded = encode_to_vec(&report).expect("encode mixed-levels report");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress mixed-levels report");
    let decompressed = decompress(&compressed).expect("decompress mixed-levels report");
    let (decoded, _): (ThreatReport, usize) =
        decode_from_slice(&decompressed).expect("decode mixed-levels report");

    assert_eq!(report, decoded);
    // Confirm all five threat levels survived the roundtrip
    let decoded_levels: Vec<&ThreatLevel> = decoded
        .actors
        .first()
        .expect("actor present")
        .indicators
        .iter()
        .map(|ind| &ind.threat_level)
        .collect();
    assert!(decoded_levels.contains(&&ThreatLevel::Info));
    assert!(decoded_levels.contains(&&ThreatLevel::Low));
    assert!(decoded_levels.contains(&&ThreatLevel::Medium));
    assert!(decoded_levels.contains(&&ThreatLevel::High));
    assert!(decoded_levels.contains(&&ThreatLevel::Critical));
}
