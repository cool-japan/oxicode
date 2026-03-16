#![cfg(feature = "async-tokio")]
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
use oxicode::async_io::{AsyncDecoder, AsyncEncoder};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThreatLevel {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IocType {
    IpAddress,
    Domain,
    Url,
    FileHash,
    EmailAddress,
    RegistryKey,
    MutexName,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MalwareCategory {
    Ransomware,
    Trojan,
    Worm,
    Rootkit,
    Spyware,
    Botnet,
    Dropper,
    Backdoor,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AttackVector {
    Network,
    AdjacentNetwork,
    Local,
    Physical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IncidentStatus {
    Open,
    Investigating,
    Contained,
    Eradicated,
    Recovered,
    Closed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IocRecord {
    ioc_id: u64,
    ioc_type: IocType,
    value: String,
    confidence: u8,
    first_seen_ts: u64,
    last_seen_ts: u64,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CvssScore {
    cve_id: String,
    base_score_x10: u8,
    attack_vector: AttackVector,
    confidentiality_impact: u8,
    integrity_impact: u8,
    availability_impact: u8,
    exploitability_score_x10: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThreatActor {
    actor_id: u32,
    alias: String,
    nation_state: Option<String>,
    active_since_year: u16,
    threat_level: ThreatLevel,
    known_ttps: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MalwareFamily {
    family_id: u32,
    name: String,
    category: MalwareCategory,
    first_observed_ts: u64,
    yara_rule_count: u16,
    c2_domains: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkIntrusionEvent {
    event_id: u64,
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    bytes_transferred: u64,
    threat_level: ThreatLevel,
    timestamp: u64,
    signature_id: u32,
    signature_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HoneypotTrigger {
    trigger_id: u64,
    honeypot_id: u32,
    attacker_ip: [u8; 4],
    attacker_port: u16,
    payload_bytes: Vec<u8>,
    timestamp: u64,
    ioc_refs: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IncidentTimelineEntry {
    entry_id: u64,
    incident_id: u32,
    status: IncidentStatus,
    actor_alias: Option<String>,
    description: String,
    timestamp: u64,
    affected_asset_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AptCampaign {
    campaign_id: u32,
    name: String,
    actor_ids: Vec<u32>,
    malware_family_ids: Vec<u32>,
    target_sectors: Vec<String>,
    start_ts: u64,
    end_ts: Option<u64>,
    total_iocs: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThreatHuntingQuery {
    query_id: u32,
    title: String,
    query_text: String,
    mitre_technique_ids: Vec<String>,
    severity: ThreatLevel,
    created_ts: u64,
    last_run_ts: Option<u64>,
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
}

// ── Test 1: async write/read single IocRecord (IP address IOC) ────────────────

#[test]
fn test_async_single_ioc_record_ip() {
    rt().block_on(async {
        let ioc = IocRecord {
            ioc_id: 1001,
            ioc_type: IocType::IpAddress,
            value: String::from("192.168.100.5"),
            confidence: 95,
            first_seen_ts: 1_700_000_000_000,
            last_seen_ts: 1_700_500_000_000,
            tags: vec![String::from("apt29"), String::from("c2")],
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&ioc).await.expect("write ioc record");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<IocRecord> = decoder
            .read_item::<IocRecord>()
            .await
            .expect("read ioc record");
        assert_eq!(result, Some(ioc));
    });
}

// ── Test 2: async write/read CvssScore with Network attack vector ─────────────

#[test]
fn test_async_cvss_score_network_vector() {
    rt().block_on(async {
        let score = CvssScore {
            cve_id: String::from("CVE-2024-12345"),
            base_score_x10: 98,
            attack_vector: AttackVector::Network,
            confidentiality_impact: 3,
            integrity_impact: 3,
            availability_impact: 3,
            exploitability_score_x10: 39,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&score).await.expect("write cvss score");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<CvssScore>()
            .await
            .expect("read cvss score");
        assert_eq!(result, Some(score));
    });
}

// ── Test 3: All ThreatLevel variants round-trip ───────────────────────────────

#[test]
fn test_all_threat_level_variants() {
    rt().block_on(async {
        let variants = vec![
            ThreatLevel::Informational,
            ThreatLevel::Low,
            ThreatLevel::Medium,
            ThreatLevel::High,
            ThreatLevel::Critical,
        ];
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for v in &variants {
            encoder
                .write_item(v)
                .await
                .expect("write ThreatLevel variant");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &variants {
            let result = decoder
                .read_item::<ThreatLevel>()
                .await
                .expect("read ThreatLevel variant");
            assert_eq!(result.as_ref(), Some(expected));
        }
    });
}

// ── Test 4: All MalwareCategory variants round-trip ──────────────────────────

#[test]
fn test_all_malware_category_variants() {
    rt().block_on(async {
        let variants = vec![
            MalwareCategory::Ransomware,
            MalwareCategory::Trojan,
            MalwareCategory::Worm,
            MalwareCategory::Rootkit,
            MalwareCategory::Spyware,
            MalwareCategory::Botnet,
            MalwareCategory::Dropper,
            MalwareCategory::Backdoor,
        ];
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for v in &variants {
            encoder
                .write_item(v)
                .await
                .expect("write MalwareCategory variant");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &variants {
            let result = decoder
                .read_item::<MalwareCategory>()
                .await
                .expect("read MalwareCategory variant");
            assert_eq!(result.as_ref(), Some(expected));
        }
    });
}

// ── Test 5: ThreatActor with nation-state and known TTPs ─────────────────────

#[test]
fn test_async_threat_actor_nation_state() {
    rt().block_on(async {
        let actor = ThreatActor {
            actor_id: 42,
            alias: String::from("FANCY BEAR"),
            nation_state: Some(String::from("RU")),
            active_since_year: 2008,
            threat_level: ThreatLevel::Critical,
            known_ttps: vec![
                String::from("T1566"),
                String::from("T1078"),
                String::from("T1059.003"),
            ],
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&actor)
            .await
            .expect("write threat actor");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<ThreatActor>()
            .await
            .expect("read threat actor");
        assert_eq!(result, Some(actor));
    });
}

// ── Test 6: ThreatActor without nation-state (criminal group) ─────────────────

#[test]
fn test_async_threat_actor_no_nation_state() {
    rt().block_on(async {
        let actor = ThreatActor {
            actor_id: 99,
            alias: String::from("LockBit"),
            nation_state: None,
            active_since_year: 2019,
            threat_level: ThreatLevel::High,
            known_ttps: vec![String::from("T1486"), String::from("T1490")],
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&actor)
            .await
            .expect("write criminal actor");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<ThreatActor>()
            .await
            .expect("read criminal actor");
        assert_eq!(result, Some(actor));
    });
}

// ── Test 7: MalwareFamily with C2 domains ────────────────────────────────────

#[test]
fn test_async_malware_family_with_c2_domains() {
    rt().block_on(async {
        let family = MalwareFamily {
            family_id: 7,
            name: String::from("Emotet"),
            category: MalwareCategory::Botnet,
            first_observed_ts: 1_410_000_000_000,
            yara_rule_count: 38,
            c2_domains: vec![
                String::from("evil1.example.com"),
                String::from("evil2.example.net"),
                String::from("malicious.biz"),
            ],
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&family)
            .await
            .expect("write malware family");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<MalwareFamily>()
            .await
            .expect("read malware family");
        assert_eq!(result, Some(family));
    });
}

// ── Test 8: NetworkIntrusionEvent – TCP scan with Critical severity ────────────

#[test]
fn test_async_network_intrusion_event_tcp_critical() {
    rt().block_on(async {
        let event = NetworkIntrusionEvent {
            event_id: 9_000_000_001,
            src_ip: [185, 220, 101, 47],
            dst_ip: [10, 0, 0, 1],
            src_port: 54321,
            dst_port: 22,
            protocol: 6,
            bytes_transferred: 2_048,
            threat_level: ThreatLevel::Critical,
            timestamp: 1_700_100_000_000,
            signature_id: 2_027_250,
            signature_name: String::from("ET SCAN SSH Brute Force Attempt"),
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&event)
            .await
            .expect("write intrusion event");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<NetworkIntrusionEvent>()
            .await
            .expect("read intrusion event");
        assert_eq!(result, Some(event));
    });
}

// ── Test 9: HoneypotTrigger with payload bytes and IOC refs ──────────────────

#[test]
fn test_async_honeypot_trigger_with_payload() {
    rt().block_on(async {
        let trigger = HoneypotTrigger {
            trigger_id: 555_000,
            honeypot_id: 3,
            attacker_ip: [91, 108, 4, 200],
            attacker_port: 12345,
            payload_bytes: vec![0x90, 0x90, 0x90, 0xcc, 0x48, 0x31, 0xc0],
            timestamp: 1_700_200_000_000,
            ioc_refs: vec![1001, 1002, 1007],
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&trigger)
            .await
            .expect("write honeypot trigger");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<HoneypotTrigger>()
            .await
            .expect("read honeypot trigger");
        assert_eq!(result, Some(trigger));
    });
}

// ── Test 10: IncidentTimelineEntry – Open status with actor ──────────────────

#[test]
fn test_async_incident_timeline_entry_open() {
    rt().block_on(async {
        let entry = IncidentTimelineEntry {
            entry_id: 1,
            incident_id: 2024_0001,
            status: IncidentStatus::Open,
            actor_alias: Some(String::from("COZY BEAR")),
            description: String::from("Initial access via spear-phishing email detected."),
            timestamp: 1_700_300_000_000,
            affected_asset_ids: vec![100, 101, 102],
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&entry)
            .await
            .expect("write incident entry open");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<IncidentTimelineEntry>()
            .await
            .expect("read incident entry open");
        assert_eq!(result, Some(entry));
    });
}

// ── Test 11: IncidentTimelineEntry – Closed status without actor ──────────────

#[test]
fn test_async_incident_timeline_entry_closed_no_actor() {
    rt().block_on(async {
        let entry = IncidentTimelineEntry {
            entry_id: 99,
            incident_id: 2024_0001,
            status: IncidentStatus::Closed,
            actor_alias: None,
            description: String::from("Incident formally closed after 30-day monitoring period."),
            timestamp: 1_703_000_000_000,
            affected_asset_ids: vec![],
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&entry)
            .await
            .expect("write closed incident entry");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<IncidentTimelineEntry>()
            .await
            .expect("read closed incident entry");
        assert_eq!(result, Some(entry));
    });
}

// ── Test 12: Batch write of 12 IocRecords across all IocTypes ────────────────

#[test]
fn test_async_batch_write_12_ioc_records() {
    rt().block_on(async {
        let ioc_types = [
            IocType::IpAddress,
            IocType::Domain,
            IocType::Url,
            IocType::FileHash,
            IocType::EmailAddress,
            IocType::RegistryKey,
            IocType::MutexName,
            IocType::IpAddress,
            IocType::Domain,
            IocType::FileHash,
            IocType::Url,
            IocType::EmailAddress,
        ];
        let iocs: Vec<IocRecord> = ioc_types
            .iter()
            .enumerate()
            .map(|(i, t)| IocRecord {
                ioc_id: 2000 + i as u64,
                ioc_type: t.clone(),
                value: format!("indicator-{i}"),
                confidence: (50 + i * 4) as u8,
                first_seen_ts: 1_700_000_000_000 + i as u64 * 1_000,
                last_seen_ts: 1_700_000_000_000 + i as u64 * 2_000,
                tags: vec![format!("tag-{i}")],
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for ioc in &iocs {
            encoder.write_item(ioc).await.expect("write ioc in batch");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &iocs {
            let result = decoder
                .read_item::<IocRecord>()
                .await
                .expect("read ioc in batch");
            assert_eq!(result.as_ref(), Some(expected));
        }

        let tail = decoder
            .read_item::<IocRecord>()
            .await
            .expect("read after ioc batch");
        assert_eq!(tail, None);
    });
}

// ── Test 13: Empty stream returns None for threat intelligence types ───────────

#[test]
fn test_async_empty_stream_threat_intel_returns_none() {
    rt().block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let encoder = AsyncEncoder::new(writer);
        encoder.finish().await.expect("finish empty encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<IocRecord>()
            .await
            .expect("read from empty stream");
        assert_eq!(result, None);
    });
}

// ── Test 14: AptCampaign with active campaign (no end_ts) ────────────────────

#[test]
fn test_async_apt_campaign_active_no_end() {
    rt().block_on(async {
        let campaign = AptCampaign {
            campaign_id: 8,
            name: String::from("Operation Sandstorm"),
            actor_ids: vec![42, 43],
            malware_family_ids: vec![7, 12, 18],
            target_sectors: vec![
                String::from("Energy"),
                String::from("Finance"),
                String::from("Government"),
            ],
            start_ts: 1_650_000_000_000,
            end_ts: None,
            total_iocs: 1_432,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&campaign)
            .await
            .expect("write apt campaign");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<AptCampaign>()
            .await
            .expect("read apt campaign");
        assert_eq!(result, Some(campaign));
    });
}

// ── Test 15: AptCampaign with concluded campaign (with end_ts) ───────────────

#[test]
fn test_async_apt_campaign_concluded_with_end_ts() {
    rt().block_on(async {
        let campaign = AptCampaign {
            campaign_id: 3,
            name: String::from("Operation Ghost"),
            actor_ids: vec![99],
            malware_family_ids: vec![5, 9],
            target_sectors: vec![String::from("Defense"), String::from("Aerospace")],
            start_ts: 1_500_000_000_000,
            end_ts: Some(1_600_000_000_000),
            total_iocs: 287,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&campaign)
            .await
            .expect("write concluded campaign");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<AptCampaign>()
            .await
            .expect("read concluded campaign");
        assert_eq!(result, Some(campaign));
    });
}

// ── Test 16: ThreatHuntingQuery with MITRE ATT&CK techniques ─────────────────

#[test]
fn test_async_threat_hunting_query_with_mitre_techniques() {
    rt().block_on(async {
        let query = ThreatHuntingQuery {
            query_id: 501,
            title: String::from("Detect Lateral Movement via SMB"),
            query_text: String::from(
                "SELECT * FROM network_events WHERE dst_port=445 AND bytes > 10000",
            ),
            mitre_technique_ids: vec![
                String::from("T1021.002"),
                String::from("T1570"),
                String::from("T1078.002"),
            ],
            severity: ThreatLevel::High,
            created_ts: 1_699_000_000_000,
            last_run_ts: Some(1_700_999_000_000),
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&query)
            .await
            .expect("write threat hunting query");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<ThreatHuntingQuery>()
            .await
            .expect("read threat hunting query");
        assert_eq!(result, Some(query));
    });
}

// ── Test 17: All IncidentStatus variants round-trip ──────────────────────────

#[test]
fn test_all_incident_status_variants() {
    rt().block_on(async {
        let variants = vec![
            IncidentStatus::Open,
            IncidentStatus::Investigating,
            IncidentStatus::Contained,
            IncidentStatus::Eradicated,
            IncidentStatus::Recovered,
            IncidentStatus::Closed,
        ];
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for v in &variants {
            encoder
                .write_item(v)
                .await
                .expect("write IncidentStatus variant");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &variants {
            let result = decoder
                .read_item::<IncidentStatus>()
                .await
                .expect("read IncidentStatus variant");
            assert_eq!(result.as_ref(), Some(expected));
        }
    });
}

// ── Test 18: Sync encode_to_vec / decode_from_slice for CvssScore ────────────

#[test]
fn test_sync_encode_decode_cvss_score() {
    let score = CvssScore {
        cve_id: String::from("CVE-2023-44487"),
        base_score_x10: 75,
        attack_vector: AttackVector::Network,
        confidentiality_impact: 0,
        integrity_impact: 0,
        availability_impact: 3,
        exploitability_score_x10: 39,
    };
    let bytes = encode_to_vec(&score).expect("sync encode CvssScore");
    let (decoded, consumed): (CvssScore, usize) =
        decode_from_slice(&bytes).expect("sync decode CvssScore");
    assert_eq!(decoded, score);
    assert_eq!(consumed, bytes.len());
}

// ── Test 19: Sync vs async consistency for ThreatActor ───────────────────────

#[test]
fn test_sync_async_consistency_threat_actor() {
    let actor = ThreatActor {
        actor_id: 77,
        alias: String::from("SANDWORM"),
        nation_state: Some(String::from("RU")),
        active_since_year: 2014,
        threat_level: ThreatLevel::Critical,
        known_ttps: vec![
            String::from("T1498"),
            String::from("T1499"),
            String::from("T1485"),
        ],
    };

    // Sync path
    let bytes = encode_to_vec(&actor).expect("sync encode ThreatActor");
    let (sync_decoded, _): (ThreatActor, usize) =
        decode_from_slice(&bytes).expect("sync decode ThreatActor");
    assert_eq!(sync_decoded, actor);

    // Async path
    rt().block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&actor)
            .await
            .expect("async write ThreatActor");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let async_decoded = decoder
            .read_item::<ThreatActor>()
            .await
            .expect("async read ThreatActor");
        assert_eq!(async_decoded, Some(actor.clone()));

        // Both paths must yield identical results
        assert_eq!(sync_decoded, async_decoded.expect("async decoded is Some"));
    });
}

// ── Test 20: Batch write of 10 NetworkIntrusionEvents ─────────────────────────

#[test]
fn test_async_batch_write_10_intrusion_events() {
    rt().block_on(async {
        let events: Vec<NetworkIntrusionEvent> = (0u64..10)
            .map(|i| NetworkIntrusionEvent {
                event_id: 3_000_000 + i,
                src_ip: [
                    (i % 256) as u8,
                    ((i + 1) % 256) as u8,
                    ((i + 2) % 256) as u8,
                    ((i + 3) % 256) as u8,
                ],
                dst_ip: [10, 0, 0, (i % 10) as u8],
                src_port: (40000 + i) as u16,
                dst_port: if i % 2 == 0 { 443 } else { 80 },
                protocol: 6,
                bytes_transferred: 512 * (i + 1),
                threat_level: match i % 5 {
                    0 => ThreatLevel::Informational,
                    1 => ThreatLevel::Low,
                    2 => ThreatLevel::Medium,
                    3 => ThreatLevel::High,
                    _ => ThreatLevel::Critical,
                },
                timestamp: 1_700_000_000_000 + i * 1_000,
                signature_id: (2_000_000 + i) as u32,
                signature_name: format!("ET POLICY Suspicious Traffic {i}"),
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for ev in &events {
            encoder
                .write_item(ev)
                .await
                .expect("write intrusion event in batch");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &events {
            let result = decoder
                .read_item::<NetworkIntrusionEvent>()
                .await
                .expect("read intrusion event in batch");
            assert_eq!(result.as_ref(), Some(expected));
        }

        let tail = decoder
            .read_item::<NetworkIntrusionEvent>()
            .await
            .expect("read tail after batch");
        assert_eq!(tail, None);
    });
}

// ── Test 21: Multiple distinct threat intel types written to same stream ───────

#[test]
fn test_async_multiple_threat_intel_types_separate_streams() {
    rt().block_on(async {
        let actor = ThreatActor {
            actor_id: 200,
            alias: String::from("LAZARUS GROUP"),
            nation_state: Some(String::from("KP")),
            active_since_year: 2009,
            threat_level: ThreatLevel::Critical,
            known_ttps: vec![String::from("T1071"), String::from("T1027")],
        };
        let family = MalwareFamily {
            family_id: 50,
            name: String::from("WannaCry"),
            category: MalwareCategory::Ransomware,
            first_observed_ts: 1_494_720_000_000,
            yara_rule_count: 12,
            c2_domains: vec![String::from(
                "iuqerfsodp9ifjaposdfjhgosurijfaewrwergwea.com",
            )],
        };
        let ioc = IocRecord {
            ioc_id: 9999,
            ioc_type: IocType::FileHash,
            value: String::from("db349b97c37d22f5ea1d1841e3c89eb4"),
            confidence: 100,
            first_seen_ts: 1_494_720_000_000,
            last_seen_ts: 1_700_000_000_000,
            tags: vec![String::from("wannacry"), String::from("ransomware")],
        };

        let (wa, ra) = tokio::io::duplex(65536);
        let (wb, rb) = tokio::io::duplex(65536);
        let (wc, rc) = tokio::io::duplex(65536);

        let mut enc_a = AsyncEncoder::new(wa);
        enc_a.write_item(&actor).await.expect("write actor");
        enc_a.finish().await.expect("finish enc_a");

        let mut enc_b = AsyncEncoder::new(wb);
        enc_b.write_item(&family).await.expect("write family");
        enc_b.finish().await.expect("finish enc_b");

        let mut enc_c = AsyncEncoder::new(wc);
        enc_c.write_item(&ioc).await.expect("write ioc");
        enc_c.finish().await.expect("finish enc_c");

        let r_actor = AsyncDecoder::new(ra)
            .read_item::<ThreatActor>()
            .await
            .expect("read actor");
        let r_family = AsyncDecoder::new(rb)
            .read_item::<MalwareFamily>()
            .await
            .expect("read family");
        let r_ioc = AsyncDecoder::new(rc)
            .read_item::<IocRecord>()
            .await
            .expect("read ioc");

        assert_eq!(r_actor, Some(actor));
        assert_eq!(r_family, Some(family));
        assert_eq!(r_ioc, Some(ioc));
    });
}

// ── Test 22: Sync encode_to_vec / decode_from_slice for AptCampaign ──────────

#[test]
fn test_sync_encode_decode_apt_campaign() {
    let campaign = AptCampaign {
        campaign_id: 17,
        name: String::from("Operation Twisted Panda"),
        actor_ids: vec![1, 5, 12],
        malware_family_ids: vec![3, 7],
        target_sectors: vec![String::from("Healthcare"), String::from("Pharmaceuticals")],
        start_ts: 1_600_000_000_000,
        end_ts: Some(1_680_000_000_000),
        total_iocs: 543,
    };
    let bytes = encode_to_vec(&campaign).expect("sync encode AptCampaign");
    let (decoded, consumed): (AptCampaign, usize) =
        decode_from_slice(&bytes).expect("sync decode AptCampaign");
    assert_eq!(decoded, campaign);
    assert_eq!(consumed, bytes.len());
}
