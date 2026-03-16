//! Advanced file I/O tests for OxiCode — domain: cybersecurity SIEM and threat intelligence

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThreatLevel {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IndicatorType {
    Ipv4Addr,
    Ipv6Addr,
    DomainName,
    Url,
    EmailAddress,
    FileHashMd5,
    FileHashSha1,
    FileHashSha256,
    Mutex,
    RegistryKey,
    UserAgent,
    JarmFingerprint,
    Ja3Hash,
    SslCertSha1,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StixIndicator {
    stix_id: String,
    indicator_type: IndicatorType,
    pattern: String,
    name: String,
    description: String,
    threat_level: ThreatLevel,
    confidence: u8,
    valid_from_epoch: u64,
    valid_until_epoch: Option<u64>,
    kill_chain_phases: Vec<String>,
    labels: Vec<String>,
    created_by_ref: String,
    external_references: Vec<ExternalReference>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExternalReference {
    source_name: String,
    reference_url: String,
    external_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TaxiiCollection {
    collection_id: String,
    title: String,
    description: String,
    can_read: bool,
    can_write: bool,
    media_types: Vec<String>,
    indicators: Vec<StixIndicator>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AttackTactic {
    Reconnaissance,
    ResourceDevelopment,
    InitialAccess,
    Execution,
    Persistence,
    PrivilegeEscalation,
    DefenseEvasion,
    CredentialAccess,
    Discovery,
    LateralMovement,
    Collection,
    CommandAndControl,
    Exfiltration,
    Impact,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MitreAttackTechnique {
    technique_id: String,
    name: String,
    tactic: AttackTactic,
    description: String,
    platforms: Vec<String>,
    detection: String,
    data_sources: Vec<String>,
    is_subtechnique: bool,
    parent_technique_id: Option<String>,
    mitigations: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CvssVersion {
    V2,
    V3,
    V31,
    V4,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CveRecord {
    cve_id: String,
    description: String,
    cvss_score: u16,
    cvss_version: CvssVersion,
    cwe_ids: Vec<String>,
    affected_products: Vec<AffectedProduct>,
    published_epoch: u64,
    last_modified_epoch: u64,
    references: Vec<String>,
    exploitability: ExploitabilityInfo,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AffectedProduct {
    vendor: String,
    product: String,
    version_start: String,
    version_end: String,
    cpe: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExploitabilityInfo {
    exploit_available: bool,
    exploit_in_wild: bool,
    poc_url: Option<String>,
    ransomware_known: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FirewallAction {
    Allow,
    Deny,
    Log,
    RateLimit { packets_per_sec: u32 },
    Redirect { destination: String },
    Drop,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Icmpv6,
    Sctp,
    Gre,
    Any,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FirewallRule {
    rule_id: u32,
    name: String,
    source_cidr: String,
    destination_cidr: String,
    source_port_range: (u16, u16),
    destination_port_range: (u16, u16),
    protocol: Protocol,
    action: FirewallAction,
    enabled: bool,
    log_enabled: bool,
    hit_count: u64,
    comment: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlertSeverity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IdsAction {
    Alert,
    Block,
    Pass,
    Reject,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IdsIpsAlert {
    alert_id: u64,
    signature_id: u32,
    signature_name: String,
    severity: AlertSeverity,
    action_taken: IdsAction,
    source_ip: String,
    source_port: u16,
    destination_ip: String,
    destination_port: u16,
    protocol: Protocol,
    timestamp_epoch: u64,
    payload_excerpt: Vec<u8>,
    classification: String,
    sensor_name: String,
    packet_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CorrelationOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    Regex,
    InList,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CorrelationCondition {
    field_name: String,
    operator: CorrelationOperator,
    value: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SiemCorrelationRule {
    rule_id: String,
    name: String,
    description: String,
    severity: AlertSeverity,
    conditions: Vec<CorrelationCondition>,
    time_window_secs: u64,
    threshold_count: u32,
    group_by_fields: Vec<String>,
    log_sources: Vec<String>,
    mitre_attack_ids: Vec<String>,
    enabled: bool,
    false_positive_notes: Vec<String>,
    response_actions: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MalwareFamily {
    Ransomware {
        ransom_note_pattern: String,
    },
    Trojan {
        capabilities: Vec<String>,
    },
    Worm {
        propagation_method: String,
    },
    Rootkit {
        kernel_mode: bool,
    },
    Backdoor {
        c2_protocol: String,
    },
    Infostealer {
        targeted_data: Vec<String>,
    },
    Cryptominer {
        coin_type: String,
    },
    Botnet {
        botnet_name: String,
    },
    Apt {
        campaign_name: String,
        nation_state: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MalwareSample {
    sha256: String,
    sha1: String,
    md5: String,
    ssdeep: String,
    file_size_bytes: u64,
    file_type: String,
    family: MalwareFamily,
    first_seen_epoch: u64,
    last_seen_epoch: u64,
    ttp_ids: Vec<String>,
    yara_matches: Vec<String>,
    sandbox_verdict: String,
    c2_addresses: Vec<String>,
    dropped_files: Vec<String>,
    mutexes_created: Vec<String>,
    registry_modifications: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetFlowRecord {
    flow_id: u64,
    source_ip: String,
    destination_ip: String,
    source_port: u16,
    destination_port: u16,
    protocol: Protocol,
    packets_sent: u64,
    packets_received: u64,
    bytes_sent: u64,
    bytes_received: u64,
    start_epoch: u64,
    end_epoch: u64,
    tcp_flags: u8,
    tos: u8,
    input_interface: u32,
    output_interface: u32,
    as_src: u32,
    as_dst: u32,
    exporter_ip: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VulnScanResult {
    scan_id: String,
    host_ip: String,
    hostname: String,
    port: u16,
    protocol: Protocol,
    service_name: String,
    cve_id: String,
    cvss_base_score: u16,
    cvss_vector: String,
    severity: AlertSeverity,
    solution: String,
    plugin_id: u32,
    plugin_name: String,
    first_discovered_epoch: u64,
    last_observed_epoch: u64,
    exploit_available: bool,
    patch_available: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IncidentPhase {
    Detection,
    Triage,
    Containment,
    Eradication,
    Recovery,
    LessonsLearned,
    Closed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IncidentTimelineEntry {
    entry_id: u32,
    timestamp_epoch: u64,
    phase: IncidentPhase,
    analyst: String,
    action_description: String,
    artifacts: Vec<String>,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IncidentResponse {
    incident_id: String,
    title: String,
    severity: AlertSeverity,
    status: IncidentPhase,
    timeline: Vec<IncidentTimelineEntry>,
    affected_hosts: Vec<String>,
    ioc_ids: Vec<String>,
    assigned_team: String,
    escalation_level: u8,
    created_epoch: u64,
    resolved_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThreatActorMotivation {
    Financial,
    Espionage,
    Hacktivism,
    Destruction,
    Ideology,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThreatActorSophistication {
    None,
    Minimal,
    Intermediate,
    Advanced,
    Expert,
    Strategic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThreatActorProfile {
    actor_id: String,
    name: String,
    aliases: Vec<String>,
    motivation: ThreatActorMotivation,
    sophistication: ThreatActorSophistication,
    country_of_origin: Option<String>,
    target_sectors: Vec<String>,
    target_countries: Vec<String>,
    known_tools: Vec<String>,
    known_ttps: Vec<String>,
    associated_campaigns: Vec<String>,
    first_observed_epoch: u64,
    description: String,
    references: Vec<ExternalReference>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IndicatorOfCompromise {
    ioc_id: String,
    indicator_type: IndicatorType,
    value: String,
    threat_level: ThreatLevel,
    source: String,
    first_seen_epoch: u64,
    last_seen_epoch: u64,
    related_campaigns: Vec<String>,
    tags: Vec<String>,
    whitelisted: bool,
    expiration_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CertTransparencyLog {
    log_id: u64,
    certificate_sha256: String,
    issuer: String,
    subject: String,
    subject_alt_names: Vec<String>,
    not_before_epoch: u64,
    not_after_epoch: u64,
    serial_number: String,
    key_algorithm: String,
    key_size_bits: u16,
    log_server: String,
    entry_timestamp_epoch: u64,
    is_precertificate: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DnsQueryType {
    A,
    Aaaa,
    Cname,
    Mx,
    Ns,
    Txt,
    Srv,
    Soa,
    Ptr,
    Dnskey,
    Ds,
    Tlsa,
    Any,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DnsResponseCode {
    NoError,
    FormErr,
    ServFail,
    NxDomain,
    NotImp,
    Refused,
    YxDomain,
    Other(u16),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DnsQueryLog {
    query_id: u64,
    timestamp_epoch: u64,
    client_ip: String,
    server_ip: String,
    query_name: String,
    query_type: DnsQueryType,
    response_code: DnsResponseCode,
    response_addresses: Vec<String>,
    ttl: u32,
    query_time_us: u32,
    recursive: bool,
    dnssec_validated: bool,
    edns_client_subnet: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AuthMethod {
    Password,
    PublicKey,
    Kerberos,
    Oauth2,
    Saml,
    Radius,
    Ldap,
    Certificate,
    Biometric,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AuthOutcome {
    Success,
    FailureInvalidCredentials,
    FailureAccountLocked,
    FailureAccountExpired,
    FailureMfaRequired,
    FailureMfaTimeout,
    FailureMfaInvalid,
    SuccessWithMfa { mfa_method: String },
    FailureGeoBlocked { country: String },
    FailureRiskScore { score: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AuthenticationEvent {
    event_id: u64,
    timestamp_epoch: u64,
    username: String,
    source_ip: String,
    auth_method: AuthMethod,
    outcome: AuthOutcome,
    service: String,
    session_id: Option<String>,
    user_agent: Option<String>,
    geo_country: Option<String>,
    geo_city: Option<String>,
    risk_score: u16,
    previous_login_epoch: Option<u64>,
}

// ---------------------------------------------------------------------------
// Composite types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThreatIntelReport {
    report_id: String,
    title: String,
    summary: String,
    threat_actors: Vec<ThreatActorProfile>,
    indicators: Vec<IndicatorOfCompromise>,
    techniques: Vec<MitreAttackTechnique>,
    cves: Vec<CveRecord>,
    malware_samples: Vec<MalwareSample>,
    published_epoch: u64,
    tlp_marking: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SiemDashboardSnapshot {
    snapshot_epoch: u64,
    active_incidents: Vec<IncidentResponse>,
    top_alerts: Vec<IdsIpsAlert>,
    recent_auth_failures: Vec<AuthenticationEvent>,
    suspicious_dns: Vec<DnsQueryLog>,
    active_correlation_rules: u32,
    total_events_last_hour: u64,
    unique_source_ips_last_hour: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SecurityPostureReport {
    report_epoch: u64,
    vuln_scan_results: Vec<VulnScanResult>,
    cert_logs: Vec<CertTransparencyLog>,
    firewall_rules: Vec<FirewallRule>,
    network_flows: Vec<NetFlowRecord>,
    total_critical_vulns: u32,
    total_high_vulns: u32,
    mean_time_to_remediate_secs: u64,
    compliance_score_percent: u8,
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

fn make_stix_indicator(id: &str, itype: IndicatorType, confidence: u8) -> StixIndicator {
    StixIndicator {
        stix_id: format!("indicator--{}", id),
        indicator_type: itype,
        pattern: "[ipv4-addr:value = '203.0.113.42']".into(),
        name: format!("Indicator {}", id),
        description: "Known malicious infrastructure".into(),
        threat_level: ThreatLevel::High,
        confidence,
        valid_from_epoch: 1700000000,
        valid_until_epoch: Some(1710000000),
        kill_chain_phases: vec!["delivery".into(), "command-and-control".into()],
        labels: vec!["malicious-activity".into()],
        created_by_ref: "identity--threat-intel-team".into(),
        external_references: vec![ExternalReference {
            source_name: "MITRE ATT&CK".into(),
            reference_url: "https://attack.mitre.org/techniques/T1071/".into(),
            external_id: Some("T1071".into()),
        }],
    }
}

fn make_mitre_technique(tid: &str, tactic: AttackTactic) -> MitreAttackTechnique {
    MitreAttackTechnique {
        technique_id: tid.into(),
        name: format!("Technique {}", tid),
        tactic,
        description: "Adversary technique description".into(),
        platforms: vec!["Windows".into(), "Linux".into(), "macOS".into()],
        detection: "Monitor network traffic for anomalies".into(),
        data_sources: vec!["Network Traffic".into(), "Process Monitoring".into()],
        is_subtechnique: tid.contains('.'),
        parent_technique_id: if tid.contains('.') {
            Some(
                tid.split('.')
                    .next()
                    .expect("split must yield at least one part")
                    .into(),
            )
        } else {
            None
        },
        mitigations: vec!["Network Segmentation".into()],
    }
}

fn make_cve(id: &str, score: u16) -> CveRecord {
    CveRecord {
        cve_id: format!("CVE-{}", id),
        description: format!("Vulnerability in component {}", id),
        cvss_score: score,
        cvss_version: CvssVersion::V31,
        cwe_ids: vec!["CWE-79".into(), "CWE-89".into()],
        affected_products: vec![AffectedProduct {
            vendor: "ExampleVendor".into(),
            product: "ExampleProduct".into(),
            version_start: "1.0.0".into(),
            version_end: "1.5.3".into(),
            cpe: "cpe:2.3:a:example:product:1.0.0:*:*:*:*:*:*:*".into(),
        }],
        published_epoch: 1690000000,
        last_modified_epoch: 1695000000,
        references: vec!["https://nvd.nist.gov/vuln/detail/CVE-EXAMPLE".into()],
        exploitability: ExploitabilityInfo {
            exploit_available: score >= 90,
            exploit_in_wild: score >= 95,
            poc_url: if score >= 90 {
                Some("https://exploit-db.com/exploits/12345".into())
            } else {
                None
            },
            ransomware_known: false,
        },
    }
}

fn make_firewall_rule(id: u32, action: FirewallAction) -> FirewallRule {
    FirewallRule {
        rule_id: id,
        name: format!("Rule-{}", id),
        source_cidr: "10.0.0.0/8".into(),
        destination_cidr: "192.168.1.0/24".into(),
        source_port_range: (1024, 65535),
        destination_port_range: (443, 443),
        protocol: Protocol::Tcp,
        action,
        enabled: true,
        log_enabled: true,
        hit_count: 1_500_000 + u64::from(id) * 1000,
        comment: format!("Firewall rule #{}", id),
    }
}

fn make_ids_alert(id: u64, severity: AlertSeverity) -> IdsIpsAlert {
    IdsIpsAlert {
        alert_id: id,
        signature_id: 2000000 + id as u32,
        signature_name: format!("ET MALWARE Potential C2 Communication {}", id),
        severity,
        action_taken: IdsAction::Alert,
        source_ip: "10.0.1.55".into(),
        source_port: 49152,
        destination_ip: "198.51.100.77".into(),
        destination_port: 8443,
        protocol: Protocol::Tcp,
        timestamp_epoch: 1700000000 + id * 60,
        payload_excerpt: vec![0x47, 0x45, 0x54, 0x20, 0x2f, 0x63, 0x32],
        classification: "A Network Trojan was detected".into(),
        sensor_name: "ids-sensor-01".into(),
        packet_count: 42,
    }
}

fn make_malware_sample(hash_suffix: &str, family: MalwareFamily) -> MalwareSample {
    MalwareSample {
        sha256: format!(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8{}",
            hash_suffix
        ),
        sha1: format!("da39a3ee5e6b4b0d3255bfef95601890afd807{}", hash_suffix),
        md5: format!("d41d8cd98f00b204e9800998ecf842{}", hash_suffix),
        ssdeep: format!("3:{}:hash", hash_suffix),
        file_size_bytes: 2_345_678,
        file_type: "PE32 executable".into(),
        family,
        first_seen_epoch: 1698000000,
        last_seen_epoch: 1700000000,
        ttp_ids: vec!["T1059.001".into(), "T1547.001".into()],
        yara_matches: vec!["rule_APT_Backdoor_Generic".into()],
        sandbox_verdict: "malicious".into(),
        c2_addresses: vec!["198.51.100.10:443".into(), "203.0.113.55:8080".into()],
        dropped_files: vec!["payload.dll".into(), "config.dat".into()],
        mutexes_created: vec!["Global\\MalwareMutex123".into()],
        registry_modifications: vec![
            "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run\\Updater".into(),
        ],
    }
}

fn make_netflow(id: u64) -> NetFlowRecord {
    NetFlowRecord {
        flow_id: id,
        source_ip: "10.0.2.100".into(),
        destination_ip: "172.16.0.50".into(),
        source_port: 50000 + (id as u16 % 15000),
        destination_port: 443,
        protocol: Protocol::Tcp,
        packets_sent: 150 + id * 3,
        packets_received: 200 + id * 5,
        bytes_sent: 45_000 + id * 1000,
        bytes_received: 120_000 + id * 2500,
        start_epoch: 1700000000 + id * 30,
        end_epoch: 1700000000 + id * 30 + 120,
        tcp_flags: 0x1B,
        tos: 0,
        input_interface: 1,
        output_interface: 2,
        as_src: 64500,
        as_dst: 13335,
        exporter_ip: "10.255.0.1".into(),
    }
}

fn make_vuln_scan(host_id: u16, cvss: u16) -> VulnScanResult {
    VulnScanResult {
        scan_id: format!("scan-2024-{:04}", host_id),
        host_ip: format!("10.1.{}.{}", host_id / 256, host_id % 256),
        hostname: format!("host-{:04}.corp.example.com", host_id),
        port: 443,
        protocol: Protocol::Tcp,
        service_name: "https".into(),
        cve_id: format!("CVE-2024-{:05}", host_id),
        cvss_base_score: cvss,
        cvss_vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H".into(),
        severity: if cvss >= 90 {
            AlertSeverity::Critical
        } else if cvss >= 70 {
            AlertSeverity::High
        } else if cvss >= 40 {
            AlertSeverity::Medium
        } else {
            AlertSeverity::Low
        },
        solution: "Apply vendor patch".into(),
        plugin_id: 100_000 + u32::from(host_id),
        plugin_name: "SSL/TLS Vulnerability Check".into(),
        first_discovered_epoch: 1695000000,
        last_observed_epoch: 1700000000,
        exploit_available: cvss >= 90,
        patch_available: true,
    }
}

fn make_auth_event(id: u64, outcome: AuthOutcome) -> AuthenticationEvent {
    AuthenticationEvent {
        event_id: id,
        timestamp_epoch: 1700000000 + id * 5,
        username: format!("user_{}", id % 100),
        source_ip: format!("192.168.1.{}", id % 254 + 1),
        auth_method: AuthMethod::Password,
        outcome,
        service: "sshd".into(),
        session_id: Some(format!("sess-{:016x}", id * 0xDEAD)),
        user_agent: Some("OpenSSH_8.9p1".into()),
        geo_country: Some("US".into()),
        geo_city: Some("San Francisco".into()),
        risk_score: (id % 100) as u16,
        previous_login_epoch: Some(1699990000),
    }
}

fn make_dns_log(id: u64, qtype: DnsQueryType, rcode: DnsResponseCode) -> DnsQueryLog {
    DnsQueryLog {
        query_id: id,
        timestamp_epoch: 1700000000 + id * 2,
        client_ip: format!("10.0.3.{}", id % 254 + 1),
        server_ip: "10.0.0.53".into(),
        query_name: format!("sub{}.example.com", id),
        query_type: qtype,
        response_code: rcode,
        response_addresses: vec!["93.184.216.34".into()],
        ttl: 3600,
        query_time_us: 1200 + id as u32 * 10,
        recursive: true,
        dnssec_validated: false,
        edns_client_subnet: None,
    }
}

fn make_cert_log(id: u64) -> CertTransparencyLog {
    CertTransparencyLog {
        log_id: id,
        certificate_sha256: format!(
            "abcdef0123456789abcdef0123456789abcdef0123456789abcdef01234567{:02x}",
            id % 256
        ),
        issuer: "CN=Let's Encrypt Authority X3, O=Let's Encrypt, C=US".into(),
        subject: format!("CN=www{}.example.com", id),
        subject_alt_names: vec![
            format!("www{}.example.com", id),
            format!("api{}.example.com", id),
        ],
        not_before_epoch: 1690000000 + id * 86400,
        not_after_epoch: 1697760000 + id * 86400,
        serial_number: format!("03:AB:CD:{:02X}:{:02X}", id % 256, (id * 7) % 256),
        key_algorithm: "RSA".into(),
        key_size_bits: 2048,
        log_server: "ct.googleapis.com/logs/argon2024".into(),
        entry_timestamp_epoch: 1690000100 + id * 86400,
        is_precertificate: id % 3 == 0,
    }
}

fn make_ioc(id: u64, itype: IndicatorType, level: ThreatLevel) -> IndicatorOfCompromise {
    IndicatorOfCompromise {
        ioc_id: format!("ioc-{:08}", id),
        indicator_type: itype,
        value: format!("203.0.113.{}", id % 254 + 1),
        threat_level: level,
        source: "ThreatFeed-Alpha".into(),
        first_seen_epoch: 1698000000,
        last_seen_epoch: 1700000000,
        related_campaigns: vec!["Operation Nightfall".into()],
        tags: vec!["apt".into(), "c2-infrastructure".into()],
        whitelisted: false,
        expiration_epoch: Some(1710000000),
    }
}

fn make_incident(id: &str, severity: AlertSeverity) -> IncidentResponse {
    IncidentResponse {
        incident_id: format!("INC-{}", id),
        title: format!("Security Incident {}", id),
        severity,
        status: IncidentPhase::Containment,
        timeline: vec![
            IncidentTimelineEntry {
                entry_id: 1,
                timestamp_epoch: 1700000000,
                phase: IncidentPhase::Detection,
                analyst: "soc-analyst-01".into(),
                action_description: "Alert triggered by SIEM correlation rule".into(),
                artifacts: vec!["pcap-capture-001.pcap".into()],
                notes: "Initial detection via anomalous DNS pattern".into(),
            },
            IncidentTimelineEntry {
                entry_id: 2,
                timestamp_epoch: 1700000600,
                phase: IncidentPhase::Triage,
                analyst: "soc-analyst-02".into(),
                action_description: "Confirmed malicious activity".into(),
                artifacts: vec!["memory-dump-host42.raw".into()],
                notes: "Matched known APT tooling signatures".into(),
            },
            IncidentTimelineEntry {
                entry_id: 3,
                timestamp_epoch: 1700001200,
                phase: IncidentPhase::Containment,
                analyst: "ir-lead-01".into(),
                action_description: "Isolated affected hosts from network".into(),
                artifacts: vec!["firewall-block-rule-42.txt".into()],
                notes: "Network segmentation applied".into(),
            },
        ],
        affected_hosts: vec![
            "host-0042.corp.example.com".into(),
            "host-0099.corp.example.com".into(),
        ],
        ioc_ids: vec!["ioc-00000001".into(), "ioc-00000002".into()],
        assigned_team: "Incident Response Team Alpha".into(),
        escalation_level: 2,
        created_epoch: 1700000000,
        resolved_epoch: None,
    }
}

fn make_threat_actor(id: &str) -> ThreatActorProfile {
    ThreatActorProfile {
        actor_id: format!("threat-actor--{}", id),
        name: format!("APT-{}", id),
        aliases: vec![format!("DarkGroup-{}", id), format!("Shadow-{}", id)],
        motivation: ThreatActorMotivation::Espionage,
        sophistication: ThreatActorSophistication::Advanced,
        country_of_origin: Some("Unknown".into()),
        target_sectors: vec!["Defense".into(), "Government".into(), "Technology".into()],
        target_countries: vec!["US".into(), "UK".into(), "JP".into()],
        known_tools: vec![
            "Cobalt Strike".into(),
            "Mimikatz".into(),
            "Custom RAT".into(),
        ],
        known_ttps: vec!["T1071".into(), "T1059.001".into(), "T1547.001".into()],
        associated_campaigns: vec!["Operation Nightfall".into()],
        first_observed_epoch: 1650000000,
        description: "Sophisticated threat actor targeting defense sector".into(),
        references: vec![ExternalReference {
            source_name: "MITRE ATT&CK".into(),
            reference_url: "https://attack.mitre.org/groups/".into(),
            external_id: Some(format!("G{}", id)),
        }],
    }
}

fn make_correlation_rule(id: &str) -> SiemCorrelationRule {
    SiemCorrelationRule {
        rule_id: format!("CORR-{}", id),
        name: format!("Brute Force Detection Rule {}", id),
        description: "Detects multiple failed authentication attempts".into(),
        severity: AlertSeverity::High,
        conditions: vec![
            CorrelationCondition {
                field_name: "event.outcome".into(),
                operator: CorrelationOperator::Equals,
                value: "failure".into(),
            },
            CorrelationCondition {
                field_name: "event.category".into(),
                operator: CorrelationOperator::Equals,
                value: "authentication".into(),
            },
            CorrelationCondition {
                field_name: "source.ip".into(),
                operator: CorrelationOperator::NotEquals,
                value: "127.0.0.1".into(),
            },
        ],
        time_window_secs: 300,
        threshold_count: 10,
        group_by_fields: vec!["source.ip".into(), "user.name".into()],
        log_sources: vec!["auth.log".into(), "sshd".into(), "pam".into()],
        mitre_attack_ids: vec!["T1110".into(), "T1110.001".into()],
        enabled: true,
        false_positive_notes: vec![
            "Service accounts with password rotation".into(),
            "Load balancer health checks".into(),
        ],
        response_actions: vec![
            "block_source_ip".into(),
            "notify_soc".into(),
            "create_incident".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_stix_indicator_roundtrip_in_memory() {
    let indicator = make_stix_indicator("abc123", IndicatorType::Ipv4Addr, 85);
    let encoded = encode_to_vec(&indicator).expect("encode stix indicator");
    let (decoded, _): (StixIndicator, usize) =
        decode_from_slice(&encoded).expect("decode stix indicator");
    assert_eq!(indicator, decoded);
}

#[test]
fn test_taxii_collection_file_roundtrip() {
    let collection = TaxiiCollection {
        collection_id: "taxii-col-001".into(),
        title: "APT Indicators Feed".into(),
        description: "Curated threat intelligence from SOC team".into(),
        can_read: true,
        can_write: false,
        media_types: vec![
            "application/stix+json;version=2.1".into(),
            "application/taxii+json;version=2.1".into(),
        ],
        indicators: vec![
            make_stix_indicator("ind-001", IndicatorType::Ipv4Addr, 90),
            make_stix_indicator("ind-002", IndicatorType::DomainName, 75),
            make_stix_indicator("ind-003", IndicatorType::FileHashSha256, 95),
            make_stix_indicator("ind-004", IndicatorType::Url, 60),
        ],
    };

    let path = temp_dir().join("oxicode_test_taxii_collection_37.bin");
    encode_to_file(&collection, &path).expect("encode taxii collection to file");
    let decoded: TaxiiCollection =
        decode_from_file(&path).expect("decode taxii collection from file");
    assert_eq!(collection, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_mitre_attack_techniques_batch() {
    let techniques = vec![
        make_mitre_technique("T1071", AttackTactic::CommandAndControl),
        make_mitre_technique("T1071.001", AttackTactic::CommandAndControl),
        make_mitre_technique("T1059", AttackTactic::Execution),
        make_mitre_technique("T1059.001", AttackTactic::Execution),
        make_mitre_technique("T1547", AttackTactic::Persistence),
        make_mitre_technique("T1547.001", AttackTactic::Persistence),
        make_mitre_technique("T1003", AttackTactic::CredentialAccess),
        make_mitre_technique("T1190", AttackTactic::InitialAccess),
    ];

    let encoded = encode_to_vec(&techniques).expect("encode techniques batch");
    let (decoded, _): (Vec<MitreAttackTechnique>, usize) =
        decode_from_slice(&encoded).expect("decode techniques batch");
    assert_eq!(techniques, decoded);
    assert_eq!(decoded.len(), 8);

    let subtechniques: Vec<_> = decoded.iter().filter(|t| t.is_subtechnique).collect();
    assert_eq!(subtechniques.len(), 3);
}

#[test]
fn test_cve_records_with_exploitability() {
    let cves = vec![
        make_cve("2024-00001", 98),
        make_cve("2024-00002", 75),
        make_cve("2024-00003", 45),
        make_cve("2024-00004", 92),
        make_cve("2024-00005", 30),
    ];

    let path = temp_dir().join("oxicode_test_cve_records_37.bin");
    encode_to_file(&cves, &path).expect("encode cve records to file");
    let decoded: Vec<CveRecord> = decode_from_file(&path).expect("decode cve records from file");
    assert_eq!(cves, decoded);

    let exploitable: Vec<_> = decoded
        .iter()
        .filter(|c| c.exploitability.exploit_available)
        .collect();
    assert_eq!(exploitable.len(), 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_firewall_rules_diverse_actions() {
    let rules = vec![
        make_firewall_rule(1, FirewallAction::Allow),
        make_firewall_rule(2, FirewallAction::Deny),
        make_firewall_rule(3, FirewallAction::Log),
        make_firewall_rule(
            4,
            FirewallAction::RateLimit {
                packets_per_sec: 1000,
            },
        ),
        make_firewall_rule(
            5,
            FirewallAction::Redirect {
                destination: "honeypot-01.internal:8080".into(),
            },
        ),
        make_firewall_rule(6, FirewallAction::Drop),
    ];

    let encoded = encode_to_vec(&rules).expect("encode firewall rules");
    let (decoded, _): (Vec<FirewallRule>, usize) =
        decode_from_slice(&encoded).expect("decode firewall rules");
    assert_eq!(rules, decoded);
    assert_eq!(decoded.len(), 6);
}

#[test]
fn test_ids_ips_alerts_file_roundtrip() {
    let alerts = vec![
        make_ids_alert(1, AlertSeverity::Critical),
        make_ids_alert(2, AlertSeverity::High),
        make_ids_alert(3, AlertSeverity::Medium),
        make_ids_alert(4, AlertSeverity::Low),
        make_ids_alert(5, AlertSeverity::Informational),
    ];

    let path = temp_dir().join("oxicode_test_ids_alerts_37.bin");
    encode_to_file(&alerts, &path).expect("encode ids alerts to file");
    let decoded: Vec<IdsIpsAlert> = decode_from_file(&path).expect("decode ids alerts from file");
    assert_eq!(alerts, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_siem_correlation_rules_complex() {
    let rules = vec![
        make_correlation_rule("001"),
        make_correlation_rule("002"),
        make_correlation_rule("003"),
    ];

    let encoded = encode_to_vec(&rules).expect("encode correlation rules");
    let (decoded, _): (Vec<SiemCorrelationRule>, usize) =
        decode_from_slice(&encoded).expect("decode correlation rules");
    assert_eq!(rules, decoded);

    for rule in &decoded {
        assert!(!rule.conditions.is_empty());
        assert!(rule.threshold_count > 0);
        assert!(rule.enabled);
    }
}

#[test]
fn test_malware_samples_various_families() {
    let samples = vec![
        make_malware_sample(
            "aa",
            MalwareFamily::Ransomware {
                ransom_note_pattern: "YOUR FILES HAVE BEEN ENCRYPTED".into(),
            },
        ),
        make_malware_sample(
            "bb",
            MalwareFamily::Apt {
                campaign_name: "Operation Nightfall".into(),
                nation_state: "Unknown".into(),
            },
        ),
        make_malware_sample(
            "cc",
            MalwareFamily::Infostealer {
                targeted_data: vec![
                    "browser_passwords".into(),
                    "crypto_wallets".into(),
                    "ssh_keys".into(),
                ],
            },
        ),
        make_malware_sample("dd", MalwareFamily::Rootkit { kernel_mode: true }),
        make_malware_sample(
            "ee",
            MalwareFamily::Botnet {
                botnet_name: "DarkCloud".into(),
            },
        ),
    ];

    let path = temp_dir().join("oxicode_test_malware_samples_37.bin");
    encode_to_file(&samples, &path).expect("encode malware samples");
    let decoded: Vec<MalwareSample> = decode_from_file(&path).expect("decode malware samples");
    assert_eq!(samples, decoded);
    assert_eq!(decoded.len(), 5);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_netflow_records_batch() {
    let flows: Vec<NetFlowRecord> = (0..20).map(make_netflow).collect();

    let encoded = encode_to_vec(&flows).expect("encode netflow records");
    let (decoded, _): (Vec<NetFlowRecord>, usize) =
        decode_from_slice(&encoded).expect("decode netflow records");
    assert_eq!(flows, decoded);
    assert_eq!(decoded.len(), 20);

    let total_bytes_sent: u64 = decoded.iter().map(|f| f.bytes_sent).sum();
    assert!(total_bytes_sent > 0);
}

#[test]
fn test_vulnerability_scan_results_severity_distribution() {
    let results = vec![
        make_vuln_scan(1, 98),
        make_vuln_scan(2, 85),
        make_vuln_scan(3, 72),
        make_vuln_scan(4, 55),
        make_vuln_scan(5, 42),
        make_vuln_scan(6, 30),
        make_vuln_scan(7, 15),
        make_vuln_scan(8, 95),
    ];

    let path = temp_dir().join("oxicode_test_vuln_scan_37.bin");
    encode_to_file(&results, &path).expect("encode vuln scan results");
    let decoded: Vec<VulnScanResult> = decode_from_file(&path).expect("decode vuln scan results");
    assert_eq!(results, decoded);

    let critical_count = decoded
        .iter()
        .filter(|v| matches!(v.severity, AlertSeverity::Critical))
        .count();
    assert_eq!(critical_count, 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_incident_response_timeline() {
    let incident = make_incident("2024-0042", AlertSeverity::Critical);

    let encoded = encode_to_vec(&incident).expect("encode incident response");
    let (decoded, _): (IncidentResponse, usize) =
        decode_from_slice(&encoded).expect("decode incident response");
    assert_eq!(incident, decoded);

    assert_eq!(decoded.timeline.len(), 3);
    assert!(matches!(
        decoded.timeline[0].phase,
        IncidentPhase::Detection
    ));
    assert!(decoded.resolved_epoch.is_none());
}

#[test]
fn test_threat_actor_profiles_file_roundtrip() {
    let actors = vec![
        make_threat_actor("0028"),
        make_threat_actor("0029"),
        make_threat_actor("0041"),
    ];

    let path = temp_dir().join("oxicode_test_threat_actors_37.bin");
    encode_to_file(&actors, &path).expect("encode threat actor profiles");
    let decoded: Vec<ThreatActorProfile> =
        decode_from_file(&path).expect("decode threat actor profiles");
    assert_eq!(actors, decoded);

    for actor in &decoded {
        assert!(!actor.aliases.is_empty());
        assert!(matches!(
            actor.sophistication,
            ThreatActorSophistication::Advanced
        ));
    }
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_ioc_collection_with_threat_levels() {
    let iocs = vec![
        make_ioc(1, IndicatorType::Ipv4Addr, ThreatLevel::Critical),
        make_ioc(2, IndicatorType::DomainName, ThreatLevel::High),
        make_ioc(3, IndicatorType::FileHashSha256, ThreatLevel::Medium),
        make_ioc(4, IndicatorType::Url, ThreatLevel::Low),
        make_ioc(5, IndicatorType::EmailAddress, ThreatLevel::Unknown),
        make_ioc(6, IndicatorType::Ja3Hash, ThreatLevel::High),
        make_ioc(7, IndicatorType::SslCertSha1, ThreatLevel::Critical),
    ];

    let encoded = encode_to_vec(&iocs).expect("encode ioc collection");
    let (decoded, _): (Vec<IndicatorOfCompromise>, usize) =
        decode_from_slice(&encoded).expect("decode ioc collection");
    assert_eq!(iocs, decoded);
    assert!(decoded.iter().all(|i| !i.whitelisted));
}

#[test]
fn test_cert_transparency_logs_batch() {
    let logs: Vec<CertTransparencyLog> = (1..=10).map(make_cert_log).collect();

    let path = temp_dir().join("oxicode_test_cert_transparency_37.bin");
    encode_to_file(&logs, &path).expect("encode cert transparency logs");
    let decoded: Vec<CertTransparencyLog> =
        decode_from_file(&path).expect("decode cert transparency logs");
    assert_eq!(logs, decoded);

    let precert_count = decoded.iter().filter(|c| c.is_precertificate).count();
    assert!(precert_count > 0);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_dns_query_logs_various_types() {
    let logs = vec![
        make_dns_log(1, DnsQueryType::A, DnsResponseCode::NoError),
        make_dns_log(2, DnsQueryType::Aaaa, DnsResponseCode::NoError),
        make_dns_log(3, DnsQueryType::Mx, DnsResponseCode::NxDomain),
        make_dns_log(4, DnsQueryType::Txt, DnsResponseCode::ServFail),
        make_dns_log(5, DnsQueryType::Cname, DnsResponseCode::NoError),
        make_dns_log(6, DnsQueryType::Ns, DnsResponseCode::Refused),
        make_dns_log(7, DnsQueryType::Srv, DnsResponseCode::NoError),
        make_dns_log(8, DnsQueryType::Ptr, DnsResponseCode::NxDomain),
        make_dns_log(9, DnsQueryType::Dnskey, DnsResponseCode::NoError),
        make_dns_log(10, DnsQueryType::Any, DnsResponseCode::Other(42)),
    ];

    let encoded = encode_to_vec(&logs).expect("encode dns query logs");
    let (decoded, _): (Vec<DnsQueryLog>, usize) =
        decode_from_slice(&encoded).expect("decode dns query logs");
    assert_eq!(logs, decoded);
    assert_eq!(decoded.len(), 10);
}

#[test]
fn test_authentication_events_mixed_outcomes() {
    let events = vec![
        make_auth_event(1, AuthOutcome::Success),
        make_auth_event(2, AuthOutcome::FailureInvalidCredentials),
        make_auth_event(3, AuthOutcome::FailureAccountLocked),
        make_auth_event(4, AuthOutcome::FailureMfaRequired),
        make_auth_event(
            5,
            AuthOutcome::SuccessWithMfa {
                mfa_method: "TOTP".into(),
            },
        ),
        make_auth_event(6, AuthOutcome::FailureMfaTimeout),
        make_auth_event(7, AuthOutcome::FailureMfaInvalid),
        make_auth_event(8, AuthOutcome::FailureAccountExpired),
        make_auth_event(
            9,
            AuthOutcome::FailureGeoBlocked {
                country: "RU".into(),
            },
        ),
        make_auth_event(10, AuthOutcome::FailureRiskScore { score: 95 }),
    ];

    let path = temp_dir().join("oxicode_test_auth_events_37.bin");
    encode_to_file(&events, &path).expect("encode auth events");
    let decoded: Vec<AuthenticationEvent> = decode_from_file(&path).expect("decode auth events");
    assert_eq!(events, decoded);
    assert_eq!(decoded.len(), 10);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_threat_intel_report_composite() {
    let report = ThreatIntelReport {
        report_id: "TIR-2024-0042".into(),
        title: "Operation Nightfall - Comprehensive Analysis".into(),
        summary: "Multi-stage APT campaign targeting defense contractors".into(),
        threat_actors: vec![make_threat_actor("0028")],
        indicators: vec![
            make_ioc(100, IndicatorType::Ipv4Addr, ThreatLevel::Critical),
            make_ioc(101, IndicatorType::DomainName, ThreatLevel::High),
            make_ioc(102, IndicatorType::FileHashSha256, ThreatLevel::Critical),
        ],
        techniques: vec![
            make_mitre_technique("T1071", AttackTactic::CommandAndControl),
            make_mitre_technique("T1059.001", AttackTactic::Execution),
        ],
        cves: vec![make_cve("2024-99001", 98), make_cve("2024-99002", 85)],
        malware_samples: vec![make_malware_sample(
            "ff",
            MalwareFamily::Backdoor {
                c2_protocol: "HTTPS with custom headers".into(),
            },
        )],
        published_epoch: 1700000000,
        tlp_marking: "TLP:AMBER".into(),
    };

    let path = temp_dir().join("oxicode_test_threat_intel_report_37.bin");
    encode_to_file(&report, &path).expect("encode threat intel report");
    let decoded: ThreatIntelReport = decode_from_file(&path).expect("decode threat intel report");
    assert_eq!(report, decoded);
    assert_eq!(decoded.indicators.len(), 3);
    assert_eq!(decoded.techniques.len(), 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_siem_dashboard_snapshot_composite() {
    let snapshot = SiemDashboardSnapshot {
        snapshot_epoch: 1700000000,
        active_incidents: vec![
            make_incident("2024-0001", AlertSeverity::Critical),
            make_incident("2024-0002", AlertSeverity::High),
        ],
        top_alerts: vec![
            make_ids_alert(100, AlertSeverity::Critical),
            make_ids_alert(101, AlertSeverity::High),
            make_ids_alert(102, AlertSeverity::Medium),
        ],
        recent_auth_failures: vec![
            make_auth_event(200, AuthOutcome::FailureInvalidCredentials),
            make_auth_event(201, AuthOutcome::FailureAccountLocked),
            make_auth_event(202, AuthOutcome::FailureMfaInvalid),
        ],
        suspicious_dns: vec![
            make_dns_log(300, DnsQueryType::Txt, DnsResponseCode::NoError),
            make_dns_log(301, DnsQueryType::A, DnsResponseCode::NxDomain),
        ],
        active_correlation_rules: 47,
        total_events_last_hour: 2_345_678,
        unique_source_ips_last_hour: 12_345,
    };

    let encoded = encode_to_vec(&snapshot).expect("encode siem dashboard snapshot");
    let (decoded, _): (SiemDashboardSnapshot, usize) =
        decode_from_slice(&encoded).expect("decode siem dashboard snapshot");
    assert_eq!(snapshot, decoded);
    assert_eq!(decoded.active_incidents.len(), 2);
    assert_eq!(decoded.total_events_last_hour, 2_345_678);
}

#[test]
fn test_security_posture_report_file_roundtrip() {
    let report = SecurityPostureReport {
        report_epoch: 1700000000,
        vuln_scan_results: vec![
            make_vuln_scan(10, 98),
            make_vuln_scan(11, 82),
            make_vuln_scan(12, 65),
            make_vuln_scan(13, 45),
            make_vuln_scan(14, 20),
        ],
        cert_logs: vec![make_cert_log(50), make_cert_log(51), make_cert_log(52)],
        firewall_rules: vec![
            make_firewall_rule(100, FirewallAction::Allow),
            make_firewall_rule(101, FirewallAction::Deny),
            make_firewall_rule(102, FirewallAction::Drop),
        ],
        network_flows: (0..5).map(make_netflow).collect(),
        total_critical_vulns: 1,
        total_high_vulns: 1,
        mean_time_to_remediate_secs: 172800,
        compliance_score_percent: 87,
    };

    let path = temp_dir().join("oxicode_test_security_posture_37.bin");
    encode_to_file(&report, &path).expect("encode security posture report");
    let decoded: SecurityPostureReport =
        decode_from_file(&path).expect("decode security posture report");
    assert_eq!(report, decoded);
    assert_eq!(decoded.compliance_score_percent, 87);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_large_ioc_feed_roundtrip() {
    let feed: Vec<IndicatorOfCompromise> = (0..200)
        .map(|i| {
            let itype = match i % 5 {
                0 => IndicatorType::Ipv4Addr,
                1 => IndicatorType::DomainName,
                2 => IndicatorType::FileHashSha256,
                3 => IndicatorType::Url,
                _ => IndicatorType::Ja3Hash,
            };
            let level = match i % 4 {
                0 => ThreatLevel::Critical,
                1 => ThreatLevel::High,
                2 => ThreatLevel::Medium,
                _ => ThreatLevel::Low,
            };
            make_ioc(i, itype, level)
        })
        .collect();

    let path = temp_dir().join("oxicode_test_large_ioc_feed_37.bin");
    encode_to_file(&feed, &path).expect("encode large ioc feed");
    let decoded: Vec<IndicatorOfCompromise> =
        decode_from_file(&path).expect("decode large ioc feed");
    assert_eq!(feed, decoded);
    assert_eq!(decoded.len(), 200);

    let critical_count = decoded
        .iter()
        .filter(|i| matches!(i.threat_level, ThreatLevel::Critical))
        .count();
    assert_eq!(critical_count, 50);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_empty_collections_roundtrip() {
    let empty_taxii = TaxiiCollection {
        collection_id: "taxii-empty".into(),
        title: "Empty Collection".into(),
        description: "No indicators yet".into(),
        can_read: true,
        can_write: true,
        media_types: vec![],
        indicators: vec![],
    };

    let empty_incident = IncidentResponse {
        incident_id: "INC-EMPTY".into(),
        title: "Empty Incident Template".into(),
        severity: AlertSeverity::Informational,
        status: IncidentPhase::Detection,
        timeline: vec![],
        affected_hosts: vec![],
        ioc_ids: vec![],
        assigned_team: "Unassigned".into(),
        escalation_level: 0,
        created_epoch: 1700000000,
        resolved_epoch: None,
    };

    let encoded_taxii = encode_to_vec(&empty_taxii).expect("encode empty taxii");
    let (decoded_taxii, _): (TaxiiCollection, usize) =
        decode_from_slice(&encoded_taxii).expect("decode empty taxii");
    assert_eq!(empty_taxii, decoded_taxii);
    assert!(decoded_taxii.indicators.is_empty());

    let encoded_incident = encode_to_vec(&empty_incident).expect("encode empty incident");
    let (decoded_incident, _): (IncidentResponse, usize) =
        decode_from_slice(&encoded_incident).expect("decode empty incident");
    assert_eq!(empty_incident, decoded_incident);
    assert!(decoded_incident.timeline.is_empty());
}

#[test]
fn test_correlated_alert_with_netflow_and_auth() {
    let alert = make_ids_alert(9999, AlertSeverity::Critical);
    let flow = make_netflow(9999);
    let auth_fail = make_auth_event(9999, AuthOutcome::FailureInvalidCredentials);
    let auth_mfa = make_auth_event(
        10000,
        AuthOutcome::SuccessWithMfa {
            mfa_method: "hardware-token".into(),
        },
    );
    let dns = make_dns_log(9999, DnsQueryType::A, DnsResponseCode::NoError);
    let ioc = make_ioc(9999, IndicatorType::Ipv4Addr, ThreatLevel::Critical);

    let bundle = (alert, flow, auth_fail, auth_mfa, dns, ioc);

    let path = temp_dir().join("oxicode_test_correlated_bundle_37.bin");
    encode_to_file(&bundle, &path).expect("encode correlated bundle");
    let decoded: (
        IdsIpsAlert,
        NetFlowRecord,
        AuthenticationEvent,
        AuthenticationEvent,
        DnsQueryLog,
        IndicatorOfCompromise,
    ) = decode_from_file(&path).expect("decode correlated bundle");
    assert_eq!(bundle, decoded);

    assert_eq!(decoded.0.alert_id, 9999);
    assert_eq!(decoded.1.flow_id, 9999);
    assert!(matches!(
        decoded.2.outcome,
        AuthOutcome::FailureInvalidCredentials
    ));
    assert!(matches!(
        decoded.3.outcome,
        AuthOutcome::SuccessWithMfa { .. }
    ));
    assert!(matches!(decoded.5.threat_level, ThreatLevel::Critical));
    let _ = std::fs::remove_file(&path);
}
