//! Operations / KPI / scheduling-focused tests for nested_structs_advanced12 (split from nested_structs_advanced12_test.rs).

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain enums
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FrequencyBand {
    N1,
    N3,
    N7,
    N28,
    N41,
    N77,
    N78,
    N79,
    N257,
    N258,
    N260,
    N261,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SliceType {
    EmBB,
    URLLC,
    MIoT,
    V2X,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HandoverTrigger {
    A1RsrpAbove(i16),
    A2RsrpBelow(i16),
    A3NeighborOffset(i16),
    A5DualThreshold { serving: i16, neighbor: i16 },
    B1InterRat(i16),
    TimeBased(u32),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NfType {
    Amf,
    Smf,
    Upf,
    Nrf,
    Nssf,
    Ausf,
    Udm,
    Pcf,
    Nef,
    Af,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ModulationScheme {
    Qpsk,
    Qam16,
    Qam64,
    Qam256,
    Qam1024,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MecAppState {
    Deployed,
    Running,
    Suspended,
    Failed,
    Migrating,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlarmSeverity {
    Critical,
    Major,
    Minor,
    Warning,
    Info,
}

// ---------------------------------------------------------------------------
// Domain structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeoLocation {
    latitude: f64,
    longitude: f64,
    altitude_m: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QosProfile {
    qci: u8,
    five_qi: u16,
    priority_level: u8,
    packet_delay_budget_ms: u32,
    packet_error_rate_exp: i8,
    max_data_burst_volume_bytes: u32,
    guaranteed_bitrate_kbps: Option<u64>,
    max_bitrate_kbps: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkSlice {
    snssai_sst: u8,
    snssai_sd: Option<u32>,
    slice_type: SliceType,
    name: String,
    qos_profiles: Vec<QosProfile>,
    max_subscribers: u32,
    isolation_level: u8,
    allowed_tai_list: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NfInstance {
    instance_id: String,
    nf_type: NfType,
    ip_addresses: Vec<String>,
    capacity: u32,
    load_percent: u8,
    heartbeat_interval_s: u16,
    supported_slices: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NfServiceChain {
    chain_id: u64,
    name: String,
    functions: Vec<NfInstance>,
    latency_budget_ms: u32,
    redundancy_level: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoreNetworkDeployment {
    deployment_id: String,
    service_chains: Vec<NfServiceChain>,
    nrf_endpoint: String,
    total_nf_count: u32,
    region: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeasurementReport {
    rsrp_dbm: i16,
    rsrq_db: i16,
    sinr_db: i16,
    cqi: u8,
    ri: u8,
    pmi: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HandoverCandidate {
    target_pci: u16,
    target_gnb_id: u64,
    measurement: MeasurementReport,
    estimated_latency_ms: u32,
    frequency_band: FrequencyBand,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HandoverDecision {
    decision_id: u64,
    ue_imsi: String,
    source_pci: u16,
    source_gnb_id: u64,
    trigger: HandoverTrigger,
    candidates: Vec<HandoverCandidate>,
    selected_target_pci: Option<u16>,
    execution_time_ms: u32,
    success: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KpiMetric {
    metric_name: String,
    value: f64,
    unit: String,
    threshold_warning: Option<f64>,
    threshold_critical: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KpiCategory {
    category_name: String,
    metrics: Vec<KpiMetric>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkKpiDashboard {
    dashboard_id: u64,
    region: String,
    timestamp: u64,
    categories: Vec<KpiCategory>,
    overall_health_score: f64,
    active_alarms: Vec<(AlarmSeverity, String)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MecApplication {
    app_id: String,
    name: String,
    state: MecAppState,
    cpu_cores: u16,
    memory_mb: u32,
    storage_gb: u32,
    latency_requirement_ms: u32,
    endpoints: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MecHost {
    host_id: String,
    location: GeoLocation,
    total_cpu_cores: u16,
    total_memory_mb: u32,
    applications: Vec<MecApplication>,
    connected_gnb_ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MecDeployment {
    deployment_name: String,
    hosts: Vec<MecHost>,
    orchestrator_endpoint: String,
    total_app_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PagingRecord {
    ue_id: String,
    paging_cause: u8,
    tai_list: Vec<u32>,
    drx_cycle_ms: u16,
    paging_attempts: u8,
    last_known_cell_pci: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PagingArea {
    area_id: u32,
    tracking_area_codes: Vec<u32>,
    records: Vec<PagingRecord>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SchedulingGrant {
    rnti: u16,
    rb_start: u16,
    rb_count: u16,
    mcs_index: u8,
    modulation: ModulationScheme,
    ndi: bool,
    harq_process_id: u8,
    tpc_command: i8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SubframeSchedule {
    subframe_index: u8,
    downlink_grants: Vec<SchedulingGrant>,
    uplink_grants: Vec<SchedulingGrant>,
    srs_scheduled: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FrameSchedule {
    frame_number: u32,
    subframes: Vec<SubframeSchedule>,
    total_prb_utilization_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SliceSla {
    slice: NetworkSlice,
    committed_latency_ms: u32,
    committed_throughput_mbps: u64,
    actual_latency_ms: f64,
    actual_throughput_mbps: f64,
    violations_last_hour: u32,
    penalty_usd_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlaSummary {
    operator: String,
    period: String,
    sla_entries: Vec<SliceSla>,
    total_penalty_usd_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RachAttempt {
    preamble_index: u8,
    timing_advance: u16,
    ra_rnti: u16,
    contention_resolved: bool,
    backoff_ms: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RachStatistics {
    cell_pci: u16,
    attempts: Vec<RachAttempt>,
    success_count: u32,
    collision_count: u32,
    avg_access_delay_ms: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyProfile {
    component_name: String,
    power_watts: f32,
    sleep_capable: bool,
    current_state: String,
    energy_kwh_last_day: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SiteEnergyReport {
    site_id: u64,
    location: GeoLocation,
    profiles: Vec<EnergyProfile>,
    total_power_watts: f32,
    renewable_pct: f32,
    battery_backup_hours: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyDashboard {
    region: String,
    sites: Vec<SiteEnergyReport>,
    region_total_kw: f64,
    carbon_tons_per_day: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SecurityEvent {
    event_id: u64,
    event_type: String,
    source_ip: String,
    severity: AlarmSeverity,
    description: String,
    timestamp: u64,
    mitigated: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SecurityZone {
    zone_name: String,
    nf_instances: Vec<NfInstance>,
    events: Vec<SecurityEvent>,
    policy_version: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SecurityPosture {
    assessment_id: u64,
    zones: Vec<SecurityZone>,
    overall_risk_score: f64,
    last_audit_timestamp: u64,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_geo(lat: f64, lon: f64, alt: f32) -> GeoLocation {
    GeoLocation {
        latitude: lat,
        longitude: lon,
        altitude_m: alt,
    }
}

fn make_qos(qci: u8, fqi: u16, delay_ms: u32) -> QosProfile {
    QosProfile {
        qci,
        five_qi: fqi,
        priority_level: qci,
        packet_delay_budget_ms: delay_ms,
        packet_error_rate_exp: -6,
        max_data_burst_volume_bytes: 1500,
        guaranteed_bitrate_kbps: Some(10_000),
        max_bitrate_kbps: Some(100_000),
    }
}

fn make_slice(sst: u8, st: SliceType, name: &str) -> NetworkSlice {
    NetworkSlice {
        snssai_sst: sst,
        snssai_sd: Some(0x010203),
        slice_type: st,
        name: name.to_string(),
        qos_profiles: vec![make_qos(1, 1, 100), make_qos(5, 5, 300)],
        max_subscribers: 50_000,
        isolation_level: 2,
        allowed_tai_list: vec![100, 200, 300],
    }
}

fn make_nf(id: &str, nf_type: NfType) -> NfInstance {
    NfInstance {
        instance_id: id.to_string(),
        nf_type,
        ip_addresses: vec!["10.0.1.1".to_string(), "10.0.1.2".to_string()],
        capacity: 10_000,
        load_percent: 45,
        heartbeat_interval_s: 30,
        supported_slices: vec![1, 2, 3],
    }
}

fn make_measurement(rsrp: i16, sinr: i16) -> MeasurementReport {
    MeasurementReport {
        rsrp_dbm: rsrp,
        rsrq_db: -12,
        sinr_db: sinr,
        cqi: 12,
        ri: 2,
        pmi: 7,
    }
}

fn make_kpi_metric(name: &str, value: f64, unit: &str) -> KpiMetric {
    KpiMetric {
        metric_name: name.to_string(),
        value,
        unit: unit.to_string(),
        threshold_warning: Some(value * 0.8),
        threshold_critical: Some(value * 0.5),
    }
}

fn make_scheduling_grant(rnti: u16, mod_scheme: ModulationScheme) -> SchedulingGrant {
    SchedulingGrant {
        rnti,
        rb_start: 0,
        rb_count: 10,
        mcs_index: 15,
        modulation: mod_scheme,
        ndi: true,
        harq_process_id: 0,
        tpc_command: 1,
    }
}

// ---------------------------------------------------------------------------
// Test 5: Core network function chain (AMF/SMF/UPF)
// ---------------------------------------------------------------------------

#[test]
fn test_core_network_chain() {
    let deployment = CoreNetworkDeployment {
        deployment_id: "CORE-JP-01".to_string(),
        service_chains: vec![
            NfServiceChain {
                chain_id: 1,
                name: "User Plane Path".to_string(),
                functions: vec![
                    make_nf("amf-01", NfType::Amf),
                    make_nf("smf-01", NfType::Smf),
                    make_nf("upf-01", NfType::Upf),
                ],
                latency_budget_ms: 20,
                redundancy_level: 2,
            },
            NfServiceChain {
                chain_id: 2,
                name: "Auth Chain".to_string(),
                functions: vec![
                    make_nf("nrf-01", NfType::Nrf),
                    make_nf("ausf-01", NfType::Ausf),
                    make_nf("udm-01", NfType::Udm),
                ],
                latency_budget_ms: 50,
                redundancy_level: 3,
            },
        ],
        nrf_endpoint: "https://nrf.core.5g.jp:8443".to_string(),
        total_nf_count: 6,
        region: "Tokyo".to_string(),
    };
    let bytes = encode_to_vec(&deployment).expect("encode core deployment");
    let (decoded, _): (CoreNetworkDeployment, usize) =
        decode_from_slice(&bytes).expect("decode core deployment");
    assert_eq!(deployment, decoded);
    assert_eq!(decoded.service_chains.len(), 2);
    assert_eq!(decoded.service_chains[0].functions.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 7: Handover decision with candidates
// ---------------------------------------------------------------------------

#[test]
fn test_handover_decision() {
    let decision = HandoverDecision {
        decision_id: 77001,
        ue_imsi: "440101111222333".to_string(),
        source_pci: 100,
        source_gnb_id: 5001,
        trigger: HandoverTrigger::A3NeighborOffset(3),
        candidates: vec![
            HandoverCandidate {
                target_pci: 101,
                target_gnb_id: 5002,
                measurement: make_measurement(-85, 20),
                estimated_latency_ms: 15,
                frequency_band: FrequencyBand::N78,
            },
            HandoverCandidate {
                target_pci: 102,
                target_gnb_id: 5003,
                measurement: make_measurement(-90, 15),
                estimated_latency_ms: 25,
                frequency_band: FrequencyBand::N77,
            },
        ],
        selected_target_pci: Some(101),
        execution_time_ms: 18,
        success: true,
    };
    let bytes = encode_to_vec(&decision).expect("encode handover decision");
    let (decoded, _): (HandoverDecision, usize) =
        decode_from_slice(&bytes).expect("decode handover decision");
    assert_eq!(decision, decoded);
    assert_eq!(decoded.candidates.len(), 2);
    assert_eq!(decoded.selected_target_pci, Some(101));
}

// ---------------------------------------------------------------------------
// Test 9: Network KPI dashboard with nested metrics
// ---------------------------------------------------------------------------

#[test]
fn test_kpi_dashboard() {
    let dashboard = NetworkKpiDashboard {
        dashboard_id: 3001,
        region: "Kanto".to_string(),
        timestamp: 1_700_200_000,
        categories: vec![
            KpiCategory {
                category_name: "Throughput".to_string(),
                metrics: vec![
                    make_kpi_metric("avg_dl_throughput_mbps", 250.5, "Mbps"),
                    make_kpi_metric("avg_ul_throughput_mbps", 45.2, "Mbps"),
                    make_kpi_metric("peak_dl_throughput_mbps", 1200.0, "Mbps"),
                ],
            },
            KpiCategory {
                category_name: "Latency".to_string(),
                metrics: vec![
                    make_kpi_metric("avg_rtt_ms", 8.5, "ms"),
                    make_kpi_metric("p99_rtt_ms", 25.0, "ms"),
                ],
            },
            KpiCategory {
                category_name: "Availability".to_string(),
                metrics: vec![
                    make_kpi_metric("cell_availability_pct", 99.95, "%"),
                    make_kpi_metric("service_availability_pct", 99.99, "%"),
                ],
            },
        ],
        overall_health_score: 92.5,
        active_alarms: vec![
            (AlarmSeverity::Minor, "High CPU on UPF-03".to_string()),
            (
                AlarmSeverity::Warning,
                "Backhaul latency elevated on gNB-22".to_string(),
            ),
        ],
    };
    let bytes = encode_to_vec(&dashboard).expect("encode KPI dashboard");
    let (decoded, _): (NetworkKpiDashboard, usize) =
        decode_from_slice(&bytes).expect("decode KPI dashboard");
    assert_eq!(dashboard, decoded);
    assert_eq!(decoded.categories.len(), 3);
    assert_eq!(decoded.active_alarms.len(), 2);
}

// ---------------------------------------------------------------------------
// Test 10: MEC deployment with apps on hosts
// ---------------------------------------------------------------------------

#[test]
fn test_mec_deployment() {
    let deployment = MecDeployment {
        deployment_name: "Edge-Tokyo-01".to_string(),
        hosts: vec![
            MecHost {
                host_id: "mec-host-001".to_string(),
                location: make_geo(35.6812, 139.7671, 10.0),
                total_cpu_cores: 64,
                total_memory_mb: 131072,
                applications: vec![
                    MecApplication {
                        app_id: "v2x-app-01".to_string(),
                        name: "V2X Collision Avoidance".to_string(),
                        state: MecAppState::Running,
                        cpu_cores: 8,
                        memory_mb: 16384,
                        storage_gb: 100,
                        latency_requirement_ms: 5,
                        endpoints: vec![
                            "https://v2x.edge.jp:8443/api".to_string(),
                            "wss://v2x.edge.jp:9443/stream".to_string(),
                        ],
                    },
                    MecApplication {
                        app_id: "ar-app-01".to_string(),
                        name: "AR Content Delivery".to_string(),
                        state: MecAppState::Deployed,
                        cpu_cores: 16,
                        memory_mb: 32768,
                        storage_gb: 500,
                        latency_requirement_ms: 10,
                        endpoints: vec!["https://ar.edge.jp:8443/render".to_string()],
                    },
                ],
                connected_gnb_ids: vec![5001, 5002, 5003],
            },
            MecHost {
                host_id: "mec-host-002".to_string(),
                location: make_geo(35.6900, 139.7000, 15.0),
                total_cpu_cores: 32,
                total_memory_mb: 65536,
                applications: vec![MecApplication {
                    app_id: "iot-gw-01".to_string(),
                    name: "IoT Gateway".to_string(),
                    state: MecAppState::Running,
                    cpu_cores: 4,
                    memory_mb: 8192,
                    storage_gb: 50,
                    latency_requirement_ms: 20,
                    endpoints: vec!["mqtt://iot.edge.jp:1883".to_string()],
                }],
                connected_gnb_ids: vec![5004, 5005],
            },
        ],
        orchestrator_endpoint: "https://mec-orch.5g.jp:9443".to_string(),
        total_app_count: 3,
    };
    let bytes = encode_to_vec(&deployment).expect("encode MEC deployment");
    let (decoded, _): (MecDeployment, usize) =
        decode_from_slice(&bytes).expect("decode MEC deployment");
    assert_eq!(deployment, decoded);
    assert_eq!(decoded.hosts.len(), 2);
    assert_eq!(decoded.hosts[0].applications.len(), 2);
}

// ---------------------------------------------------------------------------
// Test 11: Handover with A5 dual threshold trigger
// ---------------------------------------------------------------------------

#[test]
fn test_handover_a5_dual_threshold() {
    let decision = HandoverDecision {
        decision_id: 77050,
        ue_imsi: "440109998887776".to_string(),
        source_pci: 300,
        source_gnb_id: 6001,
        trigger: HandoverTrigger::A5DualThreshold {
            serving: -100,
            neighbor: -80,
        },
        candidates: vec![
            HandoverCandidate {
                target_pci: 301,
                target_gnb_id: 6002,
                measurement: make_measurement(-78, 22),
                estimated_latency_ms: 12,
                frequency_band: FrequencyBand::N79,
            },
            HandoverCandidate {
                target_pci: 302,
                target_gnb_id: 6003,
                measurement: make_measurement(-82, 18),
                estimated_latency_ms: 20,
                frequency_band: FrequencyBand::N79,
            },
            HandoverCandidate {
                target_pci: 303,
                target_gnb_id: 6004,
                measurement: make_measurement(-88, 14),
                estimated_latency_ms: 35,
                frequency_band: FrequencyBand::N41,
            },
        ],
        selected_target_pci: Some(301),
        execution_time_ms: 14,
        success: true,
    };
    let bytes = encode_to_vec(&decision).expect("encode A5 handover");
    let (decoded, _): (HandoverDecision, usize) =
        decode_from_slice(&bytes).expect("decode A5 handover");
    assert_eq!(decision, decoded);
    assert_eq!(decoded.candidates.len(), 3);
    assert!(decoded.success);
}

// ---------------------------------------------------------------------------
// Test 15: Paging area with multiple records
// ---------------------------------------------------------------------------

#[test]
fn test_paging_area_records() {
    let area = PagingArea {
        area_id: 4001,
        tracking_area_codes: vec![100, 101, 102, 103],
        records: vec![
            PagingRecord {
                ue_id: "ue-001".to_string(),
                paging_cause: 1,
                tai_list: vec![100, 101],
                drx_cycle_ms: 320,
                paging_attempts: 2,
                last_known_cell_pci: 200,
            },
            PagingRecord {
                ue_id: "ue-002".to_string(),
                paging_cause: 2,
                tai_list: vec![102],
                drx_cycle_ms: 640,
                paging_attempts: 1,
                last_known_cell_pci: 201,
            },
            PagingRecord {
                ue_id: "ue-003".to_string(),
                paging_cause: 1,
                tai_list: vec![100, 101, 102, 103],
                drx_cycle_ms: 1280,
                paging_attempts: 5,
                last_known_cell_pci: 203,
            },
        ],
    };
    let bytes = encode_to_vec(&area).expect("encode paging area");
    let (decoded, _): (PagingArea, usize) = decode_from_slice(&bytes).expect("decode paging area");
    assert_eq!(area, decoded);
    assert_eq!(decoded.records.len(), 3);
    assert_eq!(decoded.records[2].paging_attempts, 5);
}

// ---------------------------------------------------------------------------
// Test 16: Frame schedule with subframe grants
// ---------------------------------------------------------------------------

#[test]
fn test_frame_schedule_grants() {
    let frame = FrameSchedule {
        frame_number: 1024,
        subframes: vec![
            SubframeSchedule {
                subframe_index: 0,
                downlink_grants: vec![
                    make_scheduling_grant(1001, ModulationScheme::Qam256),
                    make_scheduling_grant(1002, ModulationScheme::Qam64),
                ],
                uplink_grants: vec![make_scheduling_grant(1001, ModulationScheme::Qam16)],
                srs_scheduled: true,
            },
            SubframeSchedule {
                subframe_index: 1,
                downlink_grants: vec![make_scheduling_grant(1003, ModulationScheme::Qam1024)],
                uplink_grants: vec![],
                srs_scheduled: false,
            },
            SubframeSchedule {
                subframe_index: 2,
                downlink_grants: vec![],
                uplink_grants: vec![
                    make_scheduling_grant(1001, ModulationScheme::Qpsk),
                    make_scheduling_grant(1004, ModulationScheme::Qam16),
                    make_scheduling_grant(1005, ModulationScheme::Qam64),
                ],
                srs_scheduled: false,
            },
        ],
        total_prb_utilization_pct: 72.5,
    };
    let bytes = encode_to_vec(&frame).expect("encode frame schedule");
    let (decoded, _): (FrameSchedule, usize) =
        decode_from_slice(&bytes).expect("decode frame schedule");
    assert_eq!(frame, decoded);
    assert_eq!(decoded.subframes.len(), 3);
    assert_eq!(decoded.subframes[0].downlink_grants.len(), 2);
    assert_eq!(decoded.subframes[2].uplink_grants.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 17: SLA summary with slice SLAs
// ---------------------------------------------------------------------------

#[test]
fn test_sla_summary() {
    let summary = SlaSummary {
        operator: "JP-Telecom-5G".to_string(),
        period: "2024-Q4".to_string(),
        sla_entries: vec![
            SliceSla {
                slice: make_slice(1, SliceType::EmBB, "eMBB-Consumer"),
                committed_latency_ms: 20,
                committed_throughput_mbps: 100,
                actual_latency_ms: 18.5,
                actual_throughput_mbps: 115.3,
                violations_last_hour: 0,
                penalty_usd_cents: 0,
            },
            SliceSla {
                slice: make_slice(2, SliceType::URLLC, "URLLC-Factory"),
                committed_latency_ms: 1,
                committed_throughput_mbps: 10,
                actual_latency_ms: 0.8,
                actual_throughput_mbps: 12.1,
                violations_last_hour: 2,
                penalty_usd_cents: 5000,
            },
            SliceSla {
                slice: make_slice(3, SliceType::MIoT, "MIoT-SmartCity"),
                committed_latency_ms: 500,
                committed_throughput_mbps: 1,
                actual_latency_ms: 320.0,
                actual_throughput_mbps: 1.5,
                violations_last_hour: 0,
                penalty_usd_cents: 0,
            },
        ],
        total_penalty_usd_cents: 5000,
    };
    let bytes = encode_to_vec(&summary).expect("encode SLA summary");
    let (decoded, _): (SlaSummary, usize) = decode_from_slice(&bytes).expect("decode SLA summary");
    assert_eq!(summary, decoded);
    assert_eq!(decoded.sla_entries.len(), 3);
    assert_eq!(decoded.total_penalty_usd_cents, 5000);
}

// ---------------------------------------------------------------------------
// Test 18: RACH statistics per cell
// ---------------------------------------------------------------------------

#[test]
fn test_rach_statistics() {
    let stats = RachStatistics {
        cell_pci: 500,
        attempts: vec![
            RachAttempt {
                preamble_index: 12,
                timing_advance: 45,
                ra_rnti: 1,
                contention_resolved: true,
                backoff_ms: 0,
            },
            RachAttempt {
                preamble_index: 12,
                timing_advance: 48,
                ra_rnti: 1,
                contention_resolved: false,
                backoff_ms: 20,
            },
            RachAttempt {
                preamble_index: 33,
                timing_advance: 100,
                ra_rnti: 2,
                contention_resolved: true,
                backoff_ms: 0,
            },
            RachAttempt {
                preamble_index: 55,
                timing_advance: 200,
                ra_rnti: 3,
                contention_resolved: true,
                backoff_ms: 0,
            },
        ],
        success_count: 3,
        collision_count: 1,
        avg_access_delay_ms: 12.5,
    };
    let bytes = encode_to_vec(&stats).expect("encode RACH stats");
    let (decoded, _): (RachStatistics, usize) =
        decode_from_slice(&bytes).expect("decode RACH stats");
    assert_eq!(stats, decoded);
    assert_eq!(decoded.attempts.len(), 4);
    assert_eq!(decoded.collision_count, 1);
}

// ---------------------------------------------------------------------------
// Test 19: Energy dashboard with site reports
// ---------------------------------------------------------------------------

#[test]
fn test_energy_dashboard() {
    let dashboard = EnergyDashboard {
        region: "Hokkaido".to_string(),
        sites: vec![
            SiteEnergyReport {
                site_id: 11001,
                location: make_geo(43.0618, 141.3545, 20.0),
                profiles: vec![
                    EnergyProfile {
                        component_name: "Radio Unit (N78)".to_string(),
                        power_watts: 1200.0,
                        sleep_capable: true,
                        current_state: "active".to_string(),
                        energy_kwh_last_day: 28.8,
                    },
                    EnergyProfile {
                        component_name: "Radio Unit (N257)".to_string(),
                        power_watts: 800.0,
                        sleep_capable: true,
                        current_state: "micro-sleep".to_string(),
                        energy_kwh_last_day: 12.0,
                    },
                    EnergyProfile {
                        component_name: "Baseband".to_string(),
                        power_watts: 500.0,
                        sleep_capable: false,
                        current_state: "active".to_string(),
                        energy_kwh_last_day: 12.0,
                    },
                ],
                total_power_watts: 2500.0,
                renewable_pct: 35.0,
                battery_backup_hours: 4.0,
            },
            SiteEnergyReport {
                site_id: 11002,
                location: make_geo(43.0500, 141.3400, 18.0),
                profiles: vec![EnergyProfile {
                    component_name: "Small Cell".to_string(),
                    power_watts: 150.0,
                    sleep_capable: true,
                    current_state: "active".to_string(),
                    energy_kwh_last_day: 3.6,
                }],
                total_power_watts: 150.0,
                renewable_pct: 100.0,
                battery_backup_hours: 12.0,
            },
        ],
        region_total_kw: 2.65,
        carbon_tons_per_day: 0.45,
    };
    let bytes = encode_to_vec(&dashboard).expect("encode energy dashboard");
    let (decoded, _): (EnergyDashboard, usize) =
        decode_from_slice(&bytes).expect("decode energy dashboard");
    assert_eq!(dashboard, decoded);
    assert_eq!(decoded.sites.len(), 2);
    assert_eq!(decoded.sites[0].profiles.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 20: Security posture with zones and events
// ---------------------------------------------------------------------------

#[test]
fn test_security_posture() {
    let posture = SecurityPosture {
        assessment_id: 55001,
        zones: vec![
            SecurityZone {
                zone_name: "Core-DMZ".to_string(),
                nf_instances: vec![make_nf("nef-01", NfType::Nef), make_nf("af-01", NfType::Af)],
                events: vec![
                    SecurityEvent {
                        event_id: 1,
                        event_type: "unauthorized_api_call".to_string(),
                        source_ip: "203.0.113.50".to_string(),
                        severity: AlarmSeverity::Major,
                        description: "Unauthenticated NF registration attempt".to_string(),
                        timestamp: 1_700_300_000,
                        mitigated: true,
                    },
                    SecurityEvent {
                        event_id: 2,
                        event_type: "certificate_expiry_warning".to_string(),
                        source_ip: "10.0.5.10".to_string(),
                        severity: AlarmSeverity::Warning,
                        description: "TLS cert expires in 7 days".to_string(),
                        timestamp: 1_700_300_500,
                        mitigated: false,
                    },
                ],
                policy_version: 42,
            },
            SecurityZone {
                zone_name: "RAN-Zone".to_string(),
                nf_instances: vec![make_nf("amf-02", NfType::Amf)],
                events: vec![],
                policy_version: 40,
            },
        ],
        overall_risk_score: 3.7,
        last_audit_timestamp: 1_700_250_000,
    };
    let bytes = encode_to_vec(&posture).expect("encode security posture");
    let (decoded, _): (SecurityPosture, usize) =
        decode_from_slice(&bytes).expect("decode security posture");
    assert_eq!(posture, decoded);
    assert_eq!(decoded.zones.len(), 2);
    assert_eq!(decoded.zones[0].events.len(), 2);
    assert!(decoded.zones[1].events.is_empty());
}
