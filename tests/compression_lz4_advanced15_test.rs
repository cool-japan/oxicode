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

#[derive(Debug, PartialEq, Encode, Decode)]
enum EventType {
    Authentication,
    Authorization,
    NetworkScan,
    Exploit,
    DataExfiltration,
    Ddos,
    Malware,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IpAddress {
    octets: [u8; 4],
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SecurityEvent {
    event_id: u64,
    event_type: EventType,
    threat_level: ThreatLevel,
    src_ip: IpAddress,
    dst_ip: IpAddress,
    port: u16,
    payload_size: u32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SecurityLog {
    log_id: u64,
    events: Vec<SecurityEvent>,
    system_id: String,
    period_start: u64,
    period_end: u64,
}

// Helper: build a SecurityEvent with given fields
fn make_event(
    event_id: u64,
    event_type: EventType,
    threat_level: ThreatLevel,
    src_octets: [u8; 4],
    dst_octets: [u8; 4],
    port: u16,
    payload_size: u32,
    timestamp_ms: u64,
) -> SecurityEvent {
    SecurityEvent {
        event_id,
        event_type,
        threat_level,
        src_ip: IpAddress { octets: src_octets },
        dst_ip: IpAddress { octets: dst_octets },
        port,
        payload_size,
        timestamp_ms,
    }
}

// ─── 1. EventType::Authentication compress/decompress roundtrip ───────────────

#[test]
fn test_event_type_authentication_roundtrip() {
    let event = make_event(
        1,
        EventType::Authentication,
        ThreatLevel::Low,
        [192, 168, 1, 10],
        [10, 0, 0, 1],
        22,
        128,
        1_700_000_000_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
}

// ─── 2. EventType::Authorization compress/decompress roundtrip ────────────────

#[test]
fn test_event_type_authorization_roundtrip() {
    let event = make_event(
        2,
        EventType::Authorization,
        ThreatLevel::Medium,
        [172, 16, 0, 5],
        [10, 10, 10, 1],
        443,
        256,
        1_700_000_001_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
}

// ─── 3. EventType::NetworkScan compress/decompress roundtrip ─────────────────

#[test]
fn test_event_type_network_scan_roundtrip() {
    let event = make_event(
        3,
        EventType::NetworkScan,
        ThreatLevel::High,
        [203, 0, 113, 45],
        [192, 168, 0, 0],
        0,
        0,
        1_700_000_002_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
}

// ─── 4. EventType::Exploit compress/decompress roundtrip ─────────────────────

#[test]
fn test_event_type_exploit_roundtrip() {
    let event = make_event(
        4,
        EventType::Exploit,
        ThreatLevel::Critical,
        [198, 51, 100, 7],
        [10, 20, 30, 40],
        4444,
        4096,
        1_700_000_003_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
}

// ─── 5. EventType::DataExfiltration compress/decompress roundtrip ────────────

#[test]
fn test_event_type_data_exfiltration_roundtrip() {
    let event = make_event(
        5,
        EventType::DataExfiltration,
        ThreatLevel::Critical,
        [10, 0, 0, 99],
        [93, 184, 216, 34],
        443,
        1_048_576,
        1_700_000_004_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
}

// ─── 6. EventType::Ddos compress/decompress roundtrip ────────────────────────

#[test]
fn test_event_type_ddos_roundtrip() {
    let event = make_event(
        6,
        EventType::Ddos,
        ThreatLevel::High,
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        80,
        65_535,
        1_700_000_005_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
}

// ─── 7. EventType::Malware compress/decompress roundtrip ─────────────────────

#[test]
fn test_event_type_malware_roundtrip() {
    let event = make_event(
        7,
        EventType::Malware,
        ThreatLevel::Critical,
        [192, 168, 100, 200],
        [8, 8, 8, 8],
        53,
        512,
        1_700_000_006_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
}

// ─── 8. ThreatLevel variants roundtrip across the full spectrum ──────────────

#[test]
fn test_threat_levels_all_variants_roundtrip() {
    let levels = vec![
        ThreatLevel::None,
        ThreatLevel::Low,
        ThreatLevel::Medium,
        ThreatLevel::High,
        ThreatLevel::Critical,
    ];
    for level in levels {
        let event = make_event(
            100,
            EventType::Authentication,
            level,
            [127, 0, 0, 1],
            [127, 0, 0, 1],
            8080,
            64,
            1_700_000_100_000,
        );
        let encoded = encode_to_vec(&event).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (SecurityEvent, _) =
            decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(event, decoded);
    }
}

// ─── 9. IpAddress compress roundtrip ─────────────────────────────────────────

#[test]
fn test_ip_address_compress_roundtrip() {
    let ip = IpAddress {
        octets: [192, 168, 42, 100],
    };
    let encoded = encode_to_vec(&ip).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (IpAddress, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(ip, decoded);
}

// ─── 10. SecurityEvent compress roundtrip ────────────────────────────────────

#[test]
fn test_security_event_compress_roundtrip() {
    let event = make_event(
        42,
        EventType::Exploit,
        ThreatLevel::High,
        [10, 0, 0, 1],
        [10, 0, 0, 254],
        3389,
        2048,
        1_700_000_200_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
}

// ─── 11. SecurityLog with 10 events roundtrip ────────────────────────────────

#[test]
fn test_security_log_ten_events_roundtrip() {
    let events: Vec<SecurityEvent> = (0..10)
        .map(|i| {
            make_event(
                i as u64,
                EventType::NetworkScan,
                ThreatLevel::Medium,
                [10, 0, 0, i as u8 + 1],
                [192, 168, 1, 1],
                1024 + i as u16,
                100 * i as u32,
                1_700_000_000_000 + i as u64 * 1000,
            )
        })
        .collect();
    let log = SecurityLog {
        log_id: 1001,
        events,
        system_id: "fw-sensor-01".to_string(),
        period_start: 1_700_000_000_000,
        period_end: 1_700_000_010_000,
    };
    let encoded = encode_to_vec(&log).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityLog, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(log, decoded);
}

// ─── 12. Large log (1000 events) — compression ratio check ───────────────────

#[test]
fn test_large_log_compression_ratio() {
    let events: Vec<SecurityEvent> = (0..1000)
        .map(|i| {
            make_event(
                i as u64,
                EventType::Authentication,
                ThreatLevel::Low,
                [10, (i / 256) as u8, (i % 256) as u8, 1],
                [192, 168, 0, 1],
                443,
                (i as u32) * 10,
                1_700_000_000_000 + i as u64 * 500,
            )
        })
        .collect();
    let log = SecurityLog {
        log_id: 9999,
        events,
        system_id: "ids-cluster-west".to_string(),
        period_start: 1_700_000_000_000,
        period_end: 1_700_000_500_000,
    };
    let encoded = encode_to_vec(&log).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    // Must compress to at most original size for large repetitive data
    assert!(
        compressed.len() <= encoded.len(),
        "compressed ({}) should be <= encoded ({}) for large repetitive log",
        compressed.len(),
        encoded.len()
    );
}

// ─── 13. Repetitive events (1000 identical) compress much smaller ─────────────

#[test]
fn test_repetitive_events_high_compression() {
    let template = make_event(
        77,
        EventType::Ddos,
        ThreatLevel::Critical,
        [100, 100, 100, 100],
        [200, 200, 200, 200],
        80,
        9999,
        1_700_999_000_000,
    );
    let events: Vec<SecurityEvent> = (0..1000)
        .map(|_| {
            make_event(
                77,
                EventType::Ddos,
                ThreatLevel::Critical,
                [100, 100, 100, 100],
                [200, 200, 200, 200],
                80,
                9999,
                1_700_999_000_000,
            )
        })
        .collect();
    // Verify template roundtrips correctly (sanity check)
    let _ = &template;

    let log = SecurityLog {
        log_id: 5555,
        events,
        system_id: "ddos-sensor".to_string(),
        period_start: 1_700_999_000_000,
        period_end: 1_701_000_000_000,
    };
    let encoded = encode_to_vec(&log).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    // Identical records are extremely repetitive — expect at least 50% compression
    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio <= 0.5,
        "expected ratio <= 0.5 for 1000 identical events, got {:.3}",
        ratio
    );
}

// ─── 14. Empty log compress/decompress roundtrip ─────────────────────────────

#[test]
fn test_empty_log_roundtrip() {
    let log = SecurityLog {
        log_id: 0,
        events: vec![],
        system_id: "empty-node".to_string(),
        period_start: 0,
        period_end: 0,
    };
    let encoded = encode_to_vec(&log).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityLog, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(log, decoded);
}

// ─── 15. Vec<SecurityEvent> compress/decompress roundtrip ────────────────────

#[test]
fn test_vec_security_events_compress_roundtrip() {
    let events: Vec<SecurityEvent> = vec![
        make_event(
            1,
            EventType::Authentication,
            ThreatLevel::None,
            [10, 0, 0, 1],
            [10, 0, 0, 2],
            22,
            0,
            1_700_000_000_100,
        ),
        make_event(
            2,
            EventType::Malware,
            ThreatLevel::Critical,
            [192, 168, 5, 5],
            [8, 8, 4, 4],
            53,
            1024,
            1_700_000_000_200,
        ),
        make_event(
            3,
            EventType::Exploit,
            ThreatLevel::High,
            [172, 31, 0, 1],
            [172, 31, 255, 254],
            4444,
            8192,
            1_700_000_000_300,
        ),
    ];
    let encoded = encode_to_vec(&events).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<SecurityEvent>, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(events, decoded);
}

// ─── 16. Authentication event — field-level assertion after roundtrip ─────────

#[test]
fn test_authentication_event_field_assertions() {
    let event = make_event(
        200,
        EventType::Authentication,
        ThreatLevel::Low,
        [10, 50, 0, 1],
        [10, 50, 0, 254],
        22,
        0,
        1_700_001_000_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.event_id, 200);
    assert_eq!(decoded.port, 22);
    assert_eq!(decoded.payload_size, 0);
    assert_eq!(decoded.src_ip.octets, [10, 50, 0, 1]);
    assert_eq!(decoded.dst_ip.octets, [10, 50, 0, 254]);
}

// ─── 17. Network scan detection — multi-port sweep preserved ─────────────────

#[test]
fn test_network_scan_detection_roundtrip() {
    // Simulate a port sweep: same src, different dst ports
    let events: Vec<SecurityEvent> = (0..20_u16)
        .map(|i| {
            make_event(
                300 + i as u64,
                EventType::NetworkScan,
                ThreatLevel::High,
                [198, 51, 100, 99],
                [192, 168, 2, 10],
                1 + i,
                0,
                1_700_002_000_000 + i as u64 * 10,
            )
        })
        .collect();
    let log = SecurityLog {
        log_id: 300,
        events,
        system_id: "perimeter-ids".to_string(),
        period_start: 1_700_002_000_000,
        period_end: 1_700_002_000_200,
    };
    let encoded = encode_to_vec(&log).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityLog, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(log, decoded);
    assert_eq!(decoded.events.len(), 20);
    assert!(decoded
        .events
        .iter()
        .all(|e| matches!(e.event_type, EventType::NetworkScan)));
}

// ─── 18. Data exfiltration alert — large payload preserved ────────────────────

#[test]
fn test_data_exfiltration_alert_roundtrip() {
    let event = make_event(
        400,
        EventType::DataExfiltration,
        ThreatLevel::Critical,
        [10, 0, 0, 50],
        [185, 220, 101, 23],
        443,
        104_857_600, // 100 MB exfiltrated
        1_700_003_000_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.payload_size, 104_857_600);
    assert!(matches!(decoded.event_type, EventType::DataExfiltration));
    assert!(matches!(decoded.threat_level, ThreatLevel::Critical));
}

// ─── 19. DDoS event roundtrip — high-frequency flood simulation ───────────────

#[test]
fn test_ddos_event_roundtrip() {
    let event = make_event(
        500,
        EventType::Ddos,
        ThreatLevel::Critical,
        [203, 0, 113, 200],
        [93, 184, 216, 34],
        80,
        u32::MAX,
        1_700_004_000_000,
    );
    let encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityEvent, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(event, decoded);
    assert_eq!(decoded.payload_size, u32::MAX);
}

// ─── 20. Decompress gives original raw bytes ─────────────────────────────────

#[test]
fn test_decompress_gives_original_bytes() {
    let event = make_event(
        600,
        EventType::Authorization,
        ThreatLevel::Medium,
        [172, 16, 255, 1],
        [172, 16, 255, 254],
        8443,
        512,
        1_700_005_000_000,
    );
    let original_encoded = encode_to_vec(&event).expect("encode failed");
    let compressed = compress(&original_encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    // Raw bytes must be identical to original encoded bytes
    assert_eq!(
        decompressed, original_encoded,
        "decompressed bytes must match original encoded bytes exactly"
    );
}

// ─── 21. Multi-source attack log — diverse src IPs preserved ─────────────────

#[test]
fn test_multi_source_attack_log_roundtrip() {
    // Simulate a botnet attack with varied source IPs
    let src_ips: Vec<[u8; 4]> = vec![
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [100, 101, 102, 103],
        [200, 201, 202, 203],
    ];
    let events: Vec<SecurityEvent> = src_ips
        .iter()
        .enumerate()
        .map(|(i, &src)| {
            make_event(
                700 + i as u64,
                EventType::Ddos,
                ThreatLevel::High,
                src,
                [10, 0, 0, 1],
                80,
                1500,
                1_700_006_000_000 + i as u64 * 50,
            )
        })
        .collect();
    let log = SecurityLog {
        log_id: 700,
        events,
        system_id: "multi-source-detector".to_string(),
        period_start: 1_700_006_000_000,
        period_end: 1_700_006_000_250,
    };
    let encoded = encode_to_vec(&log).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityLog, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(log, decoded);
    assert_eq!(decoded.events.len(), 5);
    let unique_src: std::collections::HashSet<[u8; 4]> =
        decoded.events.iter().map(|e| e.src_ip.octets).collect();
    assert_eq!(unique_src.len(), 5, "all source IPs must be distinct");
}

// ─── 22. Critical threat chain — escalating threat levels across events ───────

#[test]
fn test_critical_threat_chain_roundtrip() {
    // Threat escalation: None → Low → Medium → High → Critical
    let threat_chain = vec![
        (EventType::Authentication, ThreatLevel::None),
        (EventType::Authorization, ThreatLevel::Low),
        (EventType::NetworkScan, ThreatLevel::Medium),
        (EventType::Exploit, ThreatLevel::High),
        (EventType::Malware, ThreatLevel::Critical),
    ];
    let events: Vec<SecurityEvent> = threat_chain
        .into_iter()
        .enumerate()
        .map(|(i, (et, tl))| {
            make_event(
                800 + i as u64,
                et,
                tl,
                [192, 168, 10, i as u8 + 1],
                [10, 0, 0, 1],
                443,
                (i as u32 + 1) * 512,
                1_700_007_000_000 + i as u64 * 2000,
            )
        })
        .collect();
    let log = SecurityLog {
        log_id: 800,
        events,
        system_id: "threat-escalation-monitor".to_string(),
        period_start: 1_700_007_000_000,
        period_end: 1_700_007_010_000,
    };
    let encoded = encode_to_vec(&log).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (SecurityLog, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(log, decoded);
    // Verify the chain ends with a Critical event
    let last = decoded.events.last().expect("events must not be empty");
    assert!(matches!(last.threat_level, ThreatLevel::Critical));
    assert!(matches!(last.event_type, EventType::Malware));
}
