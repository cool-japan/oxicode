//! Advanced nested structs test #12 — 5G telecommunications network management theme, 22 tests.

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
enum BearerType {
    DefaultBearer,
    DedicatedBearer,
    EmergencyBearer,
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
// Domain structs — deeply nested for 5G telecom
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeoLocation {
    latitude: f64,
    longitude: f64,
    altitude_m: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BeamPattern {
    beam_id: u16,
    azimuth_deg: f32,
    elevation_deg: f32,
    beamwidth_h_deg: f32,
    beamwidth_v_deg: f32,
    gain_dbi: f32,
    ssb_index: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AntennaArray {
    array_id: u32,
    rows: u16,
    columns: u16,
    polarization_count: u8,
    element_spacing_mm: f32,
    beam_patterns: Vec<BeamPattern>,
    tilt_deg: f32,
    frequency_band: FrequencyBand,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CellSector {
    sector_id: u32,
    pci: u16,
    earfcn: u32,
    bandwidth_mhz: u16,
    tx_power_dbm: f32,
    antenna: AntennaArray,
    neighbor_pcis: Vec<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CellTower {
    tower_id: u64,
    name: String,
    location: GeoLocation,
    sectors: Vec<CellSector>,
    backhaul_capacity_gbps: f32,
    power_supply_backup_hours: u16,
    operator_id: String,
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
struct BearerContext {
    bearer_id: u32,
    bearer_type: BearerType,
    qos: QosProfile,
    tft_filters: Vec<String>,
    gtp_teid: u32,
    uplink_bytes: u64,
    downlink_bytes: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SubscriberSession {
    imsi: String,
    msisdn: String,
    session_id: u64,
    apn: String,
    ip_address: String,
    bearer_contexts: Vec<BearerContext>,
    serving_cell_pci: u16,
    slice: Option<NetworkSlice>,
    registration_timestamp: u64,
    idle: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GnbNode {
    gnb_id: u64,
    name: String,
    location: GeoLocation,
    cells: Vec<CellSector>,
    connected_amf_ids: Vec<u64>,
    max_ue_capacity: u32,
    f1_interface_ip: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CuDuSplit {
    cu_id: u64,
    du_nodes: Vec<GnbNode>,
    midhaul_latency_us: u32,
    midhaul_bandwidth_gbps: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RanTopology {
    region_name: String,
    cu_du_splits: Vec<CuDuSplit>,
    total_gnb_count: u32,
    total_cell_count: u32,
    inter_gnb_x2_links: Vec<(u64, u64)>,
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
struct SpectrumBlock {
    start_freq_mhz: u32,
    end_freq_mhz: u32,
    bandwidth_mhz: u16,
    band: FrequencyBand,
    license_holder: String,
    license_expiry_year: u16,
    duplex_mode: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectrumAllocationTable {
    country_code: String,
    blocks: Vec<SpectrumBlock>,
    total_allocated_mhz: u32,
    last_auction_date: String,
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
struct MimoLayer {
    layer_id: u8,
    precoding_matrix_index: u16,
    modulation: ModulationScheme,
    code_rate: f32,
    cw_index: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MimoConfig {
    config_id: u64,
    antenna_ports: u16,
    layers: Vec<MimoLayer>,
    max_rank: u8,
    codebook_type: String,
    srs_resources: u8,
    csi_rs_resources: u16,
    digital_beamforming: bool,
    analog_beamforming: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MassiveMimoDeployment {
    site_id: u64,
    configs: Vec<MimoConfig>,
    total_antenna_elements: u32,
    supported_bands: Vec<FrequencyBand>,
    calibration_timestamp: u64,
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

fn make_beam(id: u16, az: f32, el: f32) -> BeamPattern {
    BeamPattern {
        beam_id: id,
        azimuth_deg: az,
        elevation_deg: el,
        beamwidth_h_deg: 10.0,
        beamwidth_v_deg: 8.0,
        gain_dbi: 18.5,
        ssb_index: id as u8,
    }
}

fn make_antenna(id: u32, band: FrequencyBand, beams: Vec<BeamPattern>) -> AntennaArray {
    AntennaArray {
        array_id: id,
        rows: 8,
        columns: 8,
        polarization_count: 2,
        element_spacing_mm: 17.5,
        beam_patterns: beams,
        tilt_deg: 6.0,
        frequency_band: band,
    }
}

fn make_sector(sid: u32, pci: u16, band: FrequencyBand) -> CellSector {
    CellSector {
        sector_id: sid,
        pci,
        earfcn: 630000 + sid,
        bandwidth_mhz: 100,
        tx_power_dbm: 43.0,
        antenna: make_antenna(
            sid,
            band,
            vec![
                make_beam(0, 0.0, -6.0),
                make_beam(1, 30.0, -6.0),
                make_beam(2, -30.0, -6.0),
            ],
        ),
        neighbor_pcis: vec![pci.wrapping_add(1), pci.wrapping_add(2)],
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

fn make_bearer(id: u32, bt: BearerType) -> BearerContext {
    BearerContext {
        bearer_id: id,
        bearer_type: bt,
        qos: make_qos(1, 1, 100),
        tft_filters: vec!["permit ip any any".to_string()],
        gtp_teid: 0xABCD_0000 + id,
        uplink_bytes: 1_000_000,
        downlink_bytes: 5_000_000,
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

fn make_mimo_layer(lid: u8, mod_scheme: ModulationScheme) -> MimoLayer {
    MimoLayer {
        layer_id: lid,
        precoding_matrix_index: 42,
        modulation: mod_scheme,
        code_rate: 0.65,
        cw_index: lid % 2,
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
// Test 1: Cell tower with multi-sector antenna arrays
// ---------------------------------------------------------------------------

#[test]
fn test_cell_tower_multisector() {
    let tower = CellTower {
        tower_id: 1001,
        name: "Tokyo-Shibuya-01".to_string(),
        location: make_geo(35.6595, 139.7004, 45.0),
        sectors: vec![
            make_sector(1, 100, FrequencyBand::N78),
            make_sector(2, 101, FrequencyBand::N78),
            make_sector(3, 102, FrequencyBand::N257),
        ],
        backhaul_capacity_gbps: 25.0,
        power_supply_backup_hours: 8,
        operator_id: "JP-OPER-01".to_string(),
    };
    let bytes = encode_to_vec(&tower).expect("encode cell tower");
    let (decoded, _): (CellTower, usize) = decode_from_slice(&bytes).expect("decode cell tower");
    assert_eq!(tower, decoded);
    assert_eq!(decoded.sectors.len(), 3);
    assert_eq!(decoded.sectors[0].antenna.beam_patterns.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 2: Network slice with QoS profiles
// ---------------------------------------------------------------------------

#[test]
fn test_network_slice_qos() {
    let slice = NetworkSlice {
        snssai_sst: 1,
        snssai_sd: Some(0xABCDEF),
        slice_type: SliceType::EmBB,
        name: "Enhanced Mobile Broadband".to_string(),
        qos_profiles: vec![
            make_qos(1, 1, 100),
            make_qos(5, 5, 300),
            make_qos(9, 9, 500),
            QosProfile {
                qci: 65,
                five_qi: 65,
                priority_level: 1,
                packet_delay_budget_ms: 75,
                packet_error_rate_exp: -2,
                max_data_burst_volume_bytes: 4096,
                guaranteed_bitrate_kbps: Some(500_000),
                max_bitrate_kbps: None,
            },
        ],
        max_subscribers: 200_000,
        isolation_level: 3,
        allowed_tai_list: vec![10001, 10002, 10003, 10004],
    };
    let bytes = encode_to_vec(&slice).expect("encode network slice");
    let (decoded, _): (NetworkSlice, usize) =
        decode_from_slice(&bytes).expect("decode network slice");
    assert_eq!(slice, decoded);
    assert_eq!(decoded.qos_profiles.len(), 4);
    assert_eq!(decoded.qos_profiles[3].max_bitrate_kbps, None);
}

// ---------------------------------------------------------------------------
// Test 3: Subscriber session with bearer contexts
// ---------------------------------------------------------------------------

#[test]
fn test_subscriber_session_bearers() {
    let session = SubscriberSession {
        imsi: "440101234567890".to_string(),
        msisdn: "+81901234567".to_string(),
        session_id: 9999001,
        apn: "internet.5g.jp".to_string(),
        ip_address: "100.64.10.55".to_string(),
        bearer_contexts: vec![
            make_bearer(5, BearerType::DefaultBearer),
            make_bearer(6, BearerType::DedicatedBearer),
            make_bearer(7, BearerType::DedicatedBearer),
        ],
        serving_cell_pci: 100,
        slice: Some(make_slice(1, SliceType::EmBB, "eMBB-01")),
        registration_timestamp: 1_700_000_000,
        idle: false,
    };
    let bytes = encode_to_vec(&session).expect("encode subscriber session");
    let (decoded, _): (SubscriberSession, usize) =
        decode_from_slice(&bytes).expect("decode subscriber session");
    assert_eq!(session, decoded);
    assert_eq!(decoded.bearer_contexts.len(), 3);
    assert!(decoded.slice.is_some());
}

// ---------------------------------------------------------------------------
// Test 4: RAN topology with CU-DU split
// ---------------------------------------------------------------------------

#[test]
fn test_ran_topology_cu_du() {
    let gnb1 = GnbNode {
        gnb_id: 5001,
        name: "gNB-Osaka-01".to_string(),
        location: make_geo(34.6937, 135.5023, 30.0),
        cells: vec![make_sector(10, 200, FrequencyBand::N77)],
        connected_amf_ids: vec![1, 2],
        max_ue_capacity: 5000,
        f1_interface_ip: "10.100.0.1".to_string(),
    };
    let gnb2 = GnbNode {
        gnb_id: 5002,
        name: "gNB-Osaka-02".to_string(),
        location: make_geo(34.6950, 135.5050, 28.0),
        cells: vec![
            make_sector(11, 201, FrequencyBand::N77),
            make_sector(12, 202, FrequencyBand::N258),
        ],
        connected_amf_ids: vec![1],
        max_ue_capacity: 8000,
        f1_interface_ip: "10.100.0.2".to_string(),
    };
    let topology = RanTopology {
        region_name: "Kansai".to_string(),
        cu_du_splits: vec![CuDuSplit {
            cu_id: 9001,
            du_nodes: vec![gnb1, gnb2],
            midhaul_latency_us: 250,
            midhaul_bandwidth_gbps: 25.0,
        }],
        total_gnb_count: 2,
        total_cell_count: 3,
        inter_gnb_x2_links: vec![(5001, 5002)],
    };
    let bytes = encode_to_vec(&topology).expect("encode RAN topology");
    let (decoded, _): (RanTopology, usize) =
        decode_from_slice(&bytes).expect("decode RAN topology");
    assert_eq!(topology, decoded);
    assert_eq!(decoded.cu_du_splits[0].du_nodes.len(), 2);
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
// Test 6: Spectrum allocation table
// ---------------------------------------------------------------------------

#[test]
fn test_spectrum_allocation() {
    let table = SpectrumAllocationTable {
        country_code: "JP".to_string(),
        blocks: vec![
            SpectrumBlock {
                start_freq_mhz: 3600,
                end_freq_mhz: 3700,
                bandwidth_mhz: 100,
                band: FrequencyBand::N78,
                license_holder: "OperatorA".to_string(),
                license_expiry_year: 2035,
                duplex_mode: "TDD".to_string(),
            },
            SpectrumBlock {
                start_freq_mhz: 27500,
                end_freq_mhz: 27900,
                bandwidth_mhz: 400,
                band: FrequencyBand::N257,
                license_holder: "OperatorB".to_string(),
                license_expiry_year: 2033,
                duplex_mode: "TDD".to_string(),
            },
            SpectrumBlock {
                start_freq_mhz: 700,
                end_freq_mhz: 730,
                bandwidth_mhz: 30,
                band: FrequencyBand::N28,
                license_holder: "OperatorC".to_string(),
                license_expiry_year: 2040,
                duplex_mode: "FDD".to_string(),
            },
        ],
        total_allocated_mhz: 530,
        last_auction_date: "2024-06-15".to_string(),
    };
    let bytes = encode_to_vec(&table).expect("encode spectrum table");
    let (decoded, _): (SpectrumAllocationTable, usize) =
        decode_from_slice(&bytes).expect("decode spectrum table");
    assert_eq!(table, decoded);
    assert_eq!(decoded.blocks.len(), 3);
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
// Test 8: Massive MIMO deployment
// ---------------------------------------------------------------------------

#[test]
fn test_massive_mimo_deployment() {
    let deployment = MassiveMimoDeployment {
        site_id: 2001,
        configs: vec![
            MimoConfig {
                config_id: 1,
                antenna_ports: 64,
                layers: vec![
                    make_mimo_layer(0, ModulationScheme::Qam256),
                    make_mimo_layer(1, ModulationScheme::Qam256),
                    make_mimo_layer(2, ModulationScheme::Qam64),
                    make_mimo_layer(3, ModulationScheme::Qam64),
                ],
                max_rank: 4,
                codebook_type: "TypeII".to_string(),
                srs_resources: 4,
                csi_rs_resources: 32,
                digital_beamforming: true,
                analog_beamforming: true,
            },
            MimoConfig {
                config_id: 2,
                antenna_ports: 32,
                layers: vec![
                    make_mimo_layer(0, ModulationScheme::Qam1024),
                    make_mimo_layer(1, ModulationScheme::Qam1024),
                ],
                max_rank: 2,
                codebook_type: "TypeI".to_string(),
                srs_resources: 2,
                csi_rs_resources: 16,
                digital_beamforming: true,
                analog_beamforming: false,
            },
        ],
        total_antenna_elements: 256,
        supported_bands: vec![FrequencyBand::N78, FrequencyBand::N257],
        calibration_timestamp: 1_700_100_000,
    };
    let bytes = encode_to_vec(&deployment).expect("encode MIMO deployment");
    let (decoded, _): (MassiveMimoDeployment, usize) =
        decode_from_slice(&bytes).expect("decode MIMO deployment");
    assert_eq!(deployment, decoded);
    assert_eq!(decoded.configs[0].layers.len(), 4);
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
// Test 12: URLLC slice with strict QoS
// ---------------------------------------------------------------------------

#[test]
fn test_urllc_slice_strict_qos() {
    let slice = NetworkSlice {
        snssai_sst: 2,
        snssai_sd: Some(0x000001),
        slice_type: SliceType::URLLC,
        name: "Ultra-Reliable Factory Automation".to_string(),
        qos_profiles: vec![
            QosProfile {
                qci: 82,
                five_qi: 82,
                priority_level: 1,
                packet_delay_budget_ms: 1,
                packet_error_rate_exp: -5,
                max_data_burst_volume_bytes: 256,
                guaranteed_bitrate_kbps: Some(1_000),
                max_bitrate_kbps: Some(5_000),
            },
            QosProfile {
                qci: 83,
                five_qi: 83,
                priority_level: 2,
                packet_delay_budget_ms: 5,
                packet_error_rate_exp: -5,
                max_data_burst_volume_bytes: 512,
                guaranteed_bitrate_kbps: Some(2_000),
                max_bitrate_kbps: Some(10_000),
            },
        ],
        max_subscribers: 1_000,
        isolation_level: 5,
        allowed_tai_list: vec![50001],
    };
    let bytes = encode_to_vec(&slice).expect("encode URLLC slice");
    let (decoded, _): (NetworkSlice, usize) =
        decode_from_slice(&bytes).expect("decode URLLC slice");
    assert_eq!(slice, decoded);
    assert_eq!(decoded.qos_profiles[0].packet_delay_budget_ms, 1);
}

// ---------------------------------------------------------------------------
// Test 13: Subscriber session without slice (no slice assigned)
// ---------------------------------------------------------------------------

#[test]
fn test_subscriber_session_no_slice() {
    let session = SubscriberSession {
        imsi: "440105556667778".to_string(),
        msisdn: "+81905556677".to_string(),
        session_id: 8888001,
        apn: "legacy.lte.jp".to_string(),
        ip_address: "100.64.20.99".to_string(),
        bearer_contexts: vec![make_bearer(5, BearerType::DefaultBearer)],
        serving_cell_pci: 400,
        slice: None,
        registration_timestamp: 1_700_050_000,
        idle: true,
    };
    let bytes = encode_to_vec(&session).expect("encode session no slice");
    let (decoded, _): (SubscriberSession, usize) =
        decode_from_slice(&bytes).expect("decode session no slice");
    assert_eq!(session, decoded);
    assert!(decoded.slice.is_none());
    assert!(decoded.idle);
}

// ---------------------------------------------------------------------------
// Test 14: Complex RAN topology with multiple CU-DU splits
// ---------------------------------------------------------------------------

#[test]
fn test_ran_topology_multi_cu() {
    let make_gnb = |id: u64, name: &str, lat: f64, lon: f64| -> GnbNode {
        GnbNode {
            gnb_id: id,
            name: name.to_string(),
            location: make_geo(lat, lon, 25.0),
            cells: vec![
                make_sector(id as u32 * 10, (id * 10) as u16, FrequencyBand::N78),
                make_sector(
                    id as u32 * 10 + 1,
                    (id * 10 + 1) as u16,
                    FrequencyBand::N258,
                ),
            ],
            connected_amf_ids: vec![1, 2],
            max_ue_capacity: 6000,
            f1_interface_ip: format!("10.200.{}.1", id),
        }
    };

    let topology = RanTopology {
        region_name: "Chubu".to_string(),
        cu_du_splits: vec![
            CuDuSplit {
                cu_id: 8001,
                du_nodes: vec![
                    make_gnb(7001, "gNB-Nagoya-01", 35.1815, 136.9066),
                    make_gnb(7002, "gNB-Nagoya-02", 35.1700, 136.8900),
                ],
                midhaul_latency_us: 200,
                midhaul_bandwidth_gbps: 50.0,
            },
            CuDuSplit {
                cu_id: 8002,
                du_nodes: vec![make_gnb(7003, "gNB-Shizuoka-01", 34.9756, 138.3828)],
                midhaul_latency_us: 350,
                midhaul_bandwidth_gbps: 25.0,
            },
        ],
        total_gnb_count: 3,
        total_cell_count: 6,
        inter_gnb_x2_links: vec![(7001, 7002), (7001, 7003)],
    };
    let bytes = encode_to_vec(&topology).expect("encode multi-CU topology");
    let (decoded, _): (RanTopology, usize) =
        decode_from_slice(&bytes).expect("decode multi-CU topology");
    assert_eq!(topology, decoded);
    assert_eq!(decoded.cu_du_splits.len(), 2);
    assert_eq!(decoded.inter_gnb_x2_links.len(), 2);
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

// ---------------------------------------------------------------------------
// Test 21: Full 5G deployment snapshot (deeply nested aggregate)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Full5gDeploymentSnapshot {
    operator_name: String,
    snapshot_timestamp: u64,
    towers: Vec<CellTower>,
    slices: Vec<NetworkSlice>,
    core: CoreNetworkDeployment,
    mec: MecDeployment,
    kpi: NetworkKpiDashboard,
    spectrum: SpectrumAllocationTable,
}

#[test]
fn test_full_5g_deployment_snapshot() {
    let snapshot = Full5gDeploymentSnapshot {
        operator_name: "5G-Japan-Corp".to_string(),
        snapshot_timestamp: 1_700_400_000,
        towers: vec![CellTower {
            tower_id: 1,
            name: "Central-Tower".to_string(),
            location: make_geo(35.6762, 139.6503, 50.0),
            sectors: vec![
                make_sector(1, 10, FrequencyBand::N78),
                make_sector(2, 11, FrequencyBand::N257),
            ],
            backhaul_capacity_gbps: 100.0,
            power_supply_backup_hours: 12,
            operator_id: "5GJC".to_string(),
        }],
        slices: vec![
            make_slice(1, SliceType::EmBB, "Consumer-Broadband"),
            make_slice(2, SliceType::URLLC, "Industrial-Control"),
        ],
        core: CoreNetworkDeployment {
            deployment_id: "CORE-MAIN".to_string(),
            service_chains: vec![NfServiceChain {
                chain_id: 1,
                name: "Primary-UP".to_string(),
                functions: vec![
                    make_nf("amf-main", NfType::Amf),
                    make_nf("smf-main", NfType::Smf),
                    make_nf("upf-main", NfType::Upf),
                ],
                latency_budget_ms: 15,
                redundancy_level: 3,
            }],
            nrf_endpoint: "https://nrf.main.5gjc.jp:8443".to_string(),
            total_nf_count: 3,
            region: "National".to_string(),
        },
        mec: MecDeployment {
            deployment_name: "MEC-National".to_string(),
            hosts: vec![MecHost {
                host_id: "mec-central".to_string(),
                location: make_geo(35.6762, 139.6503, 5.0),
                total_cpu_cores: 128,
                total_memory_mb: 262144,
                applications: vec![MecApplication {
                    app_id: "cdn-edge".to_string(),
                    name: "CDN Edge Cache".to_string(),
                    state: MecAppState::Running,
                    cpu_cores: 32,
                    memory_mb: 65536,
                    storage_gb: 2000,
                    latency_requirement_ms: 5,
                    endpoints: vec!["https://cdn.edge.5gjc.jp".to_string()],
                }],
                connected_gnb_ids: vec![1, 2, 3],
            }],
            orchestrator_endpoint: "https://mec-orch.5gjc.jp:9443".to_string(),
            total_app_count: 1,
        },
        kpi: NetworkKpiDashboard {
            dashboard_id: 1,
            region: "National".to_string(),
            timestamp: 1_700_400_000,
            categories: vec![KpiCategory {
                category_name: "Overall".to_string(),
                metrics: vec![make_kpi_metric("connected_ues", 1_500_000.0, "count")],
            }],
            overall_health_score: 95.0,
            active_alarms: vec![],
        },
        spectrum: SpectrumAllocationTable {
            country_code: "JP".to_string(),
            blocks: vec![SpectrumBlock {
                start_freq_mhz: 3700,
                end_freq_mhz: 3800,
                bandwidth_mhz: 100,
                band: FrequencyBand::N78,
                license_holder: "5G-Japan-Corp".to_string(),
                license_expiry_year: 2038,
                duplex_mode: "TDD".to_string(),
            }],
            total_allocated_mhz: 100,
            last_auction_date: "2023-03-01".to_string(),
        },
    };
    let bytes = encode_to_vec(&snapshot).expect("encode full snapshot");
    let (decoded, _): (Full5gDeploymentSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode full snapshot");
    assert_eq!(snapshot, decoded);
    assert_eq!(decoded.towers.len(), 1);
    assert_eq!(decoded.slices.len(), 2);
    assert_eq!(decoded.core.service_chains[0].functions.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 22: V2X custom slice with emergency bearer session
// ---------------------------------------------------------------------------

#[test]
fn test_v2x_slice_emergency_bearer() {
    let v2x_slice = NetworkSlice {
        snssai_sst: 4,
        snssai_sd: Some(0x040000),
        slice_type: SliceType::V2X,
        name: "Vehicle-to-Everything".to_string(),
        qos_profiles: vec![
            QosProfile {
                qci: 75,
                five_qi: 75,
                priority_level: 1,
                packet_delay_budget_ms: 3,
                packet_error_rate_exp: -5,
                max_data_burst_volume_bytes: 1200,
                guaranteed_bitrate_kbps: Some(50_000),
                max_bitrate_kbps: Some(200_000),
            },
            QosProfile {
                qci: 79,
                five_qi: 79,
                priority_level: 3,
                packet_delay_budget_ms: 50,
                packet_error_rate_exp: -3,
                max_data_burst_volume_bytes: 8000,
                guaranteed_bitrate_kbps: None,
                max_bitrate_kbps: Some(500_000),
            },
        ],
        max_subscribers: 10_000,
        isolation_level: 4,
        allowed_tai_list: vec![60001, 60002, 60003],
    };

    let session = SubscriberSession {
        imsi: "440107777888999".to_string(),
        msisdn: "+81907778899".to_string(),
        session_id: 7777001,
        apn: "v2x.connected.jp".to_string(),
        ip_address: "100.64.50.1".to_string(),
        bearer_contexts: vec![
            make_bearer(5, BearerType::DefaultBearer),
            BearerContext {
                bearer_id: 6,
                bearer_type: BearerType::EmergencyBearer,
                qos: QosProfile {
                    qci: 69,
                    five_qi: 69,
                    priority_level: 0,
                    packet_delay_budget_ms: 1,
                    packet_error_rate_exp: -6,
                    max_data_burst_volume_bytes: 256,
                    guaranteed_bitrate_kbps: Some(256),
                    max_bitrate_kbps: Some(512),
                },
                tft_filters: vec![
                    "permit ip 10.0.0.0/8 any".to_string(),
                    "permit ip any 224.0.0.0/4".to_string(),
                ],
                gtp_teid: 0xE000_0006,
                uplink_bytes: 500,
                downlink_bytes: 1200,
            },
        ],
        serving_cell_pci: 600,
        slice: Some(v2x_slice),
        registration_timestamp: 1_700_500_000,
        idle: false,
    };
    let bytes = encode_to_vec(&session).expect("encode V2X session");
    let (decoded, _): (SubscriberSession, usize) =
        decode_from_slice(&bytes).expect("decode V2X session");
    assert_eq!(session, decoded);
    assert_eq!(decoded.bearer_contexts.len(), 2);
    let emergency = &decoded.bearer_contexts[1];
    assert_eq!(emergency.bearer_type, BearerType::EmergencyBearer);
    assert_eq!(emergency.qos.priority_level, 0);
    let decoded_slice = decoded.slice.expect("V2X slice should be present");
    assert_eq!(decoded_slice.slice_type, SliceType::V2X);
}
