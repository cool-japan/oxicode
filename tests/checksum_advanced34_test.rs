//! Advanced checksum tests for OxiCode — cybersecurity SOC / threat intelligence theme.
//! Exactly 22 #[test] functions.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced34_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// 1. SIEM Alert Record
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct SiemAlert {
    alert_id: u64,
    timestamp_epoch: u64,
    severity: u8,
    rule_name: String,
    source_ip: String,
    dest_ip: String,
    message: String,
}

#[test]
fn test_siem_alert_record() {
    let val = SiemAlert {
        alert_id: 1_000_042,
        timestamp_epoch: 1_710_500_000,
        severity: 3,
        rule_name: "ET TROJAN CobaltStrike Beacon".into(),
        source_ip: "10.0.14.55".into(),
        dest_ip: "198.51.100.7".into(),
        message: "Suspected C2 beacon activity on port 443".into(),
    };
    let bytes = encode_with_checksum(&val).expect("encode SiemAlert");
    let (decoded, consumed): (SiemAlert, _) =
        decode_with_checksum(&bytes).expect("decode SiemAlert");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 2. MITRE ATT&CK Technique Mapping
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct MitreTechnique {
    technique_id: String,
    tactic: String,
    name: String,
    is_subtechnique: bool,
    platforms: Vec<String>,
    detection_score: u8,
}

#[test]
fn test_mitre_attack_technique() {
    let val = MitreTechnique {
        technique_id: "T1059.001".into(),
        tactic: "Execution".into(),
        name: "PowerShell".into(),
        is_subtechnique: true,
        platforms: vec!["Windows".into(), "Linux".into()],
        detection_score: 78,
    };
    let bytes = encode_with_checksum(&val).expect("encode MitreTechnique");
    let (decoded, _): (MitreTechnique, _) =
        decode_with_checksum(&bytes).expect("decode MitreTechnique");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 3. IOC (Indicator of Compromise) Entry
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum IocType {
    Ipv4(String),
    Domain(String),
    Sha256(String),
    Url(String),
    Email(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IocEntry {
    ioc_type: IocType,
    confidence: u8,
    tlp_level: String,
    first_seen_epoch: u64,
    tags: Vec<String>,
}

#[test]
fn test_ioc_entry() {
    let val = IocEntry {
        ioc_type: IocType::Sha256(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".into(),
        ),
        confidence: 95,
        tlp_level: "TLP:AMBER".into(),
        first_seen_epoch: 1_710_300_000,
        tags: vec!["APT29".into(), "SolarWinds".into(), "supply-chain".into()],
    };
    let bytes = encode_with_checksum(&val).expect("encode IocEntry");
    let (decoded, _): (IocEntry, _) = decode_with_checksum(&bytes).expect("decode IocEntry");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 4. Firewall Rule Configuration
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum FirewallAction {
    Allow,
    Deny,
    Drop,
    Log,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FirewallRule {
    rule_id: u32,
    action: FirewallAction,
    src_cidr: String,
    dst_cidr: String,
    dst_port_start: u16,
    dst_port_end: u16,
    protocol: String,
    enabled: bool,
    comment: String,
}

#[test]
fn test_firewall_rule_config() {
    let val = FirewallRule {
        rule_id: 4001,
        action: FirewallAction::Drop,
        src_cidr: "0.0.0.0/0".into(),
        dst_cidr: "10.0.0.0/8".into(),
        dst_port_start: 22,
        dst_port_end: 22,
        protocol: "TCP".into(),
        enabled: true,
        comment: "Block external SSH to internal nets".into(),
    };
    let bytes = encode_with_checksum(&val).expect("encode FirewallRule");
    let (decoded, consumed): (FirewallRule, _) =
        decode_with_checksum(&bytes).expect("decode FirewallRule");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 5. Vulnerability Scan Result (CVSS)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct VulnScanResult {
    cve_id: String,
    cvss_base_x10: u16,
    cvss_vector: String,
    affected_host: String,
    affected_port: u16,
    remediation: String,
    exploitable: bool,
}

#[test]
fn test_vuln_scan_result() {
    let val = VulnScanResult {
        cve_id: "CVE-2024-3094".into(),
        cvss_base_x10: 100,
        cvss_vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H".into(),
        affected_host: "build-server-01.corp.local".into(),
        affected_port: 22,
        remediation: "Downgrade xz-utils to 5.4.x and rebuild sshd".into(),
        exploitable: true,
    };
    let bytes = encode_with_checksum(&val).expect("encode VulnScanResult");
    let (decoded, _): (VulnScanResult, _) =
        decode_with_checksum(&bytes).expect("decode VulnScanResult");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 6. Incident Response Playbook Step
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum PlaybookPhase {
    Preparation,
    Identification,
    Containment,
    Eradication,
    Recovery,
    LessonsLearned,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlaybookStep {
    step_number: u16,
    phase: PlaybookPhase,
    title: String,
    description: String,
    assignee_role: String,
    sla_minutes: u32,
    automated: bool,
}

#[test]
fn test_incident_response_playbook_step() {
    let val = PlaybookStep {
        step_number: 3,
        phase: PlaybookPhase::Containment,
        title: "Isolate affected endpoint".into(),
        description:
            "Use EDR to network-isolate the compromised host while preserving volatile memory"
                .into(),
        assignee_role: "SOC Tier 2 Analyst".into(),
        sla_minutes: 15,
        automated: false,
    };
    let bytes = encode_with_checksum(&val).expect("encode PlaybookStep");
    let (decoded, _): (PlaybookStep, _) =
        decode_with_checksum(&bytes).expect("decode PlaybookStep");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 7. Threat Actor Profile
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ThreatActorProfile {
    alias: String,
    aka: Vec<String>,
    origin_country: String,
    motivation: String,
    first_observed_year: u16,
    active: bool,
    targeted_sectors: Vec<String>,
    confidence_level: u8,
}

#[test]
fn test_threat_actor_profile() {
    let val = ThreatActorProfile {
        alias: "Fancy Bear".into(),
        aka: vec!["APT28".into(), "Sofacy".into(), "Sednit".into()],
        origin_country: "RU".into(),
        motivation: "Espionage".into(),
        first_observed_year: 2004,
        active: true,
        targeted_sectors: vec![
            "Government".into(),
            "Defense".into(),
            "Energy".into(),
            "Media".into(),
        ],
        confidence_level: 90,
    };
    let bytes = encode_with_checksum(&val).expect("encode ThreatActorProfile");
    let (decoded, _): (ThreatActorProfile, _) =
        decode_with_checksum(&bytes).expect("decode ThreatActorProfile");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 8. Malware Sample Metadata
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct MalwareSample {
    sha256: String,
    md5: String,
    family: String,
    file_size_bytes: u64,
    file_type: String,
    submission_epoch: u64,
    detection_names: Vec<String>,
    sandbox_score: u8,
}

#[test]
fn test_malware_sample_metadata() {
    let val = MalwareSample {
        sha256: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".into(),
        md5: "d41d8cd98f00b204e9800998ecf8427e".into(),
        family: "Emotet".into(),
        file_size_bytes: 245_760,
        file_type: "PE32 executable (DLL)".into(),
        submission_epoch: 1_710_400_000,
        detection_names: vec!["Trojan.Emotet".into(), "Win32/Emotet.AW".into()],
        sandbox_score: 88,
    };
    let bytes = encode_with_checksum(&val).expect("encode MalwareSample");
    let (decoded, _): (MalwareSample, _) =
        decode_with_checksum(&bytes).expect("decode MalwareSample");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 9. Network Flow Record (5-tuple + metadata)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct NetFlowRecord {
    src_ip: String,
    dst_ip: String,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    packets: u64,
    bytes_total: u64,
    start_epoch: u64,
    duration_ms: u32,
    tcp_flags: u8,
}

#[test]
fn test_netflow_record() {
    let val = NetFlowRecord {
        src_ip: "192.168.1.100".into(),
        dst_ip: "203.0.113.50".into(),
        src_port: 49152,
        dst_port: 443,
        protocol: 6,
        packets: 1_247,
        bytes_total: 1_843_200,
        start_epoch: 1_710_450_000,
        duration_ms: 32_500,
        tcp_flags: 0x1B,
    };
    let bytes = encode_with_checksum(&val).expect("encode NetFlowRecord");
    let (decoded, consumed): (NetFlowRecord, _) =
        decode_with_checksum(&bytes).expect("decode NetFlowRecord");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 10. SOAR Automation Playbook State
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum SoarState {
    Pending,
    Running,
    AwaitingApproval(String),
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SoarPlaybookRun {
    run_id: u64,
    playbook_name: String,
    state: SoarState,
    trigger_alert_id: u64,
    actions_executed: u16,
    actions_total: u16,
}

#[test]
fn test_soar_playbook_state() {
    let val = SoarPlaybookRun {
        run_id: 99_001,
        playbook_name: "Phishing-Triage-v3".into(),
        state: SoarState::AwaitingApproval("manager@corp.local".into()),
        trigger_alert_id: 1_000_042,
        actions_executed: 4,
        actions_total: 7,
    };
    let bytes = encode_with_checksum(&val).expect("encode SoarPlaybookRun");
    let (decoded, _): (SoarPlaybookRun, _) =
        decode_with_checksum(&bytes).expect("decode SoarPlaybookRun");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 11. EDR Telemetry Event
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum EdrEventKind {
    ProcessCreate,
    FileWrite,
    RegistryModify,
    NetworkConnect,
    DnsQuery,
    ImageLoad,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EdrTelemetryEvent {
    event_id: u64,
    hostname: String,
    kind: EdrEventKind,
    process_name: String,
    process_pid: u32,
    parent_pid: u32,
    command_line: String,
    timestamp_epoch: u64,
    suspicious: bool,
}

#[test]
fn test_edr_telemetry_event() {
    let val = EdrTelemetryEvent {
        event_id: 8_000_123,
        hostname: "WS-FINANCE-07".into(),
        kind: EdrEventKind::ProcessCreate,
        process_name: "powershell.exe".into(),
        process_pid: 5544,
        parent_pid: 3312,
        command_line: "powershell -enc SQBFAFgAIAAoA...".into(),
        timestamp_epoch: 1_710_480_000,
        suspicious: true,
    };
    let bytes = encode_with_checksum(&val).expect("encode EdrTelemetryEvent");
    let (decoded, _): (EdrTelemetryEvent, _) =
        decode_with_checksum(&bytes).expect("decode EdrTelemetryEvent");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 12. Certificate Transparency Log Entry
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct CtLogEntry {
    log_id: u64,
    domain: String,
    issuer: String,
    not_before_epoch: u64,
    not_after_epoch: u64,
    serial_hex: String,
    key_algorithm: String,
    key_bits: u16,
    san_count: u16,
}

#[test]
fn test_certificate_transparency_log() {
    let val = CtLogEntry {
        log_id: 55_000_001,
        domain: "*.corp-internal.example.com".into(),
        issuer: "CN=DigiCert Global G2 TLS RSA SHA256 2020 CA1".into(),
        not_before_epoch: 1_704_067_200,
        not_after_epoch: 1_735_689_600,
        serial_hex: "0A1B2C3D4E5F6A7B8C9D0E1F".into(),
        key_algorithm: "RSA".into(),
        key_bits: 2048,
        san_count: 5,
    };
    let bytes = encode_with_checksum(&val).expect("encode CtLogEntry");
    let (decoded, _): (CtLogEntry, _) = decode_with_checksum(&bytes).expect("decode CtLogEntry");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 13. DNS Query Anomaly Score
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct DnsAnomalyRecord {
    query_domain: String,
    query_type: String,
    client_ip: String,
    anomaly_score_x100: u16,
    entropy_x1000: u32,
    label_count: u8,
    max_label_len: u8,
    is_dga_suspect: bool,
    resolved_ips: Vec<String>,
}

#[test]
fn test_dns_anomaly_score() {
    let val = DnsAnomalyRecord {
        query_domain: "aXk2bG9jYWxob3N0.evil-c2.example.net".into(),
        query_type: "TXT".into(),
        client_ip: "10.0.3.88".into(),
        anomaly_score_x100: 9450,
        entropy_x1000: 4_120,
        label_count: 3,
        max_label_len: 18,
        is_dga_suspect: true,
        resolved_ips: vec!["198.51.100.33".into()],
    };
    let bytes = encode_with_checksum(&val).expect("encode DnsAnomalyRecord");
    let (decoded, _): (DnsAnomalyRecord, _) =
        decode_with_checksum(&bytes).expect("decode DnsAnomalyRecord");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 14. Zero-Trust Policy Decision
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum ZtDecision {
    Allow,
    Deny,
    StepUp,
    Quarantine,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ZeroTrustPolicyDecision {
    request_id: u64,
    user_principal: String,
    device_id: String,
    resource: String,
    decision: ZtDecision,
    risk_score: u8,
    device_compliant: bool,
    mfa_verified: bool,
    geo_location: String,
}

#[test]
fn test_zero_trust_policy_decision() {
    let val = ZeroTrustPolicyDecision {
        request_id: 77_000_001,
        user_principal: "jdoe@corp.example.com".into(),
        device_id: "DEV-A1B2C3D4".into(),
        resource: "/api/v2/financial-reports".into(),
        decision: ZtDecision::StepUp,
        risk_score: 65,
        device_compliant: true,
        mfa_verified: false,
        geo_location: "US-VA".into(),
    };
    let bytes = encode_with_checksum(&val).expect("encode ZeroTrustPolicyDecision");
    let (decoded, _): (ZeroTrustPolicyDecision, _) =
        decode_with_checksum(&bytes).expect("decode ZeroTrustPolicyDecision");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 15. YARA Rule Match
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct YaraRuleMatch {
    rule_name: String,
    rule_namespace: String,
    matched_file: String,
    matched_strings: Vec<String>,
    file_offset: u64,
    meta_author: String,
    meta_severity: String,
}

#[test]
fn test_yara_rule_match() {
    let val = YaraRuleMatch {
        rule_name: "CobaltStrike_Beacon_Config".into(),
        rule_namespace: "malware".into(),
        matched_file: "/tmp/uploads/suspect_payload.bin".into(),
        matched_strings: vec![
            "$beacon_config".into(),
            "$watermark".into(),
            "$sleep_mask".into(),
        ],
        file_offset: 0x1A00,
        meta_author: "Florian Roth".into(),
        meta_severity: "critical".into(),
    };
    let bytes = encode_with_checksum(&val).expect("encode YaraRuleMatch");
    let (decoded, _): (YaraRuleMatch, _) =
        decode_with_checksum(&bytes).expect("decode YaraRuleMatch");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 16. Sigma Rule Detection
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum SigmaLevel {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SigmaDetection {
    rule_id: String,
    title: String,
    level: SigmaLevel,
    logsource_category: String,
    logsource_product: String,
    matched_fields: Vec<String>,
    false_positive_rate_pct: u8,
}

#[test]
fn test_sigma_rule_detection() {
    let val = SigmaDetection {
        rule_id: "d4c3b2a1-1234-5678-9abc-def012345678".into(),
        title: "Suspicious LSASS Memory Dump via comsvcs.dll".into(),
        level: SigmaLevel::Critical,
        logsource_category: "process_creation".into(),
        logsource_product: "windows".into(),
        matched_fields: vec!["CommandLine".into(), "ParentImage".into()],
        false_positive_rate_pct: 2,
    };
    let bytes = encode_with_checksum(&val).expect("encode SigmaDetection");
    let (decoded, _): (SigmaDetection, _) =
        decode_with_checksum(&bytes).expect("decode SigmaDetection");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 17. Phishing Email Analysis
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct PhishingAnalysis {
    message_id: String,
    sender: String,
    return_path: String,
    subject: String,
    spf_pass: bool,
    dkim_pass: bool,
    dmarc_pass: bool,
    url_count: u16,
    attachment_count: u8,
    spam_score_x10: u16,
    verdict: String,
}

#[test]
fn test_phishing_email_analysis() {
    let val = PhishingAnalysis {
        message_id: "<abc123@mail.evil-phish.example>".into(),
        sender: "support@paypa1-secure.example.net".into(),
        return_path: "bounce@different-domain.example.org".into(),
        subject: "Urgent: Verify your account now!".into(),
        spf_pass: false,
        dkim_pass: false,
        dmarc_pass: false,
        url_count: 3,
        attachment_count: 1,
        spam_score_x10: 92,
        verdict: "MALICIOUS".into(),
    };
    let bytes = encode_with_checksum(&val).expect("encode PhishingAnalysis");
    let (decoded, _): (PhishingAnalysis, _) =
        decode_with_checksum(&bytes).expect("decode PhishingAnalysis");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 18. Privilege Escalation Attempt
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum EscalationMethod {
    SuidBinary(String),
    KernelExploit(String),
    TokenManipulation,
    ScheduledTask,
    ServiceCreation(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PrivEscAttempt {
    event_id: u64,
    hostname: String,
    username: String,
    method: EscalationMethod,
    success: bool,
    target_privilege: String,
    timestamp_epoch: u64,
}

#[test]
fn test_privilege_escalation_attempt() {
    let val = PrivEscAttempt {
        event_id: 12_345_678,
        hostname: "db-prod-03".into(),
        username: "www-data".into(),
        method: EscalationMethod::SuidBinary("/usr/bin/pkexec".into()),
        success: false,
        target_privilege: "root".into(),
        timestamp_epoch: 1_710_490_000,
    };
    let bytes = encode_with_checksum(&val).expect("encode PrivEscAttempt");
    let (decoded, _): (PrivEscAttempt, _) =
        decode_with_checksum(&bytes).expect("decode PrivEscAttempt");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 19. Data Loss Prevention (DLP) Event
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct DlpEvent {
    event_id: u64,
    policy_name: String,
    user: String,
    channel: String,
    matched_pattern: String,
    match_count: u32,
    action_taken: String,
    file_name: String,
    file_size_bytes: u64,
    blocked: bool,
}

#[test]
fn test_dlp_event() {
    let val = DlpEvent {
        event_id: 300_001,
        policy_name: "PII-SSN-Detection".into(),
        user: "analyst3@corp.example.com".into(),
        channel: "email-outbound".into(),
        matched_pattern: r"\b\d{3}-\d{2}-\d{4}\b".into(),
        match_count: 47,
        action_taken: "block-and-notify".into(),
        file_name: "q4_payroll_export.xlsx".into(),
        file_size_bytes: 2_048_576,
        blocked: true,
    };
    let bytes = encode_with_checksum(&val).expect("encode DlpEvent");
    let (decoded, _): (DlpEvent, _) = decode_with_checksum(&bytes).expect("decode DlpEvent");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 20. Honeypot Interaction Log
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct HoneypotInteraction {
    honeypot_id: String,
    attacker_ip: String,
    service_emulated: String,
    interaction_type: String,
    credentials_tried: Vec<String>,
    payload_captured: bool,
    session_duration_sec: u32,
    timestamp_epoch: u64,
}

#[test]
fn test_honeypot_interaction_log() {
    let val = HoneypotInteraction {
        honeypot_id: "HP-SSH-DMZ-01".into(),
        attacker_ip: "45.33.32.156".into(),
        service_emulated: "SSH".into(),
        interaction_type: "brute-force".into(),
        credentials_tried: vec![
            "root:admin".into(),
            "root:password".into(),
            "root:toor".into(),
            "admin:123456".into(),
        ],
        payload_captured: false,
        session_duration_sec: 87,
        timestamp_epoch: 1_710_460_000,
    };
    let bytes = encode_with_checksum(&val).expect("encode HoneypotInteraction");
    let (decoded, _): (HoneypotInteraction, _) =
        decode_with_checksum(&bytes).expect("decode HoneypotInteraction");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 21. Threat Intel Feed Subscription Status
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum FeedFormat {
    Stix21,
    Csv,
    MispJson,
    OpenIoc,
    Custom(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ThreatIntelFeed {
    feed_name: String,
    provider: String,
    format: FeedFormat,
    last_poll_epoch: u64,
    ioc_count: u64,
    enabled: bool,
    poll_interval_sec: u32,
    error_count_last_24h: u16,
}

#[test]
fn test_threat_intel_feed_subscription() {
    let val = ThreatIntelFeed {
        feed_name: "AlienVault-OTX-Pulses".into(),
        provider: "AT&T Cybersecurity".into(),
        format: FeedFormat::Stix21,
        last_poll_epoch: 1_710_499_000,
        ioc_count: 1_250_000,
        enabled: true,
        poll_interval_sec: 3600,
        error_count_last_24h: 0,
    };
    let bytes = encode_with_checksum(&val).expect("encode ThreatIntelFeed");
    let (decoded, _): (ThreatIntelFeed, _) =
        decode_with_checksum(&bytes).expect("decode ThreatIntelFeed");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 22. Lateral Movement Detection
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum LateralTechnique {
    PsExec,
    Wmi,
    RdpHijack,
    SshTunnel,
    PassTheHash,
    PassTheTicket,
    Dcom,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LateralMovementDetection {
    detection_id: u64,
    source_host: String,
    dest_host: String,
    technique: LateralTechnique,
    source_user: String,
    dest_user: String,
    mitre_id: String,
    confidence_pct: u8,
    timestamp_epoch: u64,
    blocked: bool,
}

#[test]
fn test_lateral_movement_detection() {
    let val = LateralMovementDetection {
        detection_id: 450_000_789,
        source_host: "WS-HR-12".into(),
        dest_host: "DC-CORP-01".into(),
        technique: LateralTechnique::PassTheHash,
        source_user: "jsmith".into(),
        dest_user: "Administrator".into(),
        mitre_id: "T1550.002".into(),
        confidence_pct: 87,
        timestamp_epoch: 1_710_495_000,
        blocked: true,
    };
    let bytes = encode_with_checksum(&val).expect("encode LateralMovementDetection");
    let (decoded, consumed): (LateralMovementDetection, _) =
        decode_with_checksum(&bytes).expect("decode LateralMovementDetection");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}
