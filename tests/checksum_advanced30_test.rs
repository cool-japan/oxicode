//! Advanced checksum tests for OxiCode — 5G telecom network management domain.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced30_test

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types: 5G telecom network management
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FrequencyBand {
    N1,
    N3,
    N5,
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
enum SubcarrierSpacing {
    Scs15kHz,
    Scs30kHz,
    Scs60kHz,
    Scs120kHz,
    Scs240kHz,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SliceType {
    EmBB,
    UrLLC,
    MIoT,
    V2X,
    Custom(u32),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HandoverTrigger {
    A1Event,
    A2Event,
    A3Event,
    A5Event,
    B1Event,
    B2Event,
    RLF,
    LoadBalancing,
    CoverageOptimization,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RicActionType {
    TrafficSteeringUpdate,
    QoSOptimization,
    LoadBalancing,
    InterferenceManagement,
    MassiveMimoBeamAdjust,
    EnergyEfficiency,
    SliceAdmissionControl,
    AnomalyMitigation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SONActionType {
    AutoNeighborRelation,
    PhysicalCellIdOptimization,
    MobilityRobustness,
    MobilityLoadBalancing,
    RachOptimization,
    CoverageCapacityOptimization,
    EnergyAutoSaving,
    CellOutageCompensation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CoreNfStatus {
    Active,
    Standby,
    Degraded,
    Overloaded,
    Restarting,
    Decommissioned,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CoreNfType {
    AMF,
    SMF,
    UPF,
    NRF,
    NSSF,
    AUSF,
    UDM,
    PCF,
    NEF,
    NWDAF,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InterferenceMitigationType {
    ICIC,
    #[allow(non_camel_case_types)]
    eICIC,
    FeICIC,
    CoMP,
    CBF,
    NOMA,
    FullDuplexSelfInterference,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GnbCellSiteConfig {
    gnb_id: u64,
    cell_id: u32,
    physical_cell_id: u16,
    tracking_area_code: u32,
    frequency_band: FrequencyBand,
    subcarrier_spacing: SubcarrierSpacing,
    bandwidth_mhz: u16,
    tx_power_dbm: f32,
    antenna_height_m: f32,
    latitude: f64,
    longitude: f64,
    max_ue_capacity: u32,
    active_ue_count: u32,
    sector_count: u8,
    is_mmwave: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BeamManagementRecord {
    gnb_id: u64,
    cell_id: u32,
    beam_index: u16,
    ssb_index: u8,
    azimuth_deg: f32,
    elevation_deg: f32,
    beam_width_deg: f32,
    rsrp_dbm: f32,
    sinr_db: f32,
    csi_rs_resource_id: u16,
    ue_associations: Vec<u64>,
    sweep_period_ms: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkSlicingProfile {
    nssai_sst: u8,
    nssai_sd: u32,
    slice_type: SliceType,
    slice_name: String,
    max_bitrate_mbps: u32,
    guaranteed_bitrate_mbps: u32,
    max_latency_ms: f32,
    max_jitter_ms: f32,
    max_packet_loss_ppm: u32,
    priority_level: u8,
    isolation_level: u8,
    max_ue_count: u32,
    active_ue_count: u32,
    upf_instance_ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HandoverEvent {
    event_id: u64,
    timestamp_ns: u64,
    ue_id: u64,
    source_gnb_id: u64,
    source_cell_id: u32,
    target_gnb_id: u64,
    target_cell_id: u32,
    trigger: HandoverTrigger,
    preparation_time_ms: u32,
    execution_time_ms: u32,
    success: bool,
    rsrp_source_dbm: f32,
    rsrp_target_dbm: f32,
    sinr_source_db: f32,
    ping_pong_detected: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RicAction {
    action_id: u64,
    timestamp_ns: u64,
    action_type: RicActionType,
    xapp_id: String,
    target_gnb_ids: Vec<u64>,
    parameters: Vec<f32>,
    predicted_gain_pct: f32,
    actual_gain_pct: Option<f32>,
    rollback: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectrumAllocation {
    allocation_id: u64,
    band: FrequencyBand,
    center_freq_khz: u64,
    bandwidth_khz: u32,
    subcarrier_spacing: SubcarrierSpacing,
    num_prbs: u16,
    guard_band_khz: u16,
    dss_enabled: bool,
    lte_share_pct: u8,
    nr_share_pct: u8,
    allocated_slices: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MimoAntennaConfig {
    gnb_id: u64,
    cell_id: u32,
    num_tx_ports: u16,
    num_rx_ports: u16,
    antenna_panel_count: u8,
    layers_per_panel: u8,
    max_rank: u8,
    codebook_type: u8,
    beamforming_weights: Vec<f32>,
    polarization_type: u8,
    element_spacing_lambda: f32,
    digital_tilt_deg: f32,
    mechanical_tilt_deg: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QosFlowParameters {
    qfi: u8,
    five_qi: u16,
    arp_priority: u8,
    arp_preemption_capability: bool,
    arp_preemption_vulnerability: bool,
    max_bitrate_ul_kbps: u64,
    max_bitrate_dl_kbps: u64,
    guaranteed_bitrate_ul_kbps: u64,
    guaranteed_bitrate_dl_kbps: u64,
    packet_delay_budget_ms: u16,
    packet_error_rate_exponent: u8,
    averaging_window_ms: u32,
    reflective_qos: bool,
    notification_control: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UeCapabilityReport {
    ue_id: u64,
    imsi_hash: u64,
    supported_bands: Vec<FrequencyBand>,
    max_mimo_layers_dl: u8,
    max_mimo_layers_ul: u8,
    carrier_aggregation_max_cc: u8,
    supported_scs: Vec<SubcarrierSpacing>,
    max_bandwidth_mhz: u16,
    dual_connectivity: bool,
    nr_dc: bool,
    en_dc: bool,
    ue_category_dl: u8,
    ue_category_ul: u8,
    supports_slicing: bool,
    supports_v2x: bool,
    battery_level_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoreNfStatusReport {
    nf_instance_id: u64,
    nf_type: CoreNfType,
    status: CoreNfStatus,
    cpu_load_pct: f32,
    memory_used_mb: u32,
    memory_total_mb: u32,
    active_sessions: u64,
    throughput_rps: u64,
    avg_latency_us: u32,
    p99_latency_us: u32,
    error_rate_ppm: u32,
    uptime_seconds: u64,
    plmn_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdgeComputingNode {
    node_id: u64,
    location_name: String,
    latitude: f64,
    longitude: f64,
    connected_gnb_ids: Vec<u64>,
    total_vcpus: u32,
    used_vcpus: u32,
    total_memory_gb: u32,
    used_memory_gb: u32,
    total_gpu_units: u16,
    used_gpu_units: u16,
    avg_rtt_to_ue_ms: f32,
    hosted_app_ids: Vec<u64>,
    energy_consumption_watts: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransportCapacity {
    link_id: u64,
    source_node_id: u64,
    destination_node_id: u64,
    is_fronthaul: bool,
    max_bandwidth_gbps: f32,
    used_bandwidth_gbps: f32,
    latency_us: u32,
    jitter_us: u16,
    packet_loss_ppm: u16,
    ecpri_enabled: bool,
    oran_compliant: bool,
    fiber_length_km: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CarrierAggregationCombo {
    combo_id: u32,
    primary_band: FrequencyBand,
    secondary_bands: Vec<FrequencyBand>,
    total_bandwidth_mhz: u16,
    max_dl_throughput_mbps: u32,
    max_ul_throughput_mbps: u32,
    supported_ue_count: u32,
    active_ue_count: u32,
    cross_carrier_scheduling: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InterferenceMitigation {
    event_id: u64,
    timestamp_ns: u64,
    gnb_id: u64,
    cell_id: u32,
    mitigation_type: InterferenceMitigationType,
    victim_prb_indices: Vec<u16>,
    aggressor_cell_ids: Vec<u32>,
    sinr_before_db: f32,
    sinr_after_db: f32,
    throughput_gain_pct: f32,
    power_reduction_db: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SONAction {
    action_id: u64,
    timestamp_ns: u64,
    action_type: SONActionType,
    affected_gnb_ids: Vec<u64>,
    affected_cell_ids: Vec<u32>,
    old_parameters: Vec<f32>,
    new_parameters: Vec<f32>,
    expected_improvement_pct: f32,
    measured_improvement_pct: Option<f32>,
    rollback_triggered: bool,
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

/// Test 1: gNB cell site configuration roundtrip
#[test]
fn test_gnb_cell_site_config_roundtrip() {
    let config = GnbCellSiteConfig {
        gnb_id: 0x0010_ABCD_0001,
        cell_id: 36,
        physical_cell_id: 504,
        tracking_area_code: 12345,
        frequency_band: FrequencyBand::N78,
        subcarrier_spacing: SubcarrierSpacing::Scs30kHz,
        bandwidth_mhz: 100,
        tx_power_dbm: 46.0,
        antenna_height_m: 30.5,
        latitude: 59.437_222,
        longitude: 24.745_018,
        max_ue_capacity: 1200,
        active_ue_count: 847,
        sector_count: 3,
        is_mmwave: false,
    };
    let encoded = encode_with_checksum(&config).expect("encode GnbCellSiteConfig");
    let (decoded, consumed): (GnbCellSiteConfig, _) =
        decode_with_checksum(&encoded).expect("decode GnbCellSiteConfig");
    assert_eq!(config, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 2: mmWave gNB with high frequency band
#[test]
fn test_mmwave_gnb_config_roundtrip() {
    let config = GnbCellSiteConfig {
        gnb_id: 0x0020_BEEF_0002,
        cell_id: 1,
        physical_cell_id: 1007,
        tracking_area_code: 55000,
        frequency_band: FrequencyBand::N257,
        subcarrier_spacing: SubcarrierSpacing::Scs120kHz,
        bandwidth_mhz: 400,
        tx_power_dbm: 35.0,
        antenna_height_m: 8.2,
        latitude: 35.689_487,
        longitude: 139.691_711,
        max_ue_capacity: 200,
        active_ue_count: 42,
        sector_count: 1,
        is_mmwave: true,
    };
    let encoded = encode_with_checksum(&config).expect("encode mmWave GnbCellSiteConfig");
    let (decoded, _): (GnbCellSiteConfig, _) =
        decode_with_checksum(&encoded).expect("decode mmWave GnbCellSiteConfig");
    assert_eq!(config, decoded);
}

/// Test 3: beam management record roundtrip
#[test]
fn test_beam_management_record_roundtrip() {
    let record = BeamManagementRecord {
        gnb_id: 0x0030_CAFE_0003,
        cell_id: 12,
        beam_index: 7,
        ssb_index: 3,
        azimuth_deg: 120.5,
        elevation_deg: -8.0,
        beam_width_deg: 15.0,
        rsrp_dbm: -85.3,
        sinr_db: 18.7,
        csi_rs_resource_id: 64,
        ue_associations: vec![1001, 1002, 1003, 1004, 1005],
        sweep_period_ms: 20,
    };
    let encoded = encode_with_checksum(&record).expect("encode BeamManagementRecord");
    let (decoded, consumed): (BeamManagementRecord, _) =
        decode_with_checksum(&encoded).expect("decode BeamManagementRecord");
    assert_eq!(record, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 4: network slicing profile for eMBB
#[test]
fn test_network_slicing_profile_embb_roundtrip() {
    let profile = NetworkSlicingProfile {
        nssai_sst: 1,
        nssai_sd: 0x00_0001,
        slice_type: SliceType::EmBB,
        slice_name: String::from("enterprise-embb-gold"),
        max_bitrate_mbps: 10_000,
        guaranteed_bitrate_mbps: 5_000,
        max_latency_ms: 10.0,
        max_jitter_ms: 2.0,
        max_packet_loss_ppm: 100,
        priority_level: 1,
        isolation_level: 3,
        max_ue_count: 50_000,
        active_ue_count: 32_187,
        upf_instance_ids: vec![9001, 9002, 9003],
    };
    let encoded = encode_with_checksum(&profile).expect("encode eMBB slice profile");
    let (decoded, _): (NetworkSlicingProfile, _) =
        decode_with_checksum(&encoded).expect("decode eMBB slice profile");
    assert_eq!(profile, decoded);
}

/// Test 5: URLLC network slice profile
#[test]
fn test_network_slicing_profile_urllc_roundtrip() {
    let profile = NetworkSlicingProfile {
        nssai_sst: 2,
        nssai_sd: 0x00_0010,
        slice_type: SliceType::UrLLC,
        slice_name: String::from("factory-urllc-critical"),
        max_bitrate_mbps: 500,
        guaranteed_bitrate_mbps: 200,
        max_latency_ms: 1.0,
        max_jitter_ms: 0.1,
        max_packet_loss_ppm: 1,
        priority_level: 0,
        isolation_level: 5,
        max_ue_count: 2000,
        active_ue_count: 812,
        upf_instance_ids: vec![8001, 8002],
    };
    let encoded = encode_with_checksum(&profile).expect("encode URLLC slice profile");
    let (decoded, consumed): (NetworkSlicingProfile, _) =
        decode_with_checksum(&encoded).expect("decode URLLC slice profile");
    assert_eq!(profile, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 6: handover event with A3 trigger
#[test]
fn test_handover_event_a3_roundtrip() {
    let event = HandoverEvent {
        event_id: 0xDEAD_0001,
        timestamp_ns: 1_700_000_000_000_000_000,
        ue_id: 55_001,
        source_gnb_id: 0x0010_ABCD_0001,
        source_cell_id: 36,
        target_gnb_id: 0x0010_ABCD_0002,
        target_cell_id: 37,
        trigger: HandoverTrigger::A3Event,
        preparation_time_ms: 45,
        execution_time_ms: 12,
        success: true,
        rsrp_source_dbm: -98.5,
        rsrp_target_dbm: -88.2,
        sinr_source_db: 3.1,
        ping_pong_detected: false,
    };
    let encoded = encode_with_checksum(&event).expect("encode HandoverEvent A3");
    let (decoded, _): (HandoverEvent, _) =
        decode_with_checksum(&encoded).expect("decode HandoverEvent A3");
    assert_eq!(event, decoded);
}

/// Test 7: failed handover due to RLF
#[test]
fn test_handover_event_rlf_roundtrip() {
    let event = HandoverEvent {
        event_id: 0xDEAD_0002,
        timestamp_ns: 1_700_000_001_000_000_000,
        ue_id: 55_099,
        source_gnb_id: 0x0050_1234_0010,
        source_cell_id: 100,
        target_gnb_id: 0x0050_1234_0011,
        target_cell_id: 101,
        trigger: HandoverTrigger::RLF,
        preparation_time_ms: 0,
        execution_time_ms: 0,
        success: false,
        rsrp_source_dbm: -120.0,
        rsrp_target_dbm: -110.5,
        sinr_source_db: -5.0,
        ping_pong_detected: false,
    };
    let encoded = encode_with_checksum(&event).expect("encode HandoverEvent RLF");
    let (decoded, consumed): (HandoverEvent, _) =
        decode_with_checksum(&encoded).expect("decode HandoverEvent RLF");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 8: RIC action for traffic steering
#[test]
fn test_ric_action_traffic_steering_roundtrip() {
    let action = RicAction {
        action_id: 7001,
        timestamp_ns: 1_700_500_000_000_000,
        action_type: RicActionType::TrafficSteeringUpdate,
        xapp_id: String::from("xapp-ts-v2.3.1"),
        target_gnb_ids: vec![0x0010_ABCD_0001, 0x0010_ABCD_0002, 0x0010_ABCD_0003],
        parameters: vec![0.65, 0.20, 0.15, 2.5, 1.0],
        predicted_gain_pct: 12.5,
        actual_gain_pct: Some(10.8),
        rollback: false,
    };
    let encoded = encode_with_checksum(&action).expect("encode RicAction");
    let (decoded, _): (RicAction, _) = decode_with_checksum(&encoded).expect("decode RicAction");
    assert_eq!(action, decoded);
}

/// Test 9: spectrum allocation with DSS enabled
#[test]
fn test_spectrum_allocation_dss_roundtrip() {
    let alloc = SpectrumAllocation {
        allocation_id: 4001,
        band: FrequencyBand::N3,
        center_freq_khz: 1_842_500,
        bandwidth_khz: 20_000,
        subcarrier_spacing: SubcarrierSpacing::Scs15kHz,
        num_prbs: 106,
        guard_band_khz: 845,
        dss_enabled: true,
        lte_share_pct: 60,
        nr_share_pct: 40,
        allocated_slices: vec![1, 2, 3],
    };
    let encoded = encode_with_checksum(&alloc).expect("encode SpectrumAllocation DSS");
    let (decoded, consumed): (SpectrumAllocation, _) =
        decode_with_checksum(&encoded).expect("decode SpectrumAllocation DSS");
    assert_eq!(alloc, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 10: massive MIMO antenna configuration
#[test]
fn test_mimo_antenna_config_massive_roundtrip() {
    let mimo = MimoAntennaConfig {
        gnb_id: 0x0060_DEAD_BEEF,
        cell_id: 42,
        num_tx_ports: 64,
        num_rx_ports: 64,
        antenna_panel_count: 2,
        layers_per_panel: 16,
        max_rank: 8,
        codebook_type: 2,
        beamforming_weights: (0..128).map(|i| (i as f32) * 0.01).collect(),
        polarization_type: 1,
        element_spacing_lambda: 0.5,
        digital_tilt_deg: 6.0,
        mechanical_tilt_deg: 4.5,
    };
    let encoded = encode_with_checksum(&mimo).expect("encode MimoAntennaConfig");
    let (decoded, _): (MimoAntennaConfig, _) =
        decode_with_checksum(&encoded).expect("decode MimoAntennaConfig");
    assert_eq!(mimo, decoded);
}

/// Test 11: QoS flow parameters for voice (5QI=1)
#[test]
fn test_qos_flow_voice_roundtrip() {
    let qos = QosFlowParameters {
        qfi: 1,
        five_qi: 1,
        arp_priority: 2,
        arp_preemption_capability: true,
        arp_preemption_vulnerability: false,
        max_bitrate_ul_kbps: 128,
        max_bitrate_dl_kbps: 128,
        guaranteed_bitrate_ul_kbps: 64,
        guaranteed_bitrate_dl_kbps: 64,
        packet_delay_budget_ms: 100,
        packet_error_rate_exponent: 2,
        averaging_window_ms: 2000,
        reflective_qos: false,
        notification_control: true,
    };
    let encoded = encode_with_checksum(&qos).expect("encode QoS voice flow");
    let (decoded, consumed): (QosFlowParameters, _) =
        decode_with_checksum(&encoded).expect("decode QoS voice flow");
    assert_eq!(qos, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 12: UE capability report with multiple bands and CA support
#[test]
fn test_ue_capability_report_roundtrip() {
    let report = UeCapabilityReport {
        ue_id: 99_001,
        imsi_hash: 0xABCD_EF01_2345_6789,
        supported_bands: vec![
            FrequencyBand::N1,
            FrequencyBand::N3,
            FrequencyBand::N7,
            FrequencyBand::N28,
            FrequencyBand::N78,
        ],
        max_mimo_layers_dl: 4,
        max_mimo_layers_ul: 2,
        carrier_aggregation_max_cc: 5,
        supported_scs: vec![
            SubcarrierSpacing::Scs15kHz,
            SubcarrierSpacing::Scs30kHz,
            SubcarrierSpacing::Scs60kHz,
        ],
        max_bandwidth_mhz: 100,
        dual_connectivity: true,
        nr_dc: true,
        en_dc: true,
        ue_category_dl: 20,
        ue_category_ul: 13,
        supports_slicing: true,
        supports_v2x: false,
        battery_level_pct: 78,
    };
    let encoded = encode_with_checksum(&report).expect("encode UeCapabilityReport");
    let (decoded, _): (UeCapabilityReport, _) =
        decode_with_checksum(&encoded).expect("decode UeCapabilityReport");
    assert_eq!(report, decoded);
}

/// Test 13: core network function status report for AMF
#[test]
fn test_core_nf_status_amf_roundtrip() {
    let status = CoreNfStatusReport {
        nf_instance_id: 0x0001_0000_0000_0001,
        nf_type: CoreNfType::AMF,
        status: CoreNfStatus::Active,
        cpu_load_pct: 42.3,
        memory_used_mb: 8192,
        memory_total_mb: 16384,
        active_sessions: 1_250_000,
        throughput_rps: 85_000,
        avg_latency_us: 1200,
        p99_latency_us: 5800,
        error_rate_ppm: 15,
        uptime_seconds: 2_592_000,
        plmn_id: String::from("24801"),
    };
    let encoded = encode_with_checksum(&status).expect("encode CoreNfStatus AMF");
    let (decoded, consumed): (CoreNfStatusReport, _) =
        decode_with_checksum(&encoded).expect("decode CoreNfStatus AMF");
    assert_eq!(status, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 14: edge computing node placement
#[test]
fn test_edge_computing_node_roundtrip() {
    let node = EdgeComputingNode {
        node_id: 0xE001,
        location_name: String::from("Helsinki-Central-MEC-01"),
        latitude: 60.170_833,
        longitude: 24.941_389,
        connected_gnb_ids: vec![
            0x0010_0001,
            0x0010_0002,
            0x0010_0003,
            0x0010_0004,
            0x0010_0005,
        ],
        total_vcpus: 256,
        used_vcpus: 180,
        total_memory_gb: 1024,
        used_memory_gb: 720,
        total_gpu_units: 8,
        used_gpu_units: 5,
        avg_rtt_to_ue_ms: 4.2,
        hosted_app_ids: vec![50001, 50002, 50003, 50004],
        energy_consumption_watts: 12_500.0,
    };
    let encoded = encode_with_checksum(&node).expect("encode EdgeComputingNode");
    let (decoded, _): (EdgeComputingNode, _) =
        decode_with_checksum(&encoded).expect("decode EdgeComputingNode");
    assert_eq!(node, decoded);
}

/// Test 15: fronthaul transport capacity link
#[test]
fn test_transport_capacity_fronthaul_roundtrip() {
    let link = TransportCapacity {
        link_id: 0xF001,
        source_node_id: 0x0010_0001,
        destination_node_id: 0xE001,
        is_fronthaul: true,
        max_bandwidth_gbps: 25.0,
        used_bandwidth_gbps: 18.7,
        latency_us: 100,
        jitter_us: 5,
        packet_loss_ppm: 0,
        ecpri_enabled: true,
        oran_compliant: true,
        fiber_length_km: 2.3,
    };
    let encoded = encode_with_checksum(&link).expect("encode TransportCapacity fronthaul");
    let (decoded, consumed): (TransportCapacity, _) =
        decode_with_checksum(&encoded).expect("decode TransportCapacity fronthaul");
    assert_eq!(link, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 16: carrier aggregation combo with three bands
#[test]
fn test_carrier_aggregation_combo_roundtrip() {
    let combo = CarrierAggregationCombo {
        combo_id: 1001,
        primary_band: FrequencyBand::N78,
        secondary_bands: vec![FrequencyBand::N3, FrequencyBand::N1],
        total_bandwidth_mhz: 160,
        max_dl_throughput_mbps: 3200,
        max_ul_throughput_mbps: 800,
        supported_ue_count: 15000,
        active_ue_count: 8923,
        cross_carrier_scheduling: true,
    };
    let encoded = encode_with_checksum(&combo).expect("encode CarrierAggregationCombo");
    let (decoded, _): (CarrierAggregationCombo, _) =
        decode_with_checksum(&encoded).expect("decode CarrierAggregationCombo");
    assert_eq!(combo, decoded);
}

/// Test 17: interference mitigation CoMP action
#[test]
fn test_interference_mitigation_comp_roundtrip() {
    let mitigation = InterferenceMitigation {
        event_id: 0xBEEF_0001,
        timestamp_ns: 1_700_600_000_000_000,
        gnb_id: 0x0010_ABCD_0001,
        cell_id: 36,
        mitigation_type: InterferenceMitigationType::CoMP,
        victim_prb_indices: vec![10, 11, 12, 50, 51, 52, 90, 91],
        aggressor_cell_ids: vec![37, 38],
        sinr_before_db: 2.1,
        sinr_after_db: 8.7,
        throughput_gain_pct: 35.2,
        power_reduction_db: 3.0,
    };
    let encoded = encode_with_checksum(&mitigation).expect("encode InterferenceMitigation");
    let (decoded, consumed): (InterferenceMitigation, _) =
        decode_with_checksum(&encoded).expect("decode InterferenceMitigation");
    assert_eq!(mitigation, decoded);
    assert_eq!(consumed, encoded.len());
}

/// Test 18: SON action for mobility robustness optimization
#[test]
fn test_son_action_mro_roundtrip() {
    let action = SONAction {
        action_id: 6001,
        timestamp_ns: 1_700_700_000_000_000,
        action_type: SONActionType::MobilityRobustness,
        affected_gnb_ids: vec![0x0010_ABCD_0001, 0x0010_ABCD_0002],
        affected_cell_ids: vec![36, 37, 38, 39],
        old_parameters: vec![3.0, 5.0, 480.0],
        new_parameters: vec![4.0, 6.0, 640.0],
        expected_improvement_pct: 15.0,
        measured_improvement_pct: Some(11.3),
        rollback_triggered: false,
    };
    let encoded = encode_with_checksum(&action).expect("encode SON MRO action");
    let (decoded, _): (SONAction, _) =
        decode_with_checksum(&encoded).expect("decode SON MRO action");
    assert_eq!(action, decoded);
}

/// Test 19: corruption detection — flip a byte in gnb config payload
#[test]
fn test_corruption_detection_gnb_config() {
    let config = GnbCellSiteConfig {
        gnb_id: 0xCCCC_0001,
        cell_id: 99,
        physical_cell_id: 255,
        tracking_area_code: 77777,
        frequency_band: FrequencyBand::N41,
        subcarrier_spacing: SubcarrierSpacing::Scs30kHz,
        bandwidth_mhz: 40,
        tx_power_dbm: 43.0,
        antenna_height_m: 25.0,
        latitude: 48.856_613,
        longitude: 2.352_222,
        max_ue_capacity: 800,
        active_ue_count: 500,
        sector_count: 3,
        is_mmwave: false,
    };
    let mut encoded = encode_with_checksum(&config).expect("encode for corruption test");
    let mid = encoded.len() / 2;
    encoded[mid] ^= 0xFF;
    let result: Result<(GnbCellSiteConfig, usize), _> = decode_with_checksum(&encoded);
    assert!(
        result.is_err(),
        "corrupted gnb config must fail checksum validation"
    );
}

/// Test 20: corruption detection — truncated handover event
#[test]
fn test_corruption_detection_truncated_handover() {
    let event = HandoverEvent {
        event_id: 0xDDDD_0001,
        timestamp_ns: 1_700_800_000_000_000,
        ue_id: 77_001,
        source_gnb_id: 0x0020_0001,
        source_cell_id: 1,
        target_gnb_id: 0x0020_0002,
        target_cell_id: 2,
        trigger: HandoverTrigger::A5Event,
        preparation_time_ms: 30,
        execution_time_ms: 8,
        success: true,
        rsrp_source_dbm: -95.0,
        rsrp_target_dbm: -80.0,
        sinr_source_db: 5.0,
        ping_pong_detected: false,
    };
    let encoded = encode_with_checksum(&event).expect("encode for truncation test");
    let truncated = &encoded[..encoded.len().saturating_sub(4)];
    let result: Result<(HandoverEvent, usize), _> = decode_with_checksum(truncated);
    assert!(
        result.is_err(),
        "truncated handover event must fail checksum validation"
    );
}

/// Test 21: corruption detection — multi-byte corruption in edge node data
#[test]
fn test_corruption_detection_edge_node_multibyte() {
    let node = EdgeComputingNode {
        node_id: 0xEEEE_0001,
        location_name: String::from("Tokyo-Shibuya-MEC-03"),
        latitude: 35.658_034,
        longitude: 139.701_636,
        connected_gnb_ids: vec![0x3001, 0x3002, 0x3003],
        total_vcpus: 128,
        used_vcpus: 96,
        total_memory_gb: 512,
        used_memory_gb: 380,
        total_gpu_units: 4,
        used_gpu_units: 3,
        avg_rtt_to_ue_ms: 2.8,
        hosted_app_ids: vec![60001, 60002],
        energy_consumption_watts: 8800.0,
    };
    let mut encoded = encode_with_checksum(&node).expect("encode for multi-byte corruption test");
    let len = encoded.len();
    if len > 10 {
        for b in encoded[len - 10..len - 6].iter_mut() {
            *b ^= 0xAA;
        }
    }
    let result: Result<(EdgeComputingNode, usize), _> = decode_with_checksum(&encoded);
    assert!(
        result.is_err(),
        "multi-byte corrupted edge node must fail checksum validation"
    );
}

/// Test 22: encode_to_vec/decode_from_slice basic consistency check with checksum path
#[test]
fn test_basic_encode_decode_consistency_with_checksum() {
    let qos = QosFlowParameters {
        qfi: 5,
        five_qi: 9,
        arp_priority: 8,
        arp_preemption_capability: false,
        arp_preemption_vulnerability: true,
        max_bitrate_ul_kbps: 50_000,
        max_bitrate_dl_kbps: 100_000,
        guaranteed_bitrate_ul_kbps: 0,
        guaranteed_bitrate_dl_kbps: 0,
        packet_delay_budget_ms: 300,
        packet_error_rate_exponent: 6,
        averaging_window_ms: 0,
        reflective_qos: true,
        notification_control: false,
    };

    // plain encode/decode must match checksum encode/decode
    let plain_encoded = encode_to_vec(&qos).expect("plain encode QoS");
    let (plain_decoded, _): (QosFlowParameters, _) =
        decode_from_slice(&plain_encoded).expect("plain decode QoS");

    let checksum_encoded = encode_with_checksum(&qos).expect("checksum encode QoS");
    let (checksum_decoded, _): (QosFlowParameters, _) =
        decode_with_checksum(&checksum_encoded).expect("checksum decode QoS");

    assert_eq!(
        plain_decoded, checksum_decoded,
        "plain and checksum paths must produce equal values"
    );
    assert_eq!(qos, checksum_decoded);

    // checksum-encoded must be larger than plain-encoded due to header
    assert!(
        checksum_encoded.len() > plain_encoded.len(),
        "checksum-encoded buffer must be larger than plain-encoded buffer"
    );
}
