//! Advanced Zstd compression tests for OxiCode — Network Security / SIEM domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world intrusion detection and SIEM data: network flows, security
//! alerts, firewall rules, TLS certificates, and IDS signatures.

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AttackType {
    PortScan,
    BruteForce,
    SqlInjection,
    Xss,
    DDoS,
    Malware,
    Phishing,
    ZeroDay,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NetworkProtocol {
    TCP,
    UDP,
    ICMP,
    HTTP,
    HTTPS,
    DNS,
    SMTP,
    FTP,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlertStatus {
    Open,
    InProgress,
    Escalated,
    Resolved,
    FalsePositive,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkFlow {
    flow_id: u64,
    src_ip: u32,
    dst_ip: u32,
    src_port: u16,
    dst_port: u16,
    protocol: NetworkProtocol,
    bytes: u64,
    packets: u32,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SecurityAlert {
    alert_id: u64,
    threat_level: ThreatLevel,
    attack_type: AttackType,
    status: AlertStatus,
    src_ip: u32,
    detected_at: u64,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FirewallRule {
    rule_id: u32,
    priority: u16,
    src_cidr: u32,
    dst_cidr: u32,
    protocol: NetworkProtocol,
    action_deny: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TlsCertificate {
    cert_id: u64,
    domain: String,
    issuer: String,
    valid_from: u64,
    valid_until: u64,
    fingerprint: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IdsSignature {
    sig_id: u32,
    name: String,
    pattern: Vec<u8>,
    threat_level: ThreatLevel,
    enabled: bool,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_network_flow(id: u64) -> NetworkFlow {
    NetworkFlow {
        flow_id: id,
        src_ip: 0xC0A80001_u32.wrapping_add(id as u32 % 254),
        dst_ip: 0x08080808,
        src_port: (49152 + (id % 16384)) as u16,
        dst_port: 443,
        protocol: NetworkProtocol::HTTPS,
        bytes: 1024 * (id % 10 + 1),
        packets: 10 + (id % 90) as u32,
        timestamp: 1_700_000_000 + id * 100,
    }
}

fn make_security_alert(id: u64) -> SecurityAlert {
    SecurityAlert {
        alert_id: id,
        threat_level: ThreatLevel::High,
        attack_type: AttackType::BruteForce,
        status: AlertStatus::Open,
        src_ip: 0xC0A80001_u32.wrapping_add(id as u32 % 100),
        detected_at: 1_700_000_000 + id * 60,
        description: "Repeated failed authentication attempts from external host".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Basic round-trip for a single NetworkFlow.
#[test]
fn test_zstd_network_flow_roundtrip() {
    let flow = make_network_flow(42);
    let encoded = encode_to_vec(&flow).expect("encode NetworkFlow failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (NetworkFlow, usize) =
        decode_from_slice(&decompressed).expect("decode NetworkFlow failed");
    assert_eq!(flow, decoded);
}

/// 2. Round-trip for a SecurityAlert carrying a long description string.
#[test]
fn test_zstd_security_alert_roundtrip() {
    let alert = SecurityAlert {
        alert_id: 1001,
        threat_level: ThreatLevel::Critical,
        attack_type: AttackType::ZeroDay,
        status: AlertStatus::Escalated,
        src_ip: 0xAC100001,
        detected_at: 1_700_100_000,
        description: "Zero-day exploit detected targeting CVE-2024-99999; \
                       lateral movement observed across subnet 172.16.0.0/16."
            .to_string(),
    };
    let encoded = encode_to_vec(&alert).expect("encode SecurityAlert failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (SecurityAlert, usize) =
        decode_from_slice(&decompressed).expect("decode SecurityAlert failed");
    assert_eq!(alert, decoded);
}

/// 3. Round-trip for a FirewallRule (deny rule).
#[test]
fn test_zstd_firewall_rule_roundtrip() {
    let rule = FirewallRule {
        rule_id: 500,
        priority: 10,
        src_cidr: 0x00000000,
        dst_cidr: 0x0A000001,
        protocol: NetworkProtocol::TCP,
        action_deny: true,
    };
    let encoded = encode_to_vec(&rule).expect("encode FirewallRule failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (FirewallRule, usize) =
        decode_from_slice(&decompressed).expect("decode FirewallRule failed");
    assert_eq!(rule, decoded);
}

/// 4. Round-trip for a TlsCertificate with binary fingerprint.
#[test]
fn test_zstd_tls_certificate_roundtrip() {
    let cert = TlsCertificate {
        cert_id: 9999,
        domain: "secure.example.com".to_string(),
        issuer: "DigiCert SHA2 Extended Validation Server CA".to_string(),
        valid_from: 1_680_000_000,
        valid_until: 1_743_000_000,
        fingerprint: vec![
            0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
            0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44,
        ],
    };
    let encoded = encode_to_vec(&cert).expect("encode TlsCertificate failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (TlsCertificate, usize) =
        decode_from_slice(&decompressed).expect("decode TlsCertificate failed");
    assert_eq!(cert, decoded);
}

/// 5. Round-trip for an IdsSignature with a binary pattern payload.
#[test]
fn test_zstd_ids_signature_roundtrip() {
    let sig = IdsSignature {
        sig_id: 2001,
        name: "ET SCAN Nmap Scripting Engine User-Agent Detected".to_string(),
        pattern: b"Mozilla/5.0 (compatible; Nmap Scripting Engine;"
            .iter()
            .copied()
            .collect(),
        threat_level: ThreatLevel::Medium,
        enabled: true,
    };
    let encoded = encode_to_vec(&sig).expect("encode IdsSignature failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (IdsSignature, usize) =
        decode_from_slice(&decompressed).expect("decode IdsSignature failed");
    assert_eq!(sig, decoded);
}

/// 6. Round-trip for a Vec of NetworkFlow (small batch, no ratio assertion).
#[test]
fn test_zstd_vec_network_flows_roundtrip() {
    let flows: Vec<NetworkFlow> = (0u64..50).map(make_network_flow).collect();
    let encoded = encode_to_vec(&flows).expect("encode Vec<NetworkFlow> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<NetworkFlow>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<NetworkFlow> failed");
    assert_eq!(flows, decoded);
}

/// 7. Large network flow log — compression ratio must be > 1.0 (1 000+ repetitive flows).
#[test]
fn test_zstd_large_network_flow_log_compression_ratio() {
    // 1 200 flows built with a tight repetition pattern — highly compressible.
    let flows: Vec<NetworkFlow> = (0u64..1_200)
        .map(|i| NetworkFlow {
            flow_id: i,
            src_ip: 0xC0A80001,
            dst_ip: 0x08080808,
            src_port: 50000,
            dst_port: 443,
            protocol: NetworkProtocol::HTTPS,
            bytes: 2048,
            packets: 20,
            timestamp: 1_700_000_000 + i,
        })
        .collect();

    let encoded = encode_to_vec(&flows).expect("encode large flow log failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 1 200 repetitive flows",
        compressed.len(),
        encoded.len(),
    );

    // Verify full round-trip integrity.
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(
        decompressed.len(),
        encoded.len(),
        "Decompressed length must equal original encoded length"
    );
    let (decoded, _): (Vec<NetworkFlow>, usize) =
        decode_from_slice(&decompressed).expect("decode large flow log failed");
    assert_eq!(flows, decoded);
}

/// 8. Large alert log — compression ratio must be > 1.0 (1 000+ repetitive alerts).
#[test]
fn test_zstd_large_alert_log_compression_ratio() {
    let alerts: Vec<SecurityAlert> = (0u64..1_000)
        .map(|i| SecurityAlert {
            alert_id: i,
            threat_level: ThreatLevel::High,
            attack_type: AttackType::BruteForce,
            status: AlertStatus::Open,
            src_ip: 0xC0A800FE,
            detected_at: 1_700_000_000,
            description: "Repeated failed authentication attempts from external host".to_string(),
        })
        .collect();

    let encoded = encode_to_vec(&alerts).expect("encode large alert log failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 1 000 repetitive alerts",
        compressed.len(),
        encoded.len(),
    );

    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(
        decompressed.len(),
        encoded.len(),
        "Decompressed length must equal original encoded length"
    );
    let (decoded, _): (Vec<SecurityAlert>, usize) =
        decode_from_slice(&decompressed).expect("decode large alert log failed");
    assert_eq!(alerts, decoded);
}

/// 9. Verify compressed bytes differ from the original encoded bytes.
#[test]
fn test_zstd_compressed_differs_from_original() {
    let flow = make_network_flow(7);
    let encoded = encode_to_vec(&flow).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert_ne!(
        encoded, compressed,
        "Compressed data must differ from the original encoded bytes"
    );
}

/// 10. Decompressed length equals original encoded length.
#[test]
fn test_zstd_decompressed_length_equals_original() {
    let alert = make_security_alert(55);
    let encoded = encode_to_vec(&alert).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(
        decompressed.len(),
        encoded.len(),
        "Decompressed length ({}) must equal original encoded length ({})",
        decompressed.len(),
        encoded.len()
    );
}

/// 11. Error on truncated compressed data.
#[test]
fn test_zstd_truncated_data_returns_error() {
    let flow = make_network_flow(3);
    let encoded = encode_to_vec(&flow).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    // Truncate to just the first 8 bytes — not enough for a valid zstd frame.
    let truncated = &compressed[..8.min(compressed.len())];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress() must return Err for truncated zstd data"
    );
}

/// 12. Multiple independent compress/decompress cycles yield identical results.
#[test]
fn test_zstd_multiple_compress_decompress_cycles() {
    let cert = TlsCertificate {
        cert_id: 42,
        domain: "api.siem.internal".to_string(),
        issuer: "Internal CA".to_string(),
        valid_from: 1_690_000_000,
        valid_until: 1_750_000_000,
        fingerprint: (0u8..32).collect(),
    };
    let encoded = encode_to_vec(&cert).expect("encode TlsCertificate failed");

    for cycle in 1u32..=5 {
        let compressed = compress(&encoded, Compression::Zstd)
            .unwrap_or_else(|e| panic!("compress failed on cycle {cycle}: {e}"));
        let decompressed = decompress(&compressed)
            .unwrap_or_else(|e| panic!("decompress failed on cycle {cycle}: {e}"));
        let (decoded, _): (TlsCertificate, usize) = decode_from_slice(&decompressed)
            .unwrap_or_else(|e| panic!("decode failed on cycle {cycle}: {e}"));
        assert_eq!(cert, decoded, "round-trip mismatch on cycle {cycle}");
    }
}

/// 13. All ThreatLevel variants survive a compress/decompress round-trip.
#[test]
fn test_zstd_all_threat_levels_roundtrip() {
    let levels = vec![
        ThreatLevel::Low,
        ThreatLevel::Medium,
        ThreatLevel::High,
        ThreatLevel::Critical,
    ];
    let encoded = encode_to_vec(&levels).expect("encode Vec<ThreatLevel> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ThreatLevel>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<ThreatLevel> failed");
    assert_eq!(levels, decoded);
}

/// 14. All AttackType variants survive a compress/decompress round-trip.
#[test]
fn test_zstd_all_attack_types_roundtrip() {
    let types = vec![
        AttackType::PortScan,
        AttackType::BruteForce,
        AttackType::SqlInjection,
        AttackType::Xss,
        AttackType::DDoS,
        AttackType::Malware,
        AttackType::Phishing,
        AttackType::ZeroDay,
    ];
    let encoded = encode_to_vec(&types).expect("encode Vec<AttackType> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<AttackType>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<AttackType> failed");
    assert_eq!(types, decoded);
}

/// 15. All NetworkProtocol variants survive a compress/decompress round-trip.
#[test]
fn test_zstd_all_network_protocols_roundtrip() {
    let protocols = vec![
        NetworkProtocol::TCP,
        NetworkProtocol::UDP,
        NetworkProtocol::ICMP,
        NetworkProtocol::HTTP,
        NetworkProtocol::HTTPS,
        NetworkProtocol::DNS,
        NetworkProtocol::SMTP,
        NetworkProtocol::FTP,
    ];
    let encoded = encode_to_vec(&protocols).expect("encode Vec<NetworkProtocol> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<NetworkProtocol>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<NetworkProtocol> failed");
    assert_eq!(protocols, decoded);
}

/// 16. All AlertStatus variants survive a compress/decompress round-trip.
#[test]
fn test_zstd_all_alert_statuses_roundtrip() {
    let statuses = vec![
        AlertStatus::Open,
        AlertStatus::InProgress,
        AlertStatus::Escalated,
        AlertStatus::Resolved,
        AlertStatus::FalsePositive,
    ];
    let encoded = encode_to_vec(&statuses).expect("encode Vec<AlertStatus> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<AlertStatus>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<AlertStatus> failed");
    assert_eq!(statuses, decoded);
}

/// 17. Vec of mixed FirewallRules (allow and deny) round-trip.
#[test]
fn test_zstd_mixed_firewall_rules_roundtrip() {
    let rules: Vec<FirewallRule> = (0u32..20)
        .map(|i| FirewallRule {
            rule_id: i,
            priority: (i * 10) as u16,
            src_cidr: 0xC0A80000 + i,
            dst_cidr: 0xC0A90000 + i,
            protocol: if i % 2 == 0 {
                NetworkProtocol::TCP
            } else {
                NetworkProtocol::UDP
            },
            action_deny: i % 3 == 0,
        })
        .collect();
    let encoded = encode_to_vec(&rules).expect("encode Vec<FirewallRule> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<FirewallRule>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<FirewallRule> failed");
    assert_eq!(rules, decoded);
}

/// 18. Vec of TlsCertificates with varying fingerprint sizes round-trip.
#[test]
fn test_zstd_vec_tls_certificates_roundtrip() {
    let certs: Vec<TlsCertificate> = (0u64..10)
        .map(|i| TlsCertificate {
            cert_id: i,
            domain: format!("host{i}.siem.example.com"),
            issuer: "Corporate Internal CA G2".to_string(),
            valid_from: 1_700_000_000 + i * 1_000,
            valid_until: 1_730_000_000 + i * 1_000,
            fingerprint: (0u8..((20 + i % 12) as u8)).collect(),
        })
        .collect();
    let encoded = encode_to_vec(&certs).expect("encode Vec<TlsCertificate> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<TlsCertificate>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<TlsCertificate> failed");
    assert_eq!(certs, decoded);
}

/// 19. Vec of IDS signatures with binary patterns round-trip.
#[test]
fn test_zstd_vec_ids_signatures_roundtrip() {
    let patterns: &[&[u8]] = &[
        b"SELECT * FROM users WHERE",
        b"<script>alert(",
        b"/../../../etc/passwd",
        b"cmd.exe /c",
        b"powershell -EncodedCommand",
    ];
    let sigs: Vec<IdsSignature> = patterns
        .iter()
        .enumerate()
        .map(|(i, pat)| IdsSignature {
            sig_id: (3000 + i) as u32,
            name: format!("SIG-{:04}", 3000 + i),
            pattern: pat.to_vec(),
            threat_level: ThreatLevel::High,
            enabled: true,
        })
        .collect();
    let encoded = encode_to_vec(&sigs).expect("encode Vec<IdsSignature> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<IdsSignature>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<IdsSignature> failed");
    assert_eq!(sigs, decoded);
}

/// 20. SecurityAlert with FalsePositive status (edge-case enum variant) round-trip.
#[test]
fn test_zstd_false_positive_alert_roundtrip() {
    let alert = SecurityAlert {
        alert_id: 9_999_999,
        threat_level: ThreatLevel::Low,
        attack_type: AttackType::PortScan,
        status: AlertStatus::FalsePositive,
        src_ip: 0x7F000001,
        detected_at: 1_700_500_000,
        description: "Internal vulnerability scanner — confirmed false positive".to_string(),
    };
    let encoded = encode_to_vec(&alert).expect("encode FalsePositive alert failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (SecurityAlert, usize) =
        decode_from_slice(&decompressed).expect("decode FalsePositive alert failed");
    assert_eq!(alert, decoded);
}

/// 21. NetworkFlow carrying maximum field values (u64::MAX, u32::MAX, etc.) round-trip.
#[test]
fn test_zstd_network_flow_max_values_roundtrip() {
    let flow = NetworkFlow {
        flow_id: u64::MAX,
        src_ip: u32::MAX,
        dst_ip: u32::MAX,
        src_port: u16::MAX,
        dst_port: u16::MAX,
        protocol: NetworkProtocol::DNS,
        bytes: u64::MAX,
        packets: u32::MAX,
        timestamp: u64::MAX,
    };
    let encoded = encode_to_vec(&flow).expect("encode max-value NetworkFlow failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (NetworkFlow, usize) =
        decode_from_slice(&decompressed).expect("decode max-value NetworkFlow failed");
    assert_eq!(flow, decoded);
}

/// 22. Heterogeneous SIEM snapshot — tuple of all five struct types — round-trip.
#[test]
fn test_zstd_siem_snapshot_all_types_roundtrip() {
    // Encode a snapshot struct containing one instance of every domain type.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SiemSnapshot {
        flow: NetworkFlow,
        alert: SecurityAlert,
        rule: FirewallRule,
        cert: TlsCertificate,
        sig: IdsSignature,
    }

    let snapshot = SiemSnapshot {
        flow: make_network_flow(100),
        alert: SecurityAlert {
            alert_id: 200,
            threat_level: ThreatLevel::Critical,
            attack_type: AttackType::Malware,
            status: AlertStatus::InProgress,
            src_ip: 0xC0A80064,
            detected_at: 1_700_300_000,
            description: "C2 beaconing activity detected — malware dropper confirmed".to_string(),
        },
        rule: FirewallRule {
            rule_id: 1,
            priority: 1,
            src_cidr: 0x00000000,
            dst_cidr: 0xFFFFFFFF,
            protocol: NetworkProtocol::ICMP,
            action_deny: true,
        },
        cert: TlsCertificate {
            cert_id: 300,
            domain: "c2.malicious.example".to_string(),
            issuer: "Let's Encrypt Authority X3".to_string(),
            valid_from: 1_690_000_000,
            valid_until: 1_700_000_000,
            fingerprint: vec![0xBA, 0xD0, 0xC0, 0xFF, 0xEE],
        },
        sig: IdsSignature {
            sig_id: 9001,
            name: "ET MALWARE CobaltStrike Beacon Checkin".to_string(),
            pattern: b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04".to_vec(),
            threat_level: ThreatLevel::Critical,
            enabled: true,
        },
    };

    let encoded = encode_to_vec(&snapshot).expect("encode SiemSnapshot failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert_ne!(
        encoded, compressed,
        "Compressed snapshot must differ from encoded snapshot"
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(
        decompressed.len(),
        encoded.len(),
        "Decompressed length must equal original encoded length"
    );
    let (decoded, _): (SiemSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode SiemSnapshot failed");
    assert_eq!(snapshot, decoded);
}
